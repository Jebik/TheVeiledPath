use crate::map::map_manager::MapManager;

pub enum GameState {
    Menu,
    _Game,
    _GameOver,
}

pub struct StateManager {
    pub(crate) map_manager: MapManager,
    pub(crate) _game_state: GameState,
}