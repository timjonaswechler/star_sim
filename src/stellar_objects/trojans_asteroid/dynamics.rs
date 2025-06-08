use crate::physics::units::*;
use serde::{Deserialize, Serialize};

/// Erweiterte Trojaner-Dynamik basierend auf "tadpole oscillations" aus dem Artikel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanDynamics {
    /// Art der Oszillation um den Lagrange-Punkt
    pub oscillation_pattern: OscillationPattern,
    /// Librations-Amplitude (maximale Abweichung vom L-Punkt)
    pub libration_amplitude: Distance,
    /// Librations-Periode (typisch mehrere Orbitalperioden)
    pub libration_period: Time,
    /// Säkulare Drift über Millionen Jahre
    pub secular_drift_rate: f64, // AU/Myr
    /// Langzeit-Stabilität (0.0-1.0)
    pub long_term_stability: f64,
}

/// Oszillationsmuster für Trojaner (vereinfacht nach Artikel)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OscillationPattern {
    /// Tadpole-Orbit: Kleine Oszillationen um L4/L5
    Tadpole {
        center_point: u8,       // 4 oder 5
        amplitude_degrees: f64, // Winkelabweichung in Grad
    },
    /// Horseshoe-Orbit: Größere Oszillationen zwischen L3, L4, L5
    Horseshoe {
        transition_probability: f64, // Wahrscheinlichkeit für L4↔L5 Wechsel
        period_ratio: f64,           // Verhältnis zur Orbitalperiode
    },
    /// Quasi-stable: Nur temporär an Lagrange-Punkt gefangen
    QuasiStable {
        escape_timescale: Time,
        drift_direction: f64, // Richtung der Drift in Grad
    },
}

/// Dynamik zwischen mehreren Trojanern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualDynamics {
    /// Barycenter der Trojaner-Gruppe
    pub trojan_barycenter: (Distance, Distance), // (x, y) relativ zu L4/L5
    /// Relative Orbitalperioden innerhalb der Gruppe
    pub internal_periods: Vec<Time>,
    /// Massenverhältnisse der Trojaner
    pub mass_ratios: Vec<f64>,
    /// Hill-Sphäre der Trojaner-Gruppe
    pub collective_hill_sphere: Distance,
}
