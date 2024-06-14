use super::*;

pub fn t_touch_modifier(s: NomSpan) -> PResult<TouchModifier> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, variants) = many0(ws(one_of("f")))(s)?;
    let modifier = variants
        .iter()
        .try_fold(TouchModifier::default(), |acc, &x| {
            acc + TouchModifier {
                is_firework: x == 'f',
            }
        })
        .map_err(|e| nom::Err::Failure(e.into()))?;

    Ok((if variants.is_empty() { s } else { s1 }, modifier))
}

pub fn t_touch_param(s: NomSpan) -> PResult<TouchParams> {
    let (s, sensor) = t_touch_sensor(s)?;
    let (s, modifier) = t_touch_modifier(s)?;

    Ok((s, TouchParams { sensor, modifier }))
}

pub fn t_touch(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) = t_touch_param(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawNoteInsn::Touch(params).with_span(span)))
}
