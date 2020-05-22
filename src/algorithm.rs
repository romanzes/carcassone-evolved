use crate::evolution::create_empty_board;
use crate::model::{Board, Cell, Pos};

#[derive(Clone)]
pub struct Algorithm {
    pub cells: Vec<Cell>,
    pub arranged_cells: Vec<Cell>,
}

impl Algorithm {
    pub fn new(cells: Vec<Cell>) -> Algorithm {
        let arranged_cells = rearrange_overlaps(&cells);
        Algorithm {
            cells,
            arranged_cells,
        }
    }
}

fn rearrange_overlaps(cells: &Vec<Cell>) -> Vec<Cell> {
    let mut board = create_empty_board();
    let mut cells = cells.clone();
    for index in 0..cells.len() {
        let cell = &cells[index];
        if board.cells[cell.pos.x][cell.pos.y].is_some() {
            let closest_free_pos = &find_closest_free_pos(&mut board, &cell.pos);
            cells[index] = Cell {
                pos: closest_free_pos.clone(),
                card: cell.card.clone(),
                card_side: cell.card_side.clone(),
            };
            board.cells[closest_free_pos.x][closest_free_pos.y] = Some(cells[index].clone());
        } else {
            board.cells[cell.pos.x][cell.pos.y] = Some(cell.clone());
        }
    }
    cells
}

fn find_closest_free_pos(board: &mut Board, pos: &Pos) -> Pos {
    let mut free_cell: Option<Pos> = None;
    let mut distance = 1;
    while free_cell.is_none() {
        free_cell = get_free_cell(board, &get_halo(board, pos, distance));
        distance += 1;
    }
    free_cell.unwrap()
}

fn get_free_cell(board: &Board, positions: &Vec<Pos>) -> Option<Pos> {
    positions
        .iter()
        .find(|pos| board.cells[pos.x][pos.y].is_none())
        .cloned()
}

fn get_halo(board: &Board, pos: &Pos, distance: usize) -> Vec<Pos> {
    let mut result: Vec<(i32, i32)> = vec![];
    for x in pos.x as i32 - distance as i32..pos.x as i32 + distance as i32 {
        result.push((x as i32, pos.y as i32 - distance as i32));
        result.push((x as i32, pos.y as i32 + distance as i32));
    }
    for y in pos.y as i32 - distance as i32..pos.y as i32 + distance as i32 {
        result.push((pos.x as i32 - distance as i32, y as i32));
        result.push((pos.x as i32 + distance as i32, y as i32));
    }
    result
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < board.width as i32 && *y < board.height as i32)
        .map(|(x, y)| Pos {
            x: x as usize,
            y: y as usize,
        })
        .collect()
}
