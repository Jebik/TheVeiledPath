use bevy::prelude::{Plugin, App, Resource, IntoSystemAppConfig};
use bevy::ecs::schedule::OnEnter;
use super::systems::setup_game;
use crate::plugins::state::types::GameState;

// Game Plugin
pub struct GamePlugin;

#[derive(Resource)]
pub struct GameData {
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_game.in_schedule(OnEnter(GameState::Game)));
    }    
}
