use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum Dimension {
    Light,
    Dark,
}

impl FromStr for Dimension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(Dimension::Light),
            "dark" => Ok(Dimension::Dark),
            _ => Err(()),
        }
    }
}

impl Dimension {
    pub fn switch_dimension(&mut self) {
        match self {
            Dimension::Light => *self = Dimension::Dark,
            Dimension::Dark => *self = Dimension::Light,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MapData {
    pub name: String,
    pub size: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub goal_x: i32,
    pub goal_y: i32,
    pub walls: Vec<Wall>,
    pub doors: Vec<Door>,
    pub keys: Vec<Key>,
}

#[derive(Deserialize, Serialize)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub dimension: Dimension,
}

#[derive(Deserialize, Serialize)]
pub struct Door {
    pub x: i32,
    pub y: i32,
    pub id: u32,
    pub dimension: Dimension,
}

#[derive(Deserialize, Serialize)]
pub struct Key {
    pub x: i32,
    pub y: i32,
    pub door_id: u32,
    pub dimension: Dimension,
}