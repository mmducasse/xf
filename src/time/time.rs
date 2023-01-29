use std::time::{Duration, SystemTime};

/// A record of the current frame time and duration.
pub struct Time {
    now: SystemTime,
    delta_s: f32,
    frame_num: u64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            now: SystemTime::now(),
            delta_s: 0.0,
            frame_num: 0,
        }
    }

    /// Records the duration of the previous frame.
    /// Call this once at the begining of each frame.
    pub fn update(&mut self) {
        let prev_time = self.now;
        self.now = SystemTime::now();

        self.delta_s = self
            .now
            .duration_since(prev_time)
            .unwrap_or(Duration::new(0, 0))
            .as_secs_f32();

        self.frame_num = u64::saturating_add(self.frame_num, 1);
    }

    /// The duration of the previous frame (in seconds).
    pub fn delta_s(&self) -> f32 {
        self.delta_s
    }

    /// The number of frames that have elapsed since the game started.
    pub fn frame_num(&self) -> u64 {
        self.frame_num
    }

    /// The frames-per-second derived from the previous frame duration.
    pub fn fps(&self) -> f32 {
        f32::round(1.0 / self.delta_s)
    }
}
