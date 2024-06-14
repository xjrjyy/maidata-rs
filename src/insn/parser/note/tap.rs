use super::*;

pub fn t_tap_modifier(s: NomSpan) -> PResult<TapModifier> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, variants) = many0(ws(one_of("bx")))(s)?;
    let modifier = variants
        .iter()
        .try_fold(TapModifier::default(), |acc, &x| {
            acc + TapModifier {
                is_break: x == 'b',
                is_ex: x == 'x',
                shape: None,
            }
        })
        .map_err(|e| nom::Err::Failure(e.into()))?;

    Ok((if variants.is_empty() { s } else { s1 }, modifier))
}

pub fn t_tap_param(s: NomSpan) -> PResult<TapParams> {
    let (s, key) = t_key(s)?;
    let (s, modifier) = t_tap_modifier(s)?;

    Ok((s, TapParams { key, modifier }))
}

pub fn t_tap(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) = t_tap_param(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawNoteInsn::Tap(params).with_span(span)))
}

pub fn t_tap_multi_simplified_every(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Tap(TapParams {
            key,
            // all taps are regular ones when using simplified notation
            modifier: Default::default(),
        })
        .with_span(span),
    ))
}

pub fn t_tap_multi_simplified(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    // all whitespaces are ignored, including those inside a taps bundle
    // we must parse every key individually (also for getting proper span info)
    let (s, notes) = ws_list1(t_tap_multi_simplified_every)(s)?;
    let (s, _) = ws(t_note_sep)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::NoteBundle(notes).with_span(span)))
}
