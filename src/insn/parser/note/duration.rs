use super::*;

pub fn t_dur_spec_num_beats(s: NomSpan) -> PResult<Option<Duration>> {
    use nom::character::complete::{char, digit1};

    // TODO: support floating point
    let (s, divisor_str) = digit1(s)?;
    let (s, _) = ws(char(':'))(s)?;
    let (s, num_str) = ws(digit1).expect("expected number of beats")(s)?;
    if num_str.is_none() {
        return Ok((s, None));
    }

    // TODO: handle conversion errors
    let divisor = divisor_str.fragment().parse().unwrap();
    let num = num_str.unwrap().fragment().parse().unwrap();

    Ok((s, Some(Duration::NumBeats(NumBeatsParams { divisor, num }))))
}

pub fn t_dur_spec_bpm_num_beats(s: NomSpan) -> PResult<Option<Duration>> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, bpm) = float(s)?;
    let (s, _) = ws(char('#'))(s)?;
    let (s, dur) = ws(t_dur_spec_num_beats)(s)?;
    let (divisor, num) = match dur {
        Some(Duration::NumBeats(NumBeatsParams { divisor, num })) => (divisor, num),
        _ => return Ok((s, None)),
    };

    Ok((
        s,
        Some(Duration::BpmNumBeats(BpmNumBeatsParams {
            bpm,
            divisor,
            num,
        })),
    ))
}

pub fn t_dur_spec_absolute(s: NomSpan) -> PResult<Option<Duration>> {
    let (s, dur) = t_absolute_duration(s)?;

    Ok((s, Some(Duration::Seconds(dur))))
}

pub fn t_dur_spec(s: NomSpan) -> PResult<Option<Duration>> {
    use nom::branch::alt;

    alt((
        t_dur_spec_num_beats,
        t_dur_spec_bpm_num_beats,
        t_dur_spec_absolute,
    ))(s)
}

pub fn t_dur(s: NomSpan) -> PResult<Option<Duration>> {
    use nom::character::complete::char;
    use nom::sequence::delimited;

    // TODO: star-time/BPM overrides
    let (s, dur) = delimited(
        char('['),
        ws(t_dur_spec).expect("expected duration specification"),
        ws(char(']').expect("missing `]` after duration specification")),
    )(s)?;

    Ok((s, dur.flatten()))
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_parser_err, test_parser_ok};
    use super::*;

    #[test]
    fn test_t_dur() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            test_parser_ok(t_dur, "[ 4 : 3 ]", " ,").unwrap(),
            Duration::NumBeats(NumBeatsParams { divisor: 4, num: 3 })
        );
        test_parser_err(t_dur, " [4:3]");
        test_parser_err(t_dur, "[4.5:2]");
        test_parser_err(t_dur, "[4:2.5]");

        assert_eq!(
            test_parser_ok(t_dur, "[#2.5]", " ,").unwrap(),
            Duration::Seconds(2.5)
        );
        assert_eq!(
            test_parser_ok(t_dur, "[ # 1 ]", "").unwrap(),
            Duration::Seconds(1.0)
        );
        test_parser_err(t_dur, "[#2.5.0]");
        test_parser_err(t_dur, "[#2 .5]");

        assert_eq!(
            test_parser_ok(t_dur, "[ 120.0 #4: 1]", " ,").unwrap(),
            Duration::BpmNumBeats(BpmNumBeatsParams {
                bpm: 120.0,
                divisor: 4,
                num: 1
            })
        );
        test_parser_err(t_dur, "[120#4:1.5]");

        test_parser_err(t_dur, "[4:1#160]");
        test_parser_err(t_dur, "[4:1#4:1]");
        test_parser_err(t_dur, "[160#2.0]");
        test_parser_err(t_dur, "[3.0##1.5]");
        test_parser_err(t_dur, "[3.0##4:1]");
        test_parser_err(t_dur, "[3.0##160#4:1]");

        Ok(())
    }
}
