use bevy::prelude::{Plugin, App};
use super::systems::over_state_system;

// Menu Plugin
pub struct OverPlugin;

impl Plugin for OverPlugin {
    fn build(&self, app: &mut App) {
        // Add systems, resources, and setup tasks specific to the menu game state
        app.add_system(over_state_system);
        // Add other systems and resources as needed
    }
}