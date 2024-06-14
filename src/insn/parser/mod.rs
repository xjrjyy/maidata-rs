mod note;
mod position;

use super::*;
use crate::{NomSpan, PError, PResult, WithSpan};
use nom::character::complete::multispace0;
use note::{t_bundle, t_single_note, t_tap_multi_simplified};
use position::*;

/// remove leading whitespace
fn ws<'a, F, O>(f: F) -> impl FnMut(NomSpan<'a>) -> PResult<'a, O>
where
    F: 'a + FnMut(NomSpan<'a>) -> PResult<'a, O>,
{
    nom::sequence::preceded(multispace0, f)
}

fn _ws_list0<'a, F, O>(mut f: F) -> impl FnMut(NomSpan<'a>) -> PResult<'a, Vec<O>>
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
    use nom::multi::many0;

    let (s, insns) = many0(ws(parse_one_maidata_insn))(s)?;
    let (s, _) = ws(t_eof)(s)?;

    Ok((s, insns))
}

fn t_eof(s: NomSpan) -> PResult<NomSpan> {
    use nom::combinator::eof;
    eof(s)
}

fn parse_one_maidata_insn(s: NomSpan) -> PResult<SpRawInsn> {
    nom::branch::alt((
        t_bpm,
        t_beat_divisor,
        t_rest,
        t_single_note,
        t_tap_multi_simplified,
        t_bundle,
        t_end_mark,
    ))(s)
}

fn t_end_mark(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::character::complete::char;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('E')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::EndMark.with_span(span)))
}

fn t_note_sep(s: NomSpan) -> PResult<()> {
    use nom::character::complete::char;

    let (s, _) = char(',')(s)?;
    Ok((s, ()))
}

fn t_bpm(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('(')(s)?;
    let (s, bpm) = ws(float)(s)?;
    let (s, _) = ws(char(')'))(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);

    Ok((s, RawInsn::Bpm(BpmParams { new_bpm: bpm }).with_span(span)))
}

fn t_absolute_duration(s: NomSpan) -> PResult<f32> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = char('#')(s)?;
    let (s, dur) = ws(float)(s)?;

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

fn t_beat_divisor(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::character::complete::char;

    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('{')(s)?;
    let (s, params) = ws(t_beat_divisor_param)(s)?;
    let (s, _) = ws(char('}'))(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::BeatDivisor(params).with_span(span)))
}

fn t_rest(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::Rest.with_span(span)))
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
    fn test_t_bpm() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            *test_parser_ok!(t_bpm, "(123456)", ""),
            RawInsn::Bpm(BpmParams { new_bpm: 123456.0 })
        );
        assert_eq!(
            *test_parser_ok!(t_bpm, "( 123.4 )", " { 4}1, "),
            RawInsn::Bpm(BpmParams { new_bpm: 123.4 })
        );

        test_parser_err!(t_bpm, "(123 456)");
        test_parser_err!(t_bpm, "()");

        Ok(())
    }

    #[test]
    fn test_t_rest() -> Result<(), Box<dyn Error>> {
        assert_eq!(*test_parser_ok!(t_rest, ",", ""), RawInsn::Rest);
        assert_eq!(*test_parser_ok!(t_rest, ",", " (123) {1}1,"), RawInsn::Rest);

        test_parser_err!(t_rest, " ,");
        test_parser_err!(t_rest, "(123) ,,,");

        Ok(())
    }
}
