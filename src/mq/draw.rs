use macroquad::{
    prelude::{Color, WHITE},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::num::{irect::IRect, ivec2::IVec2};

pub fn draw_rect(rect: IRect, color: Color) {
    draw_rectangle(
        rect.x() as f32,
        rect.y() as f32,
        rect.w() as f32,
        rect.h() as f32,
        color,
    );
}

pub fn draw_texture(texture: &Texture2D, src: Option<IRect>, dst: IVec2) {
    draw_texture_ex(
        &texture,
        dst.x as f32,
        dst.y as f32,
        WHITE,
        DrawTextureParams {
            dest_size: None,
            source: src.map(|s| s.as_rect()),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    )
}
