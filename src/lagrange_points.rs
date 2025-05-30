// lagrange_points.rs - Lagrange-Punkte und Trojaner basierend auf dem Artikel

use crate::constants::MIN_LAGRANGE_MASS_RATIO;
use crate::stellar_properties::*;
use crate::units::*;
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
    pub oscillation_period: crate::units::Time,
    /// Stabilität des Trojaners (0.0-1.0)
    pub stability: f64,
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
            oscillation_period: crate::units::Time::years(oscillation_period_years),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::Time;

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
