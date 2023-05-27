use bevy::{prelude::{Plugin, App, PluginGroup, default}, window::{WindowPlugin, WindowMode, WindowPosition, Window}, DefaultPlugins};
use bevy_egui::{EguiPlugin, egui::{FontFamily, TextStyle, FontId}, EguiContexts};
use crate::plugins::{menu::plugin::MenuPlugin, win::plugin::WinPlugin};
use crate::plugins::game::plugin::GamePlugin;
use crate::plugins::over::plugin::OverPlugin;
use crate::plugins::input::plugin::InputPlugin;
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
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }));
        app.add_state::<GameState>();
        app.add_plugin(EguiPlugin);
        app.add_startup_system(configure_egui);
        app.add_plugin(InputPlugin);
        app.add_plugin(MenuPlugin);
        app.add_plugin(GamePlugin);
        app.add_plugin(OverPlugin);
        app.add_plugin(WinPlugin);
        app.add_system(window_resize_system);
        // Add other systems and resources as needed
    }
}

fn configure_egui(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(150.0, Proportional)),
        (TextStyle::Body, FontId::new(50.0, Proportional)),
        (TextStyle::Monospace, FontId::new(100.0, Monospace)),
        (TextStyle::Button, FontId::new(100.0, Proportional)),
        (TextStyle::Small, FontId::new(100.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
