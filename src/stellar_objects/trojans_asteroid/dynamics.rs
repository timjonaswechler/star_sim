use crate::physics::astrophysics::OscillationPattern;
use crate::physics::units::*;
use serde::{Deserialize, Serialize};

/// Erweiterte Trojaner-Dynamik basierend auf "tadpole oscillations" aus dem Artikel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanDynamics {
    /// Art der Oszillation um den Lagrange-Punkt
    pub oscillation_pattern: OscillationPattern,
    /// Librations-Amplitude (maximale Abweichung vom L-Punkt)
    pub libration_amplitude: Distance<Meter>,
    /// Librations-Periode (typisch mehrere Orbitalperioden)
    pub libration_period: Time<Year>,
    /// Säkulare Drift über Millionen Jahre
    pub secular_drift_rate: f64, // AU/Myr
    /// Langzeit-Stabilität (0.0-1.0)
    pub long_term_stability: f64,
}

/// Dynamik zwischen mehreren Trojanern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualDynamics {
    /// Barycenter der Trojaner-Gruppe
    pub trojan_barycenter: (Distance<Meter>, Distance<Meter>), // (x, y) relativ zu L4/L5
    /// Relative Orbitalperioden innerhalb der Gruppe
    pub internal_periods: Vec<Time<Year>>,
    /// Massenverhältnisse der Trojaner
    pub mass_ratios: Vec<f64>,
    /// Hill-Sphäre der Trojaner-Gruppe
    pub collective_hill_sphere: Distance<Meter>,
}
