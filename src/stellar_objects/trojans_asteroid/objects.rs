use crate::physics::astrophysics::OscillationPattern;
use crate::physics::constants::MIN_LAGRANGE_MASS_RATIO;
use crate::physics::units::*;
use crate::stellar_objects::trojans_asteroid::dynamics::{MutualDynamics, TrojanDynamics};
use serde::{Deserialize, Serialize};

/// Gezeiten-Analyse für Trojaner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanTidalAnalysis {
    /// Tidal Heating durch Librations
    pub libration_heating: f64,
    /// Gravitationsgradienten von beiden Sternen
    pub dual_tidal_stress: f64,
    /// Rotationsstabilität
    pub rotation_stability: f64,
}

/// Spezielle Konfigurationen von Trojanern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrojanConfiguration {
    /// Einzelner Trojaner
    Single(TrojanObject),
    /// Mehrere Trojaner am selben Punkt (Co-orbital)
    Multiple {
        trojans: Vec<TrojanObject>,
        lagrange_point: u8,
    },
    /// Mutual Trojans (gleiche Masse)
    MutualTrojans {
        trojan_a: TrojanObject,
        trojan_b: TrojanObject,
    },
}

/// Trojaner-Objekt an L4 oder L5
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanObject {
    /// Masse des Trojaners
    pub mass: Mass<Kilogram>,
    /// Lagrange-Punkt (4 oder 5)
    pub lagrange_point: u8,
    /// Oszillationsamplitude um den L-Punkt (tadpole orbit)
    pub oscillation_amplitude: Distance<Meter>,
    /// Oszillationsperiode
    pub oscillation_period: Time<Year>,
    /// Stabilität des Trojaners (0.0-1.0)
    pub stability: f64,
}

/// Mutual Trojans: Mehrere Objekte am selben Lagrange-Punkt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualTrojanSystem {
    /// Primärer Trojaner (größte Masse)
    pub primary_trojan: TrojanObject,
    /// Sekundäre Trojaner (co-orbital)
    pub secondary_trojans: Vec<TrojanObject>,
    /// Gegenseitige Gravitations-Interaktion
    pub mutual_dynamics: MutualDynamics,
    /// Gesamtstabilität des Systems
    pub system_stability: f64,
}

impl TrojanConfiguration {
    /// Prüft Stabilität der Konfiguration
    pub fn is_stable(&self) -> bool {
        match self {
            Self::Single(trojan) => trojan.is_long_term_stable(),
            Self::Multiple { trojans, .. } => {
                trojans.iter().all(|t| t.is_long_term_stable()) && trojans.len() <= 3
            }
            Self::MutualTrojans { trojan_a, trojan_b } => {
                trojan_a.is_long_term_stable()
                    && trojan_b.is_long_term_stable()
                    && (trojan_a.mass.in_kg() - trojan_b.mass.in_kg()).abs() / trojan_a.mass.in_kg()
                        < 0.1
            }
        }
    }
}

impl TrojanObject {
    /// Prüft ob der Trojaner langfristig stabil ist
    pub fn is_long_term_stable(&self) -> bool {
        self.stability > 0.7
    }

    /// Berechnet maximale Entfernung vom Lagrange-Punkt während Oszillation
    pub fn maximum_distance_from_lagrange_point(&self) -> Distance<Meter> {
        // Tadpole-Orbit: maximale Abweichung etwa die Oszillationsamplitude
        self.oscillation_amplitude.clone()
    }

    /// Berechnet detaillierte Trojaner-Dynamik basierend auf dem Artikel
    pub fn calculate_libration_dynamics(
        &self,
        primary_mass: &Mass<Kilogram>,
        secondary_mass: &Mass<Kilogram>,
        separation: &Distance<Meter>,
    ) -> TrojanDynamics {
        let mass_parameter =
            secondary_mass.in_kg() / (primary_mass.in_kg() + secondary_mass.in_kg());
        let orbital_period =
            self.calculate_orbital_period(primary_mass, secondary_mass, separation);

        // Librations-Frequenz (vereinfacht nach restricted 3-body problem)
        let libration_frequency = (3.0 * mass_parameter).sqrt();
        let libration_period_years = orbital_period.in_years() / libration_frequency;

        // Oszillationsmuster basierend auf Trojaner-Eigenschaften
        let pattern = self.determine_oscillation_pattern(mass_parameter);

        // Amplitude basierend auf Anfangsbedingungen und Masse
        let amplitude = self.calculate_libration_amplitude(separation, mass_parameter);

        // Säkulare Drift durch Perturbationen
        let drift_rate = self.estimate_secular_drift(primary_mass, secondary_mass);

        // Langzeit-Stabilität
        let stability = self.assess_long_term_stability(primary_mass, secondary_mass);

        TrojanDynamics {
            oscillation_pattern: pattern,
            libration_amplitude: amplitude,
            libration_period: Time::years(libration_period_years),
            secular_drift_rate: drift_rate,
            long_term_stability: stability,
        }
    }

    /// Bestimmt Oszillationsmuster basierend auf Systemparametern
    fn determine_oscillation_pattern(&self, mass_parameter: f64) -> OscillationPattern {
        let trojan_to_secondary_ratio = self.mass.in_kg() / (mass_parameter * 1.98847e30); // Relative zu Sonnenmasse

        if trojan_to_secondary_ratio < 1e-9 {
            // Sehr kleine Trojaner: Stabile Tadpole-Orbits
            OscillationPattern::Tadpole {
                center_point: self.lagrange_point,
                amplitude_degrees: 10.0 + trojan_to_secondary_ratio * 1e8, // 10-20°
            }
        } else if trojan_to_secondary_ratio < 1e-6 {
            // Mittlere Trojaner: Mögliche Horseshoe-Orbits
            OscillationPattern::Horseshoe {
                transition_probability: (trojan_to_secondary_ratio * 1e6).min(0.3),
                period_ratio: 100.0 + trojan_to_secondary_ratio * 1e5,
            }
        } else {
            // Große Trojaner: Nur quasi-stabil
            OscillationPattern::QuasiStable {
                escape_timescale: Time::years(1e6 / (trojan_to_secondary_ratio * 1e6)),
                drift_direction: if self.lagrange_point == 4 {
                    60.0
                } else {
                    -60.0
                },
            }
        }
    }

    /// Berechnet Librations-Amplitude
    fn calculate_libration_amplitude(
        &self,
        separation: &Distance<Meter>,
        mass_parameter: f64,
    ) -> Distance<Meter> {
        // Vereinfachte Formel basierend auf Hill-Sphäre und Trojaner-Masse
        let hill_scale = (mass_parameter / 3.0).powf(1.0 / 3.0);
        let trojan_factor = (self.mass.in_kg() / 1e15).powf(0.1); // Schwache Massenabhängigkeit

        let amplitude_fraction = hill_scale * trojan_factor * 0.1; // ~10% der Hill-Sphäre
        (separation.value * amplitude_fraction).get()
    }

    /// Schätzt säkulare Drift durch Perturbationen
    fn estimate_secular_drift(
        &self,
        primary_mass: &Mass<Kilogram>,
        secondary_mass: &Mass<Kilogram>,
    ) -> f32 {
        let mass_ratio = primary_mass.in_kg() / secondary_mass.in_kg();

        // Drift ist stärker für:
        // 1. Kleinere Massenverhältnisse (näher an 24.96:1 Grenze)
        // 2. Größere Trojaner-Massen
        // 3. Unregelmäßige Trojaner-Formen (hier vereinfacht ignoriert)

        let ratio_factor = if mass_ratio > MIN_LAGRANGE_MASS_RATIO {
            (MIN_LAGRANGE_MASS_RATIO / mass_ratio).powf(2.0)
        } else {
            10.0 // Instabil
        };

        let mass_factor = (self.mass.in_kg() / 1e16).powf(0.5);

        ratio_factor * mass_factor * 0.001 // AU/Myr
    }

    /// Bewertet Langzeit-Stabilität über Millionen Jahre
    fn assess_long_term_stability(
        &self,
        primary_mass: &Mass<Kilogram>,
        secondary_mass: &Mass<Kilogram>,
    ) -> f64 {
        let mass_ratio = primary_mass.in_kg() / secondary_mass.in_kg();

        // Basiert auf dem 24.96:1 Kriterium aus dem Artikel
        if mass_ratio < MIN_LAGRANGE_MASS_RATIO {
            return 0.1; // Fundamental instabil
        }

        let mut stability = 0.9;

        // Reduktion durch Größe des Trojaners
        let trojan_ratio = self.mass.in_kg() / secondary_mass.in_kg();
        if trojan_ratio > 0.001 {
            stability *= (1.0 - trojan_ratio * 0.5).max(0.1);
        }

        // Reduktion durch niedrige anfängliche Stabilität
        stability *= self.stability;

        // Bonus für Tadpole-Orbits
        if matches!(
            self.calculate_libration_dynamics(primary_mass, secondary_mass, &Distance::au(1.0))
                .oscillation_pattern,
            OscillationPattern::Tadpole { .. }
        ) {
            stability *= 1.1;
        }

        stability.min(1.0)
    }

    /// Berechnet Orbitalperiode für Trojaner-System
    fn calculate_orbital_period(
        &self,
        primary_mass: &Mass<Kilogram>,
        secondary_mass: &Mass<Kilogram>,
        separation: &Distance<Meter>,
    ) -> Time<Year> {
        let total_mass = Mass::kilograms(primary_mass.in_kg() + secondary_mass.in_kg());

        // Trojaner haben dieselbe Periode wie der sekundäre Körper
        let period_years = (separation.in_au().powf(3.0) / total_mass.in_solar_masses()).sqrt();
        Time::years(period_years)
    }
}

/// Hilfsfunktionen für Trojaner-Analyse
impl MutualTrojanSystem {
    /// Prüft ob das Mutual Trojaner System langzeit-stabil ist
    pub fn is_long_term_stable(&self) -> bool {
        self.system_stability > 0.7
            && self.primary_trojan.is_long_term_stable()
            && self.secondary_trojans.iter().all(|t| t.stability > 0.5)
    }

    /// Berechnet Gesamtmasse aller Trojaner
    pub fn total_trojan_mass(&self) -> Mass<Kilogram> {
        let total_kg = self.primary_trojan.mass.in_kg()
            + self
                .secondary_trojans
                .iter()
                .map(|t| t.mass.in_kg())
                .sum::<f64>();
        Mass::kilograms(total_kg)
    }

    /// Schätzt Lebensdauer des Systems
    pub fn estimated_lifetime(&self) -> Time<Year> {
        let base_lifetime = 1e9; // 1 Gyr für perfekte Bedingungen
        let stability_factor = self.system_stability;
        let crowding_penalty = 0.9_f64.powf(self.secondary_trojans.len() as f64);

        let lifetime_years = base_lifetime * stability_factor * crowding_penalty;
        Time::years(lifetime_years.max(1e6)) // Minimum 1 Myr
    }
}
