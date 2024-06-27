mod context;

pub use context::*;

use crate::insn::{Key, TouchSensor};
use crate::transform::NormalizedSlideSegmentShape;
use serde::{Deserialize, Serialize};

pub type TimestampInSeconds = f64;

pub type DurationInSeconds = f64;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Note {
    #[serde(rename = "bpm")]
    Bpm(MaterializedBpm),
    #[serde(rename = "tap")]
    Tap(MaterializedTap),
    #[serde(rename = "touch")]
    Touch(MaterializedTouch),
    #[serde(rename = "hold")]
    Hold(MaterializedHold),
    #[serde(rename = "touch_hold")]
    TouchHold(MaterializedTouchHold),
    #[serde(rename = "slide_track")]
    SlideTrack(MaterializedSlideTrack),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedBpm {
    pub ts: TimestampInSeconds,
    pub bpm: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedTap {
    pub ts: TimestampInSeconds,
    pub key: Key,
    pub shape: MaterializedTapShape,
    pub is_break: bool,
    pub is_ex: bool,
    pub is_each: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum MaterializedTapShape {
    Ring,
    Star,
    StarSpin,
    Invalid,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedTouch {
    pub ts: TimestampInSeconds,
    pub sensor: TouchSensor,
    pub is_each: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedHold {
    pub ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub key: Key,
    pub is_break: bool,
    pub is_ex: bool,
    pub is_each: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedTouchHold {
    pub ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub sensor: TouchSensor,
    pub is_each: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedSlideTrack {
    pub ts: TimestampInSeconds,
    pub start_ts: TimestampInSeconds,
    pub groups: Vec<MaterializedSlideSegmentGroup>,
    pub is_break: bool,
    pub is_sudden: bool,
    pub is_each: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedSlideSegmentGroup {
    pub dur: DurationInSeconds,
    pub segments: Vec<MaterializedSlideSegment>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedSlideSegment {
    pub start: Key,
    pub destination: Key,
    pub shape: NormalizedSlideSegmentShape,
}
