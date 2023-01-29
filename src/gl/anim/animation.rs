use super::frame::Frame;

/// An associations between times and sprite frames.
pub trait Animation {
    /// Number of frames in the animation.
    fn len(&self) -> usize;

    /// Duration of a frame (in game cycles).
    fn frame_dur(&self) -> u32;

    /// Whether the animation loops back to the
    /// first frame after completing.
    fn loops(&self) -> bool;

    /// Returns the frame associated with the given time (in game cycles).
    fn at(&self, time: u32) -> Frame;
}
