
use crate::{
    num::{ivec2::IVec2, irect::ir}, 
    gl::{texture::Texture, bitmap::Bitmap},
};

use super::{Animation, Frame};

/// A function that returns the animation associated with a
/// given animation key.
pub type AnimatorMapFn<T> = fn(key: T) -> &'static dyn Animation;

/// Stores the state of an animation.
pub struct Animator<T> {
    curr_key: T,
    pub curr_time: u32,
    sprite_size: IVec2,
    map_fn: AnimatorMapFn<T>,
}

impl<T> Animator<T> 
where T: Copy + PartialEq
{
    pub fn new(
        start_key: T,
        sprite_size: IVec2,
        map_fn: AnimatorMapFn<T>
    )
    -> Self {
        Self {
            curr_key: start_key,
            curr_time: 0,
            sprite_size,
            map_fn,
        }
    }

    pub fn update(&mut self) {
        self.curr_time += 1;
    }

    pub fn get_key(&self) -> T {
        self.curr_key
    }

    pub fn set_key(&mut self, key: T, reset: bool) {
        if self.curr_key != key || reset {
            self.curr_key = key;
            self.curr_time = 0;
        }
    }

    pub fn curr_animation(&self) -> &'static dyn Animation {
        (self.map_fn)(self.curr_key)
    }

    pub fn curr_frame(&self) -> Frame {
        self.curr_animation().at(self.curr_time)
    }

    pub fn completion(&self) -> f32 {
        let animation = self.curr_animation();
        if animation.loops() { 0.0 } else {
            let total_dur = animation.frame_dur() * animation.len() as u32;
            self.curr_time as f32 / total_dur as f32
        }
    }

    pub fn is_done(&self) -> bool {
        self.completion() >= 1.0
    }
    
    pub fn draw(&self, texture: &Texture, origin: IVec2, target: &mut dyn Bitmap) {
        let frame = self.curr_animation().at(self.curr_time);
        let src = ir(frame.src_loc * self.sprite_size, self.sprite_size);
        let dst = origin + frame.draw_offset;

        target.draw_texture(&texture, src, dst);
    }
}