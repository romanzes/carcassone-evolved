use std::collections::HashSet;
use crate::model::{Cell, Board, Pos, top_side, TerrainType, bottom_side, left_side, right_side, Struct, CardSide};
use crate::evolution::create_empty_board;
use crate::algorithm::Algorithm;

pub fn evaluate_algorithm(algorithm: &Algorithm) -> usize {
    let original_board = fill_board(&algorithm.arranged_cells);
    let mut board = fill_board(&algorithm.arranged_cells);
    let clusters = extract_clusters(&mut board);
    let cluster_count = clusters.len() - 1;
    let unclosed_town_parts = count_unclosed_town_parts(&original_board);
    let non_matching_tiles = count_non_matching_tiles(&original_board);
    let town_count = extract_towns(&original_board).len();
    cluster_count + unclosed_town_parts + non_matching_tiles + town_count
}

pub fn fill_board(cells: &Vec<Cell>) -> Board {
    let mut board = create_empty_board();
    cells.iter().for_each(|cell| {
        board.cells[cell.pos.x][cell.pos.y] = Some(cell.clone());
    });
    board
}

fn extract_clusters(board: &mut Board) -> Vec<Cluster> {
    let mut result = vec![];
    for x in 0..board.width {
        for y in 0..board.height {
            if let Some(cell) = board.cells[x][y].clone() {
                let mut cluster_cells = vec![];
                let mut cells = vec![cell];
                while cells.len() != 0 {
                    cluster_cells.extend(cells.clone());
                    remove_cells(board, &cells);
                    cells = get_leaves(board, &cells);
                }
                result.push(Cluster { cells: cluster_cells });
            }
        }
    }
    return result;
}

fn count_non_matching_tiles(board: &Board) -> usize {
    let mut result = 0;
    for x in 0..board.width - 1 {
        for y in 0..board.height {
            if let (Some(cell1), Some(cell2)) = (&board.cells[x][y], &board.cells[x + 1][y]) {
                if cell1.right() != cell2.left() { result += 1; }
            }
        }
    }
    for x in 0..board.width {
        for y in 0..board.height - 1 {
            if let (Some(cell1), Some(cell2)) = (&board.cells[x][y], &board.cells[x][y + 1]) {
                if cell1.bottom() != cell2.top() { result += 1; }
            }
        }
    }
    result
}

fn get_leaves(board: &Board, cells: &Vec<Cell>) -> Vec<Cell> {
    let mut checked_leaves: HashSet<Pos> = HashSet::new();
    let mut leaves = vec![];
    cells.iter().for_each(|cell| {
        get_neighbours(board, &cell.pos).iter().for_each(|neighbour| {
            if !checked_leaves.contains(neighbour) {
                checked_leaves.insert(neighbour.clone());
                leaves.push(board.cells[neighbour.x][neighbour.y].clone().unwrap());
            }
        })
    });
    leaves
}

fn get_neighbours(board: &Board, pos: &Pos) -> Vec<Pos> {
    vec![
        (pos.x as i32 - 1, pos.y as i32),
        (pos.x as i32 + 1, pos.y as i32),
        (pos.x as i32, pos.y as i32 - 1),
        (pos.x as i32, pos.y as i32 + 1),
    ]
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < board.width as i32 && *y < board.height as i32 && board.cells[*x as usize][*y as usize].is_some())
        .map(|(x, y)| Pos { x: x as usize, y: y as usize })
        .collect()
}

fn remove_cells(board: &mut Board, cells: &Vec<Cell>) {
    cells.iter().for_each(|cell| board.cells[cell.pos.x][cell.pos.y] = None);
}

fn count_unclosed_town_parts(board: &Board) -> usize {
    let mut result = 0;
    for x in 0..board.width {
        if top_side(&board.cells[x][0]) == TerrainType::TOWN {
            result += 1;
        }
        if bottom_side(&board.cells[x][board.height - 1]) == TerrainType::TOWN {
            result += 1;
        }
    }
    for y in 0..board.height {
        if left_side(&board.cells[0][y]) == TerrainType::TOWN {
            result += 1;
        }
        if right_side(&board.cells[board.width - 1][y]) == TerrainType::TOWN {
            result += 1;
        }
    }
    for x in 0..board.width - 1 {
        for y in 0..board.height {
            if xor(right_side(&board.cells[x][y]) == TerrainType::TOWN, left_side(&board.cells[x + 1][y]) == TerrainType::TOWN) {
                result += 1;
            }
        }
    }
    for x in 0..board.width {
        for y in 0..board.height - 1 {
            if xor(bottom_side(&board.cells[x][y]) == TerrainType::TOWN, top_side(&board.cells[x][y + 1]) == TerrainType::TOWN) {
                result += 1;
            }
        }
    }
    result
}

fn xor(a: bool, b: bool) -> bool {
    (a && !b) || (b && !a)
}

struct Cluster {
    cells: Vec<Cell>,
}

struct TownCluster {
    town_tiles: Vec<TownTile>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct TownTile {
    town: Struct,
    tile: Cell,
}

fn extract_towns(board: &Board) -> Vec<TownCluster> {
    let mut result = vec![];
    let mut checked_town_tiles = HashSet::new();
    for x in 0..board.width {
        for y in 0..board.height {
            if let Some(cell) = &board.cells[x][y] {
                for struc in &cell.card.structs {
                    if struc.terrain == TerrainType::TOWN {
                        let town_tile = TownTile { town: struc.clone(), tile: cell.clone() };
                        if !checked_town_tiles.contains(&town_tile) {
                            checked_town_tiles.insert(town_tile.clone());
                            let mut all_town_leaves = vec![town_tile.clone()];
                            let mut town_leaves = vec![town_tile];
                            let mut result_found = false;
                            while !result_found {
                                town_leaves = find_town_leaves(board, &town_leaves, &checked_town_tiles);
                                for leaf in &town_leaves {
                                    checked_town_tiles.insert(leaf.clone());
                                }
                                all_town_leaves.append(&mut town_leaves);
                                result_found = town_leaves.is_empty();
                            }
                            result.push(TownCluster { town_tiles: all_town_leaves });
                        }
                    }
                }
            }
        }
    }
    result
}

fn find_town_leaves(
    board: &Board,
    tiles: &Vec<TownTile>,
    checked_tiles: &HashSet<TownTile>
) -> Vec<TownTile> {
    let mut result = vec![];
    for tile in tiles {
        for side in &tile.town.sides {
            let geom_side = get_geom_side(&side, &tile.tile.card_side);
            let neighboring_cell = get_neighboring_cell(board, &tile.tile, &geom_side);
            if let Some(neighboring_cell) = neighboring_cell {
                let neighboring_side = geom_side.get_opposite();
                let neighboring_terrain = neighboring_cell.get_side(&neighboring_side);
                if neighboring_terrain == TerrainType::TOWN {
                    let struc = get_struct(&neighboring_cell, &neighboring_side).unwrap();
                    let town_tile = TownTile { town: struc, tile: neighboring_cell.clone() };
                    if !checked_tiles.contains(&town_tile) {
                        result.push(town_tile);
                    }
                }
            }
        }
    }
    result
}

fn get_geom_side(side: &CardSide, tile_side: &CardSide) -> CardSide {
    match side {
        CardSide::LEFT => {
            match tile_side {
                CardSide::LEFT => CardSide::LEFT,
                CardSide::TOP => CardSide::BOTTOM,
                CardSide::RIGHT => CardSide::RIGHT,
                CardSide::BOTTOM => CardSide::TOP,
            }
        },
        CardSide::TOP => {
            match tile_side {
                CardSide::LEFT => CardSide::TOP,
                CardSide::TOP => CardSide::LEFT,
                CardSide::RIGHT => CardSide::BOTTOM,
                CardSide::BOTTOM => CardSide::RIGHT,
            }
        },
        CardSide::RIGHT => {
            match tile_side {
                CardSide::LEFT => CardSide::RIGHT,
                CardSide::TOP => CardSide::TOP,
                CardSide::RIGHT => CardSide::LEFT,
                CardSide::BOTTOM => CardSide::BOTTOM,
            }
        },
        CardSide::BOTTOM => {
            match tile_side {
                CardSide::LEFT => CardSide::BOTTOM,
                CardSide::TOP => CardSide::RIGHT,
                CardSide::RIGHT => CardSide::TOP,
                CardSide::BOTTOM => CardSide::LEFT,
            }
        },
    }
}

fn get_neighboring_cell(board: &Board, cell: &Cell, side: &CardSide) -> Option<Cell> {
    match side {
        CardSide::LEFT => {
            if cell.pos.x > 0 {
                board.cells[cell.pos.x - 1][cell.pos.y].clone()
            } else {
                None
            }
        },
        CardSide::TOP => {
            if cell.pos.y > 0 {
                board.cells[cell.pos.x][cell.pos.y - 1].clone()
            } else {
                None
            }
        },
        CardSide::RIGHT => {
            if cell.pos.x < board.width - 1 {
                board.cells[cell.pos.x + 1][cell.pos.y].clone()
            } else {
                None
            }
        },
        CardSide::BOTTOM => {
            if cell.pos.y < board.height - 1 {
                board.cells[cell.pos.x][cell.pos.y + 1].clone()
            } else {
                None
            }
        },
    }
}

fn get_struct(cell: &Cell, cell_side: &CardSide) -> Option<Struct> {
    for struc in &cell.card.structs {
        for side in &struc.sides {
            let geom_side = get_geom_side(&side, &cell.card_side);
            if &geom_side == cell_side {
                return Some(struc.clone());
            }
        }
    }
    None
}
