use super::*;

pub fn t_key(s: NomSpan) -> PResult<Key> {
    use nom::character::complete::one_of;
    use nom::combinator::map;

    map(one_of("12345678"), |s| {
        Key::new(s.to_digit(10).unwrap() as u8 - 1).unwrap()
    })(s)
}

pub fn t_touch_sensor(s: NomSpan) -> PResult<TouchSensor> {
    use nom::character::complete::{char, one_of};
    use nom::combinator::{map, opt};
    use nom::sequence::pair;

    let (s, touch_sensor) = nom::branch::alt((
        pair(one_of("ABDE"), ws(map(one_of("12345678"), Some))),
        pair(char('C'), ws(map(opt(one_of("12")), |_| None))),
    ))(s)?;

    let index = touch_sensor.1.map(|x| x.to_digit(10).unwrap() as u8 - 1);
    Ok((s, TouchSensor::new(touch_sensor.0, index).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    macro_rules! test_parser_ok {
        ($parser: ident, $start: expr, $rest: expr) => {{
            let (s, result) = $parser(concat!($start, $rest).into())?;
            assert_eq!(*s.fragment(), $rest);
            result
        }};
    }

    macro_rules! test_parser_err {
        ($parser: ident, $start: expr) => {{
            let result = $parser($start.into());
            assert!(result.is_err());
        }};
    }

    #[test]
    fn test_t_key() -> Result<(), Box<dyn Error>> {
        assert_eq!(test_parser_ok!(t_key, "1", " ,"), 0.try_into().unwrap());

        test_parser_err!(t_key, " 2");
        test_parser_err!(t_key, "0");
        test_parser_err!(t_key, "9");
        test_parser_err!(t_key, "A1");

        Ok(())
    }

    #[test]
    fn test_t_touch_sensor() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            test_parser_ok!(t_touch_sensor, "E1", " ,"),
            ('E', Some(0)).try_into().unwrap()
        );
        assert_eq!(
            test_parser_ok!(t_touch_sensor, "C", ""),
            ('C', None).try_into().unwrap()
        );
        assert_eq!(
            test_parser_ok!(t_touch_sensor, "C1", ""),
            ('C', None).try_into().unwrap()
        );
        assert_eq!(
            test_parser_ok!(t_touch_sensor, "C2", ""),
            ('C', None).try_into().unwrap()
        );
        assert_eq!(
            test_parser_ok!(t_touch_sensor, "C", "3"),
            ('C', None).try_into().unwrap()
        );

        test_parser_err!(t_touch_sensor, " C");
        test_parser_err!(t_touch_sensor, "E,");
        test_parser_err!(t_touch_sensor, "B9");
        test_parser_err!(t_touch_sensor, "D0");
        test_parser_err!(t_touch_sensor, "1");

        Ok(())
    }
}
