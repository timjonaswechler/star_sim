// src/physics/units/distance.rs
use super::generic::{UnitConversion, UnitSystem};
use crate::physics::constants::{
    AU_TO_M, EARTH_RADIUS_IN_METERS, KM_TO_M, M_TO_AU, M_TO_KM, METERS_TO_EARTH_RADIUS,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    pub value: f64,
    pub unit: String,
    pub system: UnitSystem,
}

impl Distance {
    // Definieren Strings für Basiseinheiten spezifisch für die Distanz-Größe
    const SI_BASE_UNIT_LABEL: &'static str = "m";
    const ASTRO_BASE_UNIT_LABEL: &'static str = "AU";

    /// Erstellt eine Distanz in Metern (SI Basiseinheit für Distanz).
    pub fn from_meters(value_in_m: f64) -> Self {
        Distance {
            value: value_in_m,
            unit: Self::SI_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::SI,
        }
    }

    /// Erstellt eine Distanz in Kilometern.
    pub fn from_km(value_in_km: f64) -> Self {
        Distance {
            value: value_in_km * KM_TO_M,
            unit: Self::SI_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::SI,
        }
    }

    /// Erstellt eine Distanz in Astronomischen Einheiten (Astronomische Basiseinheit für Distanz).
    pub fn from_au(value_in_au: f64) -> Self {
        Distance {
            value: value_in_au,
            unit: Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::Astronomical,
        }
    }

    /// Erstellt eine Distanz in Erdradien.
    pub fn from_earth_radii(value_in_er: f64) -> Self {
        Distance {
            value: value_in_er * EARTH_RADIUS_IN_METERS,
            unit: "R⊕".to_string(),
            system: UnitSystem::SI,
        }
    }

    /// Generische Konstruktion aus Wert und Zielsystem
    pub fn new(value: f64, system: UnitSystem) -> Self {
        match system {
            UnitSystem::SI => Self::from_meters(value),
            UnitSystem::Astronomical => Self::from_au(value),
        }
    }

    // Bequeme Kurzschreibweisen für die meistgenutzten Einheiten
    pub fn meters(value: f64) -> Self {
        Self::from_meters(value)
    }

    pub fn km(value: f64) -> Self {
        Self::from_km(value)
    }

    pub fn au(value: f64) -> Self {
        Self::from_au(value)
    }

    /// Erstellt eine Distanz in Erdradien.
    pub fn earth_radii(value: f64) -> Self {
        Self::from_earth_radii(value)
    }

    pub fn in_meters(&self) -> f64 {
        self.as_meters()
    }

    pub fn in_km(&self) -> f64 {
        self.as_km()
    }

    pub fn in_au(&self) -> f64 {
        self.as_au()
    }

    pub fn in_earth_radii(&self) -> f64 {
        self.as_earth_radii()
    }

    /// Konvertiert die Distanz in ein anderes Einheitensystem.
    /// Wenn das Zielsystem dem aktuellen System entspricht, wird eine
    /// Kopie des bestehenden Objekts zurückgegeben. Andernfalls erfolgt
    /// die Umrechnung über die im [`UnitConversion`] Trait implementierten
    /// Basisfunktionen.
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if self.system == target {
            self.clone()
        } else {
            self.to_system_base(target)
        }
    }

    /// Gibt den Wert der Distanz in Metern zurück.
    pub fn as_meters(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value,
            UnitSystem::Astronomical => self.value * AU_TO_M,
        }
    }

    /// Gibt den Wert der Distanz in Kilometern zurück.
    pub fn as_km(&self) -> f64 {
        self.as_meters() * M_TO_KM // Basiswert (Meter) zu km
    }

    /// Gibt den Wert der Distanz in Astronomischen Einheiten zurück.
    pub fn as_au(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value * M_TO_AU, // Interner Wert ist Meter, also m -> AU
            UnitSystem::Astronomical => self.value, // Interner Wert ist bereits in AU
        }
    }

    /// Gibt den Wert der Distanz in Erdradien zurück.
    pub fn as_earth_radii(&self) -> f64 {
        self.as_meters() * METERS_TO_EARTH_RADIUS
    }

    /// Gibt das ursprüngliche Einheitenlabel zurück (z.B. "km", "m", "AU").
    pub fn unit_label(&self) -> String {
        self.unit.clone()
    }

    /// Gibt das Einheitensystem zurück.
    pub fn unit_system(&self) -> UnitSystem {
        self.system
    }

    /// Gibt den intern gespeicherten Wert zurück.
    /// Dieser Wert ist immer in der Basiseinheit des aktuellen `system`
    /// (z.B. Meter für SI-Distanzen, AU für Astronomische Distanzen).
    pub fn value_in_system_base(&self) -> f64 {
        self.value
    }
}

impl UnitConversion for Distance {
    /// Konvertiert die Distanz in ihre SI-Basisrepräsentation (Meter).
    /// Das zurückgegebene `Distance`-Objekt hat "m" als `unit_label`.
    fn to_si_base(&self) -> Self {
        let meters_value = self.as_meters();
        Distance {
            value: meters_value,
            unit: Self::SI_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::SI,
        }
    }

    /// Konvertiert die Distanz in ihre Astronomische Basisrepräsentation (AU).
    /// Das zurückgegebene `Distance`-Objekt hat "AU" als `unit_label`.
    fn to_astro_base(&self) -> Self {
        let au_value = self.as_au();
        Distance {
            value: au_value,
            unit: Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            system: UnitSystem::Astronomical,
        }
    }
}
