use crate::insn::{Key, Position, TouchSensor};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedTapParams {
    pub key: Key,
}

impl std::fmt::Display for NormalizedTapParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedTouchParams {
    pub sensor: TouchSensor,
}

impl std::fmt::Display for NormalizedTouchParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sensor)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedHoldParams {
    pub key: Key,
}

impl std::fmt::Display for NormalizedHoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}h", self.key)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedTouchHoldParams {
    pub sensor: TouchSensor,
}

impl std::fmt::Display for NormalizedTouchHoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}h", self.sensor)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedSlideParams {
    pub start: NormalizedTapParams,
    pub tracks: Vec<NormalizedSlideTrack>,
}

impl std::fmt::Display for NormalizedSlideParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.start,
            self.tracks
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join("*")
        )
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedSlideTrack {
    pub groups: Vec<NormalizedSlideSegmentGroup>,
}

impl std::fmt::Display for NormalizedSlideTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.groups
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedSlideSegmentGroup {
    pub segments: Vec<NormalizedSlideSegment>,
}

impl std::fmt::Display for NormalizedSlideSegmentGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.segments
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NormalizedSlideSegment {
    Line(NormalizedSlideSegmentParams),
    Clockwise(NormalizedSlideSegmentParams),
    V(NormalizedSlideSegmentParams),
    PQ(NormalizedSlideSegmentParams),
    SZ(NormalizedSlideSegmentParams),
    PpQq(NormalizedSlideSegmentParams),
    Angle(NormalizedSlideSegmentParams),
    Spread(NormalizedSlideSegmentParams),
}

impl std::fmt::Display for NormalizedSlideSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line(param) => write!(f, "-{}", param.destination),
            Self::Clockwise(param) => {
                let end_index = param.destination.index().unwrap();
                let upper = !(2..6).contains(&end_index);
                write!(
                    f,
                    "{}{}",
                    if upper ^ param.flip.unwrap() {
                        '>'
                    } else {
                        '<'
                    },
                    param.destination
                )
            }
            Self::V(param) => write!(f, "v{}", param.destination),
            Self::PQ(param) => {
                write!(
                    f,
                    "{}{}",
                    if param.flip.unwrap() { 'p' } else { 'q' },
                    param.destination
                )
            }
            Self::SZ(param) => {
                write!(
                    f,
                    "{}{}",
                    if param.flip.unwrap() { 'z' } else { 's' },
                    param.destination
                )
            }
            Self::PpQq(param) => {
                write!(
                    f,
                    "{}{}",
                    if param.flip.unwrap() { "pp" } else { "qq" },
                    param.destination
                )
            }
            Self::Angle(param) => write!(f, "V{}{}", param.interim.unwrap(), param.destination),
            Self::Spread(param) => write!(f, "w{}", param.destination),
        }
    }
}

impl NormalizedSlideSegment {
    pub fn shape(&self) -> NormalizedSlideSegmentShape {
        match self {
            Self::Line(_) => NormalizedSlideSegmentShape::Line,
            Self::Clockwise(_) => NormalizedSlideSegmentShape::Clockwise,
            Self::V(_) => NormalizedSlideSegmentShape::V,
            Self::PQ(_) => NormalizedSlideSegmentShape::PQ,
            Self::SZ(_) => NormalizedSlideSegmentShape::SZ,
            Self::PpQq(_) => NormalizedSlideSegmentShape::PpQq,
            Self::Angle(_) => NormalizedSlideSegmentShape::Angle,
            Self::Spread(_) => NormalizedSlideSegmentShape::Spread,
        }
    }

    pub fn params(&self) -> &NormalizedSlideSegmentParams {
        match self {
            NormalizedSlideSegment::Line(p) => p,
            NormalizedSlideSegment::Clockwise(p) => p,
            NormalizedSlideSegment::V(p) => p,
            NormalizedSlideSegment::PQ(p) => p,
            NormalizedSlideSegment::SZ(p) => p,
            NormalizedSlideSegment::PpQq(p) => p,
            NormalizedSlideSegment::Angle(p) => p,
            NormalizedSlideSegment::Spread(p) => p,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NormalizedSlideSegmentShape {
    Line,
    Clockwise,
    V,
    PQ,
    SZ,
    PpQq,
    Angle,
    Spread,
}

impl From<NormalizedSlideSegment> for NormalizedSlideSegmentShape {
    fn from(x: NormalizedSlideSegment) -> Self {
        x.shape()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedSlideSegmentParams {
    pub destination: Key,
    pub interim: Option<Key>,
    pub flip: Option<bool>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NormalizedNote {
    Tap(NormalizedTapParams),
    Touch(NormalizedTouchParams),
    Hold(NormalizedHoldParams),
    TouchHold(NormalizedTouchHoldParams),
    Slide(NormalizedSlideParams),
}

impl std::fmt::Display for NormalizedNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NormalizedNote::Tap(params) => write!(f, "{}", params),
            NormalizedNote::Touch(params) => write!(f, "{}", params),
            NormalizedNote::Hold(params) => write!(f, "{}", params),
            NormalizedNote::TouchHold(params) => write!(f, "{}", params),
            NormalizedNote::Slide(params) => write!(f, "{}", params),
        }
    }
}
