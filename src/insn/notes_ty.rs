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
pub enum SlideTrack {
    Line(SlideTrackParams),
    Arc(SlideTrackParams), // ???
    CircumferenceLeft(SlideTrackParams),
    CircumferenceRight(SlideTrackParams),
    V(SlideTrackParams),
    P(SlideTrackParams),
    Q(SlideTrackParams),
    S(SlideTrackParams),
    Z(SlideTrackParams),
    Pp(SlideTrackParams),
    Qq(SlideTrackParams),
    Angle(SlideTrackParams),
    Spread(SlideTrackParams),
}

impl SlideTrack {
    pub fn shape(&self) -> SlideShape {
        match self {
            Self::Line(_) => SlideShape::Line,
            Self::Arc(_) => SlideShape::Arc,
            Self::CircumferenceLeft(_) => SlideShape::CircumferenceLeft,
            Self::CircumferenceRight(_) => SlideShape::CircumferenceRight,
            Self::V(_) => SlideShape::V,
            Self::P(_) => SlideShape::P,
            Self::Q(_) => SlideShape::Q,
            Self::S(_) => SlideShape::S,
            Self::Z(_) => SlideShape::Z,
            Self::Pp(_) => SlideShape::Pp,
            Self::Qq(_) => SlideShape::Qq,
            Self::Angle(_) => SlideShape::Angle,
            Self::Spread(_) => SlideShape::Spread,
        }
    }

    pub fn params(&self) -> &SlideTrackParams {
        match self {
            SlideTrack::Line(p) => p,
            SlideTrack::Arc(p) => p,
            SlideTrack::CircumferenceLeft(p) => p,
            SlideTrack::CircumferenceRight(p) => p,
            SlideTrack::V(p) => p,
            SlideTrack::P(p) => p,
            SlideTrack::Q(p) => p,
            SlideTrack::S(p) => p,
            SlideTrack::Z(p) => p,
            SlideTrack::Pp(p) => p,
            SlideTrack::Qq(p) => p,
            SlideTrack::Angle(p) => p,
            SlideTrack::Spread(p) => p,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SlideShape {
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

impl From<SlideTrack> for SlideShape {
    fn from(x: SlideTrack) -> Self {
        x.shape()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideTrackParams {
    pub destination: TapParams,
    pub interim: Option<TapParams>,
    pub len: SlideLength,
}
