use std::{hash::Hash, rc::Rc};

use crate::{
    mq::{draw::draw_texture, texture::Texture},
    num::{
        irect::{ir, IRect},
        ivec2::IVec2,
    },
};

use super::{animation::Animation, animation_map::AnimationMap};

/// State of an animated sprite.
pub struct Animator<T> {
    curr_key: T,
    curr_time_s: f32,
    default_key: T,
tile_size: IVec2,
    animations: Rc<AnimationMap<T>>,
    texture: Texture,
}

impl<T> Animator<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(
        start_key: T,
tile_size: IVec2,
        animations: Rc<AnimationMap<T>>,
        texture: Texture,
    ) -> Self {
        Self {
            curr_key: start_key.clone(),
            curr_time_s: 0.0,
            default_key: start_key,
tile_size,
            animations,
            texture,
        }
    }

    pub fn texture(&self) -> Texture {
        self.texture.clone()
    }

    pub fn curr_key(&self) -> T {
        self.curr_key.clone()
    }

    pub fn curr_time_s(&self) -> f32 {
        self.curr_time_s
    }

    pub fn curr_animation(&self) -> &Animation {
        let curr_key = self.curr_key();

        if let Some(curr_animation) = self.animations.get(curr_key) {
            curr_animation
        } else {
            let default_animation = self.animations.get(self.default_key.clone()).unwrap();
            default_animation
        }
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

    pub fn curr_draw_offset(&self) -> IVec2 {
        let Some(curr_animation) =
            self.animations
                .get(self.curr_key.clone()) else {
            return IVec2::ZERO
        };

        curr_animation.draw_offset * self.tile_size
    }

    pub fn curr_src_tile(&self) -> IRect {
        let Some(curr_animation) =
            self.animations
                .get(self.curr_key.clone()) else {
            return IRect::ZERO
        };

        let size = curr_animation.size_in_tiles * self.tile_size;
        let src_tile = curr_animation.at(self.curr_time_s);
        ir(src_tile * self.tile_size, size)
    }

    pub fn draw(&self, pos: IVec2) {
        let src = self.curr_src_tile();
        let offset = self.curr_draw_offset();
        draw_texture(self.texture.clone(), Some(src), pos + offset);
    }
}
