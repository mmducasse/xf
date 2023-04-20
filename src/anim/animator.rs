use std::{hash::Hash, rc::Rc};

use macroquad::texture::Texture2D;

use crate::{
    mq::draw::draw_texture,
    num::{
        irect::{ir, IRect},
        ivec2::IVec2,
    },
};

use super::animation_map::AnimationMap;

/// State of an animated sprite.
pub struct Animator<T> {
    curr_key: T,
    curr_time_s: f32,
    tile_size: IVec2,
    animations: Rc<AnimationMap<T>>,
    texture: Texture2D,
}

impl<T> Animator<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(
        start_key: T,
        tile_size: IVec2,
        animations: Rc<AnimationMap<T>>,
        texture: Texture2D,
    ) -> Self {
        Self {
            curr_key: start_key,
            curr_time_s: 0.0,
            tile_size,
            animations,
            texture,
        }
    }

    pub fn curr_key(&self) -> T {
        self.curr_key.clone()
    }

    pub fn curr_time_s(&self) -> f32 {
        self.curr_time_s
    }

    pub fn is_done(&self) -> bool {
        let Some(curr_animation) = self.animations.get(self.curr_key.clone()) else {
            return false;
        };

        !curr_animation.loops && (curr_animation.total_dur_s() < self.curr_time_s)
    }

    pub fn set_key(&mut self, key: T) {
        self.curr_key = key;
        self.curr_time_s = 0.0;
    }

    pub fn update(&mut self, delta_s: f32) {
        self.curr_time_s += delta_s;
    }

    fn curr_src_tile(&self) -> IRect {
        let Some(curr_animation) =
            self.animations
                .get(self.curr_key.clone()) else {
            return IRect::ZERO
        };

        let src_tile = curr_animation.at(self.curr_time_s);
        ir(src_tile * self.tile_size, self.tile_size)
    }

    pub fn draw(&self, pos: IVec2) {
        let src = self.curr_src_tile();
        draw_texture(self.texture, Some(src), pos)
    }
}
