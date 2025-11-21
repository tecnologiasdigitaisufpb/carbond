use std::{num::ParseFloatError, str::FromStr};
use uom::si::{f64::Mass, mass::picogram};

use crate::{constants, metrics::round};

use super::metric::Metric;

/// Used to store a CarbonIntensity in the appropriate unit gram per kWh on the filesystem.
#[derive(PartialEq, Debug)]
pub struct CpuCycleIntensity {
    mass: Mass,
}

impl ToString for CpuCycleIntensity {
    fn to_string(&self) -> String {
        let rounded_intensity = round(self.get_value().get::<picogram>());
        format!("{} pg/cycle", rounded_intensity)
    }
}

impl FromStr for CpuCycleIntensity {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let intensity = s.split_at(s.len() - 9).0;
        let mass: Mass = Mass::new::<picogram>(intensity.parse()?);
        Ok(CpuCycleIntensity { mass })
    }
}

impl Metric for CpuCycleIntensity {
    const PATH: &'static str = constants::CPU_PATH;
    const NAME: &'static str = "cpu cycle emission";

    type Unit = Mass;

    fn neutral() -> Self {
        CpuCycleIntensity {
            mass: Mass::new::<picogram>(0.0),
        }
    }

    fn from_value(value: Self::Unit) -> Self {
        CpuCycleIntensity { mass: value }
    }

    fn get_value(&self) -> Self::Unit {
        self.mass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let string_representation: String =
            CpuCycleIntensity::from_value(Mass::new::<picogram>(300.5)).to_string();

        assert_eq!("300.5 pg/cycle", string_representation);
    }

    #[test]
    fn test_from_string() {
        let string = CpuCycleIntensity::from_str("300.54 pg/cycle");

        assert_eq!(
            string.unwrap(),
            CpuCycleIntensity::from_value(Mass::new::<picogram>(300.54))
        );
    }
}
