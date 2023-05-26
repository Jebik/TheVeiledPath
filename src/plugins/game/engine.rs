use bevy::prelude::{Resource};
use super::map::Map;
use crate::map::json_types::{MapData, Dimension};


pub struct Player {
    pub x: f32,
    pub y: f32,
    pub goal_x: i32,
    pub goal_y: i32,
}
impl Player {
    fn new(level_data: &MapData) -> Player {
        Player { 
            x: level_data.start_x as f32 + 0.5,
            y: level_data.start_y as f32 + 0.5,
            goal_x: level_data.goal_x,
            goal_y: level_data.goal_y
        }
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