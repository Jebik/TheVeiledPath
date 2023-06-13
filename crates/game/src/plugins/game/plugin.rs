use bevy::prelude::{Plugin, App, IntoSystemConfig, IntoSystemAppConfig, OnExit};
use bevy::ecs::schedule::{OnEnter,OnUpdate};
use bevy::sprite::Material2dPlugin;
use super::physic::physic_system;
use super::shader::DimensionMaterial;
use super::systems::{setup_game, window_resize_system, cleanup_game, follow_player_position};
use super::input::{setup_input, move_system};
use super::tutorial::tuto_system;
use crate::plugins::state::types::GameState;

// Game Plugin
pub struct GamePlugin;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {        
        app.add_plugin(Material2dPlugin::<DimensionMaterial>::default());
        app.add_startup_system(setup_input);
        app.add_system(setup_game.in_schedule(OnEnter(GameState::Game)));
        app.add_system(move_system.in_set(OnUpdate(GameState::Game)));
        app.add_system(follow_player_position.in_set(OnUpdate(GameState::Game)));
        app.add_system(tuto_system.in_set(OnUpdate(GameState::Game)));
        app.add_system(physic_system.in_set(OnUpdate(GameState::Game)));
        app.add_system(window_resize_system.in_set(OnUpdate(GameState::Game)));    
        app.add_system(cleanup_game.in_schedule(OnExit(GameState::Game)));    
    }    
}
