#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Key {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
}

#[derive(Clone, Debug)]
pub enum KeyParseError {
    InvalidKey(char),
}

impl std::convert::TryFrom<char> for Key {
    type Error = KeyParseError;

    fn try_from(x: char) -> Result<Self, Self::Error> {
        match x {
            '1' => Ok(Self::K1),
            '2' => Ok(Self::K2),
            '3' => Ok(Self::K3),
            '4' => Ok(Self::K4),
            '5' => Ok(Self::K5),
            '6' => Ok(Self::K6),
            '7' => Ok(Self::K7),
            '8' => Ok(Self::K8),
            _ => Err(KeyParseError::InvalidKey(x)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TouchSensor {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
}

#[derive(Clone, Debug)]
pub enum TouchSensorParseError {
    InvalidTouchSensor((char, char)),
}

impl std::convert::TryFrom<(char, char)> for TouchSensor {
    type Error = TouchSensorParseError;

    fn try_from(x: (char, char)) -> Result<Self, Self::Error> {
        match x.0 {
            'A' => match x.1 {
                '1' => Ok(Self::A1),
                '2' => Ok(Self::A2),
                '3' => Ok(Self::A3),
                '4' => Ok(Self::A4),
                '5' => Ok(Self::A5),
                '6' => Ok(Self::A6),
                '7' => Ok(Self::A7),
                '8' => Ok(Self::A8),
                _ => Err(TouchSensorParseError::InvalidTouchSensor(x)),
            },
            'B' => match x.1 {
                '1' => Ok(Self::B1),
                '2' => Ok(Self::B2),
                '3' => Ok(Self::B3),
                '4' => Ok(Self::B4),
                '5' => Ok(Self::B5),
                '6' => Ok(Self::B6),
                '7' => Ok(Self::B7),
                '8' => Ok(Self::B8),
                _ => Err(TouchSensorParseError::InvalidTouchSensor(x)),
            },
            'C' => match x.1 {
                '1' => Ok(Self::C),
                '2' => Ok(Self::C),
                _ => Err(TouchSensorParseError::InvalidTouchSensor(x)),
            },
            'D' => match x.1 {
                '1' => Ok(Self::D1),
                '2' => Ok(Self::D2),
                '3' => Ok(Self::D3),
                '4' => Ok(Self::D4),
                '5' => Ok(Self::D5),
                '6' => Ok(Self::D6),
                '7' => Ok(Self::D7),
                '8' => Ok(Self::D8),
                _ => Err(TouchSensorParseError::InvalidTouchSensor(x)),
            },
            'E' => match x.1 {
                '1' => Ok(Self::E1),
                '2' => Ok(Self::E2),
                '3' => Ok(Self::E3),
                '4' => Ok(Self::E4),
                '5' => Ok(Self::E5),
                '6' => Ok(Self::E6),
                '7' => Ok(Self::E7),
                '8' => Ok(Self::E8),
                _ => Err(TouchSensorParseError::InvalidTouchSensor(x)),
            },
            _ => Err(TouchSensorParseError::InvalidTouchSensor(x)),
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
    // TODO: Break Slide
    // pub is_break: bool,
    pub groups: Vec<SlideSegmentGroup>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideSegmentGroup {
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
