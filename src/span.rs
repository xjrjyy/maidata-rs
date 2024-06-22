use serde::{Deserialize, Serialize};
use std::cell::RefCell;

pub type NomSpan<'a> = nom_locate::LocatedSpan<&'a str, &'a RefCell<ParseState>>;

/// Convenient alias for parsing result with spans.
pub type PResult<'a, T> = nom::IResult<NomSpan<'a>, T>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParseMessage {
    pub span: Span,
    // TODO: enum for error/warning?
    pub message: String,
}

impl std::fmt::Display for ParseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.span, self.message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParseState {
    pub warnings: Vec<ParseMessage>,
    pub errors: Vec<ParseMessage>,
}

impl ParseState {
    pub fn add_warning(&mut self, span: Span, message: String) {
        self.warnings.push(ParseMessage { span, message });
    }

    pub fn add_error(&mut self, span: Span, message: String) {
        self.errors.push(ParseMessage { span, message });
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_messages(&self) -> bool {
        self.has_warnings() || self.has_errors()
    }
}

pub fn expect<'a, F, E, T>(
    mut parser: F,
    error_msg: E,
) -> impl FnMut(NomSpan<'a>) -> PResult<'a, Option<T>>
where
    F: FnMut(NomSpan<'a>) -> PResult<'a, T>,
    E: ToString,
{
    move |input| {
        let (input, start_loc) = nom_locate::position(input)?;
        match parser(input) {
            Ok((remaining, out)) => Ok((remaining, Some(out))),
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                let (_, end_loc) = nom_locate::position(e.input)?;
                let span = (start_loc, end_loc).into();
                e.input
                    .extra
                    .borrow_mut()
                    .add_error(span, error_msg.to_string());
                Ok((input, None))
            }
            Err(err) => Err(err),
        }
    }
}

pub trait Expect<'a, T> {
    fn expect<E>(self, error_msg: E) -> impl FnMut(NomSpan<'a>) -> PResult<'a, Option<T>>
    where
        E: ToString;
}

impl<'a, T, U: 'a + FnMut(NomSpan<'a>) -> PResult<T>> Expect<'a, T> for U {
    fn expect<E>(self, error_msg: E) -> impl FnMut(NomSpan<'a>) -> PResult<'a, Option<T>>
    where
        E: ToString,
    {
        expect(self, error_msg)
    }
}

// TODO: refactor
pub fn expect_ws_delimited<'a, F, T>(
    mut inner: F,
    inner_name: &'a str,
    start: &'a str,
    end: &'a str,
) -> impl FnMut(NomSpan<'a>) -> PResult<'a, Option<T>>
where
    F: FnMut(NomSpan<'a>) -> PResult<'a, T>,
{
    use nom::bytes::complete::tag;
    use nom::character::complete::multispace0;
    use nom::combinator::opt;
    move |i| {
        let (i1, open) = opt(tag(start))(i)?;
        let (i2, _) = multispace0(i1)?;
        let (i2, result) = match inner(i2) {
            Ok((i, result)) => (i, Some(result)),
            Err(nom::Err::Error(_)) | Err(nom::Err::Failure(_)) => (i2, None),
            Err(err) => return Err(err),
        };
        let (i3, _) = multispace0(i2)?;
        let (i3, close) = opt(tag(end))(i3)?;

        // `x` / `(`
        if (open.is_none() || result.is_none()) && close.is_none() {
            return Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Tag,
            )));
        }
        if open.is_none() {
            let (_, end_loc) = nom_locate::position(i)?;
            i3.extra.borrow_mut().add_error(
                (end_loc, end_loc).into(),
                format!("expected `{}` before {}", start, inner_name),
            );
        }
        if result.is_none() {
            let (_, end_loc) = nom_locate::position(i1)?;
            i3.extra.borrow_mut().add_error(
                (end_loc, end_loc).into(),
                format!("expected {} between `{}` and `{}`", inner_name, start, end),
            );
        }
        if close.is_none() {
            let (_, end_loc) = nom_locate::position(i2)?;
            i3.extra.borrow_mut().add_error(
                (end_loc, end_loc).into(),
                format!("expected `{}` after {}", end, inner_name),
            );
            return Ok((i2, None));
        }
        Ok((i3, result))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Span {
    pub byte_offset: usize,
    pub line: usize,
    pub col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub len: usize,
}

impl Span {
    pub fn from_start_end(start: NomSpan<'_>, end: NomSpan<'_>) -> Self {
        use nom::Offset;

        let byte_offset = start.location_offset();
        let line = start.location_line() as usize;
        let col = start.get_utf8_column();
        let end_line = end.location_line() as usize;
        let end_col = end.get_utf8_column();
        let len = start.offset(&end);

        Self {
            byte_offset,
            line,
            col,
            end_line,
            end_col,
            len,
        }
    }
}

impl From<(NomSpan<'_>, NomSpan<'_>)> for Span {
    fn from(x: (NomSpan<'_>, NomSpan<'_>)) -> Self {
        Span::from_start_end(x.0, x.1)
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}-{}:{}]",
            self.line, self.col, self.end_line, self.end_col
        )
    }
}

/// Thing with span information attached.
pub struct Sp<T>(T, crate::Span);

/// Convenient alias for working with lists of Sp-ed things.
pub type VecSp<T> = Vec<Sp<T>>;

impl<T> std::ops::Deref for Sp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Sp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Copy for Sp<T> where T: Copy {}

impl<T> Clone for Sp<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T> PartialEq for Sp<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Sp<T> where T: Eq + PartialEq {}

impl<T> std::fmt::Display for Sp<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = self.1;
        write!(
            f,
            "[{}:{}-{}:{}]{}",
            span.line, span.col, span.end_line, span.end_col, self.0
        )
    }
}

impl<T> std::fmt::Debug for Sp<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = self.1;
        write!(
            f,
            "[{}:{}-{}:{}]{:?}",
            span.line, span.col, span.end_line, span.end_col, self.0
        )
    }
}

impl<T> Sp<T> {
    pub fn new(obj: T, span: crate::Span) -> Self {
        Self(obj, span)
    }

    pub fn span(&self) -> crate::Span {
        self.1
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

pub(crate) trait WithSpan {
    fn with_span<S: Into<crate::Span>>(self, sp: S) -> Sp<Self>
    where
        Self: Sized;
}

impl<T> WithSpan for T {
    fn with_span<S: Into<crate::Span>>(self, sp: S) -> Sp<Self> {
        crate::Sp::new(self, sp.into())
    }
}
