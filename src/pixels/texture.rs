use std::io::Cursor;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use crate::{
    gl::{color::Color, texture::Texture},
    num::{irect::IRect, ivec2::i2},
};

pub fn load_texture(bytes: &[u8]) -> Option<Texture> {
    let img: DynamicImage = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .expect("Cursor io never fails")
        .decode()
        .ok()?;

    let width = img.width() as i32;
    let height = img.height() as i32;

    let bounds = IRect::of_size(i2(width, height));

    let vec_size = (bounds.w() * bounds.h()) as usize;
    let mut v = vec![Color::WHITE; vec_size];

    if let Some(img) = img.as_rgba8() {
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
    } else if let Some(img) = img.as_rgb8() {
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
