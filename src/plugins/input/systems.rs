//use bevy::prelude::*;
use bevy::input::gamepad::{GamepadAxis, GamepadButton};
use bevy::prelude::{Axis, Input, KeyCode, Res, ResMut, Gamepads};
use super::types::{InputData, InputMap, Action};

pub fn handle_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut input_data: ResMut<InputData>,
    input_map: Res<InputMap>,
    gamepads: Res<Gamepads>,
) {
    // Reset input data
    *input_data = InputData::default();

    // Handle keyboard inputs
    for (key, action) in input_map.keyboard_map.iter() {
        if keyboard_input.just_pressed(*key) {
            if let Action::ButtonA = action {
                input_data.button_a = true;
            }
            if let Action::ButtonB = action {
                input_data.button_b = true;
            }
            if let Action::ButtonX = action {
                input_data.button_x = true;
            }
            if let Action::ButtonY = action {
                input_data.button_y = true;
            }
        } else if keyboard_input.pressed(*key) {            
            if let Action::LeftStickX(x) = action {
                input_data.left_stick_x = *x;
            }
            if let Action::LeftStickY(y) = action {
                input_data.left_stick_y = *y;
            }
        }
    }

    // Handle gamepad inputs
    if let Some(gamepad) = gamepads.iter().next() {
        for (axis_type, action) in input_map.gamepad_axis_map.iter() {
            let gamepad_axis = GamepadAxis {
                gamepad,
                axis_type: *axis_type,
            };
            if let Some(val) = axes.get(gamepad_axis) {
                if let Action::LeftStickX(x) = action {
                    input_data.left_stick_x += *x * val;
                }
                if let Action::LeftStickY(y) = action {
                    input_data.left_stick_y += *y * val;
                }
            }
        }

        for (button_type, action) in input_map.gamepad_button_map.iter() {
            let gamepad_button = GamepadButton {
                gamepad,
                button_type: *button_type,
            };
            if buttons.just_pressed(gamepad_button) {
                if let Action::ButtonA = action {
                    input_data.button_a = true;
                }
                if let Action::ButtonB = action {
                    input_data.button_b = true;
                }
                if let Action::ButtonX = action {
                    input_data.button_x = true;
                }
                if let Action::ButtonY = action {
                    input_data.button_y = true;
                }
            }
        }
    }
}
