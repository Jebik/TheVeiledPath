use bevy::prelude::warn;
use crate::map::json_types::{Dimension, MapData};

#[derive(PartialEq, Eq)]
pub struct Door {
    open: bool,
    id: u32,
}

#[derive(PartialEq, Eq)]
pub struct Key {
    door_id: u32,
}

#[derive(PartialEq, Eq)]
pub enum ItemType {
    Wall,
    Door(Door),
    Key(Key),
    None,
}

pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub item_type: ItemType
}
impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        Cell {
            x: x as f32,
            y: y as f32,
            item_type: ItemType::None
        }
    }

    fn set_data(&mut self, item_type: ItemType) {
        if self.item_type != ItemType::None {
            warn!("Cell already has data, overwriting data")
        }
        self.item_type = item_type;
    }
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub light_cells: Vec<Cell>,
    pub dark_cells: Vec<Cell>,
}

impl Map {
    pub fn new(map_data: &MapData) -> Self {
        let height = (map_data.size / 16) * 9;
        let light_cells = (0..map_data.size)
            .flat_map(|x| (0..height).map(move |y| Cell::new(x, y)))
            .collect();
        let dark_cells = (0..map_data.size)
            .flat_map(|x| (0..height).map(move |y| Cell::new(x, y)))
            .collect();

        let mut map = Map {
            width: map_data.size,
            height: height,
            light_cells,
            dark_cells
        };

        generate_map(map_data, &mut map);
        map
    }

    pub (crate) fn _at(&self, x: i32, y: i32, dimension: Dimension) -> Option<&Cell> {
        // Check if the provided coordinates are within the valid range
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            // Calculate the index based on the provided x and y coordinates
            let index = (y * self.width + x) as usize;
            match dimension {
                Dimension::Light => {
                    Some(&self.light_cells[index])
                },
                Dimension::Dark => {
                    Some(&self.dark_cells[index])
                },
            }
        } else {
            // Return None for out-of-bounds coordinates or special cases like (-1, -1)
            None
        }
    }

    pub fn get_mut_cell(&mut self, x: i32, y: i32, dimension: Dimension) -> Option<&mut Cell> {
        match dimension {
            Dimension::Light => {
                self.light_cells.get_mut((x * self.height + y) as usize)
            },
            Dimension::Dark => {
                self.dark_cells.get_mut((x * self.height + y) as usize)
            },
        }
    }
}



fn generate_map(map_data: &MapData, map: &mut Map) {
    // Iterate over the walls and add them to the corresponding cells
    for wall in &map_data.walls {
        if let Some(cell) = map.get_mut_cell(wall.x, wall.y, wall.dimension) {
            cell.set_data(ItemType::Wall);
        } else {
            warn!("Parse Map Wall in invalid position: ({}, {})", wall.x, wall.y);
        }
    }
    // Iterate over the doors and add them to the corresponding cells
    for door in &map_data.doors {
        if let Some(cell) = map.get_mut_cell(door.x, door.y, door.dimension) {
            cell.set_data(ItemType::Door(Door { open: false, id: door.id }));
        } else {
            warn!("Parse Map Door in invalid position: ({}, {})", door.x, door.y);
        }
    }
    // Iterate over the doors and add them to the corresponding cells
    for key in &map_data.keys {
        if let Some(cell) = map.get_mut_cell(key.x, key.y, key.dimension) {
            cell.set_data(ItemType::Key(Key { door_id: key.door_id }));
        } else {
            warn!("Parse Map Door in invalid position: ({}, {})", key.x, key.y);
        }
    }
}