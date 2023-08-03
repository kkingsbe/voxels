use bevy::prelude::*;

use super::resources::BlockTextureAtlas;

#[derive(PartialEq)]
pub enum BlockFace {
    BACK,
    TOP,
    LEFT,
    RIGHT,
    FRONT,
    BOTTOM
}

pub struct BlockTextures {
    pub back: String,
    pub top: String,
    pub left: String,
    pub right: String,
    pub front: String,
    pub bottom: String
}

pub enum BlockType {
    AIR,
    LIQUID,
    SOLID
}

#[derive(Debug)]
pub struct BlockPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Component)]
pub struct Block {
    pub textures: BlockTextures,
    pub size: f32,
    pub block_type: BlockType,
    pub opacity: f32,
    pub speed: f32,
    pub position: BlockPosition
}

impl Block {
    pub fn get_local_face_transformation(&self, block_face: BlockFace) -> Transform {
        let block_position = &self.position;
        let block_size = &self.size;

        let top_transform = Transform::from_xyz(block_position.x, block_position.y + block_size / 2.0, block_position.z);
        let right_transform = Transform::from_xyz(block_position.x + block_size / 2.0, block_position.y, block_position.z).with_rotation(Quat::from_rotation_z(3.0 * std::f32::consts::PI/2.0));
        let bottom_transform = Transform::from_xyz(block_position.x, block_position.y - block_size / 2.0, block_position.z).with_rotation(Quat::from_rotation_x(std::f32::consts::PI));
        let left_transform = Transform::from_xyz(block_position.x - block_size / 2.0, block_position.y, block_position.z).with_rotation(Quat::from_rotation_z(std::f32::consts::PI/2.0));
        let back_transform = Transform::from_xyz(block_position.x, block_position.y, block_position.z - block_size / 2.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            (270.0_f32).to_radians(),
            (90.0_f32).to_radians(),
            (0.0_f32).to_radians(),
        ));
        let front_transform = Transform::from_xyz(block_position.x, block_position.y, block_position.z + block_size / 2.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            (90.0_f32).to_radians(),
            (90.0_f32).to_radians(),
            (0.0_f32).to_radians(),
        ));
    
        let final_transform = match block_face {
            BlockFace::BACK => back_transform,
            BlockFace::TOP => top_transform,
            BlockFace::LEFT => left_transform,
            BlockFace::RIGHT => right_transform,
            BlockFace::FRONT => front_transform,
            BlockFace::BOTTOM => bottom_transform,
        };
    
        return final_transform;
    }

    pub fn spawn(
        &self, 
        commands: &mut Commands, 
        block_texture_atlas: &ResMut<BlockTextureAtlas>, 
        mut materials: &mut ResMut<Assets<StandardMaterial>>, 
        mut meshes: &mut ResMut<Assets<Mesh>>,
        obscured_faces: Vec<BlockFace>
    ) {
        if !obscured_faces.contains(&BlockFace::BACK) {
            self.load_face(
                commands,
                self.textures.back.clone(),
                self.get_local_face_transformation(BlockFace::BACK),
                &block_texture_atlas,
                &mut materials,
                &mut meshes
            );
        }

        if !obscured_faces.contains(&BlockFace::FRONT) {
            self.load_face(
                commands,
                self.textures.front.clone(),
                self.get_local_face_transformation(BlockFace::FRONT),
                &block_texture_atlas,
                &mut materials,
                &mut meshes
            );
        }

        if !obscured_faces.contains(&BlockFace::LEFT) {
            self.load_face(
                commands,
                self.textures.left.clone(),
                self.get_local_face_transformation(BlockFace::LEFT),
                &block_texture_atlas,
                &mut materials,
                &mut meshes
            );
        }

        if !obscured_faces.contains(&BlockFace::RIGHT) {
            self.load_face(
                commands,
                self.textures.right.clone(),
                self.get_local_face_transformation(BlockFace::RIGHT),
                &block_texture_atlas,
                &mut materials,
                &mut meshes
            );
        }

        self.load_face(
            commands,
            self.textures.top.clone(),
            self.get_local_face_transformation(BlockFace::TOP),
            &block_texture_atlas,
            &mut materials,
            &mut meshes
        );
    
        /*
        self.load_face(
            commands,
            self.textures.bottom.clone(),
            self.get_local_face_transformation(BlockFace::BOTTOM),
            &block_texture_atlas,
            &mut materials,
            &mut meshes
        );
        */
    }

    fn load_face(
        &self,
        commands: &mut Commands,
        texture_name: String,
        transform: Transform,
        block_texture_atlas: &ResMut<BlockTextureAtlas>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let texture = &block_texture_atlas.get_texture_by_name(texture_name).unwrap().handle;
    
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(1.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                ..default()
            }.into()),
            transform,
            ..default()
        });
    }
}