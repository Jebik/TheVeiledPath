use std::error::Error;
use crate::map::json_types::MapData;

pub struct MapManager {
    pub tuto_map: MapData,
    pub level1_map: MapData,
    pub custom_map: Option<MapData>,
}

impl MapManager {
    pub fn new(tuto_map: MapData, level_map: MapData) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            tuto_map: tuto_map,
            level1_map: level_map,
            custom_map: None,
        })
    }

    pub(crate) fn set_custom_map(&mut self, custom_map: super::json_types::MapData) {
        self.custom_map = Some(custom_map)
    }
}
