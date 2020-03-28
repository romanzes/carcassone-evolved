use rand::Rng;
use TerrainType::*;
use std::collections::HashSet;

const CARDS: [Card; 20] = [
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
];

const FIELD_SIZE: usize = 30;
const POPULATION_SIZE: usize = 100;

fn main() {
    let population: Vec<Algorithm> = (0..POPULATION_SIZE).map(|_| generate_algorithm()).collect();
    population.iter().for_each(|algorithm| {
        println!("{}", evaluate_algorithm(algorithm));
    });
}

fn generate_algorithm() -> Algorithm {
    let mut rng = rand::thread_rng();
    let mut free_cells: Vec<Pos> = (0..FIELD_SIZE * FIELD_SIZE)
        .map(|index| Pos { x: index / FIELD_SIZE, y: index % FIELD_SIZE })
        .collect();
    let cells = (0..CARDS.len())
        .map(|card_id| {
            let index: usize = rng.gen_range(0, free_cells.len());
            let pos = free_cells[index];
            free_cells.remove(index);
            let card_side = rng.gen_range(0, 4);
            Cell { pos, card: CARDS[card_id], card_side }
        })
        .collect();
    Algorithm { cells }
}

fn evaluate_algorithm(algorithm: &Algorithm) -> usize {
    let mut board = Board { cells: [[None; FIELD_SIZE]; FIELD_SIZE] };
    algorithm.cells.iter().for_each(|cell| {
        board.cells[cell.pos.x][cell.pos.y] = Some(cell.clone());
    });
    let clusters = extract_clusters(&mut board);
    clusters.len()
}

fn extract_clusters(board: &mut Board) -> Vec<Cluster> {
    let mut result = vec![];
    for x in 0..FIELD_SIZE {
        for y in 0..FIELD_SIZE {
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

fn get_leaves(board: &Board, cells: &Vec<Cell>) -> Vec<Cell> {
    let mut checked_leaves: HashSet<Pos> = HashSet::new();
    let mut leaves = vec![];
    cells.iter().for_each(|cell| {
        get_neighbours(board, &cell.pos).iter().for_each(|neighbour| {
            if !checked_leaves.contains(neighbour) {
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
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < FIELD_SIZE as i32 && *y < FIELD_SIZE as i32 && board.cells[*x as usize][*y as usize].is_some())
        .map(|(x, y)| Pos { x: x as usize, y: y as usize })
        .collect()
}

fn remove_cells(board: &mut Board, cells: &Vec<Cell>) {
    cells.iter().for_each(|cell| board.cells[cell.pos.x][cell.pos.y] = None);
}

struct Algorithm {
    cells: Vec<Cell>,
}

struct Board {
    cells: [[Option<Cell>; FIELD_SIZE]; FIELD_SIZE]
}

struct Cluster {
    cells: Vec<Cell>,
}

#[derive(Copy, Clone)]
struct Cell {
    pos: Pos,
    card: Card,
    card_side: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Card {
    sides: [TerrainType; 4],
}

#[derive(Copy, Clone)]
enum TerrainType {
    ROAD,
    FIELD,
    TOWN,
}
