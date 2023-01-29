use std::time::{SystemTime, Duration};


/// A record of the current frame time and duration.
pub struct Time {
    now: SystemTime,
    delta_s: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            now: SystemTime::now(),
            delta_s: 0.0,
        }
    }

    /// Records the duration of the previous frame.
    /// Call this once at the begining of each frame.
    pub fn update(&mut self) {
        let prev_time = self.now;
        self.now = SystemTime::now();

        self.delta_s =
            self.now.duration_since(prev_time)
                    .unwrap_or(Duration::new(0, 0))
                    .as_secs_f32();
    }

    /// The duration of the previous frame (in seconds).
    pub fn delta_s(&self) -> f32 { self.delta_s }
}