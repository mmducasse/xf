use std::time::Duration;

static mut DELTA_S: f32 = 0.0;
static mut FRAME_NUM: u64 = 0;

const MAX_DELTA_S: f32 = 1.0 / 30.0;

pub fn delta_s() -> f32 {
    unsafe { DELTA_S }
}

pub fn frame_num() -> u64 {
    unsafe { FRAME_NUM }
}

pub fn update(delta: &Duration) {
    unsafe {
        DELTA_S = delta.as_secs_f32().min(MAX_DELTA_S);
        FRAME_NUM += 1;
    }
}
