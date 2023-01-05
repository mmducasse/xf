use crate::num::ivec2::IVec2;

use super::color::Color;


pub trait Bitmap {
    fn size(&self) -> IVec2;
    fn get_pixel(&self, pos: IVec2) -> Color;
    fn set_pixel(&mut self, pos: IVec2, color: Color);
}