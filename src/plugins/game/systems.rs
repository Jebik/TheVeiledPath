use super::{map::Map, plugin::GameData};
use bevy::ecs::system::Commands;
use crate::plugins::menu::plugin::LevelChoice;
use crate::map::map_manager::MapManager;
use bevy::ecs::system::Res;

pub fn setup_game(
    mut commands: Commands,
    level: Res<LevelChoice>,
    map: Res<MapManager>,
) {
    let level_data = match *level {
        LevelChoice::Tutorial => &map.tuto_map,
        LevelChoice::Level1 => &map.level1_map,
        LevelChoice::Custom => match &map.custom_map {
            Some(custom_map) => custom_map,
            None => {
                panic!("Select custom level but custom_map is not set");
            }
        },
        LevelChoice::None => panic!("Level Selection to None while going inside GamePlugin")
    };
    let mut _map = Map::new(level_data);

    commands.insert_resource(GameData {
    });
}