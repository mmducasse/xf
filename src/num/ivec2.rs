
use super::{vec2::Vec2, fvec2::FVec2};

/// 2D vector of `i32` values.
pub type IVec2 = Vec2<i32>;

/// Creates a `Vec2<i32>` from x and y position.
#[inline]
pub const fn i2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}

impl IVec2 {
    pub const ZERO: IVec2 = IVec2::splat(0);

    pub fn as_fvec2(self) -> FVec2 {
        FVec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}