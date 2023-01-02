use crate::num::ivec2::IVec2;

use super::color::Color;

/// A color and a pixel coordinate to draw to.
#[derive(Clone, Copy)]
pub struct DrawPixel {
    pub org: IVec2,
    pub dst: IVec2,
    pub color: Color,
}

/// A transformation from one `DrawPixel` to another.
pub type ShaderEffect = dyn Fn(DrawPixel) -> DrawPixel;

/// An effect that can be applied to textures.
pub struct Shader {
    effect: Box<ShaderEffect>,
}


impl Shader {
    /// Applied the shader's effect to a `DrawPixel`.
    #[inline]
    pub fn apply(&self, dp: DrawPixel) -> DrawPixel {
        (self.effect)(dp)
    }


    pub fn negative() -> Self {
        Self {
            effect: Box::new(|dp| {
                let mut dp = dp;
                dp.color.r = 0xFF - dp.color.r;
                dp.color.g = 0xFF - dp.color.g;
                dp.color.b = 0xFF - dp.color.b;
                
                dp
            }),
        }
    }

    pub fn solid_color(color: Color) -> Self {
        Self {
            effect: Box::new(move |dp| {
                let mut dp = dp;
                dp.color.r = color.r;
                dp.color.g = color.g;
                dp.color.b = color.b;
                
                dp
            }),
        }
    }
}