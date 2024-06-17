use super::duration::t_dur;
use super::*;

pub fn t_hold_modifier(s: NomSpan, mut modifier: HoldModifier) -> PResult<HoldModifier> {
    use nom::character::complete::one_of;
    use nom::multi::many0;

    let (s1, start_loc) = nom_locate::position(s)?;
    let (s1, variants) = many0(ws(one_of("bx")))(s1)?;
    let (s1, end_loc) = nom_locate::position(s1)?;
    for x in &variants {
        match *x {
            'b' => {
                if modifier.is_break {
                    s.extra.borrow_mut().add_warning(
                        (start_loc, end_loc).into(),
                        "Duplicate `b` modifier in hold instruction".to_string(),
                    );
                }
                modifier.is_break = true;
            }
            'x' => {
                if modifier.is_ex {
                    s.extra.borrow_mut().add_warning(
                        (start_loc, end_loc).into(),
                        "Duplicate `x` modifier in hold instruction".to_string(),
                    );
                }
                modifier.is_ex = true;
            }
            _ => unreachable!(),
        }
    }

    Ok((if variants.is_empty() { s } else { s1 }, modifier))
}

pub fn t_hold(s: NomSpan) -> PResult<Option<SpRawNoteInsn>> {
    use nom::character::complete::char;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, modifier) = t_hold_modifier(s, HoldModifier::default())?;
    let (s, _) = ws(char('h'))(s)?;
    let (s, modifier) = t_hold_modifier(s, modifier)?;
    let (s, dur) = ws(t_dur).expect("expected duration")(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        dur.flatten()
            .map(|dur| RawNoteInsn::Hold(HoldParams { key, dur, modifier }).with_span(span)),
    ))
}
