use super::*;

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
