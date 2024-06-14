use super::duration::t_dur;
use super::*;

pub fn t_hold_modifier(s: NomSpan) -> PResult<HoldModifier> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, variants) = many0(ws(one_of("bx")))(s)?;

    Ok((
        if variants.is_empty() { s } else { s1 },
        HoldModifier {
            is_break: variants.iter().any(|&x| x == 'b'),
            is_ex: variants.iter().any(|&x| x == 'x'),
        },
    ))
}

pub fn t_hold(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::char;
    use nom::combinator::map;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, modifier) = t_hold_modifier(s)?;
    let (s, _) = ws(char('h'))(s)?;
    let (s, modifier) = map(t_hold_modifier, move |m| m + modifier)(s)?;
    let (s, dur) = ws(t_dur)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Hold(HoldParams { key, dur, modifier }).with_span(span),
    ))
}
