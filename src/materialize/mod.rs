mod context;

pub use context::*;

use crate::insn::{Key, TouchSensor};
use crate::transform::NormalizedSlideSegmentShape;

pub type TimestampInSeconds = f32;

pub type DurationInSeconds = f32;

#[derive(Clone, Debug)]
pub enum Note {
    Tap(MaterializedTap),
    Touch(MaterializedTouch),
    Hold(MaterializedHold),
    TouchHold(MaterializedTouchHold),
    SlideTrack(MaterializedSlideTrack),
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedTap {
    pub ts: TimestampInSeconds,
    pub key: Key,
    pub shape: MaterializedTapShape,
    pub is_break: bool,
    pub is_ex: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MaterializedTapShape {
    Ring,
    Star,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedTouch {
    pub ts: TimestampInSeconds,
    pub sensor: TouchSensor,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedHold {
    pub ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub key: Key,
    pub is_break: bool,
    pub is_ex: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedTouchHold {
    pub ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub sensor: TouchSensor,
}

#[derive(Clone, Debug)]
pub struct MaterializedSlideTrack {
    pub ts: TimestampInSeconds,
    pub start_ts: TimestampInSeconds,
    pub groups: Vec<MaterializedSlideSegmentGroup>,
    pub is_break: bool,
}

#[derive(Clone, Debug)]
pub struct MaterializedSlideSegmentGroup {
    pub dur: DurationInSeconds,
    pub segments: Vec<MaterializedSlideSegment>,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedSlideSegment {
    pub start: Key,
    pub destination: Key,
    pub shape: NormalizedSlideSegmentShape,
}
