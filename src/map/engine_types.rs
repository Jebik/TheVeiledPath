use serde::Deserialize;

#[derive(Deserialize)]
pub enum Dimension {
    Light,
    Dark,
}
pub enum ItemType {
    Wall,
    Door,
    Key
}

pub struct Item {
    pub item_type: ItemType,
    pub dimension: Dimension,
    //I don't want to use Some here so for walls id is 0
    pub id: u32,
}

pub struct Cell {
    pub items: Vec<Item>,
}

pub struct Map {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}