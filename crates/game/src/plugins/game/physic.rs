use super::{engine::GameData, systems::DoorId};
use crate::{plugins::{game::map::{Cell, ItemType}, types::GameState}};
use bevy::prelude::{NextState, ResMut, Commands, Query, Entity};

pub fn physic_system(
    mut commands: Commands, 
    mut game_data: ResMut<GameData>,
    mut state: ResMut<NextState<GameState>>,    
    query: Query<(Entity, &DoorId)>
) {
    /*
        BASIC PHYSIC SYSTEM
        WE ONLY CHECK 8 BOX AROUND AND GROUND
    */
    let x = game_data.player.x as i32;
    let y = game_data.player.y as i32;
    let mut surrounding_cells = Vec::new();

    for dx in -1..=1 {
        for dy in -1..=1 {
            let cell = game_data.map.at(x + dx, y + dy, game_data.dimension);
            match cell {
                Some(cell) => surrounding_cells.push(cell),
                None => { 
                    surrounding_cells.push(Cell {
                        x: (x + dx) as f32,
                        y: (y + dy) as f32,
                        item_type: ItemType::Wall,
                    });
                }
            }
        }
    }

    for cell in &mut surrounding_cells {
        match &mut cell.item_type {
            ItemType::Wall => {
                if check_wall_collision(game_data.player.x, game_data.player.y, cell.x, cell.y) {
                    state.set(GameState::Over);
                }
            }
            ItemType::Door(door) => {
                if door.open {
                    continue;
                } else if check_wall_collision(game_data.player.x, game_data.player.y, cell.x, cell.y) {
                    state.set(GameState::Over);                    
                }
            },
            ItemType::Key(key) => {
                if check_circle_collision(game_data.player.x, game_data.player.y, cell.x, cell.y) {
                    game_data.map.open_door(&mut commands, key.door_id, &query);
                }
            },
            ItemType::Goal => {
                if check_circle_collision(game_data.player.x, game_data.player.y, cell.x, cell.y) {
                    state.set(GameState::Win);
                }
            },
            ItemType::None => ()
        }
    }
}

fn check_circle_collision(player_x: f32, player_y: f32, key_x: f32, key_y: f32) -> bool {
    let circle_radius = 0.3; // radius of the circle (player), assuming diameter is 0.3

    // Calculate the distance between the two circles' centers
    let distance_x = player_x - key_x;
    let distance_y = player_y - key_y;
    let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

    // Check if the distance is less than or equal to the sum of the radii
    distance <= (circle_radius + circle_radius)
}

fn check_wall_collision(player_x: f32, player_y: f32, wall_x: f32, wall_y: f32) -> bool {
    let half_size = 0.5; // half size of the square (wall), assuming unit size is 1
    let circle_radius = 0.3; // radius of the circle (player), assuming diameter is 0.7

    // Calculate the closest point on the square to the circle
    let closest_x = if player_x < wall_x - half_size {
        wall_x - half_size
    } else if player_x > wall_x + half_size {
        wall_x + half_size
    } else {
        player_x
    };

    let closest_y = if player_y < wall_y - half_size {
        wall_y - half_size
    } else if player_y > wall_y + half_size {
        wall_y + half_size
    } else {
        player_y
    };

    // Calculate the distance between the circle's center and the closest point
    let distance_x = player_x - closest_x;
    let distance_y = player_y - closest_y;
    let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

    // Check if the distance is less than or equal to the radius of the circle
    distance <= circle_radius
}
