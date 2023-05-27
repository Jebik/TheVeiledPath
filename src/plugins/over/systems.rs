use bevy::{prelude::{EventWriter, ResMut, NextState}, app::AppExit};
use bevy_egui::{EguiContexts, egui};
use crate::plugins::types::GameState;

pub fn over_ui(
    mut contexts: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    mut state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
        ui.heading("GAME OVER");

            if ui.add(egui::Button::new("Try again").fill(egui::Color32::RED)).clicked() {
                state.set(GameState::Game);
            }
            if ui.add(egui::Button::new("Menu").fill(egui::Color32::RED)).clicked() {
                state.set(GameState::Menu);
            }
            if ui.add(egui::Button::new("Quit").fill(egui::Color32::RED)).clicked() {
                app_exit_events.send(AppExit);
            }
        });
    });
}