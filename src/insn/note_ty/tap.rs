use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TapShape {
    Ring,
    Star,
    StarSpin,
    Invalid,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TapModifier {
    pub is_break: bool,
    pub is_ex: bool,
    pub shape: Option<TapShape>,
}

impl std::ops::Add for TapModifier {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_break && rhs.is_break {
            return Err("Duplicate break modifier".to_string());
        }
        if self.is_ex && rhs.is_ex {
            return Err("Duplicate ex modifier".to_string());
        }
        if self.shape.is_some() && rhs.shape.is_some() {
            return Err("Duplicate shape modifier".to_string());
        }
        Ok(Self {
            is_break: self.is_break || rhs.is_break,
            is_ex: self.is_ex || rhs.is_ex,
            shape: self.shape.or(rhs.shape),
        })
    }
}

impl std::fmt::Display for TapModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_break {
            write!(f, "b")?;
        }
        if self.is_ex {
            write!(f, "x")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TapParams {
    pub key: Key,
    pub modifier: TapModifier,
}

impl std::fmt::Display for TapParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.key, self.modifier)
    }
}
