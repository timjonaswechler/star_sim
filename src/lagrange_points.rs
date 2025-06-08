// lagrange_points.rs - Lagrange-Punkte und Trojaner basierend auf dem Artikel
use crate::physics::constants::MIN_LAGRANGE_MASS_RATIO;
use crate::stellar_objects::stars::properties::*;
use crate::stellar_objects::stellar_systems::hierarchy::*;
use crate::physics::units::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// Lagrange-Punkt System für ein 2-Körper System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LagrangeSystem {
    /// Position von L1 (zwischen den Körpern, näher zum weniger massiven)
    pub l1_distance_from_secondary: Distance,
    /// Position von L2 (hinter dem weniger massiven Körper)
    pub l2_distance_from_secondary: Distance,
    /// Position von L3 (gegenüber dem weniger massiven Körper)
    pub l3_distance_from_primary: Distance,
    /// L4/L5 Stabilität (erfordert Massenverhältnis ≥ 24.96:1)
    pub l4_l5_stable: bool,
    /// Massenverhältnis Primary:Secondary
    pub mass_ratio: f64,
    /// Semimajor axis des Binärsystems
    pub separation: Distance,
    /// Liste von Trojanern an L4/L5
    pub trojans: Vec<TrojanObject>,
    /// Einheitensystem
    pub unit_system: UnitSystem,
}
/// Status aller Lagrange-Punkte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LagrangePointsStatus {
    pub l1_stable: bool,
    pub l2_stable: bool,
    pub l3_stable: bool,
    pub l4_stable: bool,
    pub l5_stable: bool,
    pub l4_trojans: Vec<TrojanObject>,
    pub l5_trojans: Vec<TrojanObject>,
}
/// Trojaner-Objekt an L4 oder L5
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanObject {
    /// Masse des Trojaners
    pub mass: Mass,
    /// Lagrange-Punkt (4 oder 5)
    pub lagrange_point: u8,
    /// Oszillationsamplitude um den L-Punkt (tadpole orbit)
    pub oscillation_amplitude: Distance,
    /// Oszillationsperiode
    pub oscillation_period: Time,
    /// Stabilität des Trojaners (0.0-1.0)
    pub stability: f64,
}
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
/// Stabilitätsanalyse für Trojaner-Systeme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanStabilityAnalysis {
    /// Anzahl stabiler Trojaner
    pub stable_trojans_count: usize,
    /// Anzahl instabiler Trojaner  
    pub unstable_trojans_count: usize,
    /// Mutual Trojaner Systeme
    pub mutual_systems: Vec<MutualTrojanSystem>,
    /// Durchschnittliche Trojaner-Stabilität
    pub average_trojan_stability: f64,
    /// Trojaner-spezifische Risikofaktoren
    pub trojan_risks: Vec<StabilityRiskFactor>,
    /// Lagrange-Punkte Status
    pub lagrange_points_status: LagrangePointsStatus,
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

impl LagrangeSystem {
    /// Erstellt ein neues Lagrange-System für zwei Sterne
    pub fn new(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        separation: Distance,
    ) -> Self {
        let primary_mass = primary.mass.clone();
        let secondary_mass = secondary.mass.clone();
        let mass_ratio = primary_mass.in_kg() / secondary_mass.in_kg();

        // Für Lagrange-Berechnungen nehmen wir an, dass Primary massiver ist
        let (m1, m2) = if mass_ratio >= 1.0 {
            (primary_mass, secondary_mass)
        } else {
            (secondary_mass, primary_mass)
        };

        let total_mass = Mass::kilograms(m1.in_kg() + m2.in_kg());
        let mu = m2.in_kg() / total_mass.in_kg(); // Massenverhältnis des kleineren Körpers

        // L1, L2, L3 Positionen (vereinfachte Näherungen)
        let l1_distance = Self::calculate_l1_distance(&separation, mu);
        let l2_distance = Self::calculate_l2_distance(&separation, mu);
        let l3_distance = Self::calculate_l3_distance(&separation, mu);

        // L4/L5 Stabilität nach Artikel: Massenverhältnis muss ≥ 24.96 sein
        let l4_l5_stable = mass_ratio.max(1.0 / mass_ratio) >= MIN_LAGRANGE_MASS_RATIO;

        Self {
            l1_distance_from_secondary: l1_distance,
            l2_distance_from_secondary: l2_distance,
            l3_distance_from_primary: l3_distance,
            l4_l5_stable,
            mass_ratio,
            separation: separation.clone(),
            trojans: Vec::new(),
            unit_system: separation.system,
        }
    }

    /// Berechnet L1 Position (zwischen den Körpern)
    /// Vereinfachte Näherung: r ≈ a * (μ/3)^(1/3)
    fn calculate_l1_distance(separation: &Distance, mu: f64) -> Distance {
        let distance = separation.value * (mu / 3.0).powf(1.0 / 3.0);
        Distance::new(distance, separation.system)
    }

    /// Berechnet L2 Position (hinter dem kleineren Körper)
    /// Vereinfachte Näherung: r ≈ a * (μ/3)^(1/3)
    fn calculate_l2_distance(separation: &Distance, mu: f64) -> Distance {
        let distance = separation.value * (mu / 3.0).powf(1.0 / 3.0);
        Distance::new(distance, separation.system)
    }

    /// Berechnet L3 Position (gegenüber dem kleineren Körper)
    /// Vereinfachte Näherung: r ≈ a * (7μ/12)
    fn calculate_l3_distance(separation: &Distance, mu: f64) -> Distance {
        let distance = separation.value * (7.0 * mu / 12.0);
        Distance::new(distance, separation.system)
    }

    /// Fügt einen Trojaner an L4 oder L5 hinzu
    pub fn add_trojan(&mut self, trojan: TrojanObject) -> Result<(), String> {
        if !self.l4_l5_stable {
            return Err("L4/L5 points are not stable for this mass ratio".to_string());
        }

        if trojan.lagrange_point != 4 && trojan.lagrange_point != 5 {
            return Err("Trojan must be at L4 or L5".to_string());
        }

        self.trojans.push(trojan);
        Ok(())
    }

    /// Berechnet die Position von L4 in kartesischen Koordinaten
    /// L4 bildet ein gleichseitiges Dreieck mit den beiden Hauptkörpern
    pub fn l4_position(&self) -> (Distance, Distance) {
        let x = self.separation.value * 0.5; // Mittelpunkt zwischen den Körpern
        let y = self.separation.value * (3.0_f64.sqrt() / 2.0); // Höhe des gleichseitigen Dreiecks

        (
            Distance::new(x, self.separation.system),
            Distance::new(y, self.separation.system),
        )
    }

    /// Berechnet die Position von L5 in kartesischen Koordinaten
    pub fn l5_position(&self) -> (Distance, Distance) {
        let (x, y) = self.l4_position();
        (x, Distance::new(-y.value, y.system)) // Gespiegelt über x-Achse
    }

    /// Prüft, ob ein kleiner Körper gravitativ von einem Lagrange-Punkt gefangen werden kann
    pub fn can_capture_at_lagrange_point(&self, point: u8, _test_mass: &Mass) -> bool {
        match point {
            1 | 2 | 3 => {
                // L1, L2, L3 sind nur quasi-stabil
                false
            }
            4 | 5 => {
                // L4, L5 können stabil sein, wenn Massenverhältnis ausreicht
                self.l4_l5_stable
            }
            _ => false,
        }
    }

    /// Berechnet die Hill-Sphäre an einem Lagrange-Punkt
    pub fn hill_sphere_at_lagrange_point(&self, point: u8, body_mass: &Mass) -> Option<Distance> {
        if !self.can_capture_at_lagrange_point(point, body_mass) {
            return None;
        }

        // Vereinfachte Hill-Sphäre Berechnung für Lagrange-Punkte
        let distance_to_point = match point {
            4 | 5 => self.separation.clone(),
            _ => return None,
        };

        // Hill-Radius an L4/L5 ist kleiner als bei normalen Orbits
        let total_system_mass = Mass::kilograms(10.0 * body_mass.in_kg()); // Näherung
        let hill_radius = distance_to_point.value
            * (body_mass.in_kg() / (3.0 * total_system_mass.in_kg())).powf(1.0 / 3.0)
            * 0.5;

        Some(Distance::new(hill_radius, distance_to_point.system))
    }

    /// Generiert einen Trojaner mit realistischen Eigenschaften
    pub fn generate_trojan(
        &self,
        lagrange_point: u8,
        trojan_mass: Mass,
        primary_mass: &Mass,
        secondary_mass: &Mass,
    ) -> Result<TrojanObject, String> {
        if lagrange_point != 4 && lagrange_point != 5 {
            return Err("Can only generate trojans at L4 or L5".to_string());
        }

        if !self.l4_l5_stable {
            return Err("L4/L5 not stable for this system".to_string());
        }

        // Oszillationsamplitude basierend auf Systemparametern
        let base_amplitude = self.separation.value * 0.1; // ~10% der Separation
        let mass_factor = (trojan_mass.in_kg() / secondary_mass.in_kg()).min(1.0);
        let amplitude = base_amplitude * (1.0 - mass_factor * 0.5);

        // Oszillationsperiode (mehrere Orbitalperioden)
        let orbital_period_years = (self.separation.in_au().powf(3.0)
            / (primary_mass.in_solar_masses() + secondary_mass.in_solar_masses()))
        .sqrt();
        let oscillation_period_years = orbital_period_years * (2.0 + mass_factor);

        // Stabilität basierend auf Massenverhältnissen
        let mass_ratio_factor = (self.mass_ratio / MIN_LAGRANGE_MASS_RATIO).min(1.0);
        let size_factor = (1.0 - trojan_mass.in_kg() / secondary_mass.in_kg()).max(0.1);
        let stability = mass_ratio_factor * size_factor;

        Ok(TrojanObject {
            mass: trojan_mass,
            lagrange_point,
            oscillation_amplitude: Distance::new(amplitude, self.unit_system),
            oscillation_period: Time::years(oscillation_period_years),
            stability,
        })
    }

    /// Konvertiert zu anderem Einheitensystem
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if target == self.unit_system {
            return self.clone();
        }

        Self {
            l1_distance_from_secondary: self.l1_distance_from_secondary.to_system(target),
            l2_distance_from_secondary: self.l2_distance_from_secondary.to_system(target),
            l3_distance_from_primary: self.l3_distance_from_primary.to_system(target),
            separation: self.separation.to_system(target),
            trojans: self.trojans.iter().map(|t| t.to_system(target)).collect(),
            unit_system: target,
            ..self.clone()
        }
    }
}

impl TrojanObject {
    /// Konvertiert zu anderem Einheitensystem
    pub fn to_system(&self, target: UnitSystem) -> Self {
        Self {
            mass: self.mass.to_system(target),
            oscillation_amplitude: self.oscillation_amplitude.to_system(target),
            oscillation_period: self.oscillation_period.to_system(target),
            ..self.clone()
        }
    }

    /// Prüft ob der Trojaner langfristig stabil ist
    pub fn is_long_term_stable(&self) -> bool {
        self.stability > 0.7
    }

    /// Berechnet maximale Entfernung vom Lagrange-Punkt während Oszillation
    pub fn maximum_distance_from_lagrange_point(&self) -> Distance {
        // Tadpole-Orbit: maximale Abweichung etwa die Oszillationsamplitude
        self.oscillation_amplitude.clone()
    }
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
    /// Berechnet detaillierte Trojaner-Dynamik basierend auf dem Artikel
    pub fn calculate_libration_dynamics(
        &self,
        primary_mass: &Mass,
        secondary_mass: &Mass,
        separation: &Distance,
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
        separation: &Distance,
        mass_parameter: f64,
    ) -> Distance {
        // Vereinfachte Formel basierend auf Hill-Sphäre und Trojaner-Masse
        let hill_scale = (mass_parameter / 3.0).powf(1.0 / 3.0);
        let trojan_factor = (self.mass.in_kg() / 1e15).powf(0.1); // Schwache Massenabhängigkeit

        let amplitude_fraction = hill_scale * trojan_factor * 0.1; // ~10% der Hill-Sphäre
        Distance::new(separation.value * amplitude_fraction, separation.system)
    }

    /// Schätzt säkulare Drift durch Perturbationen
    fn estimate_secular_drift(&self, primary_mass: &Mass, secondary_mass: &Mass) -> f64 {
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
    fn assess_long_term_stability(&self, primary_mass: &Mass, secondary_mass: &Mass) -> f64 {
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
        let mass_parameter =
            secondary_mass.in_kg() / (primary_mass.in_kg() + secondary_mass.in_kg());
        if matches!(
            self.determine_oscillation_pattern(mass_parameter),
            OscillationPattern::Tadpole { .. }
        ) {
            stability *= 1.1;
        }

        stability.min(1.0)
    }

    /// Berechnet Orbitalperiode für Trojaner-System
    fn calculate_orbital_period(
        &self,
        primary_mass: &Mass,
        secondary_mass: &Mass,
        separation: &Distance,
    ) -> Time {
        let total_mass = Mass::kilograms(primary_mass.in_kg() + secondary_mass.in_kg());

        // Trojaner haben dieselbe Periode wie der sekundäre Körper
        let period_years = (separation.in_au().powf(3.0) / total_mass.in_solar_masses()).sqrt();
        Time::years(period_years)
    }
}

/// Erweiterte LagrangeSystem Implementation
impl LagrangeSystem {
    /// Generiert einen Trojaner mit detaillierter Dynamik-Analyse
    pub fn generate_enhanced_trojan(
        &self,
        lagrange_point: u8,
        trojan_mass: Mass,
        primary_mass: &Mass,
        secondary_mass: &Mass,
        rng: &mut ChaCha8Rng,
    ) -> Result<TrojanObject, String> {
        if !self.l4_l5_stable {
            return Err("L4/L5 not stable - mass ratio below 24.96:1".to_string());
        }

        if lagrange_point != 4 && lagrange_point != 5 {
            return Err("Enhanced trojans only supported at L4/L5".to_string());
        }

        // Basis-Trojaner generieren
        let mut trojan =
            self.generate_trojan(lagrange_point, trojan_mass, primary_mass, secondary_mass)?;

        // Erweiterte Eigenschaften hinzufügen
        let dynamics =
            trojan.calculate_libration_dynamics(primary_mass, secondary_mass, &self.separation);

        // Anpassung der Oszillationsamplitude basierend auf Dynamik
        trojan.oscillation_amplitude = dynamics.libration_amplitude.clone();
        trojan.oscillation_period = dynamics.libration_period.clone();
        trojan.stability = dynamics.long_term_stability;

        Ok(trojan)
    }

    /// Erstellt ein Mutual Trojaner System
    pub fn create_mutual_trojan_system(
        &self,
        lagrange_point: u8,
        primary_trojan_mass: Mass,
        secondary_masses: Vec<Mass>,
        primary_mass: &Mass,
        secondary_mass: &Mass,
        rng: &mut ChaCha8Rng,
    ) -> Result<MutualTrojanSystem, String> {
        if !self.l4_l5_stable {
            return Err("Cannot create mutual trojans - system unstable".to_string());
        }

        if secondary_masses.len() > 5 {
            return Err("Too many trojans - maximum 5 supported".to_string());
        }

        // Primärer Trojaner
        let primary_trojan = self.generate_enhanced_trojan(
            lagrange_point,
            primary_trojan_mass.clone(),
            primary_mass,
            secondary_mass,
            rng,
        )?;

        // Sekundäre Trojaner
        let mut secondary_trojans = Vec::new();
        for (i, sec_mass) in secondary_masses.iter().enumerate() {
            // Leichte Variation der Position für sekundäre Trojaner
            let mut sec_trojan = self.generate_enhanced_trojan(
                lagrange_point,
                Mass::new(sec_mass.value, sec_mass.system),
                primary_mass,
                secondary_mass,
                rng,
            )?;

            // Kleine Phasenverschiebung
            let phase_shift = (i as f64 + 1.0) * 10.0; // 10° pro Trojaner
            sec_trojan.oscillation_amplitude = Distance::new(
                sec_trojan.oscillation_amplitude.value * (1.0 + i as f64 * 0.1),
                sec_trojan.oscillation_amplitude.system,
            );

            secondary_trojans.push(sec_trojan);
        }

        // Mutual Dynamics berechnen
        let mutual_dynamics = Self::calculate_mutual_dynamics(
            &primary_trojan,
            &secondary_trojans,
            primary_mass,
            secondary_mass,
        );

        // System-Stabilität
        let total_trojan_mass: f64 =
            primary_trojan_mass.in_kg() + secondary_masses.iter().map(|m| m.in_kg()).sum::<f64>();
        let stability_reduction = (total_trojan_mass / secondary_mass.in_kg()).min(0.5);
        let system_stability = (primary_trojan.stability * (1.0 - stability_reduction)).max(0.1);

        Ok(MutualTrojanSystem {
            primary_trojan,
            secondary_trojans,
            mutual_dynamics,
            system_stability,
        })
    }

    /// Berechnet Dynamik zwischen mehreren Trojanern
    fn calculate_mutual_dynamics(
        primary: &TrojanObject,
        secondaries: &[TrojanObject],
        primary_mass: &Mass,
        secondary_mass: &Mass,
    ) -> MutualDynamics {
        let total_trojan_mass =
            primary.mass.in_kg() + secondaries.iter().map(|t| t.mass.in_kg()).sum::<f64>();

        // Barycenter der Trojaner (vereinfacht am Lagrange-Punkt)
        let barycenter = (Distance::au(0.0), Distance::au(0.0));

        // Interne Perioden (basierend auf gegenseitiger Gravitation)
        let mut internal_periods = Vec::new();
        let base_period = primary.oscillation_period.in_years();

        for (i, trojan) in secondaries.iter().enumerate() {
            // Periode variiert leicht basierend auf Masse und Position
            let period_variation =
                1.0 + (i as f64 * 0.02) + (trojan.mass.in_kg() / primary.mass.in_kg() * 0.1);
            internal_periods.push(Time::years(base_period * period_variation));
        }

        // Massenverhältnisse
        let mass_ratios: Vec<f64> = secondaries
            .iter()
            .map(|t| t.mass.in_kg() / primary.mass.in_kg())
            .collect();

        // Kollektive Hill-Sphäre
        let separation = Distance::au(5.2); // Beispiel: Jupiter-Orbit
        let trojan_mass = Mass::kilograms(total_trojan_mass);
        let total_system_mass = Mass::kilograms(primary_mass.in_kg() + secondary_mass.in_kg());

        // Hill-Radius für die gesamte Trojaner-Gruppe
        let hill_factor = (trojan_mass.in_kg() / (3.0 * total_system_mass.in_kg())).powf(1.0 / 3.0);
        let collective_hill_sphere =
            Distance::new(separation.value * hill_factor, separation.system);

        MutualDynamics {
            trojan_barycenter: barycenter,
            internal_periods,
            mass_ratios,
            collective_hill_sphere,
        }
    }

    /// Validiert erweiterte Stabilitätskriterien für Trojaner
    pub fn validate_trojan_stability(&self, trojans: &[TrojanObject]) -> Vec<f64> {
        trojans
            .iter()
            .map(|trojan| {
                // Basis-Stabilität aus dem 24.96:1 Kriterium
                let mass_ratio_stability = if self.mass_ratio >= MIN_LAGRANGE_MASS_RATIO {
                    1.0
                } else {
                    0.0
                };

                // Reduktion durch Überfüllung
                let crowding_factor = if trojans.len() > 3 {
                    0.9_f64.powf((trojans.len() - 3) as f64)
                } else {
                    1.0
                };

                // Reduktion durch Masse
                let mass_factor = if trojan.mass.in_kg() > 1e18 { 0.8 } else { 1.0 };

                mass_ratio_stability * crowding_factor * mass_factor * trojan.stability
            })
            .collect()
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
    pub fn total_trojan_mass(&self) -> Mass {
        let total_kg = self.primary_trojan.mass.in_kg()
            + self
                .secondary_trojans
                .iter()
                .map(|t| t.mass.in_kg())
                .sum::<f64>();
        Mass::kilograms(total_kg)
    }

    /// Schätzt Lebensdauer des Systems
    pub fn estimated_lifetime(&self) -> Time {
        let base_lifetime = 1e9; // 1 Gyr für perfekte Bedingungen
        let stability_factor = self.system_stability;
        let crowding_penalty = 0.9_f64.powf(self.secondary_trojans.len() as f64);

        let lifetime_years = base_lifetime * stability_factor * crowding_penalty;
        Time::years(lifetime_years.max(1e6)) // Minimum 1 Myr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::units::Time;

    #[test]
    fn test_sun_jupiter_lagrange_system() {
        let sun = StellarProperties::sun_like();
        let jupiter_mass = Mass::solar_masses(0.000954); // Jupiter: ~0.1% Sonnenmasse
        let jupiter = StellarProperties::new(jupiter_mass, Time::years(4.6), 0.0);

        let separation = Distance::au(5.2); // Jupiter's orbit
        let lagrange_system = LagrangeSystem::new(&sun, &jupiter, separation);

        // Sonne-Jupiter System sollte stabile L4/L5 haben (Massenverhältnis ~1047:1)
        assert!(lagrange_system.l4_l5_stable);
        assert!(lagrange_system.mass_ratio > MIN_LAGRANGE_MASS_RATIO);

        // L4/L5 sollten in der Nähe von Jupiter's Orbit sein
        let (l4_x, l4_y) = lagrange_system.l4_position();
        assert!((l4_x.in_au() - 2.6).abs() < 0.5); // Etwa halbe Separation
        assert!((l4_y.in_au() - 4.5).abs() < 1.0); // Etwa √3/2 * separation
    }

    #[test]
    fn test_trojan_generation() {
        let sun = StellarProperties::sun_like();
        let jupiter = StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6), 0.0);
        let separation = Distance::au(5.2);
        let lagrange_system = LagrangeSystem::new(&sun, &jupiter, separation);

        // Asteroiden-masse Trojaner
        let asteroid_mass = Mass::kilograms(1e15); // Typischer Asteroid
        let trojan = lagrange_system.generate_trojan(4, asteroid_mass, &sun.mass, &jupiter.mass);

        assert!(trojan.is_ok());
        let trojan = trojan.unwrap();
        assert_eq!(trojan.lagrange_point, 4);
        assert!(trojan.stability > 0.5);
        assert!(trojan.oscillation_period.in_years() > 10.0); // Mehrere Orbitalperioden
    }

    #[test]
    fn test_unstable_lagrange_system() {
        // Zwei etwa gleich schwere Sterne (Massenverhältnis < 24.96)
        let star1 = StellarProperties::sun_like();
        let star2 = StellarProperties::new(Mass::solar_masses(0.8), Time::years(4.6), 0.0);
        let separation = Distance::au(1.0);

        let lagrange_system = LagrangeSystem::new(&star1, &star2, separation);

        // L4/L5 sollten nicht stabil sein
        assert!(!lagrange_system.l4_l5_stable);

        // Trojaner sollten nicht hinzufügbar sein
        let trojan_mass = Mass::solar_masses(0.001);
        let trojan_result =
            lagrange_system.generate_trojan(4, trojan_mass, &star1.mass, &star2.mass);
        assert!(trojan_result.is_err());
    }

    #[test]
    fn test_lagrange_point_distances() {
        let primary = StellarProperties::sun_like();
        let secondary = StellarProperties::new(Mass::solar_masses(0.001), Time::years(4.6), 0.0);
        let separation = Distance::au(1.0);

        let lagrange_system = LagrangeSystem::new(&primary, &secondary, separation);

        // L1, L2 sollten in der Nähe des kleineren Körpers sein
        assert!(lagrange_system.l1_distance_from_secondary.in_au() < 0.1);
        assert!(lagrange_system.l2_distance_from_secondary.in_au() < 0.1);

        // L3 sollte nahe der Separation sein
        assert!(lagrange_system.l3_distance_from_primary.in_au() < 0.01);
    }
}
#[cfg(test)]
mod trojan_tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_jupiter_trojan_dynamics() {
        let sun = StellarProperties::sun_like();
        let jupiter = StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6e9), 0.0);

        let separation = Distance::au(5.2);
        let lagrange_system = LagrangeSystem::new(&sun, &jupiter, separation);

        // Jupiter-System sollte stabile L4/L5 haben
        assert!(lagrange_system.l4_l5_stable);
        assert!(lagrange_system.mass_ratio > MIN_LAGRANGE_MASS_RATIO);

        // Erstelle einen Jupiter-Trojaner
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let trojan_mass = Mass::kilograms(1e16); // Typischer Asteroid

        let trojan = lagrange_system
            .generate_enhanced_trojan(4, trojan_mass, &sun.mass, &jupiter.mass, &mut rng)
            .expect("Should generate trojan successfully");

        // Berechne Dynamik
        let dynamics = trojan.calculate_libration_dynamics(
            &sun.mass,
            &jupiter.mass,
            &lagrange_system.separation,
        );

        // Validiere Eigenschaften
        assert!(dynamics.long_term_stability > 0.8);
        assert!(dynamics.libration_period.in_years() > 10.0); // Mehrere Orbitalperioden
        assert!(dynamics.secular_drift_rate < 0.01); // Niedrige Drift

        // Sollte Tadpole-Orbit sein für kleine Masse
        match dynamics.oscillation_pattern {
            OscillationPattern::Tadpole {
                center_point,
                amplitude_degrees,
            } => {
                assert_eq!(center_point, 4);
                assert!(amplitude_degrees > 5.0 && amplitude_degrees < 30.0);
            }
            _ => panic!("Small trojan should have tadpole orbit"),
        }

        println!(
            "Jupiter Trojan Stability: {:.2}",
            dynamics.long_term_stability
        );
        println!(
            "Libration Period: {:.1} years",
            dynamics.libration_period.in_years()
        );
    }

    #[test]
    fn test_mutual_trojan_system() {
        let sun = StellarProperties::sun_like();
        let jupiter = StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6e9), 0.0);

        let separation = Distance::au(5.2);
        let lagrange_system = LagrangeSystem::new(&sun, &jupiter, separation);
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // Erstelle Mutual Trojaner System
        let primary_mass = Mass::kilograms(5e15);
        let secondary_masses = vec![Mass::kilograms(2e15), Mass::kilograms(1e15)];

        let mutual_system = lagrange_system
            .create_mutual_trojan_system(
                4,
                primary_mass,
                secondary_masses,
                &sun.mass,
                &jupiter.mass,
                &mut rng,
            )
            .expect("Should create mutual trojan system");

        // Validiere System
        assert_eq!(mutual_system.secondary_trojans.len(), 2);
        assert!(mutual_system.system_stability > 0.5);
        assert!(mutual_system.is_long_term_stable());

        // Prüfe dass sekundäre Trojaner unterschiedliche Perioden haben
        let periods: Vec<f64> = mutual_system
            .mutual_dynamics
            .internal_periods
            .iter()
            .map(|p| p.in_years())
            .collect();
        assert!(periods[0] != periods[1]);

        println!(
            "Mutual Trojan System Stability: {:.2}",
            mutual_system.system_stability
        );
        println!(
            "Estimated Lifetime: {:.1} Myr",
            mutual_system.estimated_lifetime().in_years() / 1e6
        );
    }

    #[test]
    fn test_trojan_stability_criteria() {
        let sun = StellarProperties::sun_like();

        // Test mit verschiedenen Sekundärmassen für Massenverhältnis
        let test_masses = vec![
            Mass::solar_masses(0.04), // 25:1 ratio - stabil
            Mass::solar_masses(0.05), // 20:1 ratio - instabil
        ];

        for secondary_mass in test_masses {
            let secondary = StellarProperties::new(secondary_mass.clone(), Time::years(4.6e9), 0.0);
            let lagrange_system = LagrangeSystem::new(&sun, &secondary, Distance::au(1.0));

            let ratio = sun.mass.in_kg() / secondary_mass.in_kg();
            let expected_stable = ratio >= MIN_LAGRANGE_MASS_RATIO;

            assert_eq!(lagrange_system.l4_l5_stable, expected_stable);
            println!("Mass ratio {:.1}:1 -> Stable: {}", ratio, expected_stable);
        }
    }

    #[test]
    fn test_oscillation_patterns() {
        let sun = StellarProperties::sun_like();
        let jupiter = StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6e9), 0.0);
        let separation = Distance::au(5.2);

        // Teste verschiedene Trojaner-Massen
        let test_masses = vec![
            Mass::kilograms(1e14), // Sehr klein -> Tadpole
            Mass::kilograms(1e17), // Mittel -> Horseshoe möglich
            Mass::kilograms(1e20), // Groß -> Quasi-stable
        ];

        for trojan_mass in test_masses {
            let trojan = TrojanObject {
                mass: trojan_mass.clone(),
                lagrange_point: 4,
                oscillation_amplitude: Distance::au(0.1),
                oscillation_period: Time::years(12.0),
                stability: 0.9,
            };

            let dynamics =
                trojan.calculate_libration_dynamics(&sun.mass, &jupiter.mass, &separation);

            match trojan_mass.in_kg() {
                m if m < 1e15 => {
                    // Kleine Massen sollten Tadpole-Orbits haben
                    assert!(matches!(
                        dynamics.oscillation_pattern,
                        OscillationPattern::Tadpole { .. }
                    ));
                }
                m if m < 1e18 => {
                    // Mittlere Massen können Horseshoe-Orbits haben
                    assert!(matches!(
                        dynamics.oscillation_pattern,
                        OscillationPattern::Tadpole { .. } | OscillationPattern::Horseshoe { .. }
                    ));
                }
                _ => {
                    // Große Massen sind nur quasi-stabil
                    assert!(matches!(
                        dynamics.oscillation_pattern,
                        OscillationPattern::QuasiStable { .. }
                    ));
                }
            }

            println!(
                "Mass {:.0e} kg -> Pattern: {:?}",
                trojan_mass.in_kg(),
                dynamics.oscillation_pattern
            );
        }
    }
}
