
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir4 {
    N,
    E,
    S,
    W,
}

use std::fmt::{self, Display};

use Dir4::*;

use crate::num::{ivec2::{IVec2, i2}, math::mod_};

use super::spin::Spin;

impl Dir4 {
    pub const ALL: [Self; 4] = [N, E, S, W];

    pub fn unit(&self) -> IVec2 {
        match self {
            N => i2(0, -1),
            E => i2(1, 0),
            S => i2(0, 1),
            W => i2(-1, 0),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }

    pub fn rotate(self, dir: Spin) -> Self {
        let offset = match dir {
            Spin::Ccw => -1,
            Spin::Cw => 1,
        };

        let idx = mod_((self as i32) + offset, 4) as usize;

        Self::ALL[idx]
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            N | S => true,
            _ => false,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            W | E => true,
            _ => false,
        }
    }

    pub fn cw(&self, turns: i32) -> Self {
        let mut i = self.to_i32();
        i += turns;

        Self::from_i32(i)
    }

    pub fn to_i32(&self) -> i32 {
        use Dir4::*;
        match *self {
            N => 0,
            E => 1,
            S => 2,
            W => 3,
        }
    }

    pub fn from_i32(i: i32) -> Self {
        use Dir4::*;
        match i % 4 {
            0 => N,
            1 => E,
            2 => S,
            3 => W,
            _ => panic!(),
        }
    }

    pub fn from_ivec2(ivec2: IVec2) -> Option<Dir4> {
        use Dir4::*;

        if ivec2 == IVec2::splat(0) {
            None
        } else if ivec2.y.abs() >= ivec2.x.abs() {
            Some(if ivec2.y < 0 { N } else { S })
        } else {
            Some(if ivec2.x < 0 { W } else { E })
        }
    }
}

impl Display for Dir4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Dir4::*;

        let str = match *self {
            N => "N", E => "E",
            S => "S", W => "W",
        };

        write!(f, "{}", str)
    }
}