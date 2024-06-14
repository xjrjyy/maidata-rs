use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HoldParams {
    pub is_break: bool,
    pub is_ex: bool,
    pub key: Key,
    pub dur: Duration,
}

impl std::fmt::Display for HoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)?;
        if self.is_break {
            write!(f, "b")?;
        }
        if self.is_ex {
            write!(f, "x")?;
        }
        write!(f, "h[{}]", self.dur)?;
        Ok(())
    }
}
