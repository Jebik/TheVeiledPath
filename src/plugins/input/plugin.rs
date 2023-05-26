use bevy::prelude::{Plugin, App};

use super::{types::{InputData, InputMap}, systems::{gamepad_connections, handle_input_system}};

// Game Plugin
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputData::default());
        app.insert_resource(InputMap::default());
        app.add_system(gamepad_connections);
        app.add_system(handle_input_system);
    }    
}