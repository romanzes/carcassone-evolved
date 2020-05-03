use std::collections::HashSet;
use crate::model::{Cell, Board, Pos, top_side, TerrainType, bottom_side, left_side, right_side};
use crate::evolution::create_empty_board;
use crate::algorithm::Algorithm;

pub fn evaluate_algorithm(algorithm: &Algorithm) -> usize {
    let original_board = fill_board(&algorithm.arranged_cells);
    let mut board = fill_board(&algorithm.arranged_cells);
    let clusters = extract_clusters(&mut board);
    let clusters = clusters.len() - 1;
    let unclosed_town_parts = count_unclosed_town_parts(&original_board);
    let non_matching_tiles = count_non_matching_tiles(&original_board);
    clusters + unclosed_town_parts + non_matching_tiles
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
            if let Some(cell) = board.cells[x][y] {
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
            if let (Some(cell1), Some(cell2)) = (board.cells[x][y], board.cells[x + 1][y]) {
                if cell1.right() != cell2.left() { result += 1; }
            }
        }
    }
    for x in 0..board.width {
        for y in 0..board.height - 1 {
            if let (Some(cell1), Some(cell2)) = (board.cells[x][y], board.cells[x][y + 1]) {
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
                leaves.push(board.cells[neighbour.x][neighbour.y].unwrap());
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