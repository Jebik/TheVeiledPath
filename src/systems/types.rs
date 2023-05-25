use crate::map::map_manager::MapManager;

pub enum GameState {
    Menu,
    Game,
    GameOver,
}

pub struct StateManager {
    pub(crate) map_manager: MapManager,
    pub(crate) game_state: GameState,
}