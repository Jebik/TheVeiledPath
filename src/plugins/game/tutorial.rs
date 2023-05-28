use bevy::prelude::{Resource, Res};
use bevy_egui::{EguiContexts, egui};

use super::engine::GameData;

pub struct Message {
    pub text: String,
    pub col: i32,
}
#[derive(Resource)]
pub struct Tutorial {
    pub current_message_index: Option<usize>,
    message_list: Vec<Message>
}
impl Tutorial {
    pub(crate) fn new() -> Tutorial {
        Tutorial {
            current_message_index: None,
            message_list: Vec::new(),
        }
    }

    pub(crate) fn get_current_message(&self) -> Option<&Message> {
        if let Some(index) = self.current_message_index {
            self.message_list.get(index)
        } else {
            None
        }
    }

    pub(crate) fn add_message(&mut self, text: String, col: i32) {
        self.message_list.push(Message { text, col });
    }

    pub(crate) fn delete_message(&mut self) {
        if let Some(index) = self.current_message_index {
            self.message_list.remove(index);
            self.current_message_index = None;
        }
    }

    pub(crate) fn check_message(&mut self, game_data: &mut GameData) {
        if self.message_list.is_empty() {
            return;
        }

        let col = game_data.player.x as i32;
        if !game_data.dimension_enabled && col == 2 {
            game_data.dimension_enabled = true;
        }

        if self.current_message_index.is_none() {
            if let Some(message_idx) = self.message_list.iter().position(|m| m.col == col) {
                self.current_message_index = Some(message_idx);
            }
        }
    }
}


pub fn tuto_system(
    tutorial: Res<Tutorial>,
    mut contexts: EguiContexts,
) {
    if tutorial.current_message_index.is_some() {
        if let Some(message) = tutorial.get_current_message() {
            let ctx = contexts.ctx_mut();
    
            egui::TopBottomPanel::bottom("tutorial_text_panel").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(message.text.as_str());
                });
            });
        }
    }
}

pub fn init_tuto(
    game_data: &mut GameData,
    tutorial: &mut Tutorial,
) {
    game_data.dimension_enabled = false;
    tutorial.add_message("Welcome to The Veiled Path. Press A on Xbox GamePad or Space to Continue".to_string(), 0);
    tutorial.add_message("Your goal is to reach the end represented by a circle like you".to_string(), 0);
    tutorial.add_message("Control is left stick or arrow on keyboard".to_string(), 0);
    tutorial.add_message("A wall is blocking you if you touch it you die".to_string(), 2);
    tutorial.add_message("But you are lucky you can change dimension by pressing A or Space. Try It".to_string(), 2);
    tutorial.add_message("Be carefull the wall are still in the other dimension.".to_string(), 3);
    tutorial.add_message("If you switch back inside a wall you die".to_string(), 4);
    tutorial.add_message("You can see other dimension here every thing a litle lighter is a wall in the other dimmension".to_string(), 4);
    tutorial.add_message("Here the wall is present in both dimension you can't pass it".to_string(), 5);
    tutorial.add_message("But maybe one of the wall is actually a closed door. Do you see the grey button in the top of you?".to_string(), 5);
    tutorial.add_message("No door open? Are you sure you are in right dimension?".to_string(), 6);
    tutorial.add_message("Easy Right? This was only the tutorial real map is a bit more complexe".to_string(), 8);
}