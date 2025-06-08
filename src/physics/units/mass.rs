// src/physics/units/mass.rs
use super::generic::{GenericUnitValue, UnitConversion, UnitSystem};
use crate::physics::constants::{
    EARTH_MASS_IN_KG, EARTH_MASS_TO_KG, G_TO_KG, KG_TO_EARTH_MASS, KG_TO_G,
    KG_TO_SOLAR_MASS, SOLAR_MASS_TO_KG,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mass(GenericUnitValue<f64>); // Newtype Wrapper

impl Mass {
    // Basiseinheiten-Labels spezifisch für die Masse
    const SI_BASE_UNIT_LABEL: &'static str = "kg";
    const ASTRO_BASE_UNIT_LABEL: &'static str = "M☉"; // M_solar oder M_sun

    /// Erstellt eine Masse in Kilogramm (SI Basiseinheit für Masse).
    pub fn from_kilograms(value_in_kg: f64) -> Self {
        Mass(GenericUnitValue::new(
            value_in_kg, // Wert ist bereits in SI-Basiseinheit (kg)
            Self::SI_BASE_UNIT_LABEL.to_string(),
            UnitSystem::SI,
        ))
    }

    /// Erstellt eine Masse in Gramm.
    pub fn from_grams(value_in_g: f64) -> Self {
        Mass(GenericUnitValue::new(
            value_in_g * G_TO_KG, // Wert in SI-Basiseinheit (kg) umrechnen
            "g".to_string(),      // Ursprungslabel
            UnitSystem::SI,
        ))
    }

    /// Erstellt eine Masse in Tonnen (metrisch).
    pub fn from_tonnes(value_in_t: f64) -> Self {
        Mass(GenericUnitValue::new(
            value_in_t * 1000.0, // 1 Tonne = 1000 kg. Wert in SI-Basiseinheit (kg) umrechnen
            "t".to_string(),     // Ursprungslabel
            UnitSystem::SI,
        ))
    }

    /// Erstellt eine Masse in Sonnenmassen (Astronomische Basiseinheit für Masse).
    pub fn from_solar_masses(value_in_solar_masses: f64) -> Self {
        Mass(GenericUnitValue::new(
            value_in_solar_masses, // Wert ist bereits in Astro-Basiseinheit (M☉)
            Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            UnitSystem::Astronomical,
        ))
    }

    /// Erstellt eine Masse in Erdmassen.
    pub fn from_earth_masses(value_in_earth_masses: f64) -> Self {
        Mass(GenericUnitValue::new(
            value_in_earth_masses * EARTH_MASS_IN_KG,
            "M⊕".to_string(),
            UnitSystem::SI,
        ))
    }

    /// Generische Konstruktion aus Wert und Zielsystem
    pub fn new(value: f64, system: UnitSystem) -> Self {
        match system {
            UnitSystem::SI => Self::from_kilograms(value),
            UnitSystem::Astronomical => Self::from_solar_masses(value),
        }
    }

    // Kurzschreibweisen
    pub fn kilograms(value: f64) -> Self {
        Self::from_kilograms(value)
    }

    pub fn grams(value: f64) -> Self {
        Self::from_grams(value)
    }

    pub fn tonnes(value: f64) -> Self {
        Self::from_tonnes(value)
    }

    pub fn solar_masses(value: f64) -> Self {
        Self::from_solar_masses(value)
    }

    pub fn earth_masses(value: f64) -> Self {
        Self::from_earth_masses(value)
    }

    pub fn in_kg(&self) -> f64 {
        self.as_kilograms()
    }

    pub fn in_g(&self) -> f64 {
        self.as_grams()
    }

    pub fn in_tonnes(&self) -> f64 {
        self.as_tonnes()
    }

    pub fn in_solar_masses(&self) -> f64 {
        self.as_solar_masses()
    }


    pub fn in_earth_masses(&self) -> f64 {
        self.as_earth_masses()
    }

    /// Konvertiert die Masse in ein anderes Einheitensystem. Bei identischem
    /// Zielsystem wird eine Kopie zurückgegeben, ansonsten erfolgt die
    /// Umrechnung über [`UnitConversion::to_system_base`].
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if self.0.system == target {
            self.clone()
        } else {
            self.to_system_base(target)
        }
    }

    // --- Konvertierungsmethoden ---

    /// Gibt den Wert der Masse in Kilogramm zurück.
    pub fn as_kilograms(&self) -> f64 {
        let guv = &self.0;
        match guv.system {
            UnitSystem::SI => guv.value, // Interner Wert ist bereits in Kilogramm
            UnitSystem::Astronomical => guv.value * SOLAR_MASS_TO_KG, // Interner Wert ist M☉, also M☉ -> kg
        }
    }

    /// Gibt den Wert der Masse in Gramm zurück.
    pub fn as_grams(&self) -> f64 {
        self.as_kilograms() * KG_TO_G // Basiswert (kg) zu g
    }

    /// Gibt den Wert der Masse in Tonnen (metrisch) zurück.
    pub fn as_tonnes(&self) -> f64 {
        self.as_kilograms() / 1000.0 // Basiswert (kg) zu t
    }

    /// Gibt den Wert der Masse in Sonnenmassen zurück.
    pub fn as_solar_masses(&self) -> f64 {
        let guv = &self.0;
        match guv.system {
            UnitSystem::SI => guv.value * KG_TO_SOLAR_MASS, // Interner Wert ist kg, also kg -> M☉
            UnitSystem::Astronomical => guv.value,          // Interner Wert ist bereits in M☉
        }
    }

    /// Gibt den Wert der Masse in Erdmassen zurück.
    pub fn as_earth_masses(&self) -> f64 {
        let guv = &self.0;
        match guv.system {
            UnitSystem::SI => guv.value * KG_TO_EARTH_MASS,
            UnitSystem::Astronomical => guv.value * SOLAR_MASS_TO_KG * KG_TO_EARTH_MASS,
        }
    }

    /// Gibt das ursprüngliche Einheitenlabel zurück (z.B. "g", "kg", "M☉").
    pub fn unit_label(&self) -> String {
        self.0.unit.to_string()
    }

    /// Gibt das Einheitensystem zurück.
    pub fn unit_system(&self) -> UnitSystem {
        self.0.system
    }

    /// Gibt den intern gespeicherten Wert zurück.
    /// Dieser Wert ist immer in der Basiseinheit des aktuellen `system`
    /// (z.B. kg für SI-Massen, M☉ für Astronomische Massen).
    pub fn value_in_system_base(&self) -> f64 {
        self.0.value
    }
}

impl UnitConversion for Mass {
    /// Konvertiert die Masse in ihre SI-Basisrepräsentation (Kilogramm).
    /// Das zurückgegebene `Mass`-Objekt hat "kg" als `unit_label`.
    fn to_si_base(&self) -> Self {
        let kg_value = self.as_kilograms();
        Mass(GenericUnitValue::new(
            kg_value,
            Self::SI_BASE_UNIT_LABEL.to_string(),
            UnitSystem::SI,
        ))
    }

    /// Konvertiert die Masse in ihre Astronomische Basisrepräsentation (Sonnenmassen).
    /// Das zurückgegebene `Mass`-Objekt hat "M☉" als `unit_label`.
    fn to_astro_base(&self) -> Self {
        let solar_mass_value = self.as_solar_masses();
        Mass(GenericUnitValue::new(
            solar_mass_value,
            Self::ASTRO_BASE_UNIT_LABEL.to_string(),
            UnitSystem::Astronomical,
        ))
    }
}
