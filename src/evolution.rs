use crate::model::{Board, Pos, Cell, top_side, left_side, right_side, bottom_side, TerrainType};
use crate::cards::CARDS;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::carcassone::{evaluate_algorithm, fill_board};
use glib::Sender;
use crate::algorithm::Algorithm;

const FIELD_SIZE: usize = 15;
const POPULATION_SIZE: usize = 50;
const MUTATION_CHANCE: f64 = 0.5;

pub fn start_evolution(sender: &Sender<(usize, Board)>) {
    let mut population: Vec<Algorithm> = (0..POPULATION_SIZE).map(|_| generate_algorithm()).collect();
    let mut rated_algs: Vec<(usize, Algorithm)> = population.into_iter().map(|algorithm| (evaluate_algorithm(&algorithm), algorithm)).collect();
    rated_algs.sort_by_key(|(score, _)| *score);
    let (mut best_result, mut best_alg) = rated_algs[0].clone();
    let rated_algs: Vec<Algorithm> = rated_algs.into_iter().map(|(_, alg)| alg).collect();
    population = next_generation(&rated_algs);
    update_board(sender, best_result, &best_alg);
    println!("best result: {}", best_result);
    while best_result > 0 {
        let mut rated_algs: Vec<(usize, Algorithm)> = population.into_iter().map(|algorithm| (evaluate_algorithm(&algorithm), algorithm)).collect();
        rated_algs.sort_by_key(|(score, _)| *score);
        let (new_best_result, new_best_alg) = rated_algs[0].clone();
        let rated_algs: Vec<Algorithm> = rated_algs.into_iter().map(|(_, alg)| alg).collect();
        population = next_generation(&rated_algs);
        best_result = new_best_result;
        best_alg = new_best_alg;
        update_board(sender, best_result, &best_alg);
        println!("best result: {}", best_result);
    }
    let board = fill_board(&best_alg.arranged_cells);
    display_board(&board);
    let serialized = serde_json::to_string(&board).unwrap();
    println!("{:?}", serialized);
}

pub fn create_empty_board() -> Board {
    Board {
        width: FIELD_SIZE,
        height: FIELD_SIZE,
        cells: vec![vec![None; FIELD_SIZE]; FIELD_SIZE],
    }
}

fn update_board(sender: &Sender<(usize, Board)>, score: usize, algorithm: &Algorithm) {
    let board = fill_board(&algorithm.arranged_cells);
    sender.send((score, board));
}

fn generate_algorithm() -> Algorithm {
    let mut rng = rand::thread_rng();
    let cells = (0..CARDS.len())
        .map(|card_id| {
            let pos = Pos {
                x: rng.gen_range(0, FIELD_SIZE),
                y: rng.gen_range(0, FIELD_SIZE),
            };
            let card_side = rng.gen_range(0, 4);
            Cell { pos, card: CARDS[card_id], card_side }
        })
        .collect();
    Algorithm::new(cells)
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
    ((1.0 - ((1.0 - rand).sqrt())) * POPULATION_SIZE as f64) as usize
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
    Algorithm::new(cells)
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

fn display_board(board: &Board) {
    for y in 0..board.height {
        for _ in 0..board.width {
            print!("┼──────────");
        }
        println!();

        for x in 0..board.width {
            let side = match top_side(&board.cells[x][y]) {
                TerrainType::FIELD => "          ",
                TerrainType::ROAD => "    ██    ",
                TerrainType::TOWN => "██████████",
            };
            print!("│{}", side);
        }
        println!();

        for x in 0..board.width {
            let left_side = match left_side(&board.cells[x][y]) {
                TerrainType::FIELD => "  ",
                TerrainType::ROAD => "  ",
                TerrainType::TOWN => "██",
            };
            let right_side = match right_side(&board.cells[x][y]) {
                TerrainType::FIELD => "  ",
                TerrainType::ROAD => "  ",
                TerrainType::TOWN => "██",
            };
            print!("│{}      {}", left_side, right_side);
        }
        println!();

        for x in 0..board.width {
            let left_side = match left_side(&board.cells[x][y]) {
                TerrainType::FIELD => "  ",
                TerrainType::ROAD => "██",
                TerrainType::TOWN => "██",
            };
            let right_side = match right_side(&board.cells[x][y]) {
                TerrainType::FIELD => "  ",
                TerrainType::ROAD => "██",
                TerrainType::TOWN => "██",
            };
            print!("│{}      {}", left_side, right_side);
        }
        println!();

        for x in 0..board.width {
            let left_side = match left_side(&board.cells[x][y]) {
                TerrainType::FIELD => "  ",
                TerrainType::ROAD => "  ",
                TerrainType::TOWN => "██",
            };
            let right_side = match right_side(&board.cells[x][y]) {
                TerrainType::FIELD => "  ",
                TerrainType::ROAD => "  ",
                TerrainType::TOWN => "██",
            };
            print!("│{}      {}", left_side, right_side);
        }
        println!();

        for x in 0..board.width {
            let side = match bottom_side(&board.cells[x][y]) {
                TerrainType::FIELD => "          ",
                TerrainType::ROAD => "    ██    ",
                TerrainType::TOWN => "██████████",
            };
            print!("│{}", side);
        }
        println!()
    }
}