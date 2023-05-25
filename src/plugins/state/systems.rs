use super::types::StateManager;
use bevy::ecs::system::Resource;
use bevy::prelude::{EventReader, Res, info, Query};
use bevy::window::{WindowResized, Window};

pub fn window_resize_system(mut resize_reader: EventReader<WindowResized>) {   
    for e in resize_reader.iter() {
        info!("Window was resized to {} x {}", e.width, e.height);
    }
}

impl Resource for StateManager {}
