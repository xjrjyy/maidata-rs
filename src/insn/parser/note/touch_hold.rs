use super::duration::t_dur;
use super::*;

pub fn t_touch_hold_modifier(s: NomSpan) -> PResult<TouchHoldModifier> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, variants) = many0(ws(one_of("f")))(s)?;
    let modifier = variants
        .iter()
        .try_fold(TouchHoldModifier::default(), |acc, &x| {
            acc + TouchHoldModifier {
                is_firework: x == 'f',
            }
        })
        .map_err(|e| nom::Err::Failure(e.into()))?;

    Ok((if variants.is_empty() { s } else { s1 }, modifier))
}

pub fn t_touch_hold(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::char;
    use nom::combinator::map;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, sensor) = t_touch_sensor(s)?;
    let (s, modifier) = t_touch_hold_modifier(s)?;
    let (s, _) = ws(char('h'))(s)?;
    let (s, modifier) = map(t_touch_hold_modifier, move |m| m + modifier)(s)?;
    let modifier = modifier.map_err(|e| nom::Err::Failure(e.into()))?;
    let (s, dur) = ws(t_dur)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::TouchHold(TouchHoldParams {
            sensor,
            dur,
            modifier,
        })
        .with_span(span),
    ))
}
