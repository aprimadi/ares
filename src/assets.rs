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
    pub font: Font,
}

impl Assets {
    pub async fn load() -> AResult<Self> {
        Ok(Self {
            font: text::load_ttf_font("OpenSans-Regular.ttf").await?,
        })
    }
}

#[derive(Debug)]
pub struct MapObjectTextures {
    pub tile: Texture2D,
}

impl MapObjectTextures {
    async fn load() -> AResult<Self> {
        Ok(Self {
            tile: load_texture("textures/tile.png").await?,
        })
    }
}

