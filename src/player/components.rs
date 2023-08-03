use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub position: PlayerPosition
}

impl Default for Player {
    fn default() -> Self {
        Player {
            speed: 1.0,
            position: PlayerPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        }
    }
}

pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32
}