use super::duration::{t_dur, t_dur_spec};
use super::tap;
use super::*;

pub fn t_slide_dur_simple(s: NomSpan) -> PResult<SlideDuration> {
    let (s, dur) = t_dur(s)?;

    Ok((s, SlideDuration::Simple(dur)))
}

// NOTE: must run after t_slide_dur_simple
pub fn t_slide_dur_custom(s: NomSpan) -> PResult<SlideDuration> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = char('[')(s)?;
    let (s, x1) = ws(float)(s)?;
    let (s, _) = ws(char('#'))(s)?;
    let (s, dur) = ws(t_dur_spec)(s)?;
    let (s, _) = ws(char(']'))(s)?;

    // TODO: following cases are possible in this combinator:
    //
    // - `[160#8:3]` -> stop time=(as in BPM 160) dur=8:3
    // - `[3##1.5]` -> stop time=(absolute 3s) dur=1.5s
    let stop_time_spec = match dur {
        Duration::NumBeats(_) => SlideStopTimeSpec::Bpm(x1),
        Duration::Seconds(_) => SlideStopTimeSpec::Seconds(x1),
    };

    Ok((s, SlideDuration::Custom(stop_time_spec, dur)))
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

pub fn t_slide_segment_group(s: NomSpan) -> PResult<(SlideSegmentGroup, bool)> {
    use nom::character::complete::char;
    use nom::combinator::opt;

    // TODO: track with different speed
    let (s, segments) = ws_list1(t_slide_segment)(s)?;
    let (s, is_break) = opt(ws(char('b')))(s)?;
    let (s, dur) = ws(t_slide_dur)(s)?;
    let (s, is_break) = match is_break {
        Some(_) => (s, is_break),
        None => opt(ws(char('b')))(s)?,
    };

    Ok((s, (SlideSegmentGroup { segments, dur }, is_break.is_some())))
}

pub fn t_slide_track(s: NomSpan) -> PResult<SlideTrack> {
    // TODO: track with different speed
    let (s, groups) = ws_list1(t_slide_segment_group)(s)?;
    // it is slightly different from the official syntax
    let is_break = groups.iter().any(|(_, is_break)| *is_break);

    Ok((
        s,
        SlideTrack {
            groups: groups.into_iter().map(|(group, _)| group).collect(),
            is_break,
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
