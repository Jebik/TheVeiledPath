use bevy::{prelude::{Resource, KeyCode, GamepadAxisType, GamepadButtonType, Gamepad}, utils::HashMap};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ButtonA,
    ButtonB,
    ButtonX,
    ButtonY,
    LeftStickX(f32),
    LeftStickY(f32)
}

#[derive(Default, Resource)]
pub struct InputMap {
    pub keyboard_map: HashMap<KeyCode, Action>,
    pub gamepad_axis_map: HashMap<GamepadAxisType, Action>,
    pub gamepad_button_map: HashMap<GamepadButtonType, Action>,
}

#[derive(Default, Resource)]
pub struct InputData {
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub button_a: bool,
    pub button_b: bool,
    pub button_x: bool,
    pub button_y: bool,
}

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);