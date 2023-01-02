
use std::rc::Rc;

use crate::{data::arr2d::Arr2D, gl::texture::Texture};

pub struct Tileset<Tile>
{
    pub tiles: Arr2D<Tile>,
    pub texture: Rc<Texture>,
}

impl<Tile> Clone for Tileset<Tile>
where Tile: Clone {
    fn clone(&self) -> Self {
        Self { 
            tiles: self.tiles.clone(), 
            texture: self.texture.clone() 
        }
    }
}
