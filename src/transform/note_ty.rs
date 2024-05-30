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
    Straight(NormalizedSlideSegmentParams),
    Circle(NormalizedSlideSegmentParams),
    Corner(NormalizedSlideSegmentParams),
    Round(NormalizedSlideSegmentParams),
    Thunder(NormalizedSlideSegmentParams),
    Curve(NormalizedSlideSegmentParams),
    Turn(NormalizedSlideSegmentParams),
    Fan(NormalizedSlideSegmentParams),
}

impl std::fmt::Display for NormalizedSlideSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Straight(param) => write!(f, "-{}", param.destination),
            Self::Circle(param) => {
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
            Self::Corner(param) => write!(f, "v{}", param.destination),
            Self::Round(param) => {
                write!(
                    f,
                    "{}{}",
                    if param.flip.unwrap() { 'p' } else { 'q' },
                    param.destination
                )
            }
            Self::Thunder(param) => {
                write!(
                    f,
                    "{}{}",
                    if param.flip.unwrap() { 'z' } else { 's' },
                    param.destination
                )
            }
            Self::Curve(param) => {
                write!(
                    f,
                    "{}{}",
                    if param.flip.unwrap() { "pp" } else { "qq" },
                    param.destination
                )
            }
            Self::Turn(param) => {
                let interim =
                    (param.start.index().unwrap() + if param.flip.unwrap() { 2 } else { 6 }) % 8;
                let interim = Key::try_from(interim).unwrap();
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
            Self::Circle(_) => NormalizedSlideSegmentShape::Circle,
            Self::Corner(_) => NormalizedSlideSegmentShape::Corner,
            Self::Round(_) => NormalizedSlideSegmentShape::Round,
            Self::Thunder(_) => NormalizedSlideSegmentShape::Thunder,
            Self::Curve(_) => NormalizedSlideSegmentShape::Curve,
            Self::Turn(_) => NormalizedSlideSegmentShape::Turn,
            Self::Fan(_) => NormalizedSlideSegmentShape::Fan,
        }
    }

    pub fn params(&self) -> &NormalizedSlideSegmentParams {
        match self {
            NormalizedSlideSegment::Straight(p) => p,
            NormalizedSlideSegment::Circle(p) => p,
            NormalizedSlideSegment::Corner(p) => p,
            NormalizedSlideSegment::Round(p) => p,
            NormalizedSlideSegment::Thunder(p) => p,
            NormalizedSlideSegment::Curve(p) => p,
            NormalizedSlideSegment::Turn(p) => p,
            NormalizedSlideSegment::Fan(p) => p,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NormalizedSlideSegmentShape {
    Straight, // -
    Circle,   // <>^ counterclockwise, clockwise(flip)
    Corner,   // v
    Round,    // p, q(filp)
    Thunder,  // s, z(filp)
    Curve,    // pp, qq(filp)
    Turn,     // 1V75, 1V35(flip)
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
