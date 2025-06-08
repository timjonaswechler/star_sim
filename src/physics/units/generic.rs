// src/physics/units/generic.rs
use serde::{Deserialize, Serialize}; // Stelle sicher, dass Deserializer/Serializer hier sind

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum UnitSystem {
    SI,
    Astronomical,
}

/// Trait für Einheitenkonvertierung
pub trait UnitConversion: Sized + Clone {
    fn to_si_base(&self) -> Self;
    fn to_astro_base(&self) -> Self;

    fn to_system_base(&self, target_system: UnitSystem) -> Self {
        match target_system {
            UnitSystem::SI => self.to_si_base(),
            UnitSystem::Astronomical => self.to_astro_base(),
        }
    }
}
