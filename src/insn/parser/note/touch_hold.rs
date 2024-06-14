use super::duration::t_dur;
use super::*;

pub fn t_touch_hold(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::char;
    use nom::combinator::opt;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, sensor) = t_touch_sensor(s)?;
    let (s, is_firework) = opt(ws(char('f')))(s)?;
    let (s, _) = ws(char('h'))(s)?;
    let (s, is_firework) = match is_firework {
        Some(_) => (s, is_firework),
        None => opt(ws(char('f')))(s)?,
    };
    let (s, dur) = ws(t_dur)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::TouchHold(TouchHoldParams {
            is_firework: is_firework.is_some(),
            sensor,
            dur,
        })
        .with_span(span),
    ))
}
