use crate::num::ivec2::IVec2;

use super::{Animation, Frame};

/// A sequence of sprite frames of equal duration.
pub struct Seq<const LEN: usize> {
    pub src_locs: [IVec2; LEN],
    pub draw_offsets: [IVec2; LEN],
    pub frame_dur: u32,
    pub loops: bool,
}

impl<const LEN: usize> Seq<LEN> {
    pub const fn with_offsets(mut self, draw_offsets: [IVec2; LEN]) -> Self {
        self.draw_offsets = draw_offsets;
        self
    }
}

impl<const LEN: usize> Animation for Seq<LEN> {
    fn len(&self) -> usize {
        self.src_locs.len()
    }
    fn frame_dur(&self) -> u32 {
        self.frame_dur
    }
    fn loops(&self) -> bool {
        self.loops
    }

    fn at(&self, time: u32) -> Frame {
        let idx = (time / self.frame_dur) as usize;
        let idx = if self.loops {
            idx % LEN
        } else {
            idx.min(LEN - 1)
        };

        Frame {
            src_loc: self.src_locs[idx],
            draw_offset: self.draw_offsets[idx],
        }
    }
}

pub const fn seq<const LEN: usize>(frame_dur: u32, pts: [IVec2; LEN], loops: bool) -> Seq<LEN> {
    Seq {
        src_locs: pts,
        draw_offsets: [IVec2::ZERO; LEN],
        frame_dur,
        loops,
    }
}

pub const fn seq_row<const LEN: usize>(frame_dur: u32, origin: IVec2, loops: bool) -> Seq<LEN> {
    let mut pts = [origin; LEN];

    let mut idx = 0;
    while idx < LEN {
        pts[idx].x += idx as i32;

        idx += 1;
    }

    seq(frame_dur, pts, loops)
}
