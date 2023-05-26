use bevy::prelude::{ResMut, KeyCode, GamepadAxisType, GamepadButtonType};

use crate::plugins::input::types::{InputMap, Action};

pub fn setup_input(mut input_map: ResMut<InputMap>) {
    // bind keyboard keys
    input_map.keyboard_map.insert(KeyCode::Space, Action::ButtonA);
    input_map.keyboard_map.insert(KeyCode::Up, Action::LeftStickY(-1.0));
    input_map.keyboard_map.insert(KeyCode::Down, Action::LeftStickY(1.0));
    input_map.keyboard_map.insert(KeyCode::Left, Action::LeftStickY(-1.0));
    input_map.keyboard_map.insert(KeyCode::Right, Action::LeftStickY(1.0));

    // bind gamepad axis and buttons
    input_map.gamepad_axis_map.insert(GamepadAxisType::LeftStickX, Action::LeftStickX(1.0));
    input_map.gamepad_axis_map.insert(GamepadAxisType::LeftStickY, Action::LeftStickY(1.0));
    input_map.gamepad_button_map.insert(GamepadButtonType::South, Action::ButtonA);
    input_map.gamepad_button_map.insert(GamepadButtonType::East, Action::ButtonB);
    input_map.gamepad_button_map.insert(GamepadButtonType::West, Action::ButtonX);
    input_map.gamepad_button_map.insert(GamepadButtonType::North, Action::ButtonY);
}

/*
fn move_player_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
    input_data: Res<InputData>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x += input_data.dir.x * time.delta_seconds();
        transform.translation.y += input_data.dir.y * time.delta_seconds();
    }
}
*/