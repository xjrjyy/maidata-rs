use crate::insn::{Key, TouchSensor};

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
    Straight(NormalizedSlideSegmentParams), // -
    CircleL(NormalizedSlideSegmentParams),  // counterclockwise
    CircleR(NormalizedSlideSegmentParams),  // clockwise
    CurveL(NormalizedSlideSegmentParams),   // p
    CurveR(NormalizedSlideSegmentParams),   // q
    ThunderL(NormalizedSlideSegmentParams), // s
    ThunderR(NormalizedSlideSegmentParams), // z
    Corner(NormalizedSlideSegmentParams),   // v
    BendL(NormalizedSlideSegmentParams),    // qq
    BendR(NormalizedSlideSegmentParams),    // pp
    SkipL(NormalizedSlideSegmentParams),    // 1V7
    SkipR(NormalizedSlideSegmentParams),    // 1V3
    Fan(NormalizedSlideSegmentParams),      // w
}

impl std::fmt::Display for NormalizedSlideSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Straight(param) => write!(f, "-{}", param.destination),
            Self::CircleL(param) => {
                let end_index = param.destination.index();
                let upper = !(2..6).contains(&end_index);
                write!(f, "{}{}", if upper { '<' } else { '>' }, param.destination)
            }
            Self::CircleR(param) => {
                let end_index = param.destination.index();
                let upper = !(2..6).contains(&end_index);
                write!(f, "{}{}", if upper { '>' } else { '<' }, param.destination)
            }
            Self::CurveL(param) => {
                write!(f, "p{}", param.destination)
            }
            Self::CurveR(param) => {
                write!(f, "q{}", param.destination)
            }
            Self::ThunderL(param) => {
                write!(f, "s{}", param.destination)
            }
            Self::ThunderR(param) => {
                write!(f, "z{}", param.destination)
            }
            Self::Corner(param) => write!(f, "v{}", param.destination),
            Self::BendL(param) => {
                write!(f, "qq{}", param.destination)
            }
            Self::BendR(param) => {
                write!(f, "pp{}", param.destination)
            }
            Self::SkipL(param) => {
                let interim = Key::new((param.start.index() + 6) % 8).unwrap();
                write!(f, "V{}{}", interim, param.destination)
            }
            Self::SkipR(param) => {
                let interim = Key::new((param.start.index() + 2) % 8).unwrap();
                write!(f, "V{}{}", interim, param.destination)
            }
            Self::Fan(param) => write!(f, "w{}", param.destination),
        }
    }
}

impl NormalizedSlideSegment {
    pub fn shape(&self) -> NormalizedSlideSegmentShape {
        match self {
            Self::Straight(_) => NormalizedSlideSegmentShape::Straight,
            Self::CircleL(_) => NormalizedSlideSegmentShape::CircleL,
            Self::CircleR(_) => NormalizedSlideSegmentShape::CircleR,
            Self::CurveL(_) => NormalizedSlideSegmentShape::CurveL,
            Self::CurveR(_) => NormalizedSlideSegmentShape::CurveR,
            Self::ThunderL(_) => NormalizedSlideSegmentShape::ThunderL,
            Self::ThunderR(_) => NormalizedSlideSegmentShape::ThunderR,
            Self::Corner(_) => NormalizedSlideSegmentShape::Corner,
            Self::BendL(_) => NormalizedSlideSegmentShape::BendL,
            Self::BendR(_) => NormalizedSlideSegmentShape::BendR,
            Self::SkipL(_) => NormalizedSlideSegmentShape::SkipL,
            Self::SkipR(_) => NormalizedSlideSegmentShape::SkipR,
            Self::Fan(_) => NormalizedSlideSegmentShape::Fan,
        }
    }

    pub fn params(&self) -> &NormalizedSlideSegmentParams {
        match self {
            NormalizedSlideSegment::Straight(p) => p,
            NormalizedSlideSegment::CircleL(p) => p,
            NormalizedSlideSegment::CircleR(p) => p,
            NormalizedSlideSegment::CurveL(p) => p,
            NormalizedSlideSegment::CurveR(p) => p,
            NormalizedSlideSegment::ThunderL(p) => p,
            NormalizedSlideSegment::ThunderR(p) => p,
            NormalizedSlideSegment::Corner(p) => p,
            NormalizedSlideSegment::BendL(p) => p,
            NormalizedSlideSegment::BendR(p) => p,
            NormalizedSlideSegment::SkipL(p) => p,
            NormalizedSlideSegment::SkipR(p) => p,
            NormalizedSlideSegment::Fan(p) => p,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Enum)]
pub enum NormalizedSlideSegmentShape {
    Straight, // -
    CircleL,  // counterclockwise
    CircleR,  // clockwise
    CurveL,   // p
    CurveR,   // q
    ThunderL, // s
    ThunderR, // z
    Corner,   // v
    BendL,    // qq
    BendR,    // pp
    SkipL,    // 1V7
    SkipR,    // 1V3
    Fan,      // w
}

impl From<NormalizedSlideSegment> for NormalizedSlideSegmentShape {
    fn from(x: NormalizedSlideSegment) -> Self {
        x.shape()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalizedSlideSegmentParams {
    pub start: Key,
    pub destination: Key,
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
