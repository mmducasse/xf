use crate::{
    gl::{color::Color, texture::Texture}, 
    num::{
        ivec2::IVec2, 
        irect::{IRect, rect}
    }
};

use super::shader::ShaderEffect;

pub fn negative() -> ShaderEffect {
    Box::new(|dp, _| {
        let mut dp = dp;
        dp.color.r = 0xFF - dp.color.r;
        dp.color.g = 0xFF - dp.color.g;
        dp.color.b = 0xFF - dp.color.b;
        
        dp
    })
}

pub fn solid_color(color: Color) -> ShaderEffect {
    Box::new(move |dp, _| {
        let mut dp = dp;
        dp.color.r = color.r;
        dp.color.g = color.g;
        dp.color.b = color.b;
        
        dp
    })
}


pub fn outline_with_color(color: Color) -> ShaderEffect {
    Box::new(move |dp, t| {
        let mut dp = dp;
        if on_edge(dp.dst, t) {
            dp.color = color;
        }

        dp
    })
}

fn on_edge(pos: IVec2, texture: &Texture) -> bool {
    const NEIGHBOR_REGION: IRect = rect(-1, -1, 3, 3);

    if let Some(color) = texture.get(pos) {
        if color.a != 0 { return false; }

        // This pixel is transparent.
        for offset in NEIGHBOR_REGION.iter() {
            if let Some(color) = texture.get(pos + offset) {
                if color.a != 0 { 
                    // But it's neighbor is not!
                    return true; 
                }
            }
        }
    }

    return false;
}