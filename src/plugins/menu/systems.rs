use bevy::{prelude::{Res, info, EventWriter}, app::AppExit};
use bevy_egui::{ egui::{self}, EguiContexts};
use crate::plugins::state::types::StateManager;

pub fn menu_ui(
    mut contexts: EguiContexts,
    state_manager: Res<StateManager>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Menu");

            if ui.add(egui::Button::new("Tuto").fill(egui::Color32::RED)).clicked() {
                info!("Tuto clicked");
            }
            if ui.add(egui::Button::new("Level 1").fill(egui::Color32::RED)).clicked() {
                info!("Level 1 clicked");
            }
            if let Some(_) = state_manager.map_manager.custom_map {
                if ui.add(egui::Button::new("Custom").fill(egui::Color32::RED)).clicked() {
                    info!("Custom clicked");
                }
            }
            if ui.add(egui::Button::new("Quit").fill(egui::Color32::RED)).clicked() {
                app_exit_events.send(AppExit);
            }
        });
    });
}