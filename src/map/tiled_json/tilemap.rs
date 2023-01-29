use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub name: String,
    pub id: i32,
    pub type_: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Layer {
    Tilelayer {
        data: Vec<usize>,
        height: i32,
        width: i32,
        id: i32,
        name: String,
    },
    Objectgroup {
        objects: Vec<Object>,
        id: i32,
        name: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    pub source: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonTilemap {
    pub height: i32,
    pub width: i32,
    pub tileheight: i32,
    pub tilewidth: i32,
    pub tilesets: Vec<Tileset>,
    pub layers: Vec<Layer>,
}

impl JsonTilemap {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(bytes).or_else(|e| Err(e.to_string()))
    }
}
