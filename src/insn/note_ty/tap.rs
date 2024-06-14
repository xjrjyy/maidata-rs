use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TapModifier {
    pub is_break: bool,
    pub is_ex: bool,
}

impl std::ops::Add for TapModifier {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            is_break: self.is_break || rhs.is_break,
            is_ex: self.is_ex || rhs.is_ex,
        }
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
