// src/physics/units/time.rs
use super::generic::{UnitConversion, UnitSystem};
use crate::physics::constants::{
    DAYS_PER_HOUR,
    GIGAYEARS_PER_MEGAYEAR,
    HOURS_PER_DAY,
    HOURS_PER_MINUTE,
    KILOYEARS_PER_MEGAYEAR,
    KILOYEARS_PER_YEAR,
    MEGAYEARS_PER_GIGAYEAR,
    MEGAYEARS_PER_KILOYEAR,
    MINUTES_PER_HOUR,
    // Inverse Konstanten für die Rückumrechnung (optional, aber nützlich)
    MINUTES_PER_SECOND,
    SECONDS_PER_MINUTE,
    SECONDS_PER_YEAR, // Wichtig für die Konvertierung zwischen Systemen
    YEARS_PER_KILOYEAR,
    YEARS_PER_SECONDS,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub struct Time {
    pub value: f64,
    pub unit: &'static str,
    pub system: UnitSystem,
}

impl Time {
    // Basiseinheiten-Labels spezifisch für die Zeit
    const SI_BASE_UNIT_LABEL: &'static str = "s"; // Sekunden
    const ASTRO_BASE_UNIT_LABEL: &'static str = "yr"; // Jahre (typischerweise Julianische Jahre für Konsistenz)

    // --- SI-basierte Konstruktoren (intern wird alles in Sekunden gespeichert) ---

    /// Erstellt eine Zeit in Sekunden (SI Basiseinheit für Zeit).
    pub fn from_seconds(value_in_s: f64) -> Self {
        Time {
            value: value_in_s,
            unit: Self::SI_BASE_UNIT_LABEL,
            system: UnitSystem::SI,
        }
    }

    /// Erstellt eine Zeit in Minuten.
    pub fn from_minutes(value_in_min: f64) -> Self {
        Time {
            value: value_in_min * SECONDS_PER_MINUTE,
            unit: "min",
            system: UnitSystem::SI,
        }
    }

    /// Erstellt eine Zeit in Stunden.
    pub fn from_hours(value_in_hr: f64) -> Self {
        Time {
            value: value_in_hr * SECONDS_PER_MINUTE * MINUTES_PER_HOUR,
            unit: "hr",
            system: UnitSystem::SI,
        }
    }

    /// Erstellt eine Zeit in Tagen.
    pub fn from_days(value_in_days: f64) -> Self {
        Time {
            value: value_in_days * SECONDS_PER_MINUTE * MINUTES_PER_HOUR * HOURS_PER_DAY,
            unit: "d",
            system: UnitSystem::SI,
        }
    }

    // --- Astronomie-basierte Konstruktoren (intern wird alles in Jahren gespeichert) ---
    // Diese Einheiten werden typischerweise im astronomischen Kontext verwendet, daher `UnitSystem::Astronomical`

    /// Erstellt eine Zeit in Jahren (Astronomische Basiseinheit für Zeit).
    pub fn from_years(value_in_yr: f64) -> Self {
        Time {
            value: value_in_yr,
            unit: Self::ASTRO_BASE_UNIT_LABEL,
            system: UnitSystem::Astronomical,
        }
    }

    /// Erstellt eine Zeit in Kiloyahren (Tausend Jahren).
    pub fn from_kiloyears(value_in_kyr: f64) -> Self {
        Time {
            value: value_in_kyr * YEARS_PER_KILOYEAR,
            unit: "kyr",
            system: UnitSystem::Astronomical,
        }
    }

    /// Erstellt eine Zeit in Megayahren (Millionen Jahren).
    pub fn from_megayears(value_in_myr: f64) -> Self {
        Time {
            value: value_in_myr * YEARS_PER_KILOYEAR * KILOYEARS_PER_MEGAYEAR,
            unit: "Myr",
            system: UnitSystem::Astronomical,
        }
    }

    /// Erstellt eine Zeit in Gigayahren (Milliarden Jahren).
    pub fn from_gigayears(value_in_gyr: f64) -> Self {
        Time {
            value: value_in_gyr
                * YEARS_PER_KILOYEAR
                * KILOYEARS_PER_MEGAYEAR
                * MEGAYEARS_PER_GIGAYEAR,
            unit: "Gyr",
            system: UnitSystem::Astronomical,
        }
    }

    // --- Konvertierungsmethoden (`as_...` geben den reinen f64 Wert zurück) ---

    /// Gibt den Wert der Zeit in Sekunden zurück.
    pub fn as_seconds(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value,
            UnitSystem::Astronomical => self.value * SECONDS_PER_YEAR,
        }
    }

    /// Gibt den Wert der Zeit in Minuten zurück.
    pub fn as_minutes(&self) -> f64 {
        self.as_seconds() * MINUTES_PER_SECOND
    }

    /// Gibt den Wert der Zeit in Stunden zurück.
    pub fn as_hours(&self) -> f64 {
        self.as_minutes() * HOURS_PER_MINUTE
    }

    /// Gibt den Wert der Zeit in Tagen zurück.
    pub fn as_days(&self) -> f64 {
        self.as_hours() * DAYS_PER_HOUR
    }

    /// Gibt den Wert der Zeit in Jahren zurück.
    pub fn as_years(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value * YEARS_PER_SECONDS, // Interner Wert ist Sekunden, also s -> yr
            UnitSystem::Astronomical => self.value,           // Interner Wert ist bereits in Jahren
        }
    }

    /// Gibt den Wert der Zeit in Kiloyahren zurück.
    pub fn as_kiloyears(&self) -> f64 {
        self.as_years() * KILOYEARS_PER_YEAR
    }

    /// Gibt den Wert der Zeit in Megayahren zurück.
    pub fn as_megayears(&self) -> f64 {
        self.as_kiloyears() * MEGAYEARS_PER_KILOYEAR
    }

    /// Gibt den Wert der Zeit in Gigayahren zurück.
    pub fn as_gigayears(&self) -> f64 {
        self.as_megayears() * GIGAYEARS_PER_MEGAYEAR
    }

    /// Gibt das ursprüngliche Einheitenlabel zurück.
    pub fn unit_label(&self) -> &'static str {
        self.unit
    }

    /// Gibt das Einheitensystem zurück.
    pub fn unit_system(&self) -> UnitSystem {
        self.system
    }

    /// Gibt den intern gespeicherten Wert zurück.
    /// Dieser Wert ist immer in der Basiseinheit des aktuellen `system`
    /// (z.B. Sekunden für SI-Zeiten, Jahre für Astronomische Zeiten).
    pub fn value_in_system_base(&self) -> f64 {
        self.value
    }
}

impl UnitConversion for Time {
    /// Konvertiert die Zeit in ihre SI-Basisrepräsentation (Sekunden).
    /// Das zurückgegebene `Time`-Objekt hat "s" als `unit_label`.
    fn to_si_base(&self) -> Self {
        let seconds_value = self.as_seconds();
        Time {
            value: seconds_value,
            unit: Self::SI_BASE_UNIT_LABEL,
            system: UnitSystem::SI,
        }
    }

    /// Konvertiert die Zeit in ihre Astronomische Basisrepräsentation (Jahre).
    /// Das zurückgegebene `Time`-Objekt hat "yr" als `unit_label`.
    fn to_astro_base(&self) -> Self {
        let years_value = self.as_years();
        Time {
            value: years_value,
            unit: Self::ASTRO_BASE_UNIT_LABEL,
            system: UnitSystem::Astronomical,
        }
    }
}
