use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct HoldModifier {
    pub is_break: bool,
    pub is_ex: bool,
}

impl std::fmt::Display for HoldModifier {
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HoldParams {
    pub key: Key,
    pub dur: Duration,
    pub modifier: HoldModifier,
}

impl std::fmt::Display for HoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}h[{}]", self.key, self.modifier, self.dur)
    }
}
