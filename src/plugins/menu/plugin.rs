use bevy::prelude::{Plugin, App, IntoSystemConfig};
use bevy::ecs::schedule::OnUpdate;
use super::systems::menu_ui;
use crate::plugins::state::types::GameState;

// Menu Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_ui.in_set(OnUpdate(GameState::Menu)));
    }
}