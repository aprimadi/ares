use std::collections::HashMap;
use std::hash::Hash;

use mq::file::load_file;
use mq::text;
use mq::text::Font;
use mq::texture::{load_texture, Texture2D};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::AResult;
use crate::error::AError;

static INSTANCE: OnceCell<Assets> = OnceCell::new();

pub async fn load() -> AResult {
    assert!(INSTANCE.get().is_none());
    let assets = Assets::load().await?;
    INSTANCE.set(assets).expect("Can't set assets instance");
    Ok(())
}

pub fn get() -> &'static Assets {
    INSTANCE.get().expect("Assets weren't loaded")
}

/// Read a file to a string.
async fn read_file(path: &str) -> AResult<String> {
    let data = load_file(path).await?;
    Ok(String::from_utf8_lossy(&data[..]).to_string())
}

async fn deserialize_from_file<D: DeserializeOwned>(path: &str) -> AResult<D> {
    let s = read_file(path).await?;
    ron::de::from_str(&s).map_err(|e| AError::from_ron_de_error(e, path.into()))
}

async fn load_map<Key: Hash + Eq + Copy>(
    table: &[(Key, &str)],
    expand_path: fn(&str) -> String,
) -> AResult<HashMap<Key, Texture2D>> {
    let mut map = HashMap::new();
    for &(key, path) in table {
        map.insert(key, load_texture(&expand_path(path)).await?);
    }
    Ok(map)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpriteInfo {
    pub paths: HashMap<String, String>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub shadow_size_coefficient: f32,

    #[serde(default = "default_sub_tile_z")]
    pub sub_tile_z: f32,
}

fn default_sub_tile_z() -> f32 {
    0.0
}

type SpritesInfo = HashMap<String, SpriteInfo>;

#[derive(Debug)]
pub struct Assets {
    pub textures: Textures,
    pub font: Font,
    pub sprites_info: SpritesInfo,
    pub sprite_frames: HashMap<String, HashMap<String, Texture2D>>,
}

impl Assets {
    pub async fn load() -> AResult<Self> {
        let sprites_info: SpritesInfo = deserialize_from_file("sprites.ron").await?;
        let sprite_frames = {
            let mut sprite_frames = HashMap::new();
            for (obj_type, SpriteInfo { paths, .. }) in sprites_info.iter() {
                let mut frames = HashMap::new();
                for (frame_name, path) in paths {
                    frames.insert(frame_name.to_string(), load_texture(path).await?);
                }
                sprite_frames.insert(obj_type.clone(), frames);
            }
            sprite_frames
        };
        Ok(Self {
            textures: Textures::load().await?,
            font: text::load_ttf_font("OpenSans-Regular.ttf").await?,
            sprites_info,
            sprite_frames,
        })
    }
}

#[derive(Debug)]
pub struct Textures {
    pub map: MapObjectTextures,
    pub icons: IconTextures,
    pub dot: Texture2D,
}

impl Textures {
    async fn load() -> AResult<Self> {
        Ok(Self {
            map: MapObjectTextures::load().await?,
            icons: IconTextures::load().await?,
            dot: load_texture("img/dot.png").await?,
        })
    }
}

#[derive(Debug)]
pub struct MapObjectTextures {
    pub selection: Texture2D,
    pub white_hex: Texture2D,
    pub tile: Texture2D,
    pub tile_rocks: Texture2D,
    pub grass: Texture2D,
    pub blood: Texture2D,
    pub explosion_ground_mark: Texture2D,
    pub shadow: Texture2D,
}

impl MapObjectTextures {
    async fn load() -> AResult<Self> {
        Ok(Self {
            selection: load_texture("img/selection.png").await?,
            white_hex: load_texture("img/white_hex.png").await?,
            tile: load_texture("img/tile.png").await?,
            tile_rocks: load_texture("img/tile_rocks.png").await?,
            grass: load_texture("img/grass.png").await?,
            blood: load_texture("img/blood.png").await?,
            explosion_ground_mark: load_texture("img/explosion_ground_mark.png").await?,
            shadow: load_texture("img/shadow.png").await?,
        })
    }
}

#[derive(Debug)]
pub struct IconTextures {
    pub info: Texture2D,
    pub end_turn: Texture2D,
    pub main_menu: Texture2D,
}

impl IconTextures {
    async fn load() -> AResult<Self> {
        Ok(Self {
            info: load_texture("img/icon_info.png").await?,
            end_turn: load_texture("img/icon_end_turn.png").await?,
            main_menu: load_texture("img/icon_menu.png").await?,
        })
    }
}

