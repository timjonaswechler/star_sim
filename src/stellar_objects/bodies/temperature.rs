use serde::{Deserialize, Serialize};
use crate::physics::units::Distance;

/// Temperaturanalyse für einen Planeten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureAnalysis {
    /// Gleichgewichtstemperatur (ohne Atmosphäre, K)
    pub equilibrium_temperature: f64,
    /// Temperaturbereich mit dünner Atmosphäre (K)
    pub thin_atmosphere_range: (f64, f64),
    /// Temperaturbereich mit dichter Atmosphäre (K)
    pub thick_atmosphere_range: (f64, f64),
    /// Greenhouse-Effekt Potenzial
    pub greenhouse_potential: f64,
}

/// Temperaturmodell für Planeten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureModel {
    /// Gleichgewichtstemperatur (ohne Atmosphäre)
    pub equilibrium_temperature: f64, // K
    /// Oberflächentemperatur (mit Atmosphäre)
    pub surface_temperature: f64, // K
    /// Albedo (Bond-Albedo)
    pub bond_albedo: f64,
    /// Geometrische Albedo
    pub geometric_albedo: f64,
    /// Treibhaus-Verstärkung
    pub greenhouse_factor: f64,
    /// Tag-/Nachtseite für gebundene Rotation
    pub day_night_temperatures: Option<(f64, f64)>, // (Tagseite K, Nachtseite K)
}

impl TemperatureModel {
    /// Berechnet Gleichgewichtstemperatur aus Sternparametern
    pub fn from_stellar_parameters(
        stellar_luminosity: f64, // Relative zur Sonne
        orbital_distance: &Distance,
        bond_albedo: f64,
    ) -> Self {
        let distance_au = orbital_distance.in_au();

        // Stefan-Boltzmann Gleichgewichtstemperatur
        let equilibrium_temperature =
            278.0 * (stellar_luminosity * (1.0 - bond_albedo) / distance_au.powi(2)).powf(0.25);

        // Geometrische Albedo (meist höher als Bond-Albedo)
        let geometric_albedo = bond_albedo * 1.2;

        Self {
            equilibrium_temperature,
            surface_temperature: equilibrium_temperature, // Wird später durch Atmosphäre modifiziert
            bond_albedo,
            geometric_albedo: geometric_albedo.min(1.0),
            greenhouse_factor: 1.0, // Kein Treibhauseffekt ohne Atmosphäre
            day_night_temperatures: None,
        }
    }

    /// Berechnet Tag-/Nachttemperaturen für gebundene Rotation
    pub fn calculate_tidal_locked_temperatures(&mut self, thermal_redistribution: f64) {
        let base_temp = self.equilibrium_temperature;

        // Tagseite erhält 4x mehr Energie (über Hemisphäre verteilt)
        let day_temp =
            base_temp * (4.0 * (1.0 - thermal_redistribution) + thermal_redistribution).powf(0.25);
        let night_temp = base_temp * thermal_redistribution.powf(0.25);

        self.day_night_temperatures = Some((day_temp, night_temp));
        self.surface_temperature = (day_temp + night_temp) / 2.0;
    }

    /// Wendet Treibhauseffekt an
    pub fn apply_greenhouse_effect(&mut self, greenhouse_warming: f64) {
        self.greenhouse_factor = 1.0 + greenhouse_warming;
        self.surface_temperature = self.equilibrium_temperature * self.greenhouse_factor.powf(0.25);

        // Aktualisiere Tag/Nacht wenn vorhanden
        if let Some((day, night)) = self.day_night_temperatures {
            self.day_night_temperatures = Some((
                day * self.greenhouse_factor.powf(0.25),
                night * self.greenhouse_factor.powf(0.25),
            ));
        }
    }
}
