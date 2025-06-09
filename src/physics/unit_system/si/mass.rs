use crate::physics::constants::*;
use crate::physics::unit_system::{Quantity, Unit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MassUnit {
    Kilogram,
    Gram,
    Tonne,
    SolarMass,
    EarthMass,
}

impl Unit for MassUnit {
    fn to_base(self, value: f64) -> f64 {
        match self {
            MassUnit::Kilogram => value,
            MassUnit::Gram => value * G_TO_KG,
            MassUnit::Tonne => value * 1000.0,
            MassUnit::SolarMass => value * SOLAR_MASS_TO_KG,
            MassUnit::EarthMass => value * EARTH_MASS_TO_KG,
        }
    }

    fn from_base(self, value: f64) -> f64 {
        match self {
            MassUnit::Kilogram => value,
            MassUnit::Gram => value * KG_TO_G,
            MassUnit::Tonne => value / 1000.0,
            MassUnit::SolarMass => value * KG_TO_SOLAR_MASS,
            MassUnit::EarthMass => value * KG_TO_EARTH_MASS,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            MassUnit::Kilogram => "kg",
            MassUnit::Gram => "g",
            MassUnit::Tonne => "t",
            MassUnit::SolarMass => "M☉",
            MassUnit::EarthMass => "M⊕",
        }
    }
}

pub type Mass = Quantity<MassUnit>;
