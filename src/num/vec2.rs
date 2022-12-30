use std::{ops::{Add, Sub, Mul}, fmt::Debug};

/// 2D vector.
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

// pub fn iv(x: i32, y: i32) -> Vec2<i32> {
//     Vec2 { x, y }
// }

// pub fn fv(x: f32, y: f32) -> Vec2<f32> {
//     Vec2 { x, y }
// }

impl<T> Vec2<T>
where T: Copy {
    pub const fn splat(m: T) -> Self {
        Self { x: m, y: m }
    }
}

impl<T> Vec2<T>
where T: Add<T, Output = T> + Copy {
    pub fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<T> Vec2<T>
where T: Mul<T, Output = T> + Copy
{
    pub fn product(&self) -> T {
        self.x * self.y
    }
}

impl<T> Clone for Vec2<T>
where T: Clone
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Copy for Vec2<T>
where T: Copy
{
}

impl<T> Debug for Vec2<T>
where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2").field("x", &self.x).field("y", &self.y).finish()
    }
}

impl<T> PartialEq for Vec2<T>
where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Vec2<T>
where T: Eq {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> Add<Vec2<T>> for Vec2<T>
where T: Copy + Add<T, Output = T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Vec2<T>> for Vec2<T>
where T: Copy + Sub<T, Output = T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<Vec2<T>> for Vec2<T>
where T: Copy + Mul<T, Output = T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where T: Copy + Mul<T, Output = T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}