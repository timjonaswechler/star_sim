use crate::physics::units::*;
use crate::physics::constants::MIN_LAGRANGE_MASS_RATIO;
use crate::stellar_objects::stars::properties::StellarProperties;
use crate::stellar_objects::trojans_asteroid::dynamics::MutualDynamics;
use crate::stellar_objects::trojans_asteroid::objects::{
    MutualTrojanSystem, TrojanObject,
};
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
                Mass::new(sec_mass.value_in_system_base(), sec_mass.unit_system()),
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
