use serde::Deserialize;
use crate::map::engine_types::Dimension;

#[derive(Deserialize)]
pub struct MapData {
    pub name: String,
    pub size: i32,
    pub walls: Vec<Wall>,
    pub doors: Vec<Door>,
    pub keys: Vec<Key>,
}

#[derive(Deserialize)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub dimension: Dimension,
}

#[derive(Deserialize)]
pub struct Door {
    pub x: i32,
    pub y: i32,
    pub id: u32,
    pub dimension: Dimension,
}

#[derive(Deserialize)]
pub struct Key {
    pub x: i32,
    pub y: i32,
    pub door_id: u32,
    pub dimension: Dimension,
}