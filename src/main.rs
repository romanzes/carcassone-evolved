use rand::Rng;
use TerrainType::*;
use std::collections::HashSet;
use rand::prelude::ThreadRng;

const CARDS: [Card; 72] = [
    Card { sides: [FIELD, FIELD, FIELD, FIELD] },
    Card { sides: [FIELD, FIELD, FIELD, FIELD] },
    Card { sides: [FIELD, FIELD, FIELD, FIELD] },
    Card { sides: [FIELD, FIELD, FIELD, FIELD] },
    Card { sides: [FIELD, FIELD, FIELD, TOWN] },
    Card { sides: [FIELD, FIELD, FIELD, TOWN] },
    Card { sides: [FIELD, FIELD, FIELD, TOWN] },
    Card { sides: [FIELD, FIELD, FIELD, TOWN] },
    Card { sides: [FIELD, FIELD, FIELD, TOWN] },
    Card { sides: [TOWN, FIELD, TOWN, FIELD] },
    Card { sides: [TOWN, FIELD, TOWN, FIELD] },
    Card { sides: [TOWN, FIELD, TOWN, FIELD] },
    Card { sides: [TOWN, FIELD, TOWN, FIELD] },
    Card { sides: [FIELD, FIELD, TOWN, TOWN] },
    Card { sides: [FIELD, FIELD, TOWN, TOWN] },
    Card { sides: [FIELD, FIELD, TOWN, TOWN] },
    Card { sides: [FIELD, FIELD, TOWN, TOWN] },
    Card { sides: [FIELD, FIELD, TOWN, TOWN] },
    Card { sides: [TOWN, FIELD, TOWN, FIELD] },
    Card { sides: [TOWN, FIELD, TOWN, FIELD] },
    Card { sides: [FIELD, FIELD, TOWN, TOWN] },
    Card { sides: [FIELD, FIELD, TOWN, TOWN] },
    Card { sides: [FIELD, TOWN, TOWN, TOWN] },
    Card { sides: [FIELD, TOWN, TOWN, TOWN] },
    Card { sides: [FIELD, TOWN, TOWN, TOWN] },
    Card { sides: [FIELD, TOWN, TOWN, TOWN] },
    Card { sides: [TOWN, TOWN, TOWN, TOWN] },
    Card { sides: [ROAD, FIELD, FIELD, FIELD] },
    Card { sides: [ROAD, FIELD, FIELD, FIELD] },
    Card { sides: [ROAD, TOWN, TOWN, TOWN] },
    Card { sides: [ROAD, TOWN, TOWN, TOWN] },
    Card { sides: [ROAD, TOWN, TOWN, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, FIELD] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, FIELD, ROAD, TOWN] },
    Card { sides: [ROAD, ROAD, TOWN, TOWN] },
    Card { sides: [ROAD, ROAD, TOWN, TOWN] },
    Card { sides: [ROAD, ROAD, TOWN, TOWN] },
    Card { sides: [ROAD, ROAD, TOWN, TOWN] },
    Card { sides: [ROAD, ROAD, TOWN, TOWN] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, FIELD] },
    Card { sides: [ROAD, ROAD, TOWN, FIELD] },
    Card { sides: [ROAD, ROAD, TOWN, FIELD] },
    Card { sides: [ROAD, ROAD, TOWN, FIELD] },
    Card { sides: [ROAD, ROAD, FIELD, TOWN] },
    Card { sides: [ROAD, ROAD, FIELD, TOWN] },
    Card { sides: [ROAD, ROAD, FIELD, TOWN] },
    Card { sides: [ROAD, ROAD, ROAD, FIELD] },
    Card { sides: [ROAD, ROAD, ROAD, FIELD] },
    Card { sides: [ROAD, ROAD, ROAD, FIELD] },
    Card { sides: [ROAD, ROAD, ROAD, FIELD] },
    Card { sides: [ROAD, ROAD, ROAD, TOWN] },
    Card { sides: [ROAD, ROAD, ROAD, TOWN] },
    Card { sides: [ROAD, ROAD, ROAD, TOWN] },
    Card { sides: [ROAD, ROAD, ROAD, ROAD] },
];

const FIELD_SIZE: usize = 30;
const POPULATION_SIZE: usize = 100;
const MUTATION_CHANCE: f64 = 0.1;

fn main() {
    let mut population: Vec<Algorithm> = (0..POPULATION_SIZE).map(|_| generate_algorithm()).collect();
    let mut rated_algs: Vec<(usize, Algorithm)> = population.into_iter().map(|algorithm| (evaluate_algorithm(&algorithm), algorithm)).collect();
    rated_algs.sort_by_key(|(score, _)| *score);
    let (mut best_result, mut best_alg) = rated_algs[0].clone();
    let rated_algs: Vec<Algorithm> = rated_algs.into_iter().map(|(_, alg)| alg).collect();
    population = next_generation(&rated_algs);
    println!("best result: {}", best_result);
    while best_result > 1 {
        let mut rated_algs: Vec<(usize, Algorithm)> = population.into_iter().map(|algorithm| (evaluate_algorithm(&algorithm), algorithm)).collect();
        rated_algs.sort_by_key(|(score, _)| *score);
        let (new_best_result, new_best_alg) = rated_algs[0].clone();
        let rated_algs: Vec<Algorithm> = rated_algs.into_iter().map(|(_, alg)| alg).collect();
        population = next_generation(&rated_algs);
        best_result = new_best_result;
        best_alg = new_best_alg;
        println!("best result: {}", best_result);
    }
    let board = fill_board(&best_alg.cells);
    display_board(&board);
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
    let mut board = fill_board(&algorithm.cells);
    let clusters = extract_clusters(&mut board);
    clusters.len()
}

fn fill_board(cells: &Vec<Cell>) -> Board {
    let mut board = Board { cells: [[None; FIELD_SIZE]; FIELD_SIZE] };
    cells.iter().for_each(|cell| {
        board.cells[cell.pos.x][cell.pos.y] = Some(cell.clone());
    });
    board
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
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < FIELD_SIZE as i32 && *y < FIELD_SIZE as i32 && board.cells[*x as usize][*y as usize].is_some())
        .map(|(x, y)| Pos { x: x as usize, y: y as usize })
        .collect()
}

fn remove_cells(board: &mut Board, cells: &Vec<Cell>) {
    cells.iter().for_each(|cell| board.cells[cell.pos.x][cell.pos.y] = None);
}

fn next_generation(rated_algorithms: &Vec<Algorithm>) -> Vec<Algorithm> {
    let mut result = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..POPULATION_SIZE {
        let index1 = select_index(&mut rng);
        let mut index2 = select_index(&mut rng);
        while index2 == index1 {
            index2 = select_index(&mut rng);
        }
        let parent1 = &rated_algorithms[index1];
        let parent2 = &rated_algorithms[index2];
        result.push(breed(parent1, parent2));
    }
    result
}

fn select_index(rng: &mut ThreadRng) -> usize {
    let rand: f64 = rng.gen_range(0.0, 1.0);
    ((1.0 - ((1.0 - rand).sqrt())) * 100.0) as usize
}

fn breed(algorithm1: &Algorithm, algorithm2: &Algorithm) -> Algorithm {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, CARDS.len());
    let (first_part, _) = algorithm1.cells.split_at(index);
    let (_, second_part) = algorithm2.cells.split_at(index);
    let mut cells: Vec<Cell> = vec![];
    cells.extend(first_part);
    cells.extend(second_part);
    mutate(&mut rng, &mut cells);
    rearrange_overlaps(&mut cells);
    Algorithm { cells }
}

fn mutate(rng: &mut ThreadRng, cells: &mut Vec<Cell>) {
    if rng.gen_range(0.0, 1.0) > MUTATION_CHANCE {
        let mutation_index = rng.gen_range(0, cells.len());
        let mutating_cell = cells[mutation_index];
        cells[mutation_index] = Cell {
            pos: Pos { x: rng.gen_range(0, FIELD_SIZE), y: rng.gen_range(0, FIELD_SIZE) },
            card: mutating_cell.card,
            card_side: rng.gen_range(0, 4),
        };
    }
}

fn rearrange_overlaps(cells: &mut Vec<Cell>) {
    let mut board = Board { cells: [[None; FIELD_SIZE]; FIELD_SIZE] };
    for index in 0..cells.len() {
        let cell = cells[index];
        if board.cells[cell.pos.x][cell.pos.y].is_some() {
            let closest_free_pos = find_closest_free_pos(&mut board, &cell.pos);
            cells[index] = Cell {
                pos: closest_free_pos,
                card: cell.card,
                card_side: cell.card_side,
            };
            board.cells[closest_free_pos.x][closest_free_pos.y] = Some(cells[index]);
        } else {
            board.cells[cell.pos.x][cell.pos.y] = Some(cell.clone());
        }
    }
}

fn find_closest_free_pos(board: &mut Board, pos: &Pos) -> Pos {
    let mut free_cell: Option<Pos> = None;
    let mut distance = 1;
    while free_cell.is_none() {
        free_cell = get_free_cell(board, &get_halo(pos, distance));
        distance += 1;
    }
    free_cell.unwrap()
}

fn get_free_cell(board: &Board, positions: &Vec<Pos>) -> Option<Pos> {
    positions.iter().find(|pos| board.cells[pos.x][pos.y].is_none()).copied()
}

fn get_halo(pos: &Pos, distance: usize) -> Vec<Pos> {
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
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < FIELD_SIZE as i32 && *y < FIELD_SIZE as i32)
        .map(|(x, y)| Pos { x: x as usize, y: y as usize })
        .collect()
}

fn display_board(board: &Board) {
    for x in 0..FIELD_SIZE {
        for y in 0..FIELD_SIZE {
            if board.cells[x][y].is_some() {
                print!("  ");
            } else {
                print!("\u{2588}\u{2588}");
            }
        }
        println!();
    }
}

#[derive(Clone)]
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
