pub trait Position {
    fn group(&self) -> Option<char>;
    fn index(&self) -> Option<u8>;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Key {
    index: u8,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index + 1)
    }
}

impl Position for Key {
    fn group(&self) -> Option<char> {
        None
    }
    fn index(&self) -> Option<u8> {
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TouchSensor {
    group: char,
    index: Option<u8>,
}

impl std::fmt::Display for TouchSensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.group)?;
        if let Some(index) = self.index {
            write!(f, "{}", index + 1)?;
        }
        Ok(())
    }
}

impl Position for TouchSensor {
    fn group(&self) -> Option<char> {
        Some(self.group)
    }
    fn index(&self) -> Option<u8> {
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

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumBeats(params) => write!(f, "{}", params),
            Self::Seconds(seconds) => write!(f, "#{}", seconds),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SlideStopTimeSpec {
    Bpm(f32),
    Seconds(f32),
}

impl std::fmt::Display for SlideStopTimeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bpm(x) => write!(f, "{}", x),
            Self::Seconds(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SlideLength {
    Simple(Length),
    Custom(SlideStopTimeSpec, Length),
}

impl std::fmt::Display for SlideLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(length) => write!(f, "{}", length),
            Self::Custom(spec, length) => write!(f, "{}#{}", spec, length),
        }
    }
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

impl std::fmt::Display for NumBeatsParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.divisor, self.num)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TapParams {
    pub is_break: bool,
    pub is_ex: bool,
    pub key: Key,
}

impl std::fmt::Display for TapParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)?;
        if self.is_break {
            write!(f, "b")?;
        }
        if self.is_ex {
            write!(f, "x")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TouchParams {
    pub is_firework: bool,
    pub sensor: TouchSensor,
}

impl std::fmt::Display for TouchParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sensor)?;
        if self.is_firework {
            write!(f, "f")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HoldParams {
    pub is_break: bool,
    pub is_ex: bool,
    pub key: Key,
    pub len: Length,
}

impl std::fmt::Display for HoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)?;
        if self.is_break {
            write!(f, "b")?;
        }
        if self.is_ex {
            write!(f, "x")?;
        }
        write!(f, "h[{}]", self.len)?;
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TouchHoldParams {
    pub is_firework: bool,
    pub sensor: TouchSensor,
    pub len: Length,
}

impl std::fmt::Display for TouchHoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sensor)?;
        if self.is_firework {
            write!(f, "f")?;
        }
        write!(f, "h[{}]", self.len)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideParams {
    pub start: TapParams,
    pub tracks: Vec<SlideTrack>,
}

impl std::fmt::Display for SlideParams {
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

#[derive(Clone, PartialEq, Debug)]
pub struct SlideTrack {
    pub groups: Vec<SlideSegmentGroup>,
}

impl std::fmt::Display for SlideTrack {
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

#[derive(Clone, PartialEq, Debug)]
pub struct SlideSegmentGroup {
    // it is slightly different from the official syntax
    pub is_break: bool,
    pub segments: Vec<SlideSegment>,
    pub len: SlideLength,
}

impl std::fmt::Display for SlideSegmentGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in self.segments.iter() {
            write!(f, "{}", segment)?;
        }
        write!(f, "[{}]", self.len)?;
        if self.is_break {
            write!(f, "b")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
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

impl std::fmt::Display for SlideSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line(params) => write!(f, "-{}", params.destination),
            Self::Arc(params) => write!(f, "^{}", params.destination),
            Self::CircumferenceLeft(params) => write!(f, "<{}", params.destination),
            Self::CircumferenceRight(params) => write!(f, ">{}", params.destination),
            Self::V(params) => write!(f, "v{}", params.destination),
            Self::P(params) => write!(f, "p{}", params.destination),
            Self::Q(params) => write!(f, "q{}", params.destination),
            Self::S(params) => write!(f, "s{}", params.destination),
            Self::Z(params) => write!(f, "z{}", params.destination),
            Self::Pp(params) => write!(f, "pp{}", params.destination),
            Self::Qq(params) => write!(f, "qq{}", params.destination),
            Self::Angle(params) => write!(f, "V{}{}", params.interim.unwrap(), params.destination),
            Self::Spread(params) => write!(f, "w{}", params.destination),
        }
    }
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SlideSegmentParams {
    pub destination: Key,
    pub interim: Option<Key>,
}
