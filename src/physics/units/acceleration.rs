// src/physics/units/acceleration.rs
use crate::physics::constants::{
    AU_TO_M, KM_TO_M, M_TO_AU, M_TO_KM, SECONDS_PER_MINUTE, SECONDS_PER_YEAR, YEARS_PER_SECONDS,
};
use crate::physics::units::{UnitConversion, UnitSystem};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acceleration {
    pub value: f64,
    pub unit: String,
    pub system: UnitSystem,
}

impl Acceleration {
    // Basiseinheiten-Labels spezifisch für Beschleunigung
    const SI_BASE_UNIT_LABEL: &'static str = "m/s²";
    const ASTRO_BASE_UNIT_LABEL: &'static str = "AU/yr²";

    // --- SI-basierte Konstruktoren (intern wird alles in m/s² gespeichert) ---

    /// Erstellt eine Beschleunigung in Meter pro Sekunde-Quadrat (SI Basiseinheit).
    pub fn from_meters_per_second_squared(value_ms2: f64) -> Self {
        Acceleration {
            value: value_ms2,
            unit: Self::SI_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::SI,
        }
    }

    /// Erstellt eine Beschleunigung in Kilometer pro Sekunde-Quadrat.
    pub fn from_km_per_second_squared(value_kms2: f64) -> Self {
        Acceleration {
            value: value_kms2 * KM_TO_M,
            unit: Self::SI_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::SI,
        }
    }

    /// Erstellt eine Beschleunigung in Meter pro Minute-Quadrat.
    pub fn from_meters_per_minute_squared(value_mmin2: f64) -> Self {
        let seconds_in_minute_sq = SECONDS_PER_MINUTE * SECONDS_PER_MINUTE;
        Acceleration {
            value: value_mmin2 / seconds_in_minute_sq, // m/min² -> m/s²
            unit: Self::SI_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::SI,
        }
    }

    // --- Astronomie-basierte Konstruktoren (intern wird alles in AU/yr² gespeichert) ---

    /// Erstellt eine Beschleunigung in AU pro Jahr-Quadrat (Astronomische Basiseinheit).
    pub fn from_au_per_year_squared(value_au_yr2: f64) -> Self {
        Acceleration {
            value: value_au_yr2,
            unit: Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::Astronomical,
        }
    }

    /// Konstruktion aus Wert und System
    pub fn new(value: f64, system: UnitSystem) -> Self {
        match system {
            UnitSystem::SI => Self::from_meters_per_second_squared(value),
            UnitSystem::Astronomical => Self::from_au_per_year_squared(value),
        }
    }

    // Kurzschreibweisen
    pub fn meters_per_second_squared(value: f64) -> Self {
        Self::from_meters_per_second_squared(value)
    }

    pub fn km_per_second_squared(value: f64) -> Self {
        Self::from_km_per_second_squared(value)
    }

    pub fn meters_per_minute_squared(value: f64) -> Self {
        Self::from_meters_per_minute_squared(value)
    }

    pub fn au_per_year_squared(value: f64) -> Self {
        Self::from_au_per_year_squared(value)
    }

    pub fn in_ms2(&self) -> f64 {
        self.as_meters_per_second_squared()
    }

    pub fn in_kms2(&self) -> f64 {
        self.as_km_per_second_squared()
    }

    pub fn in_au_yr2(&self) -> f64 {
        self.as_au_per_year_squared()
    }

    /// Konvertiert die Beschleunigung in ein anderes Einheitensystem.
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if self.system == target {
            self.clone()
        } else {
            self.to_system_base(target)
        }
    }

    // --- Konvertierungsmethoden (`as_...` geben den reinen f64 Wert zurück) ---

    /// Gibt den Wert der Beschleunigung in Meter pro Sekunde-Quadrat zurück.
    pub fn as_meters_per_second_squared(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value,
            UnitSystem::Astronomical => {
                // Interner Wert ist AU/yr², also AU/yr² -> m/s²
                // (AU / yr^2) * (Meters / AU) * (Years / Second)^2
                // (AU / yr^2) * (Meters / AU) * (1 / (Seconds / Year))^2
                self.value * AU_TO_M * (YEARS_PER_SECONDS * YEARS_PER_SECONDS)
            }
        }
    }

    /// Gibt den Wert der Beschleunigung in Kilometer pro Sekunde-Quadrat zurück.
    pub fn as_km_per_second_squared(&self) -> f64 {
        self.as_meters_per_second_squared() * M_TO_KM
    }

    /// Gibt den Wert der Beschleunigung in AU pro Jahr-Quadrat zurück.
    pub fn as_au_per_year_squared(&self) -> f64 {
        match self.system {
            UnitSystem::SI => {
                // Interner Wert ist m/s², also m/s² -> AU/yr²
                // (m / s^2) * (AU / m) * (Seconds / Year)^2
                self.value * M_TO_AU * (SECONDS_PER_YEAR * SECONDS_PER_YEAR)
            }
            UnitSystem::Astronomical => self.value, // Interner Wert ist bereits in AU/yr²
        }
    }

    /// Gibt das ursprüngliche Einheitenlabel zurück.
    pub fn unit_label(&self) -> String {
        self.unit.clone()
    }

    /// Gibt das Einheitensystem zurück.
    pub fn unit_system(&self) -> UnitSystem {
        self.system
    }

    /// Gibt den intern gespeicherten Wert zurück.
    /// Dieser Wert ist immer in der Basiseinheit des aktuellen `system`.
    pub fn value_in_system_base(&self) -> f64 {
        self.value
    }
}

impl UnitConversion for Acceleration {
    /// Konvertiert die Beschleunigung in ihre SI-Basisrepräsentation (m/s²).
    fn to_si_base(&self) -> Self {
        let ms2_value = self.as_meters_per_second_squared();
        Acceleration {
            value: ms2_value,
            unit: Self::SI_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::SI,
        }
    }

    /// Konvertiert die Beschleunigung in ihre Astronomische Basisrepräsentation (AU/yr²).
    fn to_astro_base(&self) -> Self {
        let au_yr2_value = self.as_au_per_year_squared();
        Acceleration {
            value: au_yr2_value,
            unit: Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::Astronomical,
        }
    }
}
