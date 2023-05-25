use bevy::{prelude::{Plugin, App, PluginGroup, default}, window::{WindowPlugin, WindowMode, WindowPosition, Window}, DefaultPlugins};
use crate::plugins::menu::plugin::MenuPlugin;
use super::systems::over_state_system;
// Menu Plugin
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TileSetEditor For GB ASM".into(),
                position: WindowPosition::At((30,30).into()),
                resolution: (510., 470.).into(),
                resizable: false,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }));
        app.add_plugin(MenuPlugin);
        // Add systems, resources, and setup tasks specific to the menu game state
        app.add_system(over_state_system);
        // Add other systems and resources as needed
    }
}