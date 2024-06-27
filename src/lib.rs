pub mod container;
pub mod insn;
pub mod judge;
pub mod materialize;
mod span;
pub mod transform;
#[macro_use]
extern crate enum_map;

pub use span::*;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub enum Difficulty {
    /// The EASY difficulty.
    Easy = 1,
    /// The BASIC difficulty.
    Basic = 2,
    /// The ADVANCED difficulty.
    Advanced = 3,
    /// The EXPERT difficulty.
    Expert = 4,
    /// The MASTER difficulty.
    Master = 5,
    /// The Re:MASTER difficulty.
    ReMaster = 6,
    /// The ORIGINAL difficulty, previously called mai:EDIT in 2simai.
    Original = 7,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Level {
    /// The "Lv.X" form.
    Normal(u8),
    /// The "Lv.X+" form.
    Plus(u8),
    /// The special "Lv.<any char>" form.
    Char(char),
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Level::*;
        match self {
            Normal(lv) => write!(f, "{}", lv)?,
            Plus(lv) => write!(f, "{}+", lv)?,
            Char(lv) => write!(f, "{}", lv)?,
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Message {
    pub span: Span,
    // TODO: enum for error/warning?
    pub message: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.span, self.message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub warnings: Vec<Message>,
    pub errors: Vec<Message>,
}

impl State {
    pub fn add_warning(&mut self, span: Span, message: String) {
        self.warnings.push(Message { span, message });
    }

    pub fn add_error(&mut self, span: Span, message: String) {
        self.errors.push(Message { span, message });
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
