use super::duration::t_dur;
use super::*;

pub fn t_touch_hold_modifier(
    s: NomSpan,
    mut modifier: TouchHoldModifier,
) -> PResult<TouchHoldModifier> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, start_loc) = nom_locate::position(s)?;
    let (s1, variants) = many0(ws(one_of("f")))(s1)?;
    let (s1, end_loc) = nom_locate::position(s1)?;
    for x in &variants {
        match *x {
            'f' => {
                if modifier.is_firework {
                    s.extra.borrow_mut().add_warning(
                        (start_loc, end_loc).into(),
                        "Duplicate `f` modifier in touch hold instruction".to_string(),
                    );
                }
                modifier.is_firework = true;
            }
            _ => unreachable!(),
        }
    }

    Ok((if variants.is_empty() { s } else { s1 }, modifier))
}

pub fn t_touch_hold(s: NomSpan) -> PResult<Option<SpRawNoteInsn>> {
    use nom::character::complete::char;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, sensor) = t_touch_sensor(s)?;
    let (s, modifier) = t_touch_hold_modifier(s, TouchHoldModifier::default())?;
    let (s, _) = ws(char('h'))(s)?;
    let (s, modifier) = t_touch_hold_modifier(s, modifier)?;
    let (s, dur) = ws(t_dur).expect("expected duration")(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        dur.flatten().map(|dur| {
            RawNoteInsn::TouchHold(TouchHoldParams {
                sensor,
                dur,
                modifier,
            })
            .with_span(span)
        }),
    ))
}
