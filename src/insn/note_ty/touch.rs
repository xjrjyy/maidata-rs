use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TouchParams {
    pub is_firework: bool,
    pub sensor: TouchSensor,
}

impl std::fmt::Display for TouchParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sensor)?;
        if self.is_firework {
            write!(f, "f")?;
        }
        Ok(())
    }
}
