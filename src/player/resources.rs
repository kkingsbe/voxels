use bevy::prelude::*;

#[derive(Resource)]
pub struct MouseLook {
    pub sensitivity: f32
}

impl Default for MouseLook {
    fn default() -> Self {
        Self {
            sensitivity: 0.005
        }
    }
}