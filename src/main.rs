use bevy::prelude::*;
use block::{components::{BlockPosition, BlockTextures, Block, BlockType, BlockFace}, resources::BlockTextureAtlas};
use noise::{NoiseFn, Perlin};
use player::*;
use rand::prelude::*;

pub mod block;
pub mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .init_resource::<BlockTextureAtlas>()
        .add_systems(Startup, spawn_lighting)
        .add_systems(Startup, initialize_textures)
        .add_systems(Startup, load_initial_blocks.after(initialize_textures))
        .run()
}

fn initialize_textures(
    asset_server: Res<AssetServer>,
    mut block_texture_atlas: ResMut<BlockTextureAtlas>
) {
    block_texture_atlas.load_all_textures(asset_server);
}

fn load_initial_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    block_texture_atlas: ResMut<BlockTextureAtlas>
) {
    let perlin = Perlin::new(random());
    let block_size = 1.0;
    let noise_scale = 2.0;
    let map_size = 20;
    let map_height = 20;
    let mut terrain_height = vec![vec![0; map_size]; map_size];

    //First, we need to generate the terrain height map
    for x in 0..map_size {
        for z in 0..map_size {
            let noise_value = perlin.get([x as f64 / 10.0, z as f64 / 10.0]) as f32;
            let block_height = (noise_value * noise_scale).floor() as i32; 
            terrain_height[x][z] = block_height;
        }
    }

    for x in 0..map_size {
        for z in 0..map_size {
            for y in 0..map_height {
                let is_surface = y == terrain_height[x][z];

                if is_surface {
                    let block_position = BlockPosition {
                        x: x as f32,
                        y: y as f32,
                        z: z as f32
                    };

                    let left_obscured = x > 0 && terrain_height[x - 1][z] >= y;
                    let right_obscured = x < map_size - 1 && terrain_height[x + 1][z] >= y;
                    let front_obscured = z < map_size - 1 && terrain_height[x][z + 1] >= y;
                    let back_obscured = z > 0 && terrain_height[x][z - 1] >= y;
                    
                    let mut obscured_faces = Vec::<BlockFace>::new();
                    if left_obscured {
                        obscured_faces.push(BlockFace::LEFT);
                    }
                    if right_obscured {
                        obscured_faces.push(BlockFace::RIGHT);
                    }
                    if front_obscured {
                        obscured_faces.push(BlockFace::FRONT);
                    }
                    if back_obscured {
                        obscured_faces.push(BlockFace::BACK);
                    }

                    let texture = "dirt".to_string();
                    Block {
                        textures: BlockTextures {
                            back: texture.clone(),
                            top: texture.clone(),
                            left: texture.clone(),
                            right: texture.clone(),
                            front: texture.clone(),
                            bottom: texture.clone()
                        },
                        size: block_size,
                        block_type: BlockType::SOLID,
                        opacity: 1.0,
                        speed: 0.0,
                        position: block_position
                    }.spawn(
                        &mut commands, 
                        &block_texture_atlas,
                        &mut materials, 
                        &mut meshes,
                        obscured_faces
                    );
                }
            }
        }
    }
}

fn spawn_lighting(
    mut commands: Commands
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}