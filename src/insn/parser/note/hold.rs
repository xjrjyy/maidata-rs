use super::duration::t_dur;
use super::*;

pub fn t_hold(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::{char, one_of};
    use nom::multi::many0;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, prev_variants) = many0(ws(one_of("bx")))(s)?;
    let (s, _) = ws(char('h'))(s)?;
    let (s, next_variants) = many0(ws(one_of("bx")))(s)?;
    // TODO: `1bbh[4:1]` is currently considered legal
    let is_break = prev_variants
        .iter()
        .chain(next_variants.iter())
        .any(|&x| x == 'b');
    let is_ex = prev_variants
        .iter()
        .chain(next_variants.iter())
        .any(|&x| x == 'x');
    let (s, dur) = ws(t_dur)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Hold(HoldParams {
            is_break,
            is_ex,
            key,
            dur,
        })
        .with_span(span),
    ))
}
