use bevy::prelude::{Plugin, App, IntoSystemConfig, OnUpdate};
use crate::plugins::types::GameState;

use super::systems::over_ui;

// Menu Plugin
pub struct OverPlugin;

impl Plugin for OverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(over_ui.in_set(OnUpdate(GameState::Over)));
    }
}