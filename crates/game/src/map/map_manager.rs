use std::error::Error;
use bevy::prelude::Resource;

use map_shared::MapData;

#[derive(Resource)]
pub struct MapManager {
    pub tuto_map: MapData,
    pub level1_map: MapData,
    pub custom_map: Option<MapData>,
}

impl MapManager {
    pub fn new(tuto_map: MapData, level_map: MapData) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            tuto_map,
            level1_map: level_map,
            custom_map: None,
        })
    }

    pub fn set_custom_map(&mut self, custom_map: MapData) {
        self.custom_map = Some(custom_map)
    }
}
