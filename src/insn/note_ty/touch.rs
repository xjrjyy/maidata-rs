use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TouchModifier {
    pub is_firework: bool,
}

impl std::ops::Add for TouchModifier {
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
