use bevy::prelude::{Plugin, App, IntoSystemConfig, OnUpdate};
use crate::plugins::types::GameState;

use super::systems::win_ui;

// Menu Plugin
pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(win_ui.in_set(OnUpdate(GameState::Win)));
    }
}