use super::duration::t_dur;
use super::*;

pub fn t_touch_hold_modifier_str(s: NomSpan) -> PResult<Vec<char>> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, variants) = many0(ws(one_of("f")))(s)?;

    Ok((if variants.is_empty() { s } else { s1 }, variants))
}

pub fn t_touch_hold(s: NomSpan) -> PResult<Option<SpRawNoteInsn>> {
    use nom::character::complete::char;
    use nom::combinator::map;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, sensor) = t_touch_sensor(s)?;
    let (s, modifier_str) = t_touch_hold_modifier_str(s)?;
    let (s, _) = ws(char('h'))(s)?;
    let (s, modifier_str) = map(t_touch_hold_modifier_str, |mut m| {
        // TODO
        m.extend(modifier_str.clone());
        m
    })(s)?;
    let (s, dur) = ws(t_dur).expect("expected duration")(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let mut modifier = TouchHoldModifier::default();
    for x in &modifier_str {
        match *x {
            'f' => {
                if modifier.is_firework {
                    s.extra.borrow_mut().add_warning(
                        (start_loc, end_loc).into(),
                        "duplicate `f` modifier in touch hold instruction".to_string(),
                    );
                }
                modifier.is_firework = true;
            }
            _ => unreachable!(),
        }
    }

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
