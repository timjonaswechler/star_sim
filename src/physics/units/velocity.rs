// src/physics/units/velocity.rs
use super::generic::{GenericUnitValue, UnitConversion, UnitSystem};
use crate::physics::constants::{
    AU_TO_M,
    HOURS_PER_SECONDS, // Für km/h Umrechnung
    KM_TO_M,
    M_TO_AU,
    M_TO_KM,
    SECONDS_PER_HOUR,
    SECONDS_PER_YEAR,
    YEARS_PER_SECONDS, // Für AU/yr Umrechnung
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity(GenericUnitValue<f64>); // Newtype Wrapper

impl Velocity {
    // Basiseinheiten-Labels spezifisch für Geschwindigkeit
    const SI_BASE_UNIT_LABEL: &'static str = "m/s";
    const ASTRO_BASE_UNIT_LABEL: &'static str = "AU/yr"; // AU pro Julianisches Jahr

    // --- SI-basierte Konstruktoren (intern wird alles in m/s gespeichert) ---

    /// Erstellt eine Geschwindigkeit in Meter pro Sekunde (SI Basiseinheit).
    pub fn from_meters_per_second(value_ms: f64) -> Self {
        Velocity(GenericUnitValue::new(
            value_ms, // Wert ist bereits in SI-Basiseinheit (m/s)
            Self::SI_BASE_UNIT_LABEL.to_string(),
            UnitSystem::SI,
        ))
    }

    /// Erstellt eine Geschwindigkeit in Kilometer pro Sekunde.
    pub fn from_km_per_second(value_kms: f64) -> Self {
        Velocity(GenericUnitValue::new(
            value_kms * KM_TO_M, // Wert in SI-Basiseinheit (m/s) umrechnen
            "km/s".to_string(),  // Ursprungslabel
            UnitSystem::SI,
        ))
    }

    /// Erstellt eine Geschwindigkeit in Kilometer pro Stunde.
    pub fn from_km_per_hour(value_kmh: f64) -> Self {
        Velocity(GenericUnitValue::new(
            value_kmh * KM_TO_M * HOURS_PER_SECONDS, // km/h -> m/s
            "km/h".to_string(),
            UnitSystem::SI,
        ))
    }

    // --- Astronomie-basierte Konstruktoren (intern wird alles in AU/yr gespeichert) ---

    /// Erstellt eine Geschwindigkeit in AU pro Jahr (Astronomische Basiseinheit).
    pub fn from_au_per_year(value_au_yr: f64) -> Self {
        Velocity(GenericUnitValue::new(
            value_au_yr, // Wert ist bereits in Astro-Basiseinheit (AU/yr)
            Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            UnitSystem::Astronomical,
        ))
    }

    // --- Konvertierungsmethoden (`as_...` geben den reinen f64 Wert zurück) ---

    /// Gibt den Wert der Geschwindigkeit in Meter pro Sekunde zurück.
    pub fn as_meters_per_second(&self) -> f64 {
        let guv = &self.0;
        match guv.system {
            UnitSystem::SI => guv.value, // Interner Wert ist bereits in m/s
            UnitSystem::Astronomical => guv.value * AU_TO_M * YEARS_PER_SECONDS, // Interner Wert ist AU/yr, also AU/yr -> m/s
        }
    }

    /// Gibt den Wert der Geschwindigkeit in Kilometer pro Sekunde zurück.
    pub fn as_km_per_second(&self) -> f64 {
        self.as_meters_per_second() * M_TO_KM
    }

    /// Gibt den Wert der Geschwindigkeit in Kilometer pro Stunde zurück.
    pub fn as_km_per_hour(&self) -> f64 {
        self.as_meters_per_second() * SECONDS_PER_HOUR * M_TO_KM // m/s -> km/h
    }

    /// Gibt den Wert der Geschwindigkeit in AU pro Jahr zurück.
    pub fn as_au_per_year(&self) -> f64 {
        let guv = &self.0;
        match guv.system {
            UnitSystem::SI => guv.value * M_TO_AU * SECONDS_PER_YEAR, // Interner Wert ist m/s, also m/s -> AU/yr
            UnitSystem::Astronomical => guv.value, // Interner Wert ist bereits in AU/yr
        }
    }

    /// Gibt das ursprüngliche Einheitenlabel zurück.
    pub fn unit_label(&self) -> String {
        self.0.unit.to_string()
    }

    /// Gibt das Einheitensystem zurück.
    pub fn unit_system(&self) -> UnitSystem {
        self.0.system
    }

    /// Gibt den intern gespeicherten Wert zurück.
    /// Dieser Wert ist immer in der Basiseinheit des aktuellen `system`.
    pub fn value_in_system_base(&self) -> f64 {
        self.0.value
    }
}

impl UnitConversion for Velocity {
    /// Konvertiert die Geschwindigkeit in ihre SI-Basisrepräsentation (m/s).
    fn to_si_base(&self) -> Self {
        let ms_value = self.as_meters_per_second();
        Velocity(GenericUnitValue::new(
            ms_value,
            Self::SI_BASE_UNIT_LABEL.to_string(),
            UnitSystem::SI,
        ))
    }

    /// Konvertiert die Geschwindigkeit in ihre Astronomische Basisrepräsentation (AU/yr).
    fn to_astro_base(&self) -> Self {
        let au_yr_value = self.as_au_per_year();
        Velocity(GenericUnitValue::new(
            au_yr_value,
            Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            UnitSystem::Astronomical,
        ))
    }
}
