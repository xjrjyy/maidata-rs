use super::*;

pub fn t_tap_modifier(s: NomSpan, mut modifier: TapModifier) -> PResult<TapModifier> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::multi::many0;

    let (s1, start_loc) = nom_locate::position(s)?;
    let (s1, variants) = many0(ws(alt((tag("b"), tag("x"), tag("$$"), tag("$")))))(s1)?;
    let (s1, end_loc) = nom_locate::position(s1)?;
    for x in &variants {
        match *x.fragment() {
            "b" => {
                if modifier.is_break {
                    s.extra.borrow_mut().add_warning(
                        (start_loc, end_loc).into(),
                        "Duplicate `b` modifier in tap instruction".to_string(),
                    );
                }
                modifier.is_break = true;
            }
            "x" => {
                if modifier.is_ex {
                    s.extra.borrow_mut().add_warning(
                        (start_loc, end_loc).into(),
                        "Duplicate `x` modifier in tap instruction".to_string(),
                    );
                }
                modifier.is_ex = true;
            }
            _ => (),
        }
        let shape = match *x.fragment() {
            "$" => Some(TapShape::Star),
            "$$" => Some(TapShape::StarSpin),
            _ => None,
        };
        if let Some(shape) = shape {
            if modifier.shape.is_some() {
                s.extra.borrow_mut().add_error(
                    (start_loc, end_loc).into(),
                    format!(
                        "Duplicate `{}` shape modifier in tap instruction",
                        x.fragment()
                    ),
                );
            } else {
                modifier.shape = Some(shape);
            }
        }
    }

    Ok((if variants.is_empty() { s } else { s1 }, modifier))
}

pub fn t_tap_param(s: NomSpan) -> PResult<TapParams> {
    let (s, key) = t_key(s)?;
    let (s, modifier) = t_tap_modifier(s, TapModifier::default())?;

    Ok((s, TapParams { key, modifier }))
}

pub fn t_tap(s: NomSpan) -> PResult<Option<SpRawNoteInsn>> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) = t_tap_param(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, Some(RawNoteInsn::Tap(params).with_span(span))))
}

pub fn t_tap_multi_simplified_every(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Tap(TapParams {
            key,
            // all taps are regular ones when using simplified notation
            modifier: Default::default(),
        })
        .with_span(span),
    ))
}

pub fn t_tap_multi_simplified(s: NomSpan) -> PResult<Option<SpRawInsn>> {
    let (s, start_loc) = nom_locate::position(s)?;
    // all whitespaces are ignored, including those inside a taps bundle
    // we must parse every key individually (also for getting proper span info)
    let (s, notes) = ws_list1(t_tap_multi_simplified_every)(s)?;
    let (s, _) = ws(t_note_sep)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, Some(RawInsn::NoteBundle(notes).with_span(span))))
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_parser_err, test_parser_ok, test_parser_warn};
    use super::*;
    use std::error::Error;

    #[test]
    fn test_t_tap_param() -> Result<(), Box<dyn Error>> {
        use tap::t_tap_param;
        assert_eq!(
            test_parser_ok(t_tap_param, "1", " ,"),
            TapParams {
                key: 0.try_into().unwrap(),
                modifier: Default::default(),
            }
        );
        assert_eq!(
            test_parser_ok(t_tap_param, "1 b x", ""),
            TapParams {
                key: 0.try_into().unwrap(),
                modifier: TapModifier {
                    is_break: true,
                    is_ex: true,
                    shape: None,
                },
            }
        );
        assert_eq!(
            test_parser_ok(t_tap_param, "1 x", ""),
            TapParams {
                key: 0.try_into().unwrap(),
                modifier: TapModifier {
                    is_break: false,
                    is_ex: true,
                    shape: None,
                },
            }
        );

        test_parser_err(t_tap_param, "");
        test_parser_err(t_tap_param, " 1");
        test_parser_err(t_tap_param, "x1");
        test_parser_warn(t_tap_param, "1xx");

        Ok(())
    }
}
