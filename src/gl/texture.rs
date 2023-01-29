use crate::{
    data::arr2d::Arr2D, num::{ivec2::IVec2, irect::IRect}};

use super::{color::Color, bitmap::{Bitmap, draw_rect_default, draw_texture_x_default}, draw_params::DrawParams};

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

    fn draw_rect(&mut self, rect: IRect, color: Color) {
        draw_rect_default(self, rect, color);
    }

    fn draw_texture_x(&mut self, texture: &Texture, dst_pt: IVec2, params: DrawParams) {
        draw_texture_x_default(self, texture, dst_pt, params);
    }
}