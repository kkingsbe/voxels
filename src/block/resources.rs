use bevy::prelude::*;

pub struct BlockTexture {
    pub handle: Handle<Image>,
    pub name: String
}

#[derive(Resource, Default)]
pub struct BlockTextureAtlas {
    textures: Vec<BlockTexture>
}

impl BlockTextureAtlas {
    pub fn get_texture_by_name(&self, name: String) -> Option<&BlockTexture> {
        for texture in self.textures.iter() {
            if texture.name == name {
                return Some(texture);
            }
        }

        return None;
    }

    pub fn load_texture(
        &mut self, 
        texture_url: String, 
        texture_name: String,
        asset_server: &Res<AssetServer>
    ) {
        self.textures.push(BlockTexture {
            handle: asset_server.load(texture_url),
            name: texture_name
        });
    }

    pub fn load_all_textures(
        &mut self,
        asset_server: Res<AssetServer>
    ) {
        let texture_urls = [
            "textures/wood.png",
            "textures/stone.png",
            "textures/dirt.png",
        ];

        let texture_names = [
            "wood",
            "stone",
            "dirt"
        ];

        let mut i: usize = 0;
        for texture_url in texture_urls.iter() {
            self.load_texture(texture_url.to_string(), texture_names[i].to_string(), &asset_server);
            i += 1;
        }
    }
}