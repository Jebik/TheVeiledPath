use bevy::{
    prelude::{GamepadAxisType, GamepadButtonType, KeyCode, Query, Res, ResMut, Transform, With, Handle, Camera2d, Assets, Vec2},
    time::Time, core_pipeline::clear_color::ClearColorConfig,
};

use crate::{plugins::input::types::{Action, InputData, InputMap}};
use map_shared::Dimension;

use super::{systems::{PlayerPosition, FullScreen}, engine::{GameData, SizeDate}, dimension::DimensionHandle, tutorial::Tutorial, shader::DimensionMaterial};

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
    mut tutorial: ResMut<Tutorial>,
    mut materials: ResMut<Assets<DimensionMaterial>>,
    mut player_query: Query<&mut Transform, With<PlayerPosition>>,
    mut texture_query: Query<&mut Handle<DimensionMaterial>, With<FullScreen>>,
    mut camera_query: Query<&mut Camera2d, With<FullScreen>>,   
) {
    tutorial.check_message(&mut game_data);
    if tutorial.current_message_index.is_some() {        
        if input_data.button_a {
            tutorial.delete_message();
        } 
    } else {
        let dir_x = input_data.left_stick_x;
        let dir_y = input_data.left_stick_y;

        let move_x = dir_x * time.delta_seconds();
        let move_y = dir_y * time.delta_seconds();
        if (dir_x > 0.01 || dir_x < -0.01) || (dir_y > 0.01 || dir_y < -0.01) {
            let dir_length = (dir_x * dir_x + dir_y * dir_y).sqrt();
            game_data.player.dir_x = dir_x/dir_length;
            game_data.player.dir_y = dir_y/dir_length;
        }
        game_data.player.x += move_x * 2.;
        game_data.player.y += move_y * 2.;
        let world_x = size_date.get_world_x(game_data.player.x);
        let world_y = size_date.get_world_y(game_data.player.y);
            
        for mut transform in player_query.iter_mut() {
            transform.translation.x = world_x;
            transform.translation.y = world_y;
        }
        let light_handle = dimension.get_shader_handle(Dimension::Light);
        let dark_handle = dimension.get_shader_handle(Dimension::Dark);

        let light_material = materials.get_mut(&light_handle).unwrap();
        light_material.shader_data.player_position = Vec2::ZERO;//Vec2::new(game_data.player.x, game_data.player.y);
        light_material.shader_data.player_direction = Vec2::new(game_data.player.dir_x, game_data.player.dir_y);

        let dark_material = materials.get_mut(&dark_handle).unwrap();
        dark_material.shader_data.player_position = Vec2::ZERO;//Vec2::new(game_data.player.x, game_data.player.y);
        dark_material.shader_data.player_direction = Vec2::new(game_data.player.dir_x, game_data.player.dir_y);

        if input_data.button_a {
            if game_data.dimension_enabled {
                switch_dimension(
                    &mut game_data, 
                    &dimension, 
                    &mut texture_query,
                    &mut camera_query
                );
            }
        }
    }
}

fn switch_dimension(
    game_data: &mut GameData,
    dimension: &DimensionHandle,
    texture_query: &mut Query<&mut Handle<DimensionMaterial>, With<FullScreen>>,
    camera_query: &mut Query<&mut Camera2d, With<FullScreen>>,    
) {
    game_data.dimension.switch_dimension();

    let mew_shader_handle = dimension.get_shader_handle(game_data.dimension);
    let mew_clear_color = dimension.get_clear_color(game_data.dimension);
    for mut material_handle in texture_query.iter_mut() {
        *material_handle = mew_shader_handle.clone();
    }
    for mut camera in camera_query.iter_mut() {
        camera.clear_color = ClearColorConfig::Custom(mew_clear_color);
    }
}
