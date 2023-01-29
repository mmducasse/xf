use crate::num::{
    irect::{ir, IRect},
    ivec2::IVec2,
};

use super::{color::Color, draw_params::DrawParams, shader::DrawPixel, texture::Texture};

pub trait Bitmap {
    fn size(&self) -> IVec2;
    fn get_pixel(&self, pos: IVec2) -> Color;
    unsafe fn set_pixel(&mut self, pos: IVec2, color: Color);

    fn draw_rect(&mut self, rect: IRect, color: Color);

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

    fn draw_texture_x(&mut self, texture: &Texture, dst_pt: IVec2, params: DrawParams);
}

pub fn draw_rect_default(bitmap: &mut dyn Bitmap, rect: IRect, color: Color) {
    let rect = rect
        .intersection(IRect::of_size(bitmap.size()))
        .unwrap_or(IRect::ZERO);
    for pos in rect.iter() {
        unsafe {
            bitmap.set_pixel(pos, color);
        }
    }
}

pub fn draw_texture_x_default(
    bitmap: &mut dyn Bitmap,
    texture: &Texture,
    dst_pt: IVec2,
    params: DrawParams,
) {
    let src = params.src.unwrap_or(texture.bounds());
    let offset = dst_pt - src.pos;

    let dst = ir(dst_pt, src.size)
        .intersection(IRect::of_size(bitmap.size()))
        .unwrap_or(IRect::ZERO);

    let src = ir(dst.pos - offset, dst.size);

    for src_pt in src.iter() {
        let dst = src_pt + offset;
        if let Some(&color) = texture.get(src_pt) {
            let mut dp = DrawPixel {
                org: IVec2::ZERO,
                dst,
                color,
            };
            if let Some(shader) = &params.shader {
                dp = shader.apply(dp, texture);
            }

            unsafe {
                bitmap.set_pixel(dp.dst, dp.color);
            }
        }
    }
}
