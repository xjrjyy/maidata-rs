use super::*;

pub fn t_tap_param(s: NomSpan) -> PResult<TapParams> {
    use nom::character::complete::char;
    use nom::combinator::opt;

    let (s, _) = multispace0(s)?;
    let (s, key) = t_key(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_ex) = opt(char('x'))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_break) = opt(char('b'))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_ex) = match is_ex {
        Some(_) => (s, is_ex),
        None => opt(char('x'))(s)?,
    };
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        TapParams {
            is_break: is_break.is_some(),
            is_ex: is_ex.is_some(),
            key,
        },
    ))
}

pub fn t_tap(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) = t_tap_param(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawNoteInsn::Tap(params).with_span(span)))
}

pub fn t_tap_multi_simplified_every(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Tap(TapParams {
            // all taps are regular ones when using simplified notation
            is_break: false,
            is_ex: false,
            key,
        })
        .with_span(span),
    ))
}

pub fn t_tap_multi_simplified(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    // all whitespaces are ignored, including those inside a taps bundle
    // we must parse every key individually (also for getting proper span info)
    let (s, notes) = many1(t_tap_multi_simplified_every)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::NoteBundle(notes).with_span(span)))
}

pub fn t_touch_param(s: NomSpan) -> PResult<TouchParams> {
    use nom::character::complete::char;
    use nom::combinator::opt;

    let (s, _) = multispace0(s)?;
    let (s, sensor) = t_touch_sensor(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_firework) = opt(char('f'))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        TouchParams {
            is_firework: is_firework.is_some(),
            sensor,
        },
    ))
}

pub fn t_touch(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) = t_touch_param(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawNoteInsn::Touch(params).with_span(span)))
}

pub fn t_len_spec_beats(s: NomSpan) -> PResult<Length> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;

    let (s, divisor_str) = digit1(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, num_str) = digit1(s)?;
    let (s, _) = multispace0(s)?;

    // TODO: handle conversion errors
    let divisor = divisor_str.fragment().parse().unwrap();
    let num = num_str.fragment().parse().unwrap();

    Ok((s, Length::NumBeats(NumBeatsParams { divisor, num })))
}

pub fn t_len_spec_absolute(s: NomSpan) -> PResult<Length> {
    let (s, dur) = t_absolute_duration(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, Length::Seconds(dur)))
}

pub fn t_len_spec(s: NomSpan) -> PResult<Length> {
    use nom::branch::alt;

    alt((t_len_spec_beats, t_len_spec_absolute))(s)
}

pub fn t_len(s: NomSpan) -> PResult<Length> {
    use nom::character::complete::char;

    // TODO: star-time/BPM overrides
    let (s, _) = multispace0(s)?;
    let (s, _) = char('[')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len_spec(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(']')(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, len))
}

pub fn t_hold(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::{char, one_of};
    use nom::multi::many0;
    use nom::sequence::terminated;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, _) = multispace0(s)?;
    let (s, prev_variants) = many0(terminated(one_of("bx"), multispace0))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('h')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, next_variants) = many0(terminated(one_of("bx"), multispace0))(s)?;
    // TODO: `1bbh[4:1]` is currently considered legal
    let is_break = prev_variants
        .iter()
        .chain(next_variants.iter())
        .any(|&x| x == 'b');
    let is_ex = prev_variants
        .iter()
        .chain(next_variants.iter())
        .any(|&x| x == 'x');
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Hold(HoldParams {
            is_break,
            is_ex,
            key,
            len,
        })
        .with_span(span),
    ))
}

pub fn t_touch_hold(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::char;
    use nom::combinator::opt;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, sensor) = t_touch_sensor(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_firework) = opt(char('f'))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('h')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_firework) = match is_firework {
        Some(_) => (s, is_firework),
        None => opt(char('f'))(s)?,
    };
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::TouchHold(TouchHoldParams {
            is_firework: is_firework.is_some(),
            sensor,
            len,
        })
        .with_span(span),
    ))
}

pub fn t_slide_len_simple(s: NomSpan) -> PResult<SlideLength> {
    let (s, len) = t_len(s)?;

    Ok((s, SlideLength::Simple(len)))
}

// NOTE: must run after t_slide_len_simple
pub fn t_slide_len_custom(s: NomSpan) -> PResult<SlideLength> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('[')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, x1) = float(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('#')(s)?;
    let (s, len) = t_len_spec(s)?;
    let (s, _) = char(']')(s)?;
    let (s, _) = multispace0(s)?;

    // TODO: following cases are possible in this combinator:
    //
    // - `[160#8:3]` -> stop time=(as in BPM 160) len=8:3
    // - `[3##1.5]` -> stop time=(absolute 3s) len=1.5s
    let stop_time_spec = match len {
        Length::NumBeats(_) => SlideStopTimeSpec::Bpm(x1),
        Length::Seconds(_) => SlideStopTimeSpec::Seconds(x1),
    };

    Ok((s, SlideLength::Custom(stop_time_spec, len)))
}

pub fn t_slide_len(s: NomSpan) -> PResult<SlideLength> {
    use nom::branch::alt;

    // simple variant must come before custom
    alt((t_slide_len_simple, t_slide_len_custom))(s)
}

// FxE[len]
// covers everything except FVRE
macro_rules! define_slide_segment {
    (@ $fn_name: ident, $recog: expr, $variant: ident) => {
        #[allow(unused_imports)]
        pub fn $fn_name(s: NomSpan) -> PResult<SlideSegment> {
            use nom::character::complete::char;
            use nom::bytes::complete::tag;

            let (s, _) = multispace0(s)?;
            let (s, _) = $recog(s)?;
            let (s, _) = multispace0(s)?;
            let (s, destination) = t_key(s)?;
            let (s, _) = multispace0(s)?;

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

    let (s, _) = multispace0(s)?;
    let (s, _) = char('V')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, interim) = t_key(s)?;
    let (s, _) = multispace0(s)?;
    let (s, destination) = t_key(s)?;
    let (s, _) = multispace0(s)?;

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

pub fn t_slide_segment_group(s: NomSpan) -> PResult<SlideSegmentGroup> {
    use nom::character::complete::char;
    use nom::combinator::opt;
    use nom::multi::many1;

    let (s, _) = multispace0(s)?;
    // TODO: track with different speed
    let (s, segments) = many1(t_slide_segment)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_break) = opt(char('b'))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_slide_len(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_break) = match is_break {
        Some(_) => (s, is_break),
        None => opt(char('b'))(s)?,
    };
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        SlideSegmentGroup {
            is_break: is_break.is_some(),
            segments,
            len,
        },
    ))
}

pub fn t_slide_track(s: NomSpan) -> PResult<SlideTrack> {
    use nom::multi::many1;

    let (s, _) = multispace0(s)?;
    // TODO: track with different speed
    let (s, groups) = many1(t_slide_segment_group)(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, SlideTrack { groups }))
}

pub fn t_slide_sep_track(s: NomSpan) -> PResult<SlideTrack> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('*')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, track) = t_slide_track(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, track))
}

pub fn t_slide(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::multi::many0;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, start) = t_tap_param(s)?;
    let (s, first_track) = t_slide_track(s)?;
    let (s, rest_track) = many0(t_slide_sep_track)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

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

pub fn t_single_note(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, note) = nom::branch::alt((t_hold, t_touch_hold, t_slide, t_tap, t_touch))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::Note(note).with_span(span)))
}

pub fn t_bundle_note(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, _) = multispace0(s)?;
    // NOTE: tap and touch must come last as it can match on the simplest key, blocking holds and slides from parsing
    let (s, note) = nom::branch::alt((t_hold, t_touch_hold, t_slide, t_tap, t_touch))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

pub fn t_bundle_sep_note(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('/')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, note) = t_bundle_note(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

pub fn t_bundle(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, first) = t_bundle_note(s)?;
    let (s, _) = multispace0(s)?;
    let (s, rest) = many1(t_bundle_sep_note)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let notes = {
        let mut tmp = Vec::with_capacity(rest.len() + 1);
        tmp.push(first);
        tmp.extend(rest);
        tmp
    };

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::NoteBundle(notes).with_span(span)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    macro_rules! test_parser_ok {
        ($parser: ident, $start: expr, $rest: expr) => {{
            let (s, result) = $parser(concat!($start, $rest).into())?;
            assert_eq!(*s.fragment(), $rest);
            result
        }};
    }

    macro_rules! test_parser_err {
        ($parser: ident, $start: expr) => {{
            let result = $parser($start.into());
            assert!(result.is_err());
        }};
    }
}
