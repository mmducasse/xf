use crate::num::ivec2::IVec2;

/// A sequence of tiles to be drawn from a sprite atlas.
pub struct Animation {
    pub tiles: Vec<IVec2>,
    pub frame_dur_s: f32,
    pub loops: bool,
}

impl Animation {
    /// The tile to draw at the given time (in seconds).
    pub fn at(&self, time_s: f32) -> IVec2 {
        let len = self.tiles.len();
        let idx = (time_s / self.frame_dur_s) as usize;
        let idx = if self.loops {
            idx % len
        } else {
            idx.min(len - 1)
        };

        self.tiles[idx]
    }
}
