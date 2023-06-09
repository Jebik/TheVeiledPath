use bevy::prelude::States;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    Menu,
    Game,
    Over,
    Win,
}
impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}