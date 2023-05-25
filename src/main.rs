mod map;
mod player;
mod plugins;
mod game;

use bevy::prelude::*;
use map::parser::{parse_map, MapSource, MapFile};
use plugins::{types::{StateManager, GameState}, plugin::StatePlugin};
use std::{env, path::PathBuf};

fn main() {
    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();
    // Parse the map.
    let tuto_map = parse_map(MapSource::FileContent(MapFile::Tuto)).expect("Failed to parse the tutorial map");    
    let level_map = parse_map(MapSource::FileContent(MapFile::Level)).expect("Failed to parse the level map");   
    let mut map_manager = map::map_manager::MapManager::new(tuto_map, level_map).expect("Failed to create the map manager");

    if args.len() > 1 {
        let custom_map = parse_map(MapSource::FilePath(PathBuf::from("path/to/custom_map.json"))).expect("Failed to parse the custom map");
        map_manager.set_custom_map(custom_map);
    }

    let state_manager = StateManager {
        map_manager,
        game_state: GameState::Menu,
    };

    App::new()
    .insert_resource::<StateManager>(state_manager)
    .add_plugin(StatePlugin)
    .run();
}
