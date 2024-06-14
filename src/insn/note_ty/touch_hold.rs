use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TouchHoldModifier {
    pub is_firework: bool,
}

impl std::ops::Add for TouchHoldModifier {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_firework && rhs.is_firework {
            return Err("Duplicate firework modifier".to_string());
        }
        Ok(Self {
            is_firework: self.is_firework || rhs.is_firework,
        })
    }
}

impl std::fmt::Display for TouchHoldModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_firework {
            write!(f, "f")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TouchHoldParams {
    pub sensor: TouchSensor,
    pub dur: Duration,
    pub modifier: TouchHoldModifier,
}

impl std::fmt::Display for TouchHoldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}h[{}]", self.sensor, self.modifier, self.dur)
    }
}
