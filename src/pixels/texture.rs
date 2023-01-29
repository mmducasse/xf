use crate::{
    gl::{color::Color, texture::Texture},
    num::{irect::IRect, ivec2::i2},
};
use image::ImageFormat;

pub fn load_texture(bytes: &[u8]) -> Option<Texture> {
    let image = image::load_from_memory_with_format(bytes, ImageFormat::Png).ok()?;

    let width = image.width() as i32;
    let height = image.height() as i32;

    let bounds = IRect::of_size(i2(width, height));

    let vec_size = (bounds.w() * bounds.h()) as usize;
    let mut v = vec![Color::WHITE; vec_size];

    if let Some(img) = image.as_rgba8() {
        // RGBA8: has transparency.
        for p in bounds.iter() {
            let rgba8 = img.get_pixel(p.x as u32, p.y as u32);
            let i = (p.x + (p.y * width)) as usize;
            v[i] = Color {
                r: rgba8[0],
                g: rgba8[1],
                b: rgba8[2],
                a: rgba8[3],
            };
        }
    } else if let Some(img) = image.as_rgb8() {
        // RGB8: no transparency.
        for p in bounds.iter() {
            let rgba8 = img.get_pixel(p.x as u32, p.y as u32);
            let i = (p.x + (p.y * width)) as usize;
            v[i] = Color {
                r: rgba8[0],
                g: rgba8[1],
                b: rgba8[2],
                a: 0xFF,
            };
        }
    } else {
        // Other format...
        return None;
    }

    let texture = Texture::new(v, bounds.w() as usize);

    Some(texture)
}
