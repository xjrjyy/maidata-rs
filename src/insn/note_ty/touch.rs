use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TouchModifier {
    pub is_firework: bool,
}

impl std::ops::Add for TouchModifier {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            is_firework: self.is_firework || rhs.is_firework,
        }
    }
}

impl std::fmt::Display for TouchModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_firework {
            write!(f, "f")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TouchParams {
    pub sensor: TouchSensor,
    pub modifier: TouchModifier,
}

impl std::fmt::Display for TouchParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.sensor, self.modifier)
    }
}
