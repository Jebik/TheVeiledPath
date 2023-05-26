use bevy::{prelude::{Plugin, App, PluginGroup, default}, window::{WindowPlugin, WindowMode, WindowPosition, Window}, DefaultPlugins};
use bevy_egui::EguiPlugin;
use crate::plugins::menu::plugin::MenuPlugin;
use crate::plugins::game::plugin::GamePlugin;
use crate::plugins::over::plugin::OverPlugin;
use super::types::GameState;
use super::systems::window_resize_system;
// Menu Plugin
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Veiled Path".into(),
                position: WindowPosition::At((30,30).into()),
                resolution: (1600., 900.).into(),
                resizable: false,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }));
        app.add_state::<GameState>();
        app.add_plugin(EguiPlugin);
        app.add_plugin(MenuPlugin);
        app.add_plugin(GamePlugin);
        app.add_plugin(OverPlugin);
        app.add_system(window_resize_system);
        // Add other systems and resources as needed
    }
}