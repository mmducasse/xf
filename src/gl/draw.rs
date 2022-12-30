use crate::num::{ivec2::IVec2, irect::IRect};

use super::{texture::Texture, color::Color};


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

pub fn draw_texture_full(texture: &Texture, dst_pt: IVec2) {
    draw_texture(texture, texture.bounds(), dst_pt);
}

pub fn draw_texture(texture: &Texture, src: IRect, dst_pt: IVec2) {
    let offset = dst_pt - src.pos;

    for src_pt in src.iter() {
        let dst_pt = src_pt + offset;
        if let Some(&color) = texture.get(src_pt) {
            draw_pixel(dst_pt, color);
        }
    }
}