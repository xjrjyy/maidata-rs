use super::tap;
use super::*;

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

pub fn t_slide_segment_group(s: NomSpan) -> PResult<(SlideSegmentGroup, bool)> {
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

    Ok((s, (SlideSegmentGroup { segments, len }, is_break.is_some())))
}

pub fn t_slide_track(s: NomSpan) -> PResult<SlideTrack> {
    use nom::multi::many1;

    let (s, _) = multispace0(s)?;
    // TODO: track with different speed
    let (s, groups) = many1(t_slide_segment_group)(s)?;
    let (s, _) = multispace0(s)?;
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
    let (s, start) = tap::t_tap_param(s)?;
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
