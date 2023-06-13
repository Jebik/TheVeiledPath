pub mod map;
pub mod plugins;

pub use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
pub use map::parser::{parse_map, MapSource, MapFile};
pub use plugins::plugin::StatePlugin;
pub use map::map_manager::MapManager;
pub use std::{env, path::PathBuf};
