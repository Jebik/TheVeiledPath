use super::types::StateManager;
use bevy::ecs::system::Resource;
use bevy::prelude::{Res};

pub fn over_state_system(_state_manager: Res<StateManager>) {
    // Logic and rendering for the game over state
}

impl Resource for StateManager {}
