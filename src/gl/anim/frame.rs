use crate::num::ivec2::IVec2;


#[derive(Clone, Copy, Debug)]
/// A single frame in an animation sequence.
pub struct Frame {
    /// Source location in atlas (in units of animator sprite size).
    pub src_loc: IVec2,

    /// Offset applied to sprite when drawing this frame.
    pub draw_offset: IVec2,
}