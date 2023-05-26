use bevy::prelude::{Plugin, App, IntoSystemConfig, IntoSystemAppConfig};
use bevy::ecs::schedule::{OnEnter,OnUpdate};
use super::systems::{setup_game, window_resize_system};
use super::input::{setup_input, move_system};
use crate::plugins::state::types::GameState;

// Game Plugin
pub struct GamePlugin;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_input);
        app.add_system(setup_game.in_schedule(OnEnter(GameState::Game)));
        app.add_system(move_system.in_set(OnUpdate(GameState::Game)));
        app.add_system(window_resize_system.in_set(OnUpdate(GameState::Game)));        
    }    
}
