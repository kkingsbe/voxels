use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

use self::components::*;
use self::systems::*;
use self::resources::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MouseLook>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, mouse_look);
    }
}