use crate::model::TerrainType::FIELD;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Option<Cell>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Cell {
    pub pos: Pos,
    pub card: Card,
    pub card_side: CardSide,
}

impl Cell {
    pub fn get_side(&self, side: &CardSide) -> TerrainType {
        match side {
            CardSide::LEFT => self.left(),
            CardSide::TOP => self.top(),
            CardSide::RIGHT => self.right(),
            CardSide::BOTTOM => self.bottom(),
        }
    }

    pub fn left(&self) -> TerrainType {
        match self.card_side {
            CardSide::LEFT => self.card.left(),
            CardSide::TOP => self.card.top(),
            CardSide::RIGHT => self.card.right(),
            CardSide::BOTTOM => self.card.bottom(),
        }
    }

    pub fn top(&self) -> TerrainType {
        match self.card_side {
            CardSide::LEFT => self.card.top(),
            CardSide::TOP => self.card.right(),
            CardSide::RIGHT => self.card.bottom(),
            CardSide::BOTTOM => self.card.left(),
        }
    }

    pub fn right(&self) -> TerrainType {
        match self.card_side {
            CardSide::LEFT => self.card.right(),
            CardSide::TOP => self.card.bottom(),
            CardSide::RIGHT => self.card.left(),
            CardSide::BOTTOM => self.card.top(),
        }
    }

    pub fn bottom(&self) -> TerrainType {
        match self.card_side {
            CardSide::LEFT => self.card.bottom(),
            CardSide::TOP => self.card.left(),
            CardSide::RIGHT => self.card.top(),
            CardSide::BOTTOM => self.card.right(),
        }
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

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Card {
    #[serde(default, deserialize_with = "default_from_null")]
    pub structs: Vec<Struct>,
    #[serde(default, deserialize_with = "default_from_null")]
    pub monastery: bool,
    pub pic: String,
}

impl Card {
    pub fn left(&self) -> TerrainType {
        self.get_terrain(CardSide::LEFT)
    }

    pub fn right(&self) -> TerrainType {
        self.get_terrain(CardSide::RIGHT)
    }

    pub fn top(&self) -> TerrainType {
        self.get_terrain(CardSide::TOP)
    }

    pub fn bottom(&self) -> TerrainType {
        self.get_terrain(CardSide::BOTTOM)
    }

    pub fn get_terrain(&self, side: CardSide) -> TerrainType {
        self.structs
            .iter()
            .find(|struc| struc.sides.iter().find(|struc_side| struc_side == &&side).is_some())
            .map(|struc| struc.terrain.clone())
            .unwrap_or(TerrainType::FIELD)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Struct {
    #[serde(rename = "type")]
    pub terrain: TerrainType,
    #[serde(default, deserialize_with = "default_from_null")]
    pub sides: Vec<CardSide>,
    pub value: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum TerrainType {
    ROAD,
    FIELD,
    TOWN,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum CardSide {
    LEFT,
    TOP,
    RIGHT,
    BOTTOM
}

impl CardSide {
    pub fn get_opposite(&self) -> CardSide {
        match self {
            CardSide::LEFT => CardSide::RIGHT,
            CardSide::TOP => CardSide::BOTTOM,
            CardSide::RIGHT => CardSide::LEFT,
            CardSide::BOTTOM => CardSide::TOP,
        }
    }
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

pub(crate) fn default_from_null<'de, T, D>(de: D) -> Result<T, D::Error>
    where
        T: serde::Deserialize<'de> + Default,
        D: serde::Deserializer<'de>,
{
    Ok(<Option<T> as serde::Deserialize>::deserialize(de)?.unwrap_or_default())
}