use crate::model::{Board, Pos, Cell, Card, CardSide};
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::carcassone::{evaluate_algorithm, fill_board};
use glib::Sender;
use crate::algorithm::Algorithm;

const FIELD_SIZE: usize = 15;
const POPULATION_SIZE: usize = 50;
const MUTATION_CHANCE: f64 = 0.5;

pub fn start_evolution(cards: &Vec<Card>, sender: &Sender<RatedBoard>) {
    let mut population: Vec<Algorithm> = (0..POPULATION_SIZE).map(|_| generate_algorithm(cards)).collect();
    let mut result_found = false;
    while !result_found {
        let mut rated_algs: Vec<(usize, Algorithm)> = population.into_iter().map(|algorithm| (evaluate_algorithm(&algorithm), algorithm)).collect();
        rated_algs.sort_by_key(|(score, _)| *score);
        let (best_result, best_alg) = rated_algs[0].clone();
        let rated_algs: Vec<Algorithm> = rated_algs.into_iter().map(|(_, alg)| alg).collect();
        population = next_generation(cards, &rated_algs);
        let board = fill_board(&best_alg.arranged_cells);
        sender.send(RatedBoard { score: best_result, board });
        if best_result == 0 { result_found = true; }
    }
}

pub fn create_empty_board() -> Board {
    Board {
        width: FIELD_SIZE,
        height: FIELD_SIZE,
        cells: vec![vec![None; FIELD_SIZE]; FIELD_SIZE],
    }
}

fn generate_algorithm(cards: &Vec<Card>) -> Algorithm {
    let mut rng = rand::thread_rng();
    let cells = (0..cards.len())
        .map(|card_id| {
            let pos = Pos {
                x: rng.gen_range(0, FIELD_SIZE),
                y: rng.gen_range(0, FIELD_SIZE),
            };
            let card_side = match rng.gen_range(0, 4) {
                0 => CardSide::LEFT,
                1 => CardSide::TOP,
                2 => CardSide::RIGHT,
                _ => CardSide::BOTTOM,
            };
            Cell { pos, card: cards[card_id].clone(), card_side }
        })
        .collect();
    Algorithm::new(cells)
}

fn next_generation(cards: &Vec<Card>, rated_algorithms: &Vec<Algorithm>) -> Vec<Algorithm> {
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
        result.push(breed(cards, parent1, parent2));
    }
    result
}

fn select_index(rng: &mut ThreadRng) -> usize {
    let rand: f64 = rng.gen_range(0.0, 1.0);
    ((1.0 - ((1.0 - rand).sqrt())) * POPULATION_SIZE as f64) as usize
}

fn breed(cards: &Vec<Card>, algorithm1: &Algorithm, algorithm2: &Algorithm) -> Algorithm {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, cards.len());
    let (first_part, _) = algorithm1.cells.split_at(index);
    let (_, second_part) = algorithm2.cells.split_at(index);
    let mut cells: Vec<Cell> = vec![];
    cells.extend_from_slice(first_part);
    cells.extend_from_slice(second_part);
    mutate(&mut rng, &mut cells);
    Algorithm::new(cells)
}

fn mutate(rng: &mut ThreadRng, cells: &mut Vec<Cell>) {
    if rng.gen_range(0.0, 1.0) > MUTATION_CHANCE {
        let mutation_index = rng.gen_range(0, cells.len());
        let mutating_cell = cells[mutation_index].clone();
        let card_side = match rng.gen_range(0, 4) {
            0 => CardSide::LEFT,
            1 => CardSide::TOP,
            2 => CardSide::RIGHT,
            _ => CardSide::BOTTOM,
        };
        cells[mutation_index] = Cell {
            pos: Pos { x: rng.gen_range(0, FIELD_SIZE), y: rng.gen_range(0, FIELD_SIZE) },
            card: mutating_cell.card,
            card_side,
        };
    }
}

pub struct RatedBoard {
    pub score: usize,
    pub board: Board,
}
