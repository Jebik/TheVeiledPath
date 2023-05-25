use std::error::Error;

use crate::map::{
    engine_types::{Cell, Map, Item, ItemType},
    json_types::MapData,
};

pub struct MapManager {
    tuto_map: Map,
    level1_map: Map,
    custom_map: Option<Map>,
}

impl MapManager {
    pub fn new(tuto_map: MapData, level_map: MapData) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            tuto_map: generate_map(tuto_map)?,
            level1_map: generate_map(level_map)?,
            custom_map: None,
        })
    }

    pub(crate) fn set_custom_map(&mut self, custom_map: super::json_types::MapData) {
        match generate_map(custom_map) {
            Ok(map) => self.custom_map = Some(map),
            Err(_) => self.custom_map = None,
        }
    }
}

fn generate_map(map_data: MapData) -> Result<Map, Box<dyn Error>> {
    // Check if the size is a multiple of 16
    if map_data.size % 16 != 0 {
        return Err("Map size must be a multiple of 16".into());
    }

    let mut map = Map::new(&map_data);

    // Iterate over the walls and add them to the corresponding cells
    for wall in map_data.walls {
        let x = wall.x;
        let y = wall.y;
        map.add_to_cell(x, y, Item {
            item_type: ItemType::Wall,
            dimension: wall.dimension,
            id: 0
        });
    }

    // Iterate over the doors and add them to the corresponding cells
    for door in map_data.doors {
        let x = door.x;
        let y = door.y;
        map.add_to_cell(x, y, Item {
            item_type: ItemType::Door,
            dimension: door.dimension,
            id: door.id
        });
    }

    // Iterate over the keys and add them to the corresponding cells
    for key in map_data.keys {
        let x = key.x;
        let y = key.y;
        map.add_to_cell(x, y, Item {
            item_type: ItemType::Key,
            dimension: key.dimension,
            id: key.door_id
        });
    }

    //Return the map
    Ok(map)
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            items: Vec::new()
        }
    }
}
impl Map {
    pub fn new(map: &MapData) -> Self {
        let height = (map.size / 16) * 9;
        let cells = (0..map.size)
            .flat_map(|_x| (0..height).map(move |_y| Cell::new()))
            .collect();

        Self {
            name: map.name.clone(),
            width: map.size,
            height: height,
            cells: cells,
        }
    }

    pub (crate) fn at(&self, x: i32, y: i32) -> Option<&Cell> {
        // Check if the provided coordinates are within the valid range
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            // Calculate the index based on the provided x and y coordinates
            let index = (y * self.width + x) as usize;
            // Return a copy of the cell at the calculated index
            Some(&self.cells[index])
        } else {
            // Return None for out-of-bounds coordinates or special cases like (-1, -1)
            None
        }
    }

    pub fn add_to_cell(&mut self, x: i32, y: i32, item: Item) {
        let Some(cell) = self.cells.get_mut((y * self.width + x) as usize) else { return };
        cell.items.push(item);
    }
}
