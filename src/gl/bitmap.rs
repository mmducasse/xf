use crate::num::{ivec2::IVec2, irect::{IRect, ir}};

use super::{
    color::Color, shader::DrawPixel, 
    texture::Texture, 
    draw_params::DrawParams
};


pub trait Bitmap {
    fn size(&self) -> IVec2;
    fn get_pixel(&self, pos: IVec2) -> Color;
    unsafe fn set_pixel(&mut self, pos: IVec2, color: Color);

    fn draw_rect(&mut self, rect: IRect, color: Color) {
        let rect = rect.intersection(IRect::of_size(self.size())).unwrap_or(IRect::ZERO);
        for pos in rect.iter() {
            unsafe {
                self.set_pixel(pos, color);
            }
        }
    }

    #[inline]
    fn draw_texture_full(&mut self, texture: &Texture, dst_pt: IVec2) {
        self.draw_texture(texture, texture.bounds(), dst_pt);
    }

    fn draw_texture(&mut self, texture: &Texture, src: IRect, dst_pt: IVec2) {
        let params = DrawParams {
            src: Some(src),
            shader: None,
        };

        self.draw_texture_x(texture, dst_pt, params);
    }

    fn draw_texture_x(&mut self, texture: &Texture, dst_pt: IVec2, params: DrawParams) {
        let src = params.src.unwrap_or(texture.bounds());
        let dst = ir(dst_pt, src.size)
            .intersection(IRect::of_size(self.size()))
            .unwrap_or(IRect::ZERO);

        let src = ir(src.pos, dst.size);
        let offset = dst_pt - src.pos;

        for src_pt in src.iter() {
            let dst = src_pt + offset;
            if let Some(&color) = texture.get(src_pt) {
                let mut dp = DrawPixel { org: IVec2::ZERO, dst, color };
                if let Some(shader) = &params.shader {
                    dp = shader.apply(dp, texture);
                }

                unsafe {
                    self.set_pixel(dp.dst, dp.color);
                }
            }
        }
    }
}