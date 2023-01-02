
use serde_derive::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TileProperties {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonTile {
    pub id: i32,
    pub type_: Option<String>,
    pub properties: Option<Vec<TileProperties>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonTileset {
    pub columns: i32,
    pub image: String,
    pub imageheight: i32,
    pub imagewidth: i32,
    pub name: String,
    pub spacing: i32,
    pub tilecount: i32,
    pub tileheight: i32,
    pub tilewidth: i32,
    pub tiles: Vec<JsonTile>,
}

impl JsonTileset {
    // pub fn load(bytes: &[u8]) -> JsonTileset {
    //     let contents = String::from_utf8(bytes.to_vec()).unwrap_or_else(|e| {
    //         println!("{}", e.to_string());
    //         panic!();
    //     });
        
    //     let tileset: JsonTileset = serde_json::from_str(contents.as_str()).unwrap_or_else(|e| {
    //         println!("{}", e.to_string());
    //         panic!();
    //     });

    //     tileset
    // }
}