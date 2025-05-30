// system_hierarchy.rs - n-Körper Systemhierarchien basierend auf dem Artikel

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::cosmic_environment::GalacticDynamics;
use crate::lagrange_points::*;
use crate::orbital_mechanics::*;
use crate::stellar_properties::*;
use crate::units::*;
// Import kosmische Strukturen aus cosmic_environment
// Diese Zeilen sind korrekt und bleiben
pub use crate::cosmic_environment::{
    CosmicEpoch, CosmicRadiationEnvironment, ElementalAbundance, GalacticRegion,
};

// Import habitability Strukturen
// Diese Zeilen sind korrekt und bleiben
pub use crate::habitability::HabitabilityAssessment; // RadiationRisks wird hier nicht direkt verwendet, aber der Import schadet nicht

/// Typ des Sternsystems (aus original main.rs erweitert)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    Single(StellarProperties),
    Binary {
        primary: StellarProperties,
        secondary: StellarProperties,
        orbital_properties: BinaryOrbit,
    },
    Multiple {
        components: Vec<StellarProperties>,
        hierarchy: SystemHierarchy,
    },
}

/// Erweiterte Binärbahnparameter mit vollständigen orbitalen Elementen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOrbit {
    /// Vollständige orbitale Elemente
    pub orbital_elements: OrbitalElements,
    /// Barycenter Position (Bruchteil vom Primärstern)
    pub barycenter_position: f64,
    /// S-Type Stabilitätsgrenze für Planeten (primary, secondary)
    pub s_type_stability: (Distance, Distance),
    /// P-Type Stabilitätsgrenze für Planeten
    pub p_type_stability: Distance,
    /// Lagrange-System für das Binärsystem
    pub lagrange_system: Option<LagrangeSystem>,
    /// Gegenseitige Hill-Sphären
    pub mutual_hill_sphere: Distance,
}

/// Hierarchische Struktur für Mehrsternsysteme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHierarchy {
    /// Hierarchische Ebenen von innersten zu äußersten Orbits
    pub hierarchy_levels: Vec<HierarchyLevel>,
    /// Gesamtstabilität des Systems (0.0-1.0)
    pub stability_factor: f64,
    /// Charakteristische Zeitskala für chaotische Entwicklung
    pub chaos_timescale: Time,
}

/// Eine Ebene in der Systemhierarchie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyLevel {
    /// Orbitale Elemente für diese Ebene
    pub orbit: OrbitalElements,
    /// Komponenten auf dieser Ebene
    pub components: Vec<usize>, // Indizes in components-Array
    /// Massenverhältnis der Komponenten
    pub mass_ratio: f64,
    /// Stabilität dieser Ebene
    pub level_stability: f64,
}

/// Vollständiges Sternsystem mit kosmischer Umgebung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    /// Eindeutige Seed für Reproduzierbarkeit
    pub seed: u64,
    /// Kosmische Parameter
    pub cosmic_epoch: CosmicEpoch,
    pub galactic_distance: Distance, // Wird jetzt aus GalacticRegion abgeleitet
    pub galactic_region: GalacticRegion,
    pub radiation_environment: CosmicRadiationEnvironment,
    /// Sternsystem Konfiguration
    pub system_type: SystemType,
    /// Elementhäufigkeiten
    pub elemental_abundance: ElementalAbundance,
    /// Gesamte Bewohnbarkeitsbewertung
    pub habitability_assessment: HabitabilityAssessment,
    /// Einheitensystem für Berechnungen
    pub unit_system: UnitSystem,
    pub galactic_dynamics: GalacticDynamics,
}

impl BinaryOrbit {
    /// Erstellt neue Binärbahn mit vollständigen orbitalen Elementen
    pub fn new(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        separation: Distance,
        eccentricity: f64,
        inclination: f64,
        longitude_of_ascending_node: f64,
        argument_of_periapsis: f64,
    ) -> Self {
        let total_mass = Mass::kilograms(primary.mass.in_kg() + secondary.mass.in_kg());

        let orbital_elements = OrbitalElements::new(
            separation.clone(),
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
            0.0,
        );

        let barycenter_position = secondary.mass.in_kg() / total_mass.in_kg();

        let mu_primary = secondary.mass.in_kg() / total_mass.in_kg();
        let mu_secondary = primary.mass.in_kg() / total_mass.in_kg();

        let s_type_primary_limit =
            separation.value * (0.464 - 0.380 * mu_primary - 0.631 * eccentricity);
        let s_type_secondary_limit =
            separation.value * (0.464 - 0.380 * mu_secondary - 0.631 * eccentricity);

        let s_type_stability = (
            Distance::new(s_type_primary_limit.max(0.0), separation.system),
            Distance::new(s_type_secondary_limit.max(0.0), separation.system),
        );

        let mu_min = primary.mass.in_kg().min(secondary.mass.in_kg()) / total_mass.in_kg();
        let p_type_limit = separation.value * (1.60 + 4.12 * mu_min + 4.27 * eccentricity);
        let p_type_stability = Distance::new(p_type_limit, separation.system);

        let mut lagrange_system_opt = if primary.mass.in_kg() / secondary.mass.in_kg()
            >= MIN_LAGRANGE_MASS_RATIO
            || secondary.mass.in_kg() / primary.mass.in_kg() >= MIN_LAGRANGE_MASS_RATIO
        {
            Some(LagrangeSystem::new(primary, secondary, separation.clone()))
        } else {
            None
        };

        if let Some(ref mut lag_sys) = lagrange_system_opt {
            if lag_sys.l4_l5_stable {
                // Versuche, einen kleinen Test-Trojaner zu generieren
                let trojan_mass_val = primary.mass.value * 0.000001; // Sehr kleine Masse
                let trojan_mass = Mass::new(trojan_mass_val, primary.mass.system);
                match lag_sys.generate_trojan(4, trojan_mass, &primary.mass, &secondary.mass) {
                    Ok(trojan) => {
                        if lag_sys.add_trojan(trojan).is_ok() {
                            // Erfolgreich hinzugefügt (für Debugging)
                            // println!("Test trojan added to L4");
                        }
                    }
                    Err(_) => {} // Fehler ignorieren für diese Demo
                }
            }
        }
        let smaller_mass = if primary.mass.in_kg() < secondary.mass.in_kg() {
            &primary.mass
        } else {
            &secondary.mass
        };
        let mutual_hill_sphere = orbital_elements.hill_radius(smaller_mass, &total_mass);

        Self {
            orbital_elements,
            barycenter_position,
            s_type_stability,
            p_type_stability,
            lagrange_system: lagrange_system_opt,
            mutual_hill_sphere,
        }
    }

    pub fn distance_range(&self) -> (Distance, Distance) {
        let periapsis = self.orbital_elements.periapsis();
        let apoapsis = self.orbital_elements.apoapsis();
        (periapsis, apoapsis)
    }

    pub fn s_type_primary_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() < self.s_type_stability.0.in_meters()
    }

    pub fn s_type_secondary_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() < self.s_type_stability.1.in_meters()
    }

    pub fn p_type_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() > self.p_type_stability.in_meters()
    }

    pub fn combined_habitable_zone(
        &self,
        primary: &StellarProperties,
        secondary: &StellarProperties,
    ) -> crate::stellar_properties::HabitableZone {
        let combined_luminosity = primary.luminosity + secondary.luminosity;
        let sqrt_l_combined = combined_luminosity.sqrt();

        crate::stellar_properties::HabitableZone {
            inner_edge: Distance::new(0.95 * sqrt_l_combined, self.orbital_elements.unit_system),
            outer_edge: Distance::new(1.37 * sqrt_l_combined, self.orbital_elements.unit_system),
            optimistic_inner: Distance::new(
                0.84 * sqrt_l_combined,
                self.orbital_elements.unit_system,
            ),
            optimistic_outer: Distance::new(
                1.67 * sqrt_l_combined,
                self.orbital_elements.unit_system,
            ),
        }
    }
}

impl SystemHierarchy {
    pub fn new(components: &[StellarProperties]) -> Self {
        let mut hierarchy_levels = Vec::new();
        let n = components.len();

        if n < 3 {
            return Self {
                hierarchy_levels,
                stability_factor: 1.0,
                chaos_timescale: Time::years(1e12),
            };
        }

        let _total_mass: f64 = components.iter().map(|c| c.mass.in_kg()).sum();
        let mut current_separation = Distance::au(1.0);

        for i in 0..(n - 1) {
            let mass_ratio = components[i].mass.in_kg() / components[i + 1].mass.in_kg();
            let level_stability =
                Self::estimate_level_stability(mass_ratio, current_separation.in_au());

            let level = HierarchyLevel {
                orbit: OrbitalElements::new(current_separation.clone(), 0.1, 0.0, 0.0, 0.0, 0.0),
                components: vec![i, i + 1],
                mass_ratio,
                level_stability,
            };

            hierarchy_levels.push(level);
            current_separation =
                Distance::new(current_separation.value * 3.0, current_separation.system);
        }

        let stability_factor = hierarchy_levels
            .iter()
            .map(|level| level.level_stability)
            .fold(1.0, |acc, s| acc * s);

        let shortest_period_years = hierarchy_levels
            .first()
            .map(|level| {
                let total_mass_kg = components
                    .iter()
                    .take(2)
                    .map(|c| c.mass.in_kg())
                    .sum::<f64>();
                let total_mass_solar = Mass::kilograms(total_mass_kg);
                level.orbit.orbital_period(&total_mass_solar).in_years()
            })
            .unwrap_or(1.0);

        let chaos_timescale = Time::years(shortest_period_years * 1e6 * stability_factor);

        Self {
            hierarchy_levels,
            stability_factor,
            chaos_timescale,
        }
    }

    fn estimate_level_stability(mass_ratio: f64, separation_au: f64) -> f64 {
        let mass_factor = if mass_ratio > 10.0 || mass_ratio < 0.1 {
            0.9
        } else {
            0.5
        };

        let separation_factor = if separation_au > 10.0 {
            0.9
        } else if separation_au > 1.0 {
            0.7
        } else {
            0.3
        };

        mass_factor * separation_factor
    }

    pub fn is_long_term_stable(&self) -> bool {
        self.stability_factor > 0.7 && self.chaos_timescale.in_years() > 1e9
    }

    pub fn dynamical_timescale(&self) -> Time {
        self.hierarchy_levels
            .first()
            .map(|level| {
                let a_au = level.orbit.semimajor_axis.in_au();
                Time::years(a_au.powf(1.5))
            })
            .unwrap_or(Time::years(1.0))
    }
}

impl StarSystem {
    pub fn generate_from_seed(seed: u64) -> Self {
        Self::generate_from_seed_with_units(seed, UnitSystem::Astronomical)
    }

    pub fn generate_from_seed_with_units(seed: u64, unit_system: UnitSystem) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let age_universe_gyr = rng.gen_range(3.0..13.8);
        let cosmic_epoch = CosmicEpoch::from_age(age_universe_gyr);

        // System Type wird vor galaktischer Umgebung generiert,
        // da manche Sterneigenschaften (z.B. Alter) von der kosmischen Epoche abhängen können.
        let system_type = Self::generate_system_type(&mut rng, &cosmic_epoch, unit_system);

        let galactic_region = GalacticRegion::generate_random(&mut rng, unit_system);
        let galactic_distance = galactic_region.distance_from_center().clone();

        let radiation_environment = CosmicRadiationEnvironment::from_region_and_epoch(
            &galactic_region,
            &cosmic_epoch,
            &mut rng,
        );

        let elemental_abundance = ElementalAbundance::from_metallicity_and_epoch(
            cosmic_epoch.epoch_metallicity,
            &cosmic_epoch,
        );

        // HIER WIRD GalacticDynamics BERECHNET UND GESPEICHERT
        let galactic_dynamics = GalacticDynamics::calculate_for_position(
            &galactic_region,
            cosmic_epoch.age_universe, // Alter des Universums in Gyr, wie von der Funktion erwartet
            &mut rng,
        );
        // Das Ergebnis von calculate_for_position muss dem Feld in StarSystem zugewiesen werden.

        let target_distances: Vec<Distance> = Vec::new();
        let habitability_assessment = HabitabilityAssessment::comprehensive_analysis(
            &system_type,
            &radiation_environment,
            &target_distances,
            // GalacticDynamics wird hier nicht direkt übergeben,
            // aber es ist Teil des StarSystem-Kontexts, der indirekt relevant sein könnte.
        );

        StarSystem {
            seed,
            cosmic_epoch,
            galactic_distance,
            galactic_region,
            radiation_environment,
            system_type,
            elemental_abundance,
            habitability_assessment,
            unit_system,
            galactic_dynamics, // <<-- HIER ZUWEISEN
        }
    }

    fn generate_system_type(
        rng: &mut ChaCha8Rng,
        cosmic_epoch: &CosmicEpoch,
        unit_system: UnitSystem,
    ) -> SystemType {
        let primary_mass_solar = Self::generate_stellar_mass(rng);
        let primary_mass = Mass::solar_masses(primary_mass_solar).to_system(unit_system);

        let multiplicity_probability = match primary_mass_solar {
            m if m > 15.0 => 0.8,
            m if m > 1.5 => 0.6,
            m if m > 0.5 => 0.4,
            _ => 0.25,
        };

        // Alter des Sterns sollte geringer oder gleich dem Alter des Universums sein
        // Und nicht zu jung, um Hauptreihensterne zu haben (oder Pre-MS, wenn gewünscht)
        // Wir nehmen hier einen Bruchteil des Universumsalters, oder die Logik in StellarProperties::new kümmert sich darum.
        // cosmic_epoch.age_universe ist in Gyr. StellarProperties erwartet Time.
        let star_age_gyr = rng.gen_range(0.1..cosmic_epoch.age_universe.min(10.0)); // Sterne können jünger sein als das Universum
        let age = Time::years(star_age_gyr * 1e9).to_system(unit_system);

        if rng.r#gen::<f64>() < multiplicity_probability {
            let secondary_mass_solar = Self::generate_secondary_mass(rng, primary_mass_solar);
            let secondary_mass = Mass::solar_masses(secondary_mass_solar).to_system(unit_system);

            let primary = StellarProperties::new(
                primary_mass.clone(),
                age.clone(),
                cosmic_epoch.epoch_metallicity,
            ); // unit_system hinzugefügt
            let secondary = StellarProperties::new(
                secondary_mass.clone(),
                age.clone(),
                cosmic_epoch.epoch_metallicity,
            ); // unit_system hinzugefügt

            let separation_au =
                Self::generate_binary_separation(rng, primary_mass_solar, secondary_mass_solar);
            let separation = Distance::au(separation_au).to_system(unit_system);
            let eccentricity = rng.gen_range(0.0..0.8);
            let inclination = rng.gen_range(0.0..180.0);
            let longitude_of_ascending_node = rng.gen_range(0.0..360.0);
            let argument_of_periapsis = rng.gen_range(0.0..360.0);

            let orbital_properties = BinaryOrbit::new(
                &primary,
                &secondary,
                separation,
                eccentricity,
                inclination,
                longitude_of_ascending_node,
                argument_of_periapsis,
            );

            if rng.r#gen::<f64>() < 0.1 && primary_mass_solar > 2.0 {
                let tertiary_mass_solar = Self::generate_secondary_mass(rng, secondary_mass_solar);
                let tertiary_mass = Mass::solar_masses(tertiary_mass_solar) // .clone() nicht nötig für f64
                    .to_system(unit_system);
                let tertiary = StellarProperties::new(
                    tertiary_mass,
                    age.clone(),
                    cosmic_epoch.epoch_metallicity,
                );

                let components = vec![primary, secondary, tertiary];
                let hierarchy = SystemHierarchy::new(&components);

                SystemType::Multiple {
                    components,
                    hierarchy,
                }
            } else {
                SystemType::Binary {
                    primary,
                    secondary,
                    orbital_properties,
                }
            }
        } else {
            let star =
                StellarProperties::new(primary_mass, age.clone(), cosmic_epoch.epoch_metallicity); // unit_system hinzugefügt
            SystemType::Single(star)
        }
    }

    fn generate_stellar_mass(rng: &mut ChaCha8Rng) -> f64 {
        let r: f64 = rng.r#gen();
        match r {
            x if x < 0.6 => rng.gen_range(0.1..0.5),
            x if x < 0.85 => rng.gen_range(0.5..1.0),
            x if x < 0.95 => rng.gen_range(1.0..2.0),
            x if x < 0.99 => rng.gen_range(2.0..10.0),
            _ => rng.gen_range(10.0..50.0),
        }
    }

    fn generate_secondary_mass(rng: &mut ChaCha8Rng, primary_mass: f64) -> f64 {
        let mass_ratio = rng.gen_range(0.1..1.0);
        (primary_mass * mass_ratio).max(0.08)
    }

    fn generate_binary_separation(
        rng: &mut ChaCha8Rng,
        primary_mass: f64,
        secondary_mass: f64,
    ) -> f64 {
        let log_separation = rng.gen_range(0.0..4.0);
        let separation_au = 10.0_f64.powf(log_separation);
        let mass_factor = (primary_mass + secondary_mass) / 2.0;
        separation_au * mass_factor.sqrt()
    }

    pub fn to_ron_string(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
    }

    pub fn from_ron_string(s: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // StellarProperties::sun_like() und StellarProperties::new benötigen UnitSystem
    // Wir definieren ein Default UnitSystem für Tests oder übergeben es explizit.
    const TEST_UNIT_SYSTEM: UnitSystem = UnitSystem::Astronomical;

    #[test]
    fn test_binary_orbit_stability() {
        let sun_mass = Mass::solar_masses(1.0).to_system(TEST_UNIT_SYSTEM);
        let sun = StellarProperties::new(
            sun_mass,
            Time::years(4.6e9).to_system(TEST_UNIT_SYSTEM),
            0.0,
        );
        let jupiter_mass = Mass::solar_masses(0.000954).to_system(TEST_UNIT_SYSTEM);
        let jupiter = StellarProperties::new(
            jupiter_mass,
            Time::years(4.6e9).to_system(TEST_UNIT_SYSTEM),
            0.0,
        );

        let binary_orbit = BinaryOrbit::new(
            &sun,
            &jupiter,
            Distance::au(5.2).to_system(TEST_UNIT_SYSTEM),
            0.048,
            0.0,
            0.0,
            0.0,
        );

        assert!(binary_orbit.s_type_stability.0.value > 0.0); // .in_au() entfernt für direkten Wertvergleich
        assert!(binary_orbit.s_type_stability.1.value > 0.0);

        assert!(
            binary_orbit.p_type_stability.value
                > binary_orbit.orbital_elements.semimajor_axis.value
        );

        assert!(binary_orbit.lagrange_system.is_some());
    }

    #[test]
    fn test_hierarchy_stability() {
        let age = Time::years(4.6e9).to_system(TEST_UNIT_SYSTEM);
        let star1_mass = Mass::solar_masses(1.0).to_system(TEST_UNIT_SYSTEM);
        let star1 = StellarProperties::new(star1_mass, age.clone(), 0.0);

        let star2_mass = Mass::solar_masses(0.8).to_system(TEST_UNIT_SYSTEM);
        let star2 = StellarProperties::new(star2_mass, age.clone(), 0.0);

        let star3_mass = Mass::solar_masses(0.3).to_system(TEST_UNIT_SYSTEM);
        let star3 = StellarProperties::new(star3_mass, age.clone(), 0.0);

        let components = vec![star1, star2, star3];
        let hierarchy = SystemHierarchy::new(&components);

        assert_eq!(hierarchy.hierarchy_levels.len(), 2);
        assert!(hierarchy.stability_factor > 0.0);
        assert!(hierarchy.chaos_timescale.in_years() > 0.0);
    }

    #[test]
    fn test_system_generation() {
        for seed in [42, 1337, 9999] {
            let system = StarSystem::generate_from_seed_with_units(seed, TEST_UNIT_SYSTEM);

            assert!(system.habitability_assessment.overall_habitability >= 0.0);
            assert!(system.habitability_assessment.overall_habitability <= 1.0);

            match &system.system_type {
                SystemType::Single(star) => {
                    assert!(star.mass.value > 0.0); // .in_solar_masses() entfernt
                    assert!(star.age.value > 0.0); // .in_years() entfernt
                }
                SystemType::Binary {
                    primary,
                    secondary,
                    orbital_properties,
                } => {
                    assert!(primary.mass.value > 0.0);
                    assert!(secondary.mass.value > 0.0);
                    assert!(orbital_properties.orbital_elements.semimajor_axis.value > 0.0);
                }
                SystemType::Multiple {
                    components,
                    hierarchy,
                } => {
                    assert!(components.len() >= 3);
                    assert!(!hierarchy.hierarchy_levels.is_empty());
                }
            }
        }
    }

    #[test]
    fn test_unit_system_consistency() {
        let system_au = StarSystem::generate_from_seed_with_units(42, UnitSystem::Astronomical);
        let system_si = StarSystem::generate_from_seed_with_units(42, UnitSystem::SI);

        assert_eq!(system_au.unit_system, UnitSystem::Astronomical);
        assert_eq!(system_si.unit_system, UnitSystem::SI);

        assert_eq!(system_au.seed, system_si.seed);
        // Man könnte hier noch prüfen, ob die Werte physikalisch äquivalent sind,
        // z.B. system_au.galactic_distance.in_meters() sollte nahe an system_si.galactic_distance.value sein
    }
}
