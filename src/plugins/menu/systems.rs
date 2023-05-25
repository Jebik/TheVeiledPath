use bevy::prelude::{Res, info};
use bevy_egui::{ egui::{self, Frame}, EguiContexts};
use crate::plugins::state::types::StateManager;

pub fn menu_ui(
    mut contexts: EguiContexts,
    state_manager: Res<StateManager>
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            if ui.button("Tuto").clicked() {
                info!("Tuto clicked");
            }
            if ui.button("Level 1").clicked() {
                info!("Level clicked");
            }
            if let Some(_) = state_manager.map_manager.custom_map {
                if ui.button("Custom").clicked() {
                    info!("Custom clicked");
                }
            }
            if ui.button("Quit").clicked() {
                info!("Quit clicked");
            }
    });
}