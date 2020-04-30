use crate::evolution::FIELD_SIZE;
use crate::model::TerrainType::FIELD;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub cells: [[Option<Cell>; FIELD_SIZE]; FIELD_SIZE]
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    pub pos: Pos,
    pub card: Card,
    pub card_side: usize,
}

impl Cell {
    pub fn left(&self) -> TerrainType {
        self.card.sides[self.card_side]
    }

    pub fn top(&self) -> TerrainType {
        self.card.sides[(self.card_side + 1) % 4]
    }

    pub fn right(&self) -> TerrainType {
        self.card.sides[(self.card_side + 2) % 4]
    }

    pub fn bottom(&self) -> TerrainType {
        self.card.sides[(self.card_side + 3) % 4]
    }
}

pub fn left_side(cell: &Option<Cell>) -> TerrainType {
    match cell {
        Some(cell) => cell.left(),
        None => FIELD,
    }
}

pub fn top_side(cell: &Option<Cell>) -> TerrainType {
    match cell {
        Some(cell) => cell.top(),
        None => FIELD,
    }
}

pub fn right_side(cell: &Option<Cell>) -> TerrainType {
    match cell {
        Some(cell) => cell.right(),
        None => FIELD,
    }
}

pub fn bottom_side(cell: &Option<Cell>) -> TerrainType {
    match cell {
        Some(cell) => cell.bottom(),
        None => FIELD,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Card {
    pub sides: [TerrainType; 4],
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum TerrainType {
    ROAD,
    FIELD,
    TOWN,
}

impl std::fmt::Display for TerrainType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            TerrainType::TOWN => "town".to_owned(),
            TerrainType::ROAD => "road".to_owned(),
            TerrainType::FIELD => "field".to_owned(),
        };
        write!(f, "{}", str)
    }
}