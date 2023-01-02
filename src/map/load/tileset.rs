use std::rc::Rc;

use crate::{
    map::{
        tileset::Tileset, 
        tiled_json::tileset::{JsonTileset, JsonTile}
    }, 
    num::ivec2::i2, 
    data::arr2d::Arr2D, 
    gl::texture::Texture
};


pub fn load_tiled_tileset<Tile, F>(bytes: &[u8], texture: Texture, tile_fn: F) 
-> Result<Tileset<Tile>, String>
where Tile: Default + Clone,
      F: Fn(&JsonTile) -> Tile
{
    let json: JsonTileset = serde_json::from_slice(bytes).or_else(|e| {
        Err(e.to_string())
    })?;

    let w = json.columns;
    let h = json.tilecount / json.columns;
    let size = i2(w, h); // Todo: calculate size correctly.

    // Default the tileset.
    let mut tiles = Arr2D::default(Tile::default(), size);

    // Parse the special tiles.
    for (idx, tile) in json.tiles.iter().enumerate() {
        let idx = idx as i32;
        let pos = i2(idx % h, idx / h);
        tiles.set(pos, tile_fn(tile));
    }

    Ok(Tileset {
        tiles,
        texture: Rc::new(texture),
    })
}