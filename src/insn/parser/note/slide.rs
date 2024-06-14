use super::duration::{t_dur, t_dur_spec_absolute, t_dur_spec_bpm_num_beats, t_dur_spec_num_beats};
use super::tap;
use super::*;

fn t_slide_dur_simple(s: NomSpan) -> PResult<SlideDuration> {
    let (s, dur) = t_dur(s)?;

    Ok((s, SlideDuration::Simple(dur)))
}

fn t_slide_dur_custom_bpm(s: NomSpan) -> PResult<SlideDuration> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = char('[')(s)?;
    let (s, bpm) = ws(float)(s)?;
    let (s, _) = ws(char('#'))(s)?;
    let (s, dur) = ws(float)(s)?;
    let (s, _) = ws(char(']'))(s)?;

    Ok((
        s,
        SlideDuration::Custom(SlideStopTimeSpec::Bpm(bpm), Duration::Seconds(dur)),
    ))
}

fn t_slide_dur_custom_seconds(s: NomSpan) -> PResult<SlideDuration> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::char;
    use nom::number::complete::float;
    use nom::sequence::preceded;

    let (s, _) = char('[')(s)?;
    let (s, x1) = ws(float)(s)?;
    let (s, dur) = ws(alt((
        preceded(
            tag("##"),
            ws(alt((t_dur_spec_num_beats, t_dur_spec_bpm_num_beats))),
        ),
        preceded(char('#'), t_dur_spec_absolute), // no need to use ws
    )))(s)?;
    let (s, _) = ws(char(']'))(s)?;

    // TODO: following cases are possible in this combinator:
    //
    // - `[160#2.0]` -> stop time=(as in BPM 160) dur=2.0s
    // - `[3##1.5]` -> stop time=(absolute 3s) dur=1.5s
    // - `[3##4:1]` -> stop time=(absolute 3s) dur=4:1
    // - `[3.0##160#4:1]` -> stop time=(absolute 3s) dur=4:1(as in BPM 160)

    Ok((
        s,
        SlideDuration::Custom(SlideStopTimeSpec::Seconds(x1), dur),
    ))
}

// NOTE: must run after t_slide_dur_simple
fn t_slide_dur_custom(s: NomSpan) -> PResult<SlideDuration> {
    // TODO: following cases are possible in this combinator:
    //
    // - `[160#2.0]` -> stop time=(as in BPM 160) dur=2.0s
    // - `[3##1.5]` -> stop time=(absolute 3s) dur=1.5s
    // - `[3##4:1]` -> stop time=(absolute 3s) dur=4:1
    // - `[3.0##160#4:1]` -> stop time=(absolute 3s) dur=4:1(as in BPM 160)
    nom::branch::alt((t_slide_dur_custom_bpm, t_slide_dur_custom_seconds))(s)
}

pub fn t_slide_dur(s: NomSpan) -> PResult<SlideDuration> {
    use nom::branch::alt;

    // simple variant must come before custom
    alt((t_slide_dur_simple, t_slide_dur_custom))(s)
}

// FxE[dur]
// covers everything except FVRE
macro_rules! define_slide_segment {
    (@ $fn_name: ident, $recog: expr, $variant: ident) => {
        #[allow(unused_imports)]
        pub fn $fn_name(s: NomSpan) -> PResult<SlideSegment> {
            use nom::character::complete::char;
            use nom::bytes::complete::tag;

            let (s, _) = $recog(s)?;
            let (s, destination) = ws(t_key)(s)?;

            Ok((
                s,
                SlideSegment::$variant(SlideSegmentParams {
                    destination,
                    interim: None,
                }),
            ))
        }
    };

    ($fn_name: ident, char $ch: expr, $variant: ident) => {
        define_slide_segment!(@ $fn_name, char($ch), $variant);
    };

    ($fn_name: ident, tag $tag: expr, $variant: ident) => {
        define_slide_segment!(@ $fn_name, tag($tag), $variant);
    };
}

define_slide_segment!(t_slide_segment_line, char '-', Line);
define_slide_segment!(t_slide_segment_arc, char '^', Arc);
define_slide_segment!(t_slide_segment_circ_left, char '<', CircumferenceLeft);
define_slide_segment!(t_slide_segment_circ_right, char '>', CircumferenceRight);
define_slide_segment!(t_slide_segment_v, char 'v', V);
define_slide_segment!(t_slide_segment_p, char 'p', P);
define_slide_segment!(t_slide_segment_q, char 'q', Q);
define_slide_segment!(t_slide_segment_s, char 's', S);
define_slide_segment!(t_slide_segment_z, char 'z', Z);
define_slide_segment!(t_slide_segment_pp, tag "pp", Pp);
define_slide_segment!(t_slide_segment_qq, tag "qq", Qq);
define_slide_segment!(t_slide_segment_spread, char 'w', Spread);

pub fn t_slide_segment_angle(s: NomSpan) -> PResult<SlideSegment> {
    use nom::character::complete::char;

    let (s, _) = char('V')(s)?;
    let (s, interim) = ws(t_key)(s)?;
    let (s, destination) = ws(t_key)(s)?;

    Ok((
        s,
        SlideSegment::Angle(SlideSegmentParams {
            destination,
            interim: Some(interim),
        }),
    ))
}

pub fn t_slide_segment(s: NomSpan) -> PResult<SlideSegment> {
    nom::branch::alt((
        t_slide_segment_line,
        t_slide_segment_arc,
        t_slide_segment_circ_left,
        t_slide_segment_circ_right,
        t_slide_segment_v,
        t_slide_segment_p,
        t_slide_segment_q,
        t_slide_segment_s,
        t_slide_segment_z,
        t_slide_segment_pp,
        t_slide_segment_qq,
        t_slide_segment_angle,
        t_slide_segment_spread,
    ))(s)
}

pub fn t_slide_track_modifier(s: NomSpan) -> PResult<SlideTrackModifier> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, variants) = many0(ws(one_of("b")))(s)?;

    Ok((
        if variants.is_empty() { s } else { s1 },
        SlideTrackModifier {
            is_break: variants.iter().any(|&x| x == 'b'),
        },
    ))
}

pub fn t_slide_segment_group(s: NomSpan) -> PResult<(SlideSegmentGroup, SlideTrackModifier)> {
    use nom::combinator::map;

    // TODO: track with different speed
    let (s, segments) = ws_list1(t_slide_segment)(s)?;
    let (s, modifier) = t_slide_track_modifier(s)?;
    let (s, dur) = ws(t_slide_dur)(s)?;
    let (s, modifier) = map(t_slide_track_modifier, move |m| m + modifier)(s)?;

    Ok((s, (SlideSegmentGroup { segments, dur }, modifier)))
}

pub fn t_slide_track(s: NomSpan) -> PResult<SlideTrack> {
    // TODO: track with different speed
    let (s, groups) = ws_list1(t_slide_segment_group)(s)?;
    // it is slightly different from the official syntax
    let modifier = groups
        .iter()
        .map(|(_, modifier)| *modifier)
        .fold(Default::default(), |acc, x| acc + x);

    Ok((
        s,
        SlideTrack {
            groups: groups.into_iter().map(|(group, _)| group).collect(),
            modifier,
        },
    ))
}

pub fn t_slide_sep_track(s: NomSpan) -> PResult<SlideTrack> {
    use nom::character::complete::char;

    let (s, _) = char('*')(s)?;
    let (s, track) = ws(t_slide_track)(s)?;

    Ok((s, track))
}

pub fn t_slide(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::multi::many0;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, start) = tap::t_tap_param(s)?;
    let (s, first_track) = ws(t_slide_track)(s)?;
    let (s, rest_track) = many0(ws(t_slide_sep_track))(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let tracks = {
        let mut tmp = Vec::with_capacity(rest_track.len() + 1);
        tmp.push(first_track);
        tmp.extend(rest_track);
        tmp
    };

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Slide(SlideParams { start, tracks }).with_span(span),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parser_ok {
        ($parser: expr, $start: expr, $rest: expr) => {{
            let (s, result) = $parser(concat!($start, $rest).into())?;
            assert_eq!(*s.fragment(), $rest);
            result
        }};
    }

    macro_rules! test_parser_err {
        ($parser: expr, $start: expr) => {{
            let result = $parser($start.into());
            assert!(result.is_err());
        }};
    }

    #[test]
    fn test_t_slide_dur() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            test_parser_ok!(t_slide_dur, "[ 4 : 3 ]", " ,"),
            SlideDuration::Simple(Duration::NumBeats(NumBeatsParams { divisor: 4, num: 3 }))
        );

        assert_eq!(
            test_parser_ok!(t_slide_dur, "[#2.5]", " ,"),
            SlideDuration::Simple(Duration::Seconds(2.5))
        );

        assert_eq!(
            test_parser_ok!(t_slide_dur, "[ 120.0 #4: 1]", " ,"),
            SlideDuration::Simple(Duration::BpmNumBeats(BpmNumBeatsParams {
                bpm: 120.0,
                divisor: 4,
                num: 1
            }))
        );

        assert_eq!(
            test_parser_ok!(t_slide_dur, "[ 160 #2.0 ]", " ,"),
            SlideDuration::Custom(SlideStopTimeSpec::Bpm(160.0), Duration::Seconds(2.0))
        );
        // [160##2.0] is valid, but it is in the next group

        assert_eq!(
            test_parser_ok!(t_slide_dur, "[ 3.0## 1.5 ]", " ,"),
            SlideDuration::Custom(SlideStopTimeSpec::Seconds(3.0), Duration::Seconds(1.5))
        );
        test_parser_err!(t_slide_dur, "[3.0# #1.5]");
        test_parser_err!(t_slide_dur, "[3.0###1.5]");
        // [3.0#1.5] is valid, but it is in the previous group

        assert_eq!(
            test_parser_ok!(t_slide_dur, "[ 3.0## 4 : 1 ]", " ,"),
            SlideDuration::Custom(
                SlideStopTimeSpec::Seconds(3.0),
                Duration::NumBeats(NumBeatsParams { divisor: 4, num: 1 })
            )
        );
        test_parser_err!(t_slide_dur, "[3.0# #4:1]");
        test_parser_err!(t_slide_dur, "[3.0###4:1]");

        assert_eq!(
            test_parser_ok!(t_slide_dur, "[ 3.0 ##160 #4 : 1 ]", " ,"),
            SlideDuration::Custom(
                SlideStopTimeSpec::Seconds(3.0),
                Duration::BpmNumBeats(BpmNumBeatsParams {
                    bpm: 160.0,
                    divisor: 4,
                    num: 1
                })
            )
        );
        test_parser_err!(t_slide_dur, "[3.0# #160#4:1]");
        test_parser_err!(t_slide_dur, "[3.0###160#4:1]");
        test_parser_err!(t_slide_dur, "[3.0##160##4:1]");

        test_parser_err!(t_slide_dur, "[3.0#160##4:1]");
        test_parser_err!(t_slide_dur, "[3.0#160#1.0]");
        test_parser_err!(t_slide_dur, "[3.0#160##1.0]");
        test_parser_err!(t_slide_dur, "[3.0#4:1##160.0]");
        test_parser_err!(t_slide_dur, "[4:1#3.0##160.0]");

        Ok(())
    }
}
