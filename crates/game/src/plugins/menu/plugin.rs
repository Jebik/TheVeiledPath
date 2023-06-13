use bevy::prelude::{Plugin, App, IntoSystemConfig, Resource};
use bevy::ecs::schedule::OnUpdate;
use super::systems::menu_ui;
use crate::plugins::state::types::GameState;

// Menu Plugin
pub struct MenuPlugin;

#[derive(Resource)]
pub enum LevelChoice {
    None,
    Tutorial,
    Level1,
    Custom,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<LevelChoice>(LevelChoice::None);
        app.add_system(menu_ui.in_set(OnUpdate(GameState::Menu)));
    }
}