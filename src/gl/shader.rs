use crate::num::ivec2::IVec2;

use super::{color::Color, texture::Texture};

/// A color and a pixel coordinate to draw to.
#[derive(Clone, Copy)]
pub struct DrawPixel {
    pub org: IVec2,
    pub dst: IVec2,
    pub color: Color,
}

/// A transformation from one `DrawPixel` to another.
pub type ShaderEffect = Box<dyn Fn(DrawPixel, &Texture) -> DrawPixel>;

/// An effect that can be applied to textures.
pub struct Shader {
    effect: ShaderEffect,
}

impl Shader {
    pub fn new(effect: ShaderEffect) -> Self {
        Self { effect }
    }

    /// Applied the shader's effect to a `DrawPixel`.
    #[inline]
    pub fn apply(&self, dp: DrawPixel, texture: &Texture) -> DrawPixel {
        (self.effect)(dp, texture)
    }
}
