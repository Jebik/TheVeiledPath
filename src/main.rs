mod map;
mod plugins;

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use map::parser::{parse_map, MapSource, MapFile};
use plugins::plugin::StatePlugin;
use map::map_manager::MapManager;
use std::{env, path::PathBuf};


impl Resource for MapManager {}

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

    App::new()
    .insert_resource::<MapManager>(map_manager)
    .add_plugin(StatePlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .run();
}
