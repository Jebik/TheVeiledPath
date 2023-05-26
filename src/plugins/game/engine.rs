use bevy::prelude::{Resource};
use super::map::Map;
use crate::map::json_types::{MapData, Dimension};


pub struct Player {
    pub x: f32,
    pub y: f32,
}
impl Player {
    fn new(level_data: &MapData) -> Player {
        todo!()
    }
}

#[derive(Resource)]
pub struct GameData {
    map: Map,
    player: Player,
    dimensions: Dimension,
}

impl GameData {
    pub(crate) fn new(level_data: &MapData) -> GameData {
        GameData {
            map: Map::new(level_data),
            player: Player::new(level_data),
            dimensions: Dimension::Light,
        }
    }
}