use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TouchHoldParams {
    pub is_firework: bool,
    pub sensor: TouchSensor,
    pub dur: Duration,
}

impl std::fmt::Display for TouchHoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sensor)?;
        if self.is_firework {
            write!(f, "f")?;
        }
        write!(f, "h[{}]", self.dur)?;
        Ok(())
    }
}
