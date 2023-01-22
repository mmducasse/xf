use crate::{data::arr2d::Arr2D, num::ivec2::IVec2};

use super::{color::Color, bitmap::Bitmap};

pub type Texture = Arr2D<Color>;

const DEFAULT: Color = Color::BLACK;

impl Texture {
    pub fn of_size(size: IVec2) -> Self {
        Arr2D::default(DEFAULT, size)
    }
}

impl Bitmap for Texture {
    fn size(&self) -> IVec2 { self.size() }

    fn get_pixel(&self, pos: IVec2) -> Color {
        *self.get(pos).unwrap_or(&DEFAULT)
    }

    unsafe fn set_pixel(&mut self, pos: IVec2, color: Color) {
        self.set(pos, color);
    }
}