use std::{hash::Hash, rc::Rc};

use crate::num::{
    irect::{ir, IRect},
    ivec2::IVec2,
};

use super::animation_map::AnimationMap;

/// State of an animated sprite.
pub struct Animator<T> {
    curr_key: T,
    curr_time_s: f32,
    tile_size: IVec2,
    animations: Rc<AnimationMap<T>>,
}

impl<T> Animator<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(start_key: T, tile_size: IVec2, animations: Rc<AnimationMap<T>>) -> Self {
        Self {
            curr_key: start_key,
            curr_time_s: 0.0,
            tile_size,
            animations,
        }
    }

    pub fn update(&mut self, delta_s: f32) {
        self.curr_time_s += delta_s;
    }

    pub fn curr_src_tile(&self) -> IRect {
        let Some(curr_animation) =
            self.animations
                .get(self.curr_key.clone()) else {
            return IRect::ZERO
        };

        let src_tile = curr_animation.at(self.curr_time_s);
        ir(src_tile * self.tile_size, self.tile_size)
    }
}
