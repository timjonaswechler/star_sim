use crate::physics::units::*;
use crate::stellar_objects::stars::properties::TidalLockingAnalysis;
use crate::stellar_objects::bodies::habitability::assessment::TemperatureAnalysis;
use crate::stellar_objects::trojans_asteroid::objects::TrojanTidalAnalysis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitableZone {
    /// Innere Grenze der bewohnbaren Zone
    pub inner_edge: Distance,
    /// Äußere Grenze der bewohnbaren Zone
    pub outer_edge: Distance,
    /// Optimistische innere Grenze
    pub optimistic_inner: Distance,
    /// Optimistische äußere Grenze
    pub optimistic_outer: Distance,
}

/// Planetare Bewohnbarkeitsanalyse für spezifische Orbits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryHabitability {
    /// Orbitalentfernung des Planeten
    pub orbital_distance: Distance,
    /// Bewohnbarkeitsscore für diese Position (0.0-1.0)
    pub habitability_score: f64,
    /// Tidal Locking Analyse
    pub tidal_locking: TidalLockingAnalysis,
    /// Temperaturbereiche
    pub temperature_analysis: TemperatureAnalysis,
    /// Atmosphärische Überlegungen
    pub atmospheric_considerations: Vec<String>,
    /// Mögliche Bewohnbarkeitszonen (Tag/Nacht-Seite, etc.)
    pub habitable_regions: Vec<HabitableRegion>,
}

/// Bewohnbare Regionen auf einem Planeten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HabitableRegion {
    /// Gesamte Oberfläche bewohnbar
    Global,
    /// Nur Tag-Seite bewohnbar (tidal locked)
    DaySide,
    /// Nur Terminator-Zone bewohnbar
    TerminatorZone,
    /// Polare Regionen bewohnbar
    PolarRegions,
    /// Äquatoriale Regionen bewohnbar
    EquatorialRegions,
    /// Keine bewohnbaren Regionen
    None,
}

/// Zeitliche Entwicklung der Bewohnbarkeit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalHabitability {
    /// Bewohnbarkeit in der Vergangenheit (Gyr ago -> habitability)
    pub past_habitability: Vec<(f64, f64)>,
    /// Aktuelle Bewohnbarkeit
    pub current_habitability: f64,
    /// Zukünftige Bewohnbarkeit (Gyr from now -> habitability)
    pub future_habitability: Vec<(f64, f64)>,
    /// Gesamte bewohnbare Lebensdauer (Gyr)
    pub total_habitable_lifetime: f64,
    /// Bewohnbarkeitsfenster (Start, Ende in Gyr)
    pub habitability_window: (f64, f64),
}

/// Risikofaktoren für Bewohnbarkeit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Name des Risikofaktors
    pub name: String,
    /// Schweregrad (0.0-1.0)
    pub severity: f64,
    /// Wahrscheinlichkeit des Auftretens (0.0-1.0)
    pub probability: f64,
    /// Zeitskala des Risikos
    pub timescale: Time,
    /// Beschreibung der Auswirkungen
    pub impact_description: String,
}

/// Trojaner-spezifische Bewohnbarkeitsanalyse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanHabitability {
    /// Bewohnbarkeitsscore für Trojaner (0.0-1.0)
    pub habitability_score: f64,
    /// Stabile Temperaturbereiche
    pub temperature_stability: f64,
    /// Schutz durch Hill-Sphäre
    pub hill_sphere_protection: f64,
    /// Tidally locked Analyse für Trojaner
    pub tidal_considerations: TrojanTidalAnalysis,
    /// Langzeit-Bewohnbarkeit über Millionen Jahre
    pub long_term_viability: f64,
    /// Spezielle Habitabilitätszonen
    pub special_zones: Vec<TrojanHabitableZone>,
}

/// Spezielle bewohnbare Zonen für Trojaner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrojanHabitableZone {
    /// Stabile Region um Lagrange-Punkt
    LagrangeCore {
        radius: Distance,
        temperature_range: (f64, f64),
    },
    /// Oszillations-tolerante Zone
    LibrationZone {
        amplitude: Distance,
        seasonal_variation: f64,
    },
    /// Geschützte Zone innerhalb Hill-Sphäre
    HillSphereProtected { protection_factor: f64 },
}
