use super::*;

pub fn t_dur_spec_num_beats(s: NomSpan) -> PResult<Duration> {
    use nom::character::complete::{char, digit1};

    // TODO: support floating point
    let (s, divisor_str) = digit1(s)?;
    let (s, _) = ws(char(':'))(s)?;
    let (s, num_str) = ws(digit1)(s)?;

    // TODO: handle conversion errors
    let divisor = divisor_str.fragment().parse().unwrap();
    let num = num_str.fragment().parse().unwrap();

    Ok((s, Duration::NumBeats(NumBeatsParams { divisor, num })))
}

pub fn t_dur_spec_bpm_num_beats(s: NomSpan) -> PResult<Duration> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, bpm) = float(s)?;
    let (s, _) = ws(char('#'))(s)?;
    let (s, dur) = ws(t_dur_spec_num_beats)(s)?;
    let (divisor, num) = match dur {
        Duration::NumBeats(NumBeatsParams { divisor, num }) => (divisor, num),
        _ => unreachable!(),
    };

    Ok((
        s,
        Duration::BpmNumBeats(BpmNumBeatsParams { bpm, divisor, num }),
    ))
}

pub fn t_dur_spec_absolute(s: NomSpan) -> PResult<Duration> {
    let (s, dur) = t_absolute_duration(s)?;

    Ok((s, Duration::Seconds(dur)))
}

pub fn t_dur_spec(s: NomSpan) -> PResult<Duration> {
    use nom::branch::alt;

    alt((
        t_dur_spec_num_beats,
        t_dur_spec_bpm_num_beats,
        t_dur_spec_absolute,
    ))(s)
}

pub fn t_dur(s: NomSpan) -> PResult<Duration> {
    use nom::character::complete::char;

    // TODO: star-time/BPM overrides
    let (s, _) = char('[')(s)?;
    let (s, dur) = ws(t_dur_spec)(s)?;
    let (s, _) = ws(char(']'))(s)?;

    Ok((s, dur))
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_t_dur() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            test_parser_ok!(t_dur, "[ 4 : 3 ]", " ,"),
            Duration::NumBeats(NumBeatsParams { divisor: 4, num: 3 })
        );
        test_parser_err!(t_dur, " [4:3]");
        test_parser_err!(t_dur, "[4.5:2]");
        test_parser_err!(t_dur, "[4:2.5]");

        assert_eq!(
            test_parser_ok!(t_dur, "[#2.5]", " ,"),
            Duration::Seconds(2.5)
        );
        assert_eq!(
            test_parser_ok!(t_dur, "[ # 1 ]", ""),
            Duration::Seconds(1.0)
        );
        test_parser_err!(t_dur, "[#2.5.0]");
        test_parser_err!(t_dur, "[#2 .5]");

        assert_eq!(
            test_parser_ok!(t_dur, "[ 120.0 #4: 1]", " ,"),
            Duration::BpmNumBeats(BpmNumBeatsParams {
                bpm: 120.0,
                divisor: 4,
                num: 1
            })
        );
        test_parser_err!(t_dur, "[120#4:1.5]");

        test_parser_err!(t_dur, "[4:1#160]");
        test_parser_err!(t_dur, "[4:1#4:1]");
        test_parser_err!(t_dur, "[160#2.0]");
        test_parser_err!(t_dur, "[3.0##1.5]");
        test_parser_err!(t_dur, "[3.0##4:1]");
        test_parser_err!(t_dur, "[3.0##160#4:1]");

        Ok(())
    }
}
