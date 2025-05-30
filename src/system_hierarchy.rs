// system_hierarchy.rs - n-Körper Systemhierarchien basierend auf dem Artikel

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::cosmic_environment::GalacticDynamics;
use crate::lagrange_points::*;
use crate::lagrange_points::{MutualTrojanSystem, OscillationPattern, TrojanDynamics};

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

/// Stabilitätsanalyse für Sternsysteme über Millionen-Jahre-Zeiträume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStability {
    /// Charakteristische Stabilitäts-Zeitskala (typisch 1-10 Myr)
    pub stability_timescale: Time,
    /// Wahrscheinlichkeit für Sternauswurf in 1 Myr (0.0-1.0)
    pub ejection_probability: f64,
    /// Kollisionsrisiko in 1 Myr (0.0-1.0)
    pub collision_risk: f64,
    /// Hill-Sphären überlappen sich (instabil wenn true)
    pub hill_sphere_overlap: bool,
    /// Gesamtstabilitäts-Score (0.0-1.0, höher = stabiler)
    pub overall_stability_score: f64,
    /// Detaillierte Risikofaktoren
    pub risk_factors: Vec<StabilityRiskFactor>,
    pub trojan_analysis: Option<TrojanStabilityAnalysis>,
}

/// Spezifische Risikofaktoren für Systemstabilität
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityRiskFactor {
    pub name: String,
    pub severity: f64,    // 0.0-1.0
    pub probability: f64, // 0.0-1.0
    pub description: String,
}

/// Detaillierte Analyse der Hill-Sphären-Dynamik
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HillSphereAnalysis {
    /// Hill-Radius für jede Komponente
    pub hill_radii: Vec<Distance>,
    /// Minimum-Abstand zwischen Komponenten
    pub minimum_separations: Vec<Distance>,
    /// Stabilität basierend auf Hill-Kriterium
    pub hill_stability_ratios: Vec<f64>,
}

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
impl SystemStability {
    // ERSETZE die bestehende analyze_system Methode:
    pub fn analyze_system(system_type: &SystemType) -> Self {
        Self::analyze_system_enhanced(system_type)
    }

    // NEU hinzufügen:
    pub fn analyze_system_enhanced(system_type: &SystemType) -> Self {
        match system_type {
            SystemType::Single(_) => Self::single_star_stability(),
            SystemType::Binary {
                primary,
                secondary,
                orbital_properties,
            } => Self::binary_stability_with_trojans(primary, secondary, orbital_properties),
            SystemType::Multiple {
                components,
                hierarchy,
            } => {
                let mut base = Self::multiple_star_stability(components, hierarchy);
                base.trojan_analysis = None; // Trojaner nur in Binärsystemen
                base
            }
        }
    }

    /// Stabilität für Einzelsternsysteme (immer stabil)
    fn single_star_stability() -> Self {
        Self {
            stability_timescale: Time::years(1e10), // Stellar evolution timescale
            ejection_probability: 0.0,
            collision_risk: 0.0,
            hill_sphere_overlap: false,
            overall_stability_score: 1.0,
            risk_factors: vec![],
            trojan_analysis: None,
        }
    }

    /// Stabilitätsanalyse für Binärsysteme
    fn binary_stability(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        orbital_properties: &BinaryOrbit,
    ) -> Self {
        let mass_ratio = primary.mass.in_kg() / secondary.mass.in_kg();
        let separation = orbital_properties.orbital_elements.semimajor_axis.in_au();
        let eccentricity = orbital_properties.orbital_elements.eccentricity;

        // Einfache Heuristiken basierend auf dem Artikel
        let mut risk_factors = Vec::new();
        let mut overall_score: f64 = 1.0;

        // Risiko durch hohe Exzentrizität
        if eccentricity > 0.7 {
            let risk = StabilityRiskFactor {
                name: "High eccentricity".to_string(),
                severity: eccentricity,
                probability: 0.3,
                description: format!(
                    "Eccentricity of {:.2} may cause close approaches",
                    eccentricity
                ),
            };
            overall_score *= 0.8;
            risk_factors.push(risk);
        }

        // Risiko durch geringe Separation
        if separation < 0.1 {
            let risk = StabilityRiskFactor {
                name: "Close binary".to_string(),
                severity: 0.1 / separation.max(0.01),
                probability: 0.5,
                description: format!(
                    "Separation of {:.3} AU may lead to mass transfer",
                    separation
                ),
            };
            overall_score *= 0.6;
            risk_factors.push(risk);
        }

        // Risiko durch extreme Massenverhältnisse
        if mass_ratio > 10.0 || mass_ratio < 0.1 {
            let risk = StabilityRiskFactor {
                name: "Extreme mass ratio".to_string(),
                severity: if mass_ratio > 10.0 {
                    mass_ratio / 50.0
                } else {
                    (0.1 / mass_ratio) / 10.0
                },
                probability: 0.2,
                description: format!(
                    "Mass ratio of {:.1}:1 may cause orbital evolution",
                    mass_ratio
                ),
            };
            overall_score *= 0.9;
            risk_factors.push(risk);
        }

        // Stabilität über Millionen Jahre (meist sehr hoch für Binärsysteme)
        let stability_myr = if separation > 0.05 && eccentricity < 0.9 {
            Time::years(1e8) // 100 Myr für normale Binärsysteme
        } else {
            Time::years(1e6) // 1 Myr für extreme Binärsysteme
        };

        Self {
            stability_timescale: stability_myr,
            ejection_probability: 0.01, // Sehr niedrig für Binärsysteme
            collision_risk: if separation < 0.01 { 0.1 } else { 0.001 },
            hill_sphere_overlap: false, // Binärsysteme haben definitionsgemäß keine Überlappung
            overall_stability_score: overall_score.max(0.1),
            risk_factors,
            trojan_analysis: None,
        }
    }
    fn binary_stability_with_trojans(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        orbital_properties: &BinaryOrbit,
    ) -> Self {
        // Basis-Binär-Stabilität
        let mut base_stability = Self::binary_stability(primary, secondary, orbital_properties);

        // Trojaner-Analyse hinzufügen
        let trojan_analysis = Self::analyze_binary_trojans(primary, secondary, orbital_properties);

        // Gesamtstabilität anpassen
        if let Some(ref trojan_data) = trojan_analysis {
            if trojan_data.stable_trojans_count > 0 {
                base_stability.overall_stability_score *= 1.05;
            }
            if trojan_data.unstable_trojans_count > 0 {
                base_stability.overall_stability_score *= 0.95;
            }
        }

        base_stability.trojan_analysis = trojan_analysis;
        base_stability
    }

    fn analyze_binary_trojans(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        orbital_properties: &BinaryOrbit,
    ) -> Option<TrojanStabilityAnalysis> {
        if let Some(ref lagrange_system) = orbital_properties.lagrange_system {
            let mut trojan_risks = Vec::new();
            let mut stable_count = 0;
            let mut unstable_count = 0;
            let mut total_stability = 0.0;

            for trojan in &lagrange_system.trojans {
                let dynamics = trojan.calculate_libration_dynamics(
                    &primary.mass,
                    &secondary.mass,
                    &orbital_properties.orbital_elements.semimajor_axis,
                );

                if dynamics.long_term_stability > 0.7 {
                    stable_count += 1;
                } else {
                    unstable_count += 1;
                    trojan_risks.push(StabilityRiskFactor {
                        name: format!("Unstable trojan at L{}", trojan.lagrange_point),
                        severity: 1.0 - dynamics.long_term_stability,
                        probability: 0.8,
                        description: format!(
                            "Trojan instability: {:.2}",
                            dynamics.long_term_stability
                        ),
                    });
                }
                total_stability += dynamics.long_term_stability;
            }

            let lagrange_status = LagrangePointsStatus {
                l1_stable: false,
                l2_stable: false,
                l3_stable: false,
                l4_stable: lagrange_system.l4_l5_stable,
                l5_stable: lagrange_system.l4_l5_stable,
                l4_trojans: lagrange_system
                    .trojans
                    .iter()
                    .filter(|t| t.lagrange_point == 4)
                    .cloned()
                    .collect(),
                l5_trojans: lagrange_system
                    .trojans
                    .iter()
                    .filter(|t| t.lagrange_point == 5)
                    .cloned()
                    .collect(),
            };

            let average_stability = if lagrange_system.trojans.is_empty() {
                0.0
            } else {
                total_stability / lagrange_system.trojans.len() as f64
            };

            Some(TrojanStabilityAnalysis {
                stable_trojans_count: stable_count,
                unstable_trojans_count: unstable_count,
                mutual_systems: Vec::new(),
                average_trojan_stability: average_stability,
                trojan_risks,
                lagrange_points_status: lagrange_status,
            })
        } else {
            None
        }
    }

    /// Stabilitätsanalyse für Mehrsternsysteme (komplexer)
    fn multiple_star_stability(
        components: &[StellarProperties],
        hierarchy: &SystemHierarchy,
    ) -> Self {
        let n_bodies = components.len();
        let mut risk_factors = Vec::new();
        let mut overall_score: f64 = hierarchy.stability_factor;

        // Hill-Sphären-Analyse
        let hill_analysis = Self::analyze_hill_spheres(components, hierarchy);
        let hill_overlap = hill_analysis
            .hill_stability_ratios
            .iter()
            .any(|&ratio| ratio < 2.5); // Kritisches Verhältnis nach dem Artikel

        if hill_overlap {
            let risk = StabilityRiskFactor {
                name: "Hill sphere overlap".to_string(),
                severity: 0.9,
                probability: 0.8,
                description: "Hill spheres overlap, indicating potential instability".to_string(),
            };
            overall_score *= 0.3;
            risk_factors.push(risk);
        }

        // Risiko durch Anzahl der Körper (n-body chaos)
        if n_bodies > 3 {
            let chaos_factor = (n_bodies as f64 - 3.0) * 0.2;
            let risk = StabilityRiskFactor {
                name: format!("{}-body chaos", n_bodies),
                severity: chaos_factor.min(0.9),
                probability: 0.6,
                description: format!("Systems with {} bodies are inherently chaotic", n_bodies),
            };
            overall_score *= (1.0 - chaos_factor * 0.3).max(0.2);
            risk_factors.push(risk);
        }

        // Berechnung der charakteristischen Zeitskalen
        let shortest_period = hierarchy.dynamical_timescale().in_years();
        let n_body_factor = (n_bodies as f64).ln(); // Logarithmische Skalierung

        // Stabilität nimmt mit Komplexität ab
        let stability_years = (1e6 * hierarchy.stability_factor / n_body_factor).max(1e4);

        // Auswurfwahrscheinlichkeit steigt mit n-body Komplexität
        let ejection_prob = match n_bodies {
            3 => 0.05,
            4 => 0.15,
            5 => 0.3,
            _ => 0.5,
        } * (1.0 - hierarchy.stability_factor);

        // Kollisionsrisiko basierend auf Hill-Analyse
        let collision_risk = if hill_overlap { 0.2 } else { 0.01 };

        Self {
            stability_timescale: Time::years(stability_years),
            ejection_probability: ejection_prob.min(0.9),
            collision_risk,
            hill_sphere_overlap: hill_overlap,
            overall_stability_score: overall_score.max(0.05),
            risk_factors,
            trojan_analysis: None,
        }
    }

    /// Analysiert Hill-Sphären für Mehrsternsysteme
    fn analyze_hill_spheres(
        components: &[StellarProperties],
        hierarchy: &SystemHierarchy,
    ) -> HillSphereAnalysis {
        let mut hill_radii = Vec::new();
        let mut minimum_separations = Vec::new();
        let mut stability_ratios = Vec::new();

        for (i, level) in hierarchy.hierarchy_levels.iter().enumerate() {
            if level.components.len() >= 2 {
                let comp1_idx = level.components[0];
                let comp2_idx = level.components[1];

                if comp1_idx < components.len() && comp2_idx < components.len() {
                    let comp1 = &components[comp1_idx];
                    let comp2 = &components[comp2_idx];

                    // Hill radius für die weniger massive Komponente
                    let smaller_mass = if comp1.mass.in_kg() < comp2.mass.in_kg() {
                        &comp1.mass
                    } else {
                        &comp2.mass
                    };
                    let total_mass = Mass::kilograms(comp1.mass.in_kg() + comp2.mass.in_kg());

                    let hill_radius = level.orbit.hill_radius(smaller_mass, &total_mass);
                    let separation = level.orbit.semimajor_axis.clone();

                    // Stabilitätsverhältnis: Separation / Hill-Radius
                    let stability_ratio = separation.in_meters() / hill_radius.in_meters();

                    hill_radii.push(hill_radius);
                    minimum_separations.push(separation);
                    stability_ratios.push(stability_ratio);
                }
            }
        }

        HillSphereAnalysis {
            hill_radii,
            minimum_separations,
            hill_stability_ratios: stability_ratios,
        }
    }

    /// Gibt eine menschenlesbare Zusammenfassung der Stabilität zurück
    pub fn stability_summary(&self) -> String {
        let stability_class = match self.overall_stability_score {
            s if s > 0.8 => "Highly Stable",
            s if s > 0.6 => "Moderately Stable",
            s if s > 0.4 => "Marginally Stable",
            s if s > 0.2 => "Unstable",
            _ => "Highly Unstable",
        };

        format!(
            "{} (Score: {:.2}) - Stable for ~{:.1} Myr",
            stability_class,
            self.overall_stability_score,
            self.stability_timescale.in_years() / 1e6
        )
    }

    /// Prüft ob das System über 1 Myr stabil ist
    pub fn is_million_year_stable(&self) -> bool {
        self.stability_timescale.in_years() >= 1e6 && self.overall_stability_score > 0.5
    }
}

// Erweitere SystemHierarchy um Stabilitäts-Methoden
impl SystemHierarchy {
    /// Erweiterte Stabilitätsbewertung über Millionen-Jahre-Zeitrahmen
    pub fn assess_million_year_stability(
        &self,
        components: &[StellarProperties],
    ) -> SystemStability {
        SystemStability::multiple_star_stability(components, self)
    }

    /// Berechnet charakteristische Zeitskala für chaotische Evolution
    pub fn chaos_timescale_estimate(&self) -> Time {
        let base_timescale = self.dynamical_timescale().in_years();
        let n_levels = self.hierarchy_levels.len() as f64;

        // Chaotische Zeitskala skaliert exponentiell mit Komplexität
        let chaos_years = base_timescale * (1e3 * self.stability_factor / n_levels.ln());
        Time::years(chaos_years.max(1e4)) // Minimum 10,000 Jahre
    }

    /// Verbesserte Langzeit-Stabilitätsprüfung
    pub fn is_long_term_stable_enhanced(&self, components: &[StellarProperties]) -> bool {
        let stability = self.assess_million_year_stability(components);
        stability.is_million_year_stable() && !stability.hill_sphere_overlap
    }
}

// Erweitere StarSystem um Stabilitätsanalyse
impl StarSystem {
    /// Fügt Stabilitätsanalyse zum Sternsystem hinzu
    pub fn calculate_system_stability(&self) -> SystemStability {
        SystemStability::analyze_system(&self.system_type)
    }

    /// Erweiterte System-Generierung mit Stabilitätsprüfung
    pub fn generate_stable_system(seed: u64, max_attempts: u32) -> Option<Self> {
        for _attempt in 0..max_attempts {
            let system = Self::generate_from_seed(seed + _attempt as u64);
            let stability = system.calculate_system_stability();

            if stability.is_million_year_stable() {
                return Some(system);
            }
        }
        None // Kein stabiles System nach max_attempts gefunden
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // StellarProperties::sun_like() und StellarProperties::new benötigen UnitSystem
    // Wir definieren ein Default UnitSystem für Tests oder übergeben es explizit.
    const TEST_UNIT_SYSTEM: UnitSystem = UnitSystem::Astronomical;
    use super::*;

    #[test]
    fn test_binary_stability() {
        let sun = StellarProperties::sun_like();
        let alpha_centauri_b =
            StellarProperties::new(Mass::solar_masses(0.9), Time::years(4.6e9), 0.0);

        let binary_orbit = BinaryOrbit::new(
            &sun,
            &alpha_centauri_b,
            Distance::au(23.0), // Alpha Centauri A-B separation
            0.52,               // Alpha Centauri A-B eccentricity
            0.0,
            0.0,
            0.0,
        );

        let system_type = SystemType::Binary {
            primary: sun,
            secondary: alpha_centauri_b,
            orbital_properties: binary_orbit,
        };

        let stability = SystemStability::analyze_system(&system_type);

        assert!(stability.is_million_year_stable());
        assert!(stability.overall_stability_score > 0.7);
        assert!(!stability.hill_sphere_overlap);
        println!("Binary stability: {}", stability.stability_summary());
    }

    #[test]
    fn test_multiple_star_stability() {
        let components = vec![
            StellarProperties::sun_like(),
            StellarProperties::new(Mass::solar_masses(0.8), Time::years(6e9), 0.0),
            StellarProperties::new(Mass::solar_masses(0.3), Time::years(6e9), 0.0),
        ];

        let hierarchy = SystemHierarchy::new(&components);
        let system_type = SystemType::Multiple {
            components: components.clone(),
            hierarchy: hierarchy.clone(),
        };

        let stability = SystemStability::analyze_system(&system_type);

        // 3-Körper-System sollte weniger stabil sein als Binärsystem
        assert!(stability.overall_stability_score < 0.9);
        assert!(stability.ejection_probability > 0.01);

        println!("Triple star stability: {}", stability.stability_summary());
        println!("Risk factors: {:?}", stability.risk_factors.len());
    }

    #[test]
    fn test_hill_sphere_analysis() {
        let components = vec![
            StellarProperties::new(Mass::solar_masses(1.0), Time::years(5e9), 0.0),
            StellarProperties::new(Mass::solar_masses(1.0), Time::years(5e9), 0.0), // Equal masses
        ];

        let hierarchy = SystemHierarchy::new(&components);
        let hill_analysis = SystemStability::analyze_hill_spheres(&components, &hierarchy);

        assert!(!hill_analysis.hill_radii.is_empty());
        assert!(!hill_analysis.hill_stability_ratios.is_empty());

        // Equal mass systems should have larger Hill spheres
        for ratio in &hill_analysis.hill_stability_ratios {
            println!("Hill stability ratio: {:.2}", ratio);
        }
    }

    #[test]
    fn test_stable_system_generation() {
        // Test der erweiterten Generierung mit Stabilitätsprüfung
        let stable_system = StarSystem::generate_stable_system(42, 10);

        assert!(stable_system.is_some());
        if let Some(system) = stable_system {
            let stability = system.calculate_system_stability();
            assert!(stability.is_million_year_stable());
            println!("Generated stable system: {}", stability.stability_summary());
        }
    }

    #[test]
    fn test_single_star_always_stable() {
        let sun = StellarProperties::sun_like();
        let system_type = SystemType::Single(sun);

        let stability = SystemStability::analyze_system(&system_type);

        assert_eq!(stability.overall_stability_score, 1.0);
        assert_eq!(stability.ejection_probability, 0.0);
        assert_eq!(stability.collision_risk, 0.0);
        assert!(stability.is_million_year_stable());
    }
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
