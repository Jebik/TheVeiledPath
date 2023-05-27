use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Tutorial {
    
}
impl Tutorial {
    pub(crate) fn new() -> Tutorial {
        Tutorial {
        }
    }
}