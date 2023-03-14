use std::{collections::HashMap, hash::Hash};

use crate::{
    data::{dir4::Dir4, dir_h::DirH},
    num::ivec2::{i2, IVec2},
};

use super::animation::Animation;

/// An association of animation key and `Animation<T>`.
pub struct AnimationMap<T> {
    anims: HashMap<T, Animation>,
}

impl<T> AnimationMap<T>
where
    T: Eq + Hash,
{
    pub fn empty() -> Self {
        Self {
            anims: HashMap::new(),
        }
    }

    /// Creates a new `AnimationMap` by combining smaller ones.
    pub fn new(mut others: Vec<AnimationMap<T>>) -> Self {
        let mut map = Self::empty();

        for mut other in others.drain(0..) {
            for (key, anim) in other.anims.drain() {
                map.anims.insert(key, anim);
            }
        }

        map
    }

    pub fn get(&self, key: T) -> Option<&Animation> {
        self.anims.get(&key)
    }
}

pub fn row<T>(key: T, org: IVec2, len: usize, frame_dur_s: f32, loops: bool) -> AnimationMap<T>
where
    T: Eq + Hash,
{
    let tiles: Vec<IVec2> = (0..len).map(|i| i2(org.x + i as i32, org.y)).collect();

    let anim = Animation {
        tiles,
        frame_dur_s,
        loops,
    };

    AnimationMap {
        anims: HashMap::from([(key, anim)]),
    }
}

pub fn row_h<T>(
    key_selector: fn(DirH) -> T,
    org: IVec2,
    len: usize,
    frame_dur_s: f32,
    loops: bool,
) -> AnimationMap<T>
where
    T: Eq + Hash,
{
    AnimationMap::new(vec![
        row(
            key_selector(DirH::L),
            org + i2(0, 0),
            len,
            frame_dur_s,
            loops,
        ),
        row(
            key_selector(DirH::R),
            org + i2(0, 1),
            len,
            frame_dur_s,
            loops,
        ),
    ])
}

#[derive(PartialEq, Eq, Hash)]
pub enum TestEnum {
    Idle,
    Run(DirH),
    Move(Dir4),
}

pub fn test() -> AnimationMap<TestEnum> {
    use TestEnum::*;
    const DUR: f32 = 0.25;

    AnimationMap::new(vec![
        row(Idle, i2(0, 0), 1, DUR, true),
        row_h(|d| Run(d), i2(0, 0), 1, DUR, true),
    ])
}
