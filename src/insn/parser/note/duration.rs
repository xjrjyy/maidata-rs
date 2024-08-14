use super::*;

pub fn t_dur_spec_num_beats_params(s: NomSpan) -> PResult<Option<NumBeatsParams>> {
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

    Ok((
        s,
        Some(NumBeatsParams {
            bpm: None,
            divisor,
            num,
        })
        .filter(|_| divisor > 0 && num > 0),
    ))
}

pub fn t_dur_spec_bpm_num_beats_params(s: NomSpan) -> PResult<Option<NumBeatsParams>> {
    use nom::character::complete::char;
    use nom::number::complete::double;

    let (s, bpm) = double(s)?;
    let (s, _) = ws(char('#'))(s)?;
    let (s, mut dur) = ws(t_dur_spec_num_beats_params)(s)?;

    if let Some(dur) = dur.as_mut() {
        dur.bpm = Some(bpm);
    }

    Ok((s, dur.filter(|_| bpm.is_finite() && bpm > 0.0)))
}

pub fn t_dur_spec_num_beats(s: NomSpan) -> PResult<Option<Duration>> {
    use nom::branch::alt;

    alt((t_dur_spec_num_beats_params, t_dur_spec_bpm_num_beats_params))(s)
        .map(|(s, dur)| (s, dur.map(Duration::NumBeats)))
}

pub fn t_dur_spec_absolute(s: NomSpan) -> PResult<Option<Duration>> {
    let (s, dur) = t_absolute_duration(s)?;

    Ok((
        s,
        Some(Duration::Seconds(dur)).filter(|_| dur.is_finite() && dur > 0.0),
    ))
}

pub fn t_dur_spec(s: NomSpan) -> PResult<Option<Duration>> {
    use nom::branch::alt;

    alt((t_dur_spec_num_beats, t_dur_spec_absolute))(s)
}

pub fn t_dur(s: NomSpan) -> PResult<Option<Duration>> {
    let (s, dur) = expect_ws_delimited(t_dur_spec, "duration specification", "[", "]")(s)?;

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
            Duration::NumBeats(NumBeatsParams {
                bpm: None,
                divisor: 4,
                num: 3
            })
        );
        assert_eq!(test_parser_ok(t_dur, "[0:1]", ""), None);
        assert_eq!(test_parser_ok(t_dur, "[1:0]", ""), None);
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
        assert_eq!(test_parser_ok(t_dur, "[#0]", ""), None);
        assert_eq!(test_parser_ok(t_dur, "[#-1]", ""), None);
        assert_eq!(test_parser_ok(t_dur, "[#inf]", ""), None);
        assert_eq!(test_parser_ok(t_dur, "[#nan]", ""), None);
        test_parser_err(t_dur, "[#2.5.0]");
        test_parser_err(t_dur, "[#2 .5]");

        assert_eq!(
            test_parser_ok(t_dur, "[ 120.0 #4: 1]", " ,").unwrap(),
            Duration::NumBeats(NumBeatsParams {
                bpm: Some(120.0),
                divisor: 4,
                num: 1
            })
        );
        assert_eq!(test_parser_ok(t_dur, "[0#1:1]", ""), None);
        assert_eq!(test_parser_ok(t_dur, "[120#0:1]", ""), None);
        assert_eq!(test_parser_ok(t_dur, "[120#1:0]", ""), None);
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
