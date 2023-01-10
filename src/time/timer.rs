

/// A timer that completes after a given number
/// of frames have elapsed.
#[derive(Clone, Copy, PartialEq)]
pub struct Timer {
    duration: u32,
    elapsed: u32,
}

impl Timer {
    pub fn new(duration: u32) -> Self {
        Self {
            duration,
            elapsed: 0,
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = 0;
    }

    pub fn increment(&mut self) {
        self.elapsed += 1;
    }

    pub fn duration(&self) -> u32 { self.duration }
    pub fn elapsed(&self) -> u32 { self.elapsed }
    
    pub fn is_done(&self) -> bool {
        self.elapsed >= self.duration
    }

    pub fn completion(&self) -> f32 {
        self.elapsed as f32 / self.duration as f32
    }
}