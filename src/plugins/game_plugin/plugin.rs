use bevy::prelude::{Plugin, App};
use super::systems::game_state_system;

// Game Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Add systems, resources, and setup tasks specific to the game game state
        app.add_system(game_state_system);
        // Add other systems and resources as needed
    }    
}
