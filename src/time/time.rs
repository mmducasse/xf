use std::time::Duration;

static mut DELTA_S: f32 = 0.0;
static mut FRAME_NUM: u64 = 0;

const MAX_DELTA_S: f32 = 1.0 / 30.0;

// The time elapsed (in seconds) since the previous render frame.
pub fn delta_s() -> f32 {
    unsafe { DELTA_S }
}

// The number of frames that have been rendered since the application started.
pub fn frame_num() -> u64 {
    unsafe { FRAME_NUM }
}

// Set the time that has elapsed since the previous frame.
pub fn update_global_time(delta: &Duration) {
    unsafe {
        DELTA_S = delta.as_secs_f32().min(MAX_DELTA_S);
        FRAME_NUM += 1;
    }
}

// Extimate a fixed time that has elapsed since the previous frame.
// Useful when running in a context where the current time cannot be measured.
pub fn fixed_update(secs: f32) {
    unsafe {
        DELTA_S = secs;
        FRAME_NUM += 1;
    }
}
