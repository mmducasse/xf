
use crate::num::ivec2::IVec2;

pub trait Tileset<Tile> {
    fn get(&self, src: IVec2) -> &Tile;
    fn texture(&self) -> (); // -> &Texture
}