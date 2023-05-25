use bevy::prelude::{Plugin, App};
use super::systems::menu_ui;

// Menu Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_ui);
    }
}