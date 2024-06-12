use super::*;

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
