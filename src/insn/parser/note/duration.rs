use super::*;

pub fn t_dur_spec_beats(s: NomSpan) -> PResult<Duration> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;

    // TODO: support floating point
    let (s, divisor_str) = digit1(s)?;
    let (s, _) = ws(char(':'))(s)?;
    let (s, num_str) = ws(digit1)(s)?;

    // TODO: handle conversion errors
    let divisor = divisor_str.fragment().parse().unwrap();
    let num = num_str.fragment().parse().unwrap();

    Ok((s, Duration::NumBeats(NumBeatsParams { divisor, num })))
}

pub fn t_dur_spec_absolute(s: NomSpan) -> PResult<Duration> {
    let (s, dur) = t_absolute_duration(s)?;

    Ok((s, Duration::Seconds(dur)))
}

pub fn t_dur_spec(s: NomSpan) -> PResult<Duration> {
    use nom::branch::alt;

    alt((t_dur_spec_beats, t_dur_spec_absolute))(s)
}

pub fn t_dur(s: NomSpan) -> PResult<Duration> {
    use nom::character::complete::char;

    // TODO: star-time/BPM overrides
    let (s, _) = char('[')(s)?;
    let (s, dur) = ws(t_dur_spec)(s)?;
    let (s, _) = ws(char(']'))(s)?;

    Ok((s, dur))
}
