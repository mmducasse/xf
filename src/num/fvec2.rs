
use super::{vec2::Vec2, frac::Frac};

pub type FVec2 = Vec2<Frac<128>>;

/// Creates a `Vec2<Frac<128>>` from x and y position.
#[inline]
pub const fn f2(x: i32, y: i32) -> FVec2 {
    FVec2 { 
        x: Frac::whole(x),
        y: Frac::whole(y),
    }
}
