use bevy::{
    prelude::{info, GamepadAxisType, GamepadButtonType, KeyCode, Query, Res, ResMut},
    time::Time,
};

use crate::plugins::input::types::{Action, InputData, InputMap};

pub fn setup_input(mut input_map: ResMut<InputMap>) {
    // bind keyboard keys
    input_map.keyboard_map.insert(KeyCode::Space, Action::ButtonA);
    input_map.keyboard_map.insert(KeyCode::Up, Action::LeftStickY(-1.0));
    input_map.keyboard_map.insert(KeyCode::Down, Action::LeftStickY(1.0));
    input_map.keyboard_map.insert(KeyCode::Left, Action::LeftStickX(-1.0));
    input_map.keyboard_map.insert(KeyCode::Right, Action::LeftStickX(1.0));

    // bind gamepad axis and buttons
    input_map.gamepad_axis_map.insert(GamepadAxisType::LeftStickX, Action::LeftStickX(1.0));
    input_map.gamepad_axis_map.insert(GamepadAxisType::LeftStickY, Action::LeftStickY(-1.0));
    input_map.gamepad_button_map.insert(GamepadButtonType::South, Action::ButtonA);
    input_map.gamepad_button_map.insert(GamepadButtonType::East, Action::ButtonB);
    input_map.gamepad_button_map.insert(GamepadButtonType::West, Action::ButtonX);
    input_map.gamepad_button_map.insert(GamepadButtonType::North, Action::ButtonY);
}

pub fn move_system(
    time: Res<Time>,
    //mut query: Query<&mut Transform, With<Player>>,
    input_data: Res<InputData>,
) {
    let move_x = input_data.left_stick_x * time.delta_seconds();
    let move_y = input_data.left_stick_y * time.delta_seconds();
    //info!("move_x: {}, move_y: {}", move_x, move_y)
}
