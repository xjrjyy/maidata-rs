use crate::insn::{Key, Position, RawNoteInsn, SlideSegment, SlideSegmentGroup, SlideTrack};
use crate::transform::{
    NormalizedHoldParams, NormalizedNote, NormalizedSlideParams, NormalizedSlideSegment,
    NormalizedSlideSegmentGroup, NormalizedSlideSegmentParams, NormalizedSlideTrack,
    NormalizedTapParams, NormalizedTouchHoldParams, NormalizedTouchParams,
};

fn key_clockwise_distance(start: Key, end: Key) -> u8 {
    (end.index().unwrap() + 8 - start.index().unwrap()) % 8
}

fn slide_segment_is_clockwise(start: Key, segment: SlideSegment) -> Option<bool> {
    let upper_half = start.index().unwrap() < 2 || start.index().unwrap() >= 6;
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
    segment: SlideSegment,
) -> Option<NormalizedSlideSegment> {
    let distance = key_clockwise_distance(start, segment.params().destination);
    let normalized_params = NormalizedSlideSegmentParams {
        start,
        destination: segment.params().destination,
        flip: match segment {
            SlideSegment::Line(_) | SlideSegment::V(_) | SlideSegment::Spread(_) => None,
            SlideSegment::Arc(_)
            | SlideSegment::CircumferenceLeft(_)
            | SlideSegment::CircumferenceRight(_) => slide_segment_is_clockwise(start, segment),
            SlideSegment::P(_) | SlideSegment::S(_) | SlideSegment::Pp(_) => Some(false),
            SlideSegment::Q(_) | SlideSegment::Z(_) | SlideSegment::Qq(_) => Some(true),
            SlideSegment::Angle(params) => {
                Some(params.interim.unwrap().index().unwrap() - start.index().unwrap() == 2)
            }
        },
    };
    match segment {
        SlideSegment::Line(_) => match distance {
            2..=6 => Some(NormalizedSlideSegment::Line(normalized_params)),
            _ => None,
        },
        SlideSegment::Arc(_) => slide_segment_is_clockwise(start, segment)
            .map(|_| NormalizedSlideSegment::Clockwise(normalized_params)),
        SlideSegment::CircumferenceLeft(_) | SlideSegment::CircumferenceRight(_) => {
            Some(NormalizedSlideSegment::Clockwise(normalized_params))
        }
        SlideSegment::V(_) => match distance {
            0 | 4 => None,
            _ => Some(NormalizedSlideSegment::V(normalized_params)),
        },
        SlideSegment::P(_) | SlideSegment::Q(_) => {
            Some(NormalizedSlideSegment::PQ(normalized_params))
        }
        SlideSegment::S(_) | SlideSegment::Z(_) => match distance {
            4 => Some(NormalizedSlideSegment::SZ(normalized_params)),
            _ => None,
        },
        SlideSegment::Pp(_) | SlideSegment::Qq(_) => {
            Some(NormalizedSlideSegment::PpQq(normalized_params))
        }
        SlideSegment::Angle(params) => match key_clockwise_distance(start, params.interim.unwrap())
        {
            2 | 6 => match key_clockwise_distance(params.interim.unwrap(), params.destination) {
                2..=6 => match start != params.destination {
                    true => Some(NormalizedSlideSegment::Angle(normalized_params)),
                    false => None,
                },
                _ => None,
            },
            _ => None,
        },
        SlideSegment::Spread(_) => match distance {
            4 => Some(NormalizedSlideSegment::Spread(normalized_params)),
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
        .map(|&segment| {
            let result = normalize_slide_segment(start, segment);
            if result.is_none() {
                dbg!(start, group);
            }
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
        macro_rules! segment {
            ($variant: ident, $end: expr) => {
                SlideSegment::$variant(SlideSegmentParams {
                    destination: $end.try_into().unwrap(),
                    interim: None,
                })
            };
        }
        macro_rules! normalized_segment {
            ($variant: ident, $start: expr, $end: expr, $flip: expr) => {
                NormalizedSlideSegment::$variant(NormalizedSlideSegmentParams {
                    start: $start.try_into().unwrap(),
                    destination: $end.try_into().unwrap(),
                    flip: $flip,
                })
            };
        }
        macro_rules! normalize {
            ($variant: ident, $start: expr, $end: expr) => {
                normalize_slide_segment($start.try_into().unwrap(), segment!($variant, $end))
            };
        }

        assert_eq!(
            normalize!(Line, 0, 2),
            Some(normalized_segment!(Line, 0, 2, None))
        );
        assert_eq!(normalize!(Line, 0, 7), None);

        assert_eq!(
            normalize!(Arc, 0, 3),
            Some(normalized_segment!(Clockwise, 0, 3, Some(true)))
        );
        assert_eq!(
            normalize!(Arc, 5, 4),
            Some(normalized_segment!(Clockwise, 5, 4, Some(false)))
        );
        assert_eq!(normalize!(Arc, 0, 4), None);

        assert_eq!(
            normalize!(CircumferenceLeft, 0, 0),
            Some(normalized_segment!(Clockwise, 0, 0, Some(false)))
        );

        assert_eq!(
            normalize!(CircumferenceRight, 6, 6),
            Some(normalized_segment!(Clockwise, 6, 6, Some(true)))
        );

        assert_eq!(
            normalize!(V, 0, 1),
            Some(normalized_segment!(V, 0, 1, None))
        );
        assert_eq!(normalize!(V, 4, 0), None);

        assert_eq!(
            normalize!(P, 3, 3),
            Some(normalized_segment!(PQ, 3, 3, Some(false)))
        );

        assert_eq!(
            normalize!(Q, 5, 5),
            Some(normalized_segment!(PQ, 5, 5, Some(true)))
        );

        assert_eq!(
            normalize!(S, 0, 4),
            Some(normalized_segment!(SZ, 0, 4, Some(false)))
        );
        assert_eq!(normalize!(S, 0, 3), None);

        assert_eq!(
            normalize!(Z, 0, 4),
            Some(normalized_segment!(SZ, 0, 4, Some(true)))
        );
        assert_eq!(normalize!(Z, 0, 3), None);

        assert_eq!(
            normalize!(Pp, 0, 0),
            Some(normalized_segment!(PpQq, 0, 0, Some(false)))
        );

        assert_eq!(
            normalize!(Qq, 0, 0),
            Some(normalized_segment!(PpQq, 0, 0, Some(true)))
        );

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: 6.try_into().unwrap(),
            interim: Some(2.try_into().unwrap()),
        });
        let expected = NormalizedSlideSegment::Angle(NormalizedSlideSegmentParams {
            start: 0.try_into().unwrap(),
            destination: 6.try_into().unwrap(),
            flip: Some(true),
        });
        assert_eq!(
            normalize_slide_segment(0.try_into().unwrap(), segment),
            Some(expected)
        );

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: 3.try_into().unwrap(),
            interim: Some(6.try_into().unwrap()),
        });
        let expected = NormalizedSlideSegment::Angle(NormalizedSlideSegmentParams {
            start: 0.try_into().unwrap(),
            destination: 3.try_into().unwrap(),
            flip: Some(false),
        });
        assert_eq!(
            normalize_slide_segment(0.try_into().unwrap(), segment),
            Some(expected)
        );

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: 1.try_into().unwrap(),
            interim: Some(6.try_into().unwrap()),
        });
        let expected = NormalizedSlideSegment::Angle(NormalizedSlideSegmentParams {
            start: 0.try_into().unwrap(),
            destination: 1.try_into().unwrap(),
            flip: Some(false),
        });
        assert_eq!(
            normalize_slide_segment(0.try_into().unwrap(), segment),
            Some(expected)
        );

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: 4.try_into().unwrap(),
            interim: Some(7.try_into().unwrap()),
        });
        assert_eq!(
            normalize_slide_segment(0.try_into().unwrap(), segment),
            None
        );

        let segment = SlideSegment::Angle(SlideSegmentParams {
            destination: 2.try_into().unwrap(),
            interim: Some(2.try_into().unwrap()),
        });
        assert_eq!(
            normalize_slide_segment(0.try_into().unwrap(), segment),
            None
        );

        assert_eq!(
            normalize!(Spread, 0, 4),
            Some(normalized_segment!(Spread, 0, 4, None))
        );
        assert_eq!(normalize!(Spread, 0, 3), None);

        Ok(())
    }
}
