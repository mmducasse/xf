use super::frame::Frame;

/// An associations between times and sprite frames.
pub trait Animation {
    /// Number of frames in the animation.
    fn len(&self) -> usize;

    /// Duration of a frame (in seconds).
    fn frame_dur_s(&self) -> f32;

    /// Whether the animation loops back to the
    /// first frame after completing.
    fn loops(&self) -> bool;

    /// Returns the frame associated with the given time (in seconds).
    fn at(&self, time: f32) -> Frame;
}
