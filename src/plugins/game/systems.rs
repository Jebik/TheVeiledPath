use super::plugin::GameData;
use bevy::ecs::system::Commands;

pub fn setup_game(mut commands: Commands) {

    commands.insert_resource(GameData {
    });
}