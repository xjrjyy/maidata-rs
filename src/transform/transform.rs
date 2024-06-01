use crate::insn::{Key, Position, TouchSensor};
use crate::transform::{
    NormalizedHoldParams, NormalizedSlideParams, NormalizedSlideSegment,
    NormalizedSlideSegmentGroup, NormalizedSlideSegmentParams, NormalizedSlideTrack,
    NormalizedTapParams, NormalizedTouchHoldParams, NormalizedTouchParams,
};

use super::NormalizedNote;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Transformer {
    pub rotation: u8,
    pub flip: bool,
}

pub trait Transformable {
    fn transform(&self, transformer: Transformer) -> Self;
}

impl Transformable for Key {
    fn transform(&self, transformer: Transformer) -> Self {
        let mut index = (self.index().unwrap() + transformer.rotation) % 8;
        if transformer.flip {
            index = 7 - index;
        }
        index.try_into().unwrap()
    }
}

impl Transformable for TouchSensor {
    fn transform(&self, transformer: Transformer) -> Self {
        let group = self.group().unwrap();
        if group == 'C' {
            return *self;
        }
        let mut index = (self.index().unwrap() + transformer.rotation) % 8;
        if transformer.flip {
            index = match group {
                'A' | 'B' => 7 - index,
                'D' | 'E' => (8 - index) % 8,
                _ => unreachable!(),
            };
        }
        (group, Some(index)).try_into().unwrap()
    }
}

impl Transformable for NormalizedTapParams {
    fn transform(&self, transformer: Transformer) -> Self {
        NormalizedTapParams {
            key: self.key.transform(transformer),
        }
    }
}

impl Transformable for NormalizedTouchParams {
    fn transform(&self, transformer: Transformer) -> Self {
        NormalizedTouchParams {
            sensor: self.sensor.transform(transformer),
        }
    }
}

impl Transformable for NormalizedHoldParams {
    fn transform(&self, transformer: Transformer) -> Self {
        NormalizedHoldParams {
            key: self.key.transform(transformer),
        }
    }
}

impl Transformable for NormalizedTouchHoldParams {
    fn transform(&self, transformer: Transformer) -> Self {
        NormalizedTouchHoldParams {
            sensor: self.sensor.transform(transformer),
        }
    }
}

impl Transformable for NormalizedSlideSegment {
    fn transform(&self, transformer: Transformer) -> Self {
        let params = NormalizedSlideSegmentParams {
            start: self.params().start.transform(transformer),
            destination: self.params().destination.transform(transformer),
        };
        if transformer.flip {
            match self {
                NormalizedSlideSegment::Straight(_) => NormalizedSlideSegment::Straight(params),
                NormalizedSlideSegment::CircleL(_) => NormalizedSlideSegment::CircleR(params),
                NormalizedSlideSegment::CircleR(_) => NormalizedSlideSegment::CircleL(params),
                NormalizedSlideSegment::CurveL(_) => NormalizedSlideSegment::CurveR(params),
                NormalizedSlideSegment::CurveR(_) => NormalizedSlideSegment::CurveL(params),
                NormalizedSlideSegment::ThunderL(_) => NormalizedSlideSegment::ThunderR(params),
                NormalizedSlideSegment::ThunderR(_) => NormalizedSlideSegment::ThunderL(params),
                NormalizedSlideSegment::Corner(_) => NormalizedSlideSegment::Corner(params),
                NormalizedSlideSegment::BendL(_) => NormalizedSlideSegment::BendR(params),
                NormalizedSlideSegment::BendR(_) => NormalizedSlideSegment::BendL(params),
                NormalizedSlideSegment::SkipL(_) => NormalizedSlideSegment::SkipR(params),
                NormalizedSlideSegment::SkipR(_) => NormalizedSlideSegment::SkipL(params),
                NormalizedSlideSegment::Fan(_) => NormalizedSlideSegment::Fan(params),
            }
        } else {
            match self {
                NormalizedSlideSegment::Straight(_) => NormalizedSlideSegment::Straight(params),
                NormalizedSlideSegment::CircleL(_) => NormalizedSlideSegment::CircleL(params),
                NormalizedSlideSegment::CircleR(_) => NormalizedSlideSegment::CircleR(params),
                NormalizedSlideSegment::CurveL(_) => NormalizedSlideSegment::CurveL(params),
                NormalizedSlideSegment::CurveR(_) => NormalizedSlideSegment::CurveR(params),
                NormalizedSlideSegment::ThunderL(_) => NormalizedSlideSegment::ThunderL(params),
                NormalizedSlideSegment::ThunderR(_) => NormalizedSlideSegment::ThunderR(params),
                NormalizedSlideSegment::Corner(_) => NormalizedSlideSegment::Corner(params),
                NormalizedSlideSegment::BendL(_) => NormalizedSlideSegment::BendL(params),
                NormalizedSlideSegment::BendR(_) => NormalizedSlideSegment::BendR(params),
                NormalizedSlideSegment::SkipL(_) => NormalizedSlideSegment::SkipL(params),
                NormalizedSlideSegment::SkipR(_) => NormalizedSlideSegment::SkipR(params),
                NormalizedSlideSegment::Fan(_) => NormalizedSlideSegment::Fan(params),
            }
        }
    }
}

impl Transformable for NormalizedSlideSegmentGroup {
    fn transform(&self, transformer: Transformer) -> Self {
        NormalizedSlideSegmentGroup {
            segments: self
                .segments
                .iter()
                .map(|segment| segment.transform(transformer))
                .collect(),
        }
    }
}

impl Transformable for NormalizedSlideTrack {
    fn transform(&self, transformer: Transformer) -> Self {
        NormalizedSlideTrack {
            groups: self
                .groups
                .iter()
                .map(|group| group.transform(transformer))
                .collect(),
        }
    }
}

impl Transformable for NormalizedSlideParams {
    fn transform(&self, transformer: Transformer) -> Self {
        NormalizedSlideParams {
            start: self.start.transform(transformer),
            tracks: self
                .tracks
                .iter()
                .map(|track| track.transform(transformer))
                .collect(),
        }
    }
}

pub fn transform_note(note: &NormalizedNote, transformer: Transformer) -> NormalizedNote {
    match note {
        NormalizedNote::Tap(params) => NormalizedNote::Tap(params.transform(transformer)),
        NormalizedNote::Touch(params) => NormalizedNote::Touch(params.transform(transformer)),
        NormalizedNote::Hold(params) => NormalizedNote::Hold(params.transform(transformer)),
        NormalizedNote::TouchHold(params) => {
            NormalizedNote::TouchHold(params.transform(transformer))
        }
        NormalizedNote::Slide(params) => NormalizedNote::Slide(params.transform(transformer)),
    }
}
