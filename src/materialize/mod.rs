mod context;

pub use context::*;

use crate::insn::{Key, SlideShape, TouchSensor};

pub type TimestampInSeconds = f32;

pub type DurationInSeconds = f32;

#[derive(Copy, Clone, Debug)]
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
    pub is_ex: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MaterializedTapShape {
    Ring,
    Break,
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
    pub is_ex: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedTouchHold {
    pub ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub sensor: TouchSensor,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedSlideTrack {
    pub ts: TimestampInSeconds,
    pub start_ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub start: Key,
    pub destination: Key,
    pub interim: Option<Key>,
    pub shape: SlideShape,
}
