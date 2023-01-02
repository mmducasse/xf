use crate::num::{ivec2::IVec2, irect::IRect};

use super::{
    texture::Texture, 
    color::Color, 
    draw_params::DrawParams, 
    shader::DrawPixel
};


static mut BUFFER: Option<Texture> = None;

pub fn init(size: IVec2) {
    unsafe {
        BUFFER = Some(Texture::of_size(size));
    }
}

pub fn get_buffer() -> &'static Option<Texture> {
    unsafe { 
        &BUFFER
    }
}

#[inline]
pub fn draw_pixel(pt: IVec2, color: Color) {
    unsafe {
        if let Some(buffer) = &mut BUFFER {
            buffer.set(pt, color);
        }
    }
}

pub fn draw_rect(rect: IRect, color: Color) {
    for pt in rect.iter() {
        draw_pixel(pt, color);
    }
}

#[inline]
pub fn draw_texture_full(texture: &Texture, dst_pt: IVec2) {
    draw_texture(texture, texture.bounds(), dst_pt);
}

pub fn draw_texture(texture: &Texture, src: IRect, dst_pt: IVec2) {
    let params = DrawParams {
        src: Some(src),
        shader: None,
    };

    draw_texture_x(texture, dst_pt, params);
}

pub fn draw_texture_x(texture: &Texture, dst_pt: IVec2, params: DrawParams) {
    let src = params.src.unwrap_or(texture.bounds());

    let offset = dst_pt - src.pos;

    for src_pt in src.iter() {
        let dst = src_pt + offset;
        if let Some(&color) = texture.get(src_pt) {
            let mut dp = DrawPixel { org: IVec2::ZERO, dst, color };
            if let Some(shader) = &params.shader {
                dp = shader.apply(dp, texture);
            }

            draw_pixel(dp.dst, dp.color);
        }
    }
}