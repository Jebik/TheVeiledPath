use bevy::{
    prelude::{GamepadAxisType, GamepadButtonType, KeyCode, Query, Res, ResMut, Transform, With},
    time::Time,
};

use crate::plugins::input::types::{Action, InputData, InputMap};

use super::{systems::PlayerPosition, engine::{GameData, SizeDate}};

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
    size_date: Res<SizeDate>,
    mut game_data: ResMut<GameData>,
    mut query: Query<&mut Transform, With<PlayerPosition>>,
    input_data: Res<InputData>,
) {
    let move_x = input_data.left_stick_x * time.delta_seconds();
    let move_y = input_data.left_stick_y * time.delta_seconds();
    if (move_x > 0.1 || move_x < -0.1) && (move_y > 0.1 || move_y < -0.1) {
        game_data.player.dir_x = move_x;
        game_data.player.dir_y = move_y;
    }
    game_data.player.x += move_x * 2.;
    game_data.player.y += move_y * 2.;
    
    for mut transform in query.iter_mut() {
        let world_x = size_date.get_world_x(game_data.player.x);
        let world_y = size_date.get_world_y(game_data.player.y);
        transform.translation.x = world_x;
        transform.translation.y = world_y;
    }
}
