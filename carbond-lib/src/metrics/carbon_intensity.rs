use std::{num::ParseFloatError, str::FromStr};
use uom::si::{f64::MassPerEnergy, mass_per_energy::gram_per_kilowatt_hour};

use crate::{constants, metrics::round};

use super::metric::Metric;

/// Used to store a CarbonIntensity in the appropriate unit gram per kWh on the filesystem.
#[derive(PartialEq, Debug)]
pub struct CarbonIntensity {
    mass_per_energy: MassPerEnergy,
}

impl ToString for CarbonIntensity {
    fn to_string(&self) -> String {
        let rounded_carbon_intensity = round(self.get_value().get::<gram_per_kilowatt_hour>());
        format!("{} g/kWh", rounded_carbon_intensity)
    }
}

impl FromStr for CarbonIntensity {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let intensity = s.split_at(s.len() - 6).0;
        let mass_per_energy: MassPerEnergy =
            MassPerEnergy::new::<gram_per_kilowatt_hour>(intensity.parse()?);
        Ok(CarbonIntensity { mass_per_energy })
    }
}

impl Metric for CarbonIntensity {
    const PATH: &'static str = constants::INTENSITY_PATH;
    const NAME: &'static str = "carbon intensity";

    type Unit = MassPerEnergy;

    fn neutral() -> Self {
        CarbonIntensity {
            mass_per_energy: MassPerEnergy::new::<gram_per_kilowatt_hour>(0.0),
        }
    }

    fn from_value(value: Self::Unit) -> Self {
        CarbonIntensity {
            mass_per_energy: value,
        }
    }

    fn get_value(&self) -> Self::Unit {
        self.mass_per_energy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let string_representation: String =
            CarbonIntensity::from_value(MassPerEnergy::new::<gram_per_kilowatt_hour>(300.5))
                .to_string();

        assert_eq!("300.5 g/kWh", string_representation);
    }

    #[test]
    fn test_from_string() {
        let string = CarbonIntensity::from_str("300.54 g/kWh");

        assert_eq!(
            string.unwrap(),
            CarbonIntensity::from_value(MassPerEnergy::new::<gram_per_kilowatt_hour>(300.54))
        );
    }
}
