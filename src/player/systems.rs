use bevy::{prelude::*, input::mouse::MouseMotion};

use super::{components::Player, resources::MouseLook};

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_player(
    mut commands: Commands
) {
    commands.spawn(Player {
        speed: 0.1,
        ..default()
    });
}

pub fn mouse_look(
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mouse_look: ResMut<MouseLook>,
) {
    for ev in motion_evr.iter() {
        let mut camera_transform = camera_query.single_mut();
        // Apply yaw rotation (around global up axis)
        camera_transform.rotate(Quat::from_rotation_y(-ev.delta.x * mouse_look.sensitivity));

        // Apply pitch rotation (around local x axis)
        camera_transform.rotate_local(Quat::from_rotation_x(-ev.delta.y * mouse_look.sensitivity));
    }
}

pub fn player_movement(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let player = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    let forward = (camera_transform.local_z() * Vec3{x: 1.0, y: 0.0, z: 1.0}).normalize();
    let right = camera_transform.local_x().normalize();
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction -= right;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += right;
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction -= forward;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction += forward;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    camera_transform.translation += direction * player.speed;
}