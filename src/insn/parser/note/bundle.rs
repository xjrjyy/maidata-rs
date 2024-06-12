use super::*;
use hold::t_hold;
use slide::t_slide;
use tap::t_tap;
use touch::t_touch;
use touch_hold::t_touch_hold;

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
