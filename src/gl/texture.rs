use crate::{data::arr2d::Arr2D, num::ivec2::IVec2};

use super::color::Color;

pub type Texture = Arr2D<Color>;

impl Texture {
    pub fn of_size(size: IVec2) -> Self {
        Arr2D::default(Color::BLACK, size)
    }
}