use bevy::prelude::{Plugin, App};
use super::systems::menu_state_system;

// Menu Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Add systems, resources, and setup tasks specific to the menu game state
        app.add_system(menu_state_system);
        // Add other systems and resources as needed
    }
}