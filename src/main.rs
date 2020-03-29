use rand::Rng;
use TerrainType::*;
use std::collections::HashSet;
use rand::prelude::ThreadRng;

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
const MUTATION_CHANCE: f64 = 0.1;

fn main() {
    let mut population: Vec<Algorithm> = (0..POPULATION_SIZE).map(|_| generate_algorithm()).collect();
    let mut rated_algs: Vec<(usize, Algorithm)> = population.into_iter().map(|algorithm| (evaluate_algorithm(&algorithm), algorithm)).collect();
    rated_algs.sort_by_key(|(score, _)| *score);
    let (mut best_result, _) = rated_algs[0];
    let rated_algs: Vec<Algorithm> = rated_algs.into_iter().map(|(_, alg)| alg).collect();
    population = next_generation(&rated_algs);
    println!("best result: {}", best_result);
    while best_result > 1 {
        let mut rated_algs: Vec<(usize, Algorithm)> = population.into_iter().map(|algorithm| (evaluate_algorithm(&algorithm), algorithm)).collect();
        rated_algs.sort_by_key(|(score, _)| *score);
        let (new_best_result, _) = rated_algs[0];
        let rated_algs: Vec<Algorithm> = rated_algs.into_iter().map(|(_, alg)| alg).collect();
        population = next_generation(&rated_algs);
        best_result = new_best_result;
        println!("best result: {}", best_result);
    }
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
    if rng.gen_range(0.0, 1.0) > MUTATION_CHANCE {
        let mutation_index = rng.gen_range(0, cells.len());
        let mutating_cell = cells[mutation_index];
        cells[mutation_index] = Cell {
            pos: Pos { x: rng.gen_range(0, FIELD_SIZE), y: rng.gen_range(0, FIELD_SIZE) },
            card: mutating_cell.card,
            card_side: rng.gen_range(0, 4),
        };
    }
    Algorithm { cells }
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
