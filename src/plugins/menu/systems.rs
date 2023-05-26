use bevy::{prelude::{EventWriter}, app::AppExit};
use bevy_egui::{ egui::{self}, EguiContexts};
use bevy::ecs::system::ResMut;
use bevy::ecs::schedule::NextState;
use crate::plugins::state::types::GameState;
use crate::map::map_manager::MapManager;
use super::plugin::LevelChoice;

pub fn menu_ui(
    mut contexts: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    map: ResMut<MapManager>,
    mut level: ResMut<LevelChoice>,
    mut state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Menu");

            if ui.add(egui::Button::new("Tuto").fill(egui::Color32::RED)).clicked() {
                *level = LevelChoice::Tutorial;
                state.set(GameState::Game);
            }
            if ui.add(egui::Button::new("Level 1").fill(egui::Color32::RED)).clicked() {
                *level = LevelChoice::Level1;
                state.set(GameState::Game);
            }
            if let Some(_) = map.custom_map {
                if ui.add(egui::Button::new("Custom").fill(egui::Color32::RED)).clicked() {
                    *level = LevelChoice::Custom;
                    state.set(GameState::Game);
                }
            }
            if ui.add(egui::Button::new("Quit").fill(egui::Color32::RED)).clicked() {
                app_exit_events.send(AppExit);
            }
        });
    });
}