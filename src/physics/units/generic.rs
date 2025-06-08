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

/// Einfacher generischer Wrapper für einen Wert mit Einheit und System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericUnitValue<T> {
    pub value: T,
    pub unit: String,
    pub system: UnitSystem,
}

impl<T> GenericUnitValue<T> {
    pub fn new(value: T, unit: String, system: UnitSystem) -> Self {
        Self { value, unit, system }
    }
}
