use super::{fvec2::FVec2, vec2::Vec2};

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

    pub const fn add(a: IVec2, b: IVec2) -> IVec2 {
        IVec2 {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    }

    pub const fn mul(a: IVec2, b: IVec2) -> IVec2 {
        IVec2 {
            x: a.x * b.x,
            y: a.y * b.y,
        }
    }

    pub const fn wrap(idx: i32, width: i32) -> IVec2 {
        IVec2 {
            x: idx % width,
            y: idx / width,
        }
    }

    pub const fn unwrap(pos: IVec2, width: i32) -> i32 {
        (pos.y * width) + pos.x
    }
}
