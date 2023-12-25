pub trait Position {
    fn group(self) -> Option<char>;
    fn index(self) -> Option<u8>;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Key {
    index: u8,
}

impl Position for Key {
    fn group(self) -> Option<char> {
        None
    }
    fn index(self) -> Option<u8> {
        Some(self.index)
    }
}

#[derive(Clone, Debug)]
pub enum KeyParseError {
    InvalidKey(u8),
}

impl std::convert::TryFrom<u8> for Key {
    type Error = KeyParseError;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0..=7 => Ok(Key { index: x }),
            _ => Err(Self::Error::InvalidKey(x)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TouchSensor {
    group: char,
    index: Option<u8>,
}

impl Position for TouchSensor {
    fn group(self) -> Option<char> {
        Some(self.group)
    }
    fn index(self) -> Option<u8> {
        self.index
    }
}

#[derive(Clone, Debug)]
pub enum TouchSensorParseError {
    InvalidTouchSensor(char, Option<u8>),
}

impl std::convert::TryFrom<(char, Option<u8>)> for TouchSensor {
    type Error = TouchSensorParseError;

    fn try_from(x: (char, Option<u8>)) -> Result<Self, Self::Error> {
        match x.0 {
            'A' | 'B' | 'D' | 'E' => match x.1 {
                Some(index) => match index {
                    0..=7 => Ok(TouchSensor {
                        group: x.0,
                        index: Some(index),
                    }),
                    _ => Err(Self::Error::InvalidTouchSensor(x.0, x.1)),
                },
                _ => Err(Self::Error::InvalidTouchSensor(x.0, x.1)),
            },
            'C' => match x.1 {
                None => Ok(TouchSensor {
                    group: x.0,
                    index: None,
                }),
                _ => Err(Self::Error::InvalidTouchSensor(x.0, x.1)),
            },
            _ => Err(TouchSensorParseError::InvalidTouchSensor(x.0, x.1)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Length {
    NumBeats(NumBeatsParams),
    Seconds(f32),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SlideStopTimeSpec {
    Bpm(f32),
    Seconds(f32),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SlideLength {
    Simple(Length),
    Custom(SlideStopTimeSpec, Length),
}

impl SlideLength {
    pub fn slide_duration(&self) -> Length {
        match self {
            SlideLength::Simple(l) => *l,
            SlideLength::Custom(_, l) => *l,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NumBeatsParams {
    pub divisor: u32,
    pub num: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TapParams {
    pub is_break: bool,
    pub is_ex: bool,
    pub key: Key,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TouchParams {
    pub is_firework: bool,
    pub sensor: TouchSensor,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HoldParams {
    pub is_break: bool,
    pub is_ex: bool,
    pub key: Key,
    pub len: Length,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TouchHoldParams {
    pub is_firework: bool,
    pub sensor: TouchSensor,
    pub len: Length,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideParams {
    pub start: TapParams,
    pub tracks: Vec<SlideTrack>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideTrack {
    pub groups: Vec<SlideSegmentGroup>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideSegmentGroup {
    // it is slightly different from the official syntax
    pub is_break: bool,
    pub segments: Vec<SlideSegment>,
    pub len: SlideLength,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SlideSegment {
    Line(SlideSegmentParams),
    Arc(SlideSegmentParams), // ???
    CircumferenceLeft(SlideSegmentParams),
    CircumferenceRight(SlideSegmentParams),
    V(SlideSegmentParams),
    P(SlideSegmentParams),
    Q(SlideSegmentParams),
    S(SlideSegmentParams),
    Z(SlideSegmentParams),
    Pp(SlideSegmentParams),
    Qq(SlideSegmentParams),
    Angle(SlideSegmentParams),
    Spread(SlideSegmentParams),
}

impl SlideSegment {
    pub fn shape(&self) -> SlideSegmentShape {
        match self {
            Self::Line(_) => SlideSegmentShape::Line,
            Self::Arc(_) => SlideSegmentShape::Arc,
            Self::CircumferenceLeft(_) => SlideSegmentShape::CircumferenceLeft,
            Self::CircumferenceRight(_) => SlideSegmentShape::CircumferenceRight,
            Self::V(_) => SlideSegmentShape::V,
            Self::P(_) => SlideSegmentShape::P,
            Self::Q(_) => SlideSegmentShape::Q,
            Self::S(_) => SlideSegmentShape::S,
            Self::Z(_) => SlideSegmentShape::Z,
            Self::Pp(_) => SlideSegmentShape::Pp,
            Self::Qq(_) => SlideSegmentShape::Qq,
            Self::Angle(_) => SlideSegmentShape::Angle,
            Self::Spread(_) => SlideSegmentShape::Spread,
        }
    }

    pub fn params(&self) -> &SlideSegmentParams {
        match self {
            SlideSegment::Line(p) => p,
            SlideSegment::Arc(p) => p,
            SlideSegment::CircumferenceLeft(p) => p,
            SlideSegment::CircumferenceRight(p) => p,
            SlideSegment::V(p) => p,
            SlideSegment::P(p) => p,
            SlideSegment::Q(p) => p,
            SlideSegment::S(p) => p,
            SlideSegment::Z(p) => p,
            SlideSegment::Pp(p) => p,
            SlideSegment::Qq(p) => p,
            SlideSegment::Angle(p) => p,
            SlideSegment::Spread(p) => p,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SlideSegmentShape {
    Line,
    Arc,
    CircumferenceLeft,
    CircumferenceRight,
    V,
    P,
    Q,
    S,
    Z,
    Pp,
    Qq,
    Angle,
    Spread,
}

impl From<SlideSegment> for SlideSegmentShape {
    fn from(x: SlideSegment) -> Self {
        x.shape()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideSegmentParams {
    pub destination: Key,
    pub interim: Option<Key>,
}
