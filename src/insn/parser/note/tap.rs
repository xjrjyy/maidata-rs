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
