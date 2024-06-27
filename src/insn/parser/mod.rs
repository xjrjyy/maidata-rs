mod note;
mod position;

use super::*;
use crate::span::{expect_ws_delimited, Expect};
use crate::{NomSpan, PResult, WithSpan};
use nom::character::complete::multispace0;
use note::{t_bundle, t_tap_multi_simplified};
use position::*;

/// remove leading whitespace
fn ws<'a, F, O>(f: F) -> impl FnMut(NomSpan<'a>) -> PResult<'a, O>
where
    F: 'a + FnMut(NomSpan<'a>) -> PResult<'a, O>,
{
    nom::sequence::preceded(multispace0, f)
}

fn ws_list0<'a, F, O>(mut f: F) -> impl FnMut(NomSpan<'a>) -> PResult<'a, Vec<O>>
where
    F: 'a + FnMut(NomSpan<'a>) -> PResult<'a, O>,
{
    // TODO: nom::multi::separated_list0(multispace0, f) will not work as expected (#1691)
    // wait for nom 8.0.0...
    use nom::Err;
    move |mut i: NomSpan<'a>| {
        let mut res = Vec::new();

        match f(i) {
            Err(Err::Error(_)) => return Ok((i, res)),
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            match multispace0(i) {
                Err(Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => match f(i1) {
                    Err(Err::Error(_)) => return Ok((i, res)),
                    Err(e) => return Err(e),
                    Ok((i2, o)) => {
                        res.push(o);
                        i = i2;
                    }
                },
            }
        }
    }
}

fn ws_list1<'a, F, O>(mut f: F) -> impl FnMut(NomSpan<'a>) -> PResult<'a, Vec<O>>
where
    F: 'a + FnMut(NomSpan<'a>) -> PResult<'a, O>,
{
    // TODO: nom::multi::separated_list1(multispace0, f) will not work as expected (#1691)
    // wait for nom 8.0.0...
    use nom::Err;
    move |mut i: NomSpan<'a>| {
        let mut res = Vec::new();

        match f(i) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            match multispace0(i) {
                Err(Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => match f(i1) {
                    Err(Err::Error(_)) => return Ok((i, res)),
                    Err(e) => return Err(e),
                    Ok((i2, o)) => {
                        res.push(o);
                        i = i2;
                    }
                },
            }
        }
    }
}

pub(crate) fn parse_maidata_insns(s: NomSpan) -> PResult<Vec<SpRawInsn>> {
    let (s, _) = multispace0(s)?;
    let (s, insns) = ws_list0(parse_one_maidata_insn)(s)?;

    Ok((s, insns.into_iter().flatten().collect()))
}

fn parse_one_maidata_insn(s: NomSpan) -> PResult<Option<SpRawInsn>> {
    use nom::branch::alt;
    use nom::combinator::map;

    alt((
        t_bpm,
        t_beat_divisor,
        t_rest,
        t_tap_multi_simplified,
        t_bundle,
        t_end_mark,
        map(t_comment, |_| None),
        // TODO: handle unknown characters
        t_unknown_char,
    ))(s)
}

fn t_comment(s: NomSpan) -> PResult<()> {
    use nom::bytes::complete::tag;

    let (s, _) = tag("||")(s)?;
    let (s, _) = nom::character::complete::not_line_ending(s)?;
    Ok((s, ()))
}

fn t_unknown_char(s: NomSpan) -> PResult<Option<SpRawInsn>> {
    use nom::character::complete::anychar;

    let (start_loc, _) = nom_locate::position(s)?;
    let (s, c) = anychar(s)?;
    let (end_loc, _) = nom_locate::position(s)?;
    s.extra.borrow_mut().add_error(
        (start_loc, end_loc).into(),
        format!("unknown character: `{}`", c),
    );

    Ok((s, None))
}

fn t_end_mark(s: NomSpan) -> PResult<Option<SpRawInsn>> {
    use nom::character::complete::char;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('E')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, Some(RawInsn::EndMark.with_span(span))))
}

fn t_note_sep(s: NomSpan) -> PResult<()> {
    use nom::character::complete::char;

    let (s, _) = char(',')(s)?;
    Ok((s, ()))
}

fn t_bpm(s: NomSpan) -> PResult<Option<SpRawInsn>> {
    use nom::number::complete::double;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, bpm) = expect_ws_delimited(ws(double), "bpm value", "(", ")")(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);

    Ok((
        s,
        bpm.map(|bpm| RawInsn::Bpm(BpmParams { new_bpm: bpm }).with_span(span)),
    ))
}

fn t_absolute_duration(s: NomSpan) -> PResult<f64> {
    use nom::character::complete::char;
    use nom::number::complete::double;

    let (s, _) = char('#')(s)?;
    let (s, dur) = ws(double)(s)?;

    Ok((s, dur))
}

fn t_beat_divisor_param_int(s: NomSpan) -> PResult<BeatDivisorParams> {
    use nom::character::complete::digit1;

    let (s, divisor_str) = digit1(s)?;

    let divisor = divisor_str.fragment().parse().unwrap();

    Ok((s, BeatDivisorParams::NewDivisor(divisor)))
}

fn t_beat_divisor_param_float(s: NomSpan) -> PResult<BeatDivisorParams> {
    let (s, dur) = t_absolute_duration(s)?;

    Ok((s, BeatDivisorParams::NewAbsoluteDuration(dur)))
}

fn t_beat_divisor_param(s: NomSpan) -> PResult<BeatDivisorParams> {
    use nom::branch::alt;

    alt((t_beat_divisor_param_int, t_beat_divisor_param_float))(s)
}

fn t_beat_divisor(s: NomSpan) -> PResult<Option<SpRawInsn>> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) =
        expect_ws_delimited(ws(t_beat_divisor_param), "beat divisor parameter", "{", "}")(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        params.map(|params| RawInsn::BeatDivisor(params).with_span(span)),
    ))
}

fn t_rest(s: NomSpan) -> PResult<Option<SpRawInsn>> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, Some(RawInsn::Rest.with_span(span))))
}

#[cfg(test)]
mod tests {
    use crate::State;

    use super::*;
    use std::error::Error;

    pub fn test_parser_ok<T>(
        parser: impl Fn(NomSpan<'_>) -> PResult<T>,
        start: &str,
        rest: &str,
    ) -> T {
        let state = std::cell::RefCell::new(crate::State::default());
        let start = start.to_owned() + rest;
        let s = NomSpan::new_extra(&start, &state);
        let (s, result) = parser(s).expect("parser cannot fail");
        assert_eq!(state.borrow().warnings, vec![]);
        assert_eq!(state.borrow().errors, vec![]);
        assert_eq!(*s.fragment(), rest);
        result
    }

    pub fn test_parser_err<T>(parser: impl Fn(NomSpan<'_>) -> PResult<T>, start: &str) -> State {
        let state = std::cell::RefCell::new(crate::State::default());
        let s = NomSpan::new_extra(start, &state);
        let result = parser(s);
        // TODO: split
        assert!(result.is_err() || state.borrow().has_errors());
        state.into_inner()
    }

    pub fn test_parser_warn<T>(parser: impl Fn(NomSpan<'_>) -> PResult<T>, start: &str) -> State {
        let state = std::cell::RefCell::new(crate::State::default());
        let s = NomSpan::new_extra(start, &state);
        parser(s).unwrap();
        // TODO: split
        assert!(!state.borrow().has_errors() && state.borrow().has_warnings());
        state.into_inner()
    }

    #[test]
    fn test_t_bpm() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            *test_parser_ok(t_bpm, "(123456)", "").unwrap(),
            RawInsn::Bpm(BpmParams { new_bpm: 123456.0 })
        );
        assert_eq!(
            *test_parser_ok(t_bpm, "( 123.4 )", " { 4}1, ").unwrap(),
            RawInsn::Bpm(BpmParams { new_bpm: 123.4 })
        );

        test_parser_err(t_bpm, "(123 456)");
        test_parser_err(t_bpm, "()");

        Ok(())
    }

    #[test]
    fn test_t_rest() -> Result<(), Box<dyn Error>> {
        assert_eq!(*test_parser_ok(t_rest, ",", "").unwrap(), RawInsn::Rest);
        assert_eq!(
            *test_parser_ok(t_rest, ",", " (123) {1}1,").unwrap(),
            RawInsn::Rest
        );

        test_parser_err(t_rest, " ,");
        test_parser_err(t_rest, "(123) ,,,");

        Ok(())
    }
}
