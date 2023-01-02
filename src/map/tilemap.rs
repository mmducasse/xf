
use crate::{num::ivec2::IVec2, data::arr2d::Arr2D};

use super::tileset::Tileset;

pub struct Tilemap<Tile>
{
    pub tile_srcs: Arr2D<IVec2>,
    pub tileset: Tileset<Tile>,
}

impl<Tile> Tilemap<Tile>
where Tile: Clone
{
    pub fn size(&self) -> IVec2 { self.tile_srcs.size() }

    pub fn get(&self, pos: IVec2) -> Option<&Tile> {
        let src_pos = self.tile_srcs.get(pos)?;
        self.tileset.tiles.get(*src_pos)
    }
}

impl<Tile> Clone for Tilemap<Tile> 
where Tile: Clone
{
    fn clone(&self) -> Self {
        Self { 
            tile_srcs: self.tile_srcs.clone(), 
            tileset: self.tileset.clone() 
        }
    }
}