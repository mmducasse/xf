use std::rc::Rc;

use crate::{
    map::{
        tileset::Tileset, 
        tiled_json::tileset::{JsonTileset, JsonTile}
    },
    data::arr2d::Arr2D, 
    gl::texture::Texture
};


pub fn load_tiled_tileset<Tile, F>(bytes: &[u8], texture: Texture, tile_fn: F) 
    -> Result<Tileset<Tile>, String>
where Tile: Default + Clone,
      F: Fn(&JsonTile) -> Result<Tile, String>
{
    let json: JsonTileset = serde_json::from_slice(bytes).or_else(|e| {
        Err(e.to_string())
    })?;

    // Default the tileset.
    let tiles = vec![Tile::default(); json.tilecount];
    let mut tiles = Arr2D::new(tiles, json.columns);

    // Parse the special tiles.
    for tile in json.tiles.iter() {
        let idx = tile.id as usize;

        let tile = tile_fn(tile).or_else(|e| {
            Err(e.to_string())
        })?;

        tiles.set_i(idx, tile);
    }

    Ok(Tileset {
        tiles,
        texture: Rc::new(texture),
    })
}