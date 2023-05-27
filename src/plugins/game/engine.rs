use bevy::prelude::Resource;
use super::map::Map;
use crate::map::json_types::{MapData, Dimension};


pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir_x: f32,
    pub dir_y: f32,
    pub goal_x: i32,
    pub goal_y: i32,
}
impl Player {
    fn new(level_data: &MapData) -> Player {
        Player { 
            x: level_data.start_x as f32 + 0.5,
            y: level_data.start_y as f32 + 0.5,
            dir_x: 1.,
            dir_y: 0.,
            goal_x: level_data.goal_x,
            goal_y: level_data.goal_y
        }
    }
}


#[derive(Resource)]
pub struct SizeDate {
    pub grid_x: i32,
    pub grid_y: i32,
    pub screen_w: f32,
    pub screen_h: f32,
    pub quad_height: f32,
    pub quad_width: f32,
    pub trans_x: f32,
    pub trans_y: f32,
}
impl SizeDate {
    pub(crate) fn new(grid_x: i32, grid_y:i32, width:f32, height:f32) -> SizeDate {   
        let img_width = 1600.;
        let img_height = 900.; 
        let quad_width = img_width / grid_x as f32;
        let quad_height = img_height / grid_y as f32;
        let trans_x = (quad_width / 2.0) - (img_width / 2.0);
        let trans_y = - (quad_height / 2.0) + (img_height / 2.0);

        SizeDate {
            grid_x,
            grid_y,
            screen_w: width,
            screen_h: height,
            quad_height,
            quad_width,
            trans_x ,
            trans_y
        }
    }

    pub(crate) fn get_world_x(&self, x: f32) -> f32 {
        let res = (x * self.quad_width) + self.trans_x;
        return res;
    }
    pub(crate) fn get_world_y(&self, y: f32) -> f32 {
        let res = (-y * self.quad_height) + self.trans_y;
        return res;
    }
}

#[derive(Resource)]
pub struct GameData {
    pub map: Map,
    pub player: Player,
    pub dimension: Dimension,
}
impl GameData {
    pub(crate) fn new(level_data: &MapData) -> GameData {
        GameData {
            map: Map::new(level_data),
            player: Player::new(level_data),
            dimension: Dimension::Light,
        }
    }
}