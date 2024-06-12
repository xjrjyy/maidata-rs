use super::*;

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
