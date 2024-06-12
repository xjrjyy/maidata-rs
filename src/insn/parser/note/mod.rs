mod bundle;
mod hold;
mod slide;
mod tap;
mod touch;
mod touch_hold;

use super::*;

pub use bundle::{t_bundle, t_single_note};
pub use tap::t_tap_multi_simplified;

fn t_len_spec_beats(s: NomSpan) -> PResult<Length> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;

    // TODO: support floating point
    let (s, divisor_str) = digit1(s)?;
    let (s, _) = ws(char(':'))(s)?;
    let (s, num_str) = ws(digit1)(s)?;

    // TODO: handle conversion errors
    let divisor = divisor_str.fragment().parse().unwrap();
    let num = num_str.fragment().parse().unwrap();

    Ok((s, Length::NumBeats(NumBeatsParams { divisor, num })))
}

fn t_len_spec_absolute(s: NomSpan) -> PResult<Length> {
    let (s, dur) = t_absolute_duration(s)?;

    Ok((s, Length::Seconds(dur)))
}

fn t_len_spec(s: NomSpan) -> PResult<Length> {
    use nom::branch::alt;

    alt((t_len_spec_beats, t_len_spec_absolute))(s)
}

fn t_len(s: NomSpan) -> PResult<Length> {
    use nom::character::complete::char;

    // TODO: star-time/BPM overrides
    let (s, _) = char('[')(s)?;
    let (s, len) = ws(t_len_spec)(s)?;
    let (s, _) = ws(char(']'))(s)?;

    Ok((s, len))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    macro_rules! test_parser_ok {
        ($parser: expr, $start: expr, $rest: expr) => {{
            let (s, result) = $parser(concat!($start, $rest).into())?;
            assert_eq!(*s.fragment(), $rest);
            result
        }};
    }

    macro_rules! test_parser_err {
        ($parser: expr, $start: expr) => {{
            let result = $parser($start.into());
            assert!(result.is_err());
        }};
    }

    #[test]
    fn test_t_tap_param() -> Result<(), Box<dyn Error>> {
        use tap::t_tap_param;
        assert_eq!(
            test_parser_ok!(t_tap_param, "1", " ,"),
            TapParams {
                is_break: false,
                is_ex: false,
                key: 0.try_into().unwrap(),
            }
        );
        assert_eq!(
            test_parser_ok!(t_tap_param, "1 b x", ""),
            TapParams {
                is_break: true,
                is_ex: true,
                key: 0.try_into().unwrap(),
            }
        );
        assert_eq!(
            test_parser_ok!(t_tap_param, "1 x", ""),
            TapParams {
                is_break: false,
                is_ex: true,
                key: 0.try_into().unwrap(),
            }
        );

        test_parser_err!(t_tap_param, "");
        test_parser_err!(t_tap_param, " 1");
        test_parser_err!(t_tap_param, "x1");

        Ok(())
    }

    #[test]
    fn test_t_touch_param() -> Result<(), Box<dyn Error>> {
        use touch::t_touch_param;
        assert_eq!(
            test_parser_ok!(t_touch_param, "B7", " ,"),
            TouchParams {
                is_firework: false,
                sensor: ('B', Some(6)).try_into().unwrap(),
            }
        );
        assert_eq!(
            test_parser_ok!(t_touch_param, "C 1 f", ""),
            TouchParams {
                is_firework: true,
                sensor: ('C', None).try_into().unwrap(),
            }
        );

        test_parser_err!(t_touch_param, "");
        test_parser_err!(t_touch_param, " A1");
        test_parser_err!(t_touch_param, "Af2");

        Ok(())
    }
}
