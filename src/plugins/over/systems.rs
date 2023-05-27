use crate::plugins::types::GameState;
use bevy::{
    app::AppExit,
    prelude::{EventWriter, NextState, ResMut},
};
use bevy_egui::{egui, EguiContexts};

pub fn over_ui(
    mut contexts: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    mut state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("GAME OVER");
            ui.add_space(15.);

            if ui.add(egui::Button::new("Try again")).clicked() {
                state.set(GameState::Game);
            }
            ui.add_space(15.);
            if ui.add(egui::Button::new("Menu")).clicked() {
                state.set(GameState::Menu);
            }
            ui.add_space(15.);
            if ui.add(egui::Button::new("Quit")).clicked() {
                app_exit_events.send(AppExit);
            }
        });
    });
}
