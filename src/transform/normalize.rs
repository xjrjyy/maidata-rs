use crate::insn::{Key, RawNoteInsn, SlideSegment, SlideSegmentGroup, SlideTrack};
use crate::transform::{
    NormalizedHoldParams, NormalizedNote, NormalizedSlideParams, NormalizedSlideSegment,
    NormalizedSlideSegmentGroup, NormalizedSlideSegmentParams, NormalizedSlideTrack,
    NormalizedTapParams, NormalizedTouchHoldParams, NormalizedTouchParams,
};

fn key_clockwise_distance(start: Key, end: Key) -> u8 {
    (end.index() + 8 - start.index()) % 8
}

fn slide_segment_is_clockwise(start: Key, segment: &SlideSegment) -> Option<bool> {
    let upper_half = start.index() < 2 || start.index() >= 6;
    match segment {
        SlideSegment::Arc(params) => match key_clockwise_distance(start, params.destination) {
            1..=3 => Some(true),
            5..=7 => Some(false),
            _ => None,
        },
        SlideSegment::CircumferenceLeft(_) => Some(!upper_half),
        SlideSegment::CircumferenceRight(_) => Some(upper_half),
        _ => None,
    }
}

// pub(crate) fn normalized_circumference_slide_segment(
//     start: Key,
//     end: Key,
//     clockwise: bool,
// ) -> SlideSegment {
//     let upper_half = start.index().unwrap() < 2 || start.index().unwrap() >= 6;
//     let params = SlideSegmentParams {
//         destination: end,
//     };
//     if clockwise ^ upper_half {
//         SlideSegment::CircumferenceLeft(params)
//     } else {
//         SlideSegment::CircumferenceRight(params)
//     }
// }

pub fn normalize_slide_segment(
    start: Key,
    segment: &SlideSegment,
) -> Option<NormalizedSlideSegment> {
    let distance = key_clockwise_distance(start, segment.params().destination);
    let normalized_params = NormalizedSlideSegmentParams {
        start,
        destination: segment.params().destination,
    };
    match segment {
        SlideSegment::Line(_) => match distance {
            2..=6 => Some(NormalizedSlideSegment::Straight(normalized_params)),
            _ => None,
        },
        SlideSegment::Arc(_)
        | SlideSegment::CircumferenceLeft(_)
        | SlideSegment::CircumferenceRight(_) => match slide_segment_is_clockwise(start, segment) {
            Some(false) => Some(NormalizedSlideSegment::CircleL(normalized_params)),
            Some(true) => Some(NormalizedSlideSegment::CircleR(normalized_params)),
            None => None,
        },
        SlideSegment::P(_) => Some(NormalizedSlideSegment::CurveL(normalized_params)),
        SlideSegment::Q(_) => Some(NormalizedSlideSegment::CurveR(normalized_params)),
        SlideSegment::S(_) => match distance {
            4 => Some(NormalizedSlideSegment::ThunderL(normalized_params)),
            _ => None,
        },
        SlideSegment::Z(_) => match distance {
            4 => Some(NormalizedSlideSegment::ThunderR(normalized_params)),
            _ => None,
        },
        SlideSegment::V(_) => match distance {
            0 | 4 => None,
            _ => Some(NormalizedSlideSegment::Corner(normalized_params)),
        },
        SlideSegment::Qq(_) => Some(NormalizedSlideSegment::BendL(normalized_params)),
        SlideSegment::Pp(_) => Some(NormalizedSlideSegment::BendR(normalized_params)),
        SlideSegment::Angle(params) => {
            match key_clockwise_distance(params.interim.unwrap(), params.destination) {
                2..=6 => match start != params.destination {
                    true => match key_clockwise_distance(start, params.interim.unwrap()) {
                        6 => Some(NormalizedSlideSegment::SkipL(normalized_params)),
                        2 => Some(NormalizedSlideSegment::SkipR(normalized_params)),
                        _ => None,
                    },
                    false => None,
                },
                _ => None,
            }
        }
        SlideSegment::Spread(_) => match distance {
            4 => Some(NormalizedSlideSegment::Fan(normalized_params)),
            _ => None,
        },
    }
}

pub fn normalize_slide_segment_group(
    start: Key,
    group: &SlideSegmentGroup,
) -> Option<NormalizedSlideSegmentGroup> {
    let mut start = start;
    group
        .segments
        .iter()
        .map(|segment| {
            let result = normalize_slide_segment(start, segment);
            // TODO: trait for slide end position
            start = segment.params().destination;
            result
        })
        .collect::<Option<Vec<_>>>()
        .map(|segments| NormalizedSlideSegmentGroup { segments })
}

pub fn normalize_slide_track(start: Key, track: &SlideTrack) -> Option<NormalizedSlideTrack> {
    let mut start = start;
    track
        .groups
        .iter()
        .map(|group| {
            let result = normalize_slide_segment_group(start, group);
            // TODO: trait for slide end position
            start = group.segments.last().unwrap().params().destination;
            result
        })
        .collect::<Option<Vec<_>>>()
        .map(|groups| NormalizedSlideTrack { groups })
}

pub fn normalize_note(note: &RawNoteInsn) -> Option<NormalizedNote> {
    match note {
        RawNoteInsn::Tap(params) => {
            Some(NormalizedNote::Tap(NormalizedTapParams { key: params.key }))
        }
        RawNoteInsn::Touch(params) => Some(NormalizedNote::Touch(NormalizedTouchParams {
            sensor: params.sensor,
        })),
        RawNoteInsn::Hold(params) => Some(NormalizedNote::Hold(NormalizedHoldParams {
            key: params.key,
        })),
        RawNoteInsn::TouchHold(params) => {
            Some(NormalizedNote::TouchHold(NormalizedTouchHoldParams {
                sensor: params.sensor,
            }))
        }
        RawNoteInsn::Slide(params) => params
            .tracks
            .iter()
            .map(|track| normalize_slide_track(params.start.key, track))
            .collect::<Option<Vec<_>>>()
            .map(|mut tracks| {
                tracks.sort();
                NormalizedNote::Slide(NormalizedSlideParams {
                    start: NormalizedTapParams {
                        key: params.start.key,
                    },
                    tracks,
                })
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insn::SlideSegmentParams;
    use std::error::Error;

    #[test]
    fn test_normalize_slide_segment() -> Result<(), Box<dyn Error>> {
        let keys = (0..8).map(|x| Key::new(x).unwrap()).collect::<Vec<_>>();

        macro_rules! segment {
            ($variant: ident, $end: expr) => {
                SlideSegment::$variant(SlideSegmentParams {
                    destination: keys[$end],
                    interim: None,
                })
            };
        }
        macro_rules! normalized_segment {
            ($variant: ident, $start: expr, $end: expr) => {
                NormalizedSlideSegment::$variant(NormalizedSlideSegmentParams {
                    start: keys[$start],
                    destination: keys[$end],
                })
            };
        }
        macro_rules! normalize {
            ($variant: ident, $start: expr, $end: expr) => {
                normalize_slide_segment(keys[$start], &segment!($variant, $end))
            };
        }

        assert_eq!(
            normalize!(Line, 0, 2),
            Some(normalized_segment!(Straight, 0, 2))
        );
        assert_eq!(normalize!(Line, 0, 7), None);

        assert_eq!(
            normalize!(Arc, 0, 3),
            Some(normalized_segment!(CircleR, 0, 3))
        );
        assert_eq!(
            normalize!(Arc, 5, 4),
            Some(normalized_segment!(CircleL, 5, 4))
        );
        assert_eq!(normalize!(Arc, 0, 4), None);

        assert_eq!(
            normalize!(CircumferenceLeft, 0, 0),
            Some(normalized_segment!(CircleL, 0, 0))
        );

        assert_eq!(
            normalize!(CircumferenceRight, 6, 6),
            Some(normalized_segment!(CircleR, 6, 6))
        );

        assert_eq!(normalize!(P, 3, 3), Some(normalized_segment!(CurveL, 3, 3)));

        assert_eq!(normalize!(Q, 5, 5), Some(normalized_segment!(CurveR, 5, 5)));

        assert_eq!(
            normalize!(S, 0, 4),
            Some(normalized_segment!(ThunderL, 0, 4))
        );
        assert_eq!(normalize!(S, 0, 3), None);

        assert_eq!(
            normalize!(Z, 0, 4),
            Some(normalized_segment!(ThunderR, 0, 4))
        );
        assert_eq!(normalize!(Z, 0, 3), None);

        assert_eq!(normalize!(V, 0, 1), Some(normalized_segment!(Corner, 0, 1)));
        assert_eq!(normalize!(V, 4, 0), None);

        assert_eq!(normalize!(Qq, 0, 0), Some(normalized_segment!(BendL, 0, 0)));

        assert_eq!(normalize!(Pp, 0, 0), Some(normalized_segment!(BendR, 0, 0)));

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: keys[6],
            interim: Some(keys[2]),
        });
        let expected = NormalizedSlideSegment::SkipR(NormalizedSlideSegmentParams {
            start: keys[0],
            destination: keys[6],
        });
        assert_eq!(normalize_slide_segment(keys[0], &segment), Some(expected));

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: keys[3],
            interim: Some(keys[6]),
        });
        let expected = NormalizedSlideSegment::SkipL(NormalizedSlideSegmentParams {
            start: keys[0],
            destination: keys[3],
        });
        assert_eq!(normalize_slide_segment(keys[0], &segment), Some(expected));

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: keys[3],
            interim: Some(keys[5]),
        });
        let expected = NormalizedSlideSegment::SkipL(NormalizedSlideSegmentParams {
            start: keys[7],
            destination: keys[3],
        });
        assert_eq!(normalize_slide_segment(keys[7], &segment), Some(expected));

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: keys[2],
            interim: Some(keys[0]),
        });
        let expected = NormalizedSlideSegment::SkipR(NormalizedSlideSegmentParams {
            start: keys[6],
            destination: keys[2],
        });
        assert_eq!(normalize_slide_segment(keys[6], &segment), Some(expected));

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: keys[1],
            interim: Some(keys[6]),
        });
        let expected = NormalizedSlideSegment::SkipL(NormalizedSlideSegmentParams {
            start: keys[0],
            destination: keys[1],
        });
        assert_eq!(normalize_slide_segment(keys[0], &segment), Some(expected));

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: keys[4],
            interim: Some(keys[7]),
        });
        assert_eq!(normalize_slide_segment(keys[0], &segment), None);

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: keys[2],
            interim: Some(keys[2]),
        });
        assert_eq!(normalize_slide_segment(keys[0], &segment), None);

        assert_eq!(
            normalize!(Spread, 0, 4),
            Some(normalized_segment!(Fan, 0, 4))
        );
        assert_eq!(normalize!(Spread, 0, 3), None);

        Ok(())
    }
}
