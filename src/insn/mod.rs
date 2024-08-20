mod directives_ty;
mod note_ty;

pub use directives_ty::*;
pub use note_ty::*;

#[derive(Clone, PartialEq, Debug)]
pub enum RawNoteInsn {
    Tap(TapParams),
    Touch(TouchParams),
    Hold(HoldParams),
    TouchHold(TouchHoldParams),
    Slide(SlideParams),
}

impl std::fmt::Display for RawNoteInsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tap(params) => write!(f, "{}", params),
            Self::Touch(params) => write!(f, "{}", params),
            Self::Hold(params) => write!(f, "{}", params),
            Self::TouchHold(params) => write!(f, "{}", params),
            Self::Slide(params) => write!(f, "{}", params),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum RawInsn {
    Bpm(BpmParams),
    BeatDivisor(BeatDivisorParams),
    Rest,
    Notes(crate::VecSp<RawNoteInsn>),
    EndMark,
}

pub type SpRawInsn = crate::Sp<RawInsn>;
pub type SpRawNoteInsn = crate::Sp<RawNoteInsn>;
