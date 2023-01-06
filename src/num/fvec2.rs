
use super::{vec2::Vec2, ivec2::IVec2};

pub type FVec2 = Vec2<f32>;

/// Creates a `Vec2<f32>` from x and y position.
#[inline]
pub const fn f2(x: f32, y: f32) -> FVec2 {
    FVec2 { x, y }
}

impl FVec2 {
    pub const ZERO: FVec2 = FVec2::splat(0.0);

    pub fn as_ivec2(&self) -> IVec2 {
        IVec2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}