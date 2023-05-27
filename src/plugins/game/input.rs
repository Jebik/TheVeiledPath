use bevy::{
    prelude::{GamepadAxisType, GamepadButtonType, KeyCode, Query, Res, ResMut, Transform, With, Handle},
    time::Time, sprite::{ColorMaterial},
};

use crate::{plugins::input::types::{Action, InputData, InputMap}};

use super::{systems::{PlayerPosition, FullScreen}, engine::{GameData, SizeDate}, dimension::DimensionHandle};

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
    input_data: Res<InputData>,
    dimension: Res<DimensionHandle>,
    mut game_data: ResMut<GameData>,
    mut player_query: Query<&mut Transform, With<PlayerPosition>>,
    mut texture_query: Query<&mut Handle<ColorMaterial>, With<FullScreen>>,
) {
    let move_x = input_data.left_stick_x * time.delta_seconds();
    let move_y = input_data.left_stick_y * time.delta_seconds();
    if (move_x > 0.1 || move_x < -0.1) && (move_y > 0.1 || move_y < -0.1) {
        game_data.player.dir_x = move_x;
        game_data.player.dir_y = move_y;
    }
    game_data.player.x += move_x * 2.;
    game_data.player.y += move_y * 2.;
    
    for mut transform in player_query.iter_mut() {
        let world_x = size_date.get_world_x(game_data.player.x);
        let world_y = size_date.get_world_y(game_data.player.y);
        transform.translation.x = world_x;
        transform.translation.y = world_y;
    }

    if input_data.button_a && game_data.dimension_enabled {
        switch_dimension(&mut game_data, &dimension, &mut texture_query);
    }
}

fn switch_dimension(
    game_data: &mut GameData,
    dimension: &DimensionHandle,
    texture_query: &mut Query<&mut Handle<ColorMaterial>, With<FullScreen>>,
) {
    game_data.dimension.switch_dimension();

    let mew_material_handle = dimension.get_material_handle(game_data.dimension);
    for mut material_handle in texture_query.iter_mut() {
        *material_handle = mew_material_handle.clone();
    }
}
