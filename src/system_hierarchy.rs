// system_hierarchy.rs - n-Körper Systemhierarchien basierend auf dem Artikel

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::lagrange_points::LagrangeSystem;
use crate::orbital_mechanics::{OrbitalClassification, OrbitalElements};
use crate::stellar_properties::StellarProperties;
use crate::units::{Distance, Mass, Time, UnitConversion, UnitSystem, Velocity};

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
    pub galactic_distance: Distance,
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
}

// Kosmische Strukturen (aus original main.rs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicEpoch {
    pub age_universe: f64,
    pub era: String,
    pub star_formation_rate: f64,
    pub epoch_metallicity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GalacticRegion {
    Core,
    InnerBulge,
    HabitableZone,
    OuterDisk,
    Halo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicRadiationEnvironment {
    pub agn_risk: f64,
    pub supernova_frequency: f64,
    pub grb_risk: f64,
    pub stellar_encounter_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalAbundance {
    pub hydrogen: f64,
    pub helium: f64,
    pub lithium: f64,
    pub carbon: f64,
    pub nitrogen: f64,
    pub oxygen: f64,
    pub heavy_metals: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitabilityAssessment {
    pub overall_habitability: f64,
    pub system_habitable_zone: crate::stellar_properties::HabitableZone,
    pub radiation_risks: RadiationRisks,
    pub habitability_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationRisks {
    pub pre_main_sequence_hazard: f64,
    pub stellar_flare_risk: f64,
    pub galactic_radiation_risk: f64,
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

        // Orbitale Elemente für das Binärsystem
        let orbital_elements = OrbitalElements::new(
            separation.clone(),
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
            0.0, // True anomaly at epoch
        );

        // Barycenter Position
        let barycenter_position = secondary.mass.in_kg() / total_mass.in_kg();

        // Stabilitätsgrenzen nach Artikel-Formeln
        let mu_primary = secondary.mass.in_kg() / total_mass.in_kg();
        let mu_secondary = primary.mass.in_kg() / total_mass.in_kg();

        // S-Type Stabilität (Planeten um einzelne Sterne)
        let s_type_primary_limit =
            separation.value * (0.464 - 0.380 * mu_primary - 0.631 * eccentricity);
        let s_type_secondary_limit =
            separation.value * (0.464 - 0.380 * mu_secondary - 0.631 * eccentricity);

        let s_type_stability = (
            Distance::new(s_type_primary_limit.max(0.0), separation.system),
            Distance::new(s_type_secondary_limit.max(0.0), separation.system),
        );

        // P-Type Stabilität (zirkumbinäre Planeten)
        let mu_min = primary.mass.in_kg().min(secondary.mass.in_kg()) / total_mass.in_kg();
        let p_type_limit = separation.value * (1.60 + 4.12 * mu_min + 4.27 * eccentricity);
        let p_type_stability = Distance::new(p_type_limit, separation.system);

        // Lagrange-System (falls stabil)
        let lagrange_system = if primary.mass.in_kg() / secondary.mass.in_kg()
            >= MIN_LAGRANGE_MASS_RATIO
            || secondary.mass.in_kg() / primary.mass.in_kg() >= MIN_LAGRANGE_MASS_RATIO
        {
            Some(LagrangeSystem::new(primary, secondary, separation.clone()))
        } else {
            None
        };

        // Gegenseitige Hill-Sphäre (kleinere der beiden)
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
            lagrange_system,
            mutual_hill_sphere,
        }
    }

    /// Berechnet minimale und maximale Entfernung zwischen Sternen
    pub fn distance_range(&self) -> (Distance, Distance) {
        let periapsis = self.orbital_elements.periapsis();
        let apoapsis = self.orbital_elements.apoapsis();
        (periapsis, apoapsis)
    }

    /// Prüft ob S-Type Planeten um den Primärstern möglich sind
    pub fn s_type_primary_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() < self.s_type_stability.0.in_meters()
    }

    /// Prüft ob S-Type Planeten um den Sekundärstern möglich sind
    pub fn s_type_secondary_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() < self.s_type_stability.1.in_meters()
    }

    /// Prüft ob P-Type (zirkumbinäre) Planeten möglich sind
    pub fn p_type_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() > self.p_type_stability.in_meters()
    }

    /// Berechnet die kombinierte bewohnbare Zone des Binärsystems
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
    /// Erstellt eine neue Hierarchie für ein n-Körper System
    pub fn new(components: &[StellarProperties]) -> Self {
        let mut hierarchy_levels = Vec::new();
        let n = components.len();

        if n < 3 {
            // Für < 3 Körper ist keine Hierarchie nötig
            return Self {
                hierarchy_levels,
                stability_factor: 1.0,
                chaos_timescale: Time::years(1e12), // Sehr stabil
            };
        }

        // Vereinfachte Hierarchie-Konstruktion
        // In der Realität wäre dies sehr komplex und würde n-Body Simulationen erfordern
        let total_mass: f64 = components.iter().map(|c| c.mass.in_kg()).sum();
        let mut current_separation = Distance::au(1.0); // Startwert

        for i in 0..(n - 1) {
            let mass_ratio = components[i].mass.in_kg() / components[i + 1].mass.in_kg();
            let level_stability =
                Self::estimate_level_stability(mass_ratio, current_separation.in_au());

            let level = HierarchyLevel {
                orbit: OrbitalElements::new(
                    current_separation.clone(),
                    0.1, // Geringe Exzentrizität für Stabilität
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ),
                components: vec![i, i + 1],
                mass_ratio,
                level_stability,
            };

            hierarchy_levels.push(level);
            current_separation =
                Distance::new(current_separation.value * 3.0, current_separation.system); // Erweitere für nächste Ebene
        }

        // Gesamtstabilität
        let stability_factor = hierarchy_levels
            .iter()
            .map(|level| level.level_stability)
            .fold(1.0, |acc, s| acc * s);

        // Chaos-Zeitskala (vereinfacht)
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

    /// Schätzt die Stabilität einer Hierarchieebene
    fn estimate_level_stability(mass_ratio: f64, separation_au: f64) -> f64 {
        // Vereinfachte Stabilitätsbewertung
        let mass_factor = if mass_ratio > 10.0 || mass_ratio < 0.1 {
            0.9 // Große Massenunterschiede sind stabiler
        } else {
            0.5 // Ähnliche Massen sind weniger stabil
        };

        let separation_factor = if separation_au > 10.0 {
            0.9 // Große Separationen sind stabiler
        } else if separation_au > 1.0 {
            0.7
        } else {
            0.3 // Enge Systeme sind instabiler
        };

        mass_factor * separation_factor
    }

    /// Prüft ob das System langfristig stabil ist
    pub fn is_long_term_stable(&self) -> bool {
        self.stability_factor > 0.7 && self.chaos_timescale.in_years() > 1e9
    }

    /// Gibt die charakteristische dynamische Zeitskala zurück
    pub fn dynamical_timescale(&self) -> Time {
        self.hierarchy_levels
            .first()
            .map(|level| {
                // Vereinfachte Kepler-Zeit für innerste Bahn
                let a_au = level.orbit.semimajor_axis.in_au();
                Time::years(a_au.powf(1.5)) // √(a³/M) mit M≈1
            })
            .unwrap_or(Time::years(1.0))
    }
}

impl StarSystem {
    /// Generiert ein komplettes Sternsystem aus einem Seed (erweiterte Version)
    pub fn generate_from_seed(seed: u64) -> Self {
        Self::generate_from_seed_with_units(seed, UnitSystem::Astronomical)
    }

    /// Generiert System mit spezifischem Einheitensystem
    pub fn generate_from_seed_with_units(seed: u64, unit_system: UnitSystem) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Kosmische Parameter
        let age_universe = rng.gen_range(3.0..13.8);
        let cosmic_epoch = CosmicEpoch {
            age_universe,
            era: format!("Stellar Era ({:.1} Gyr)", age_universe),
            star_formation_rate: 1.0,
            epoch_metallicity: rng.gen_range(-0.5..0.5),
        };

        // Galaktische Position
        let galactic_distance = Self::generate_galactic_distance(&mut rng, unit_system);
        let galactic_region = Self::classify_galactic_region(&galactic_distance);
        let radiation_environment =
            Self::assess_radiation_environment(&galactic_region, age_universe, &mut rng);

        // Elementhäufigkeiten
        let elemental_abundance =
            Self::calculate_elemental_abundance(cosmic_epoch.epoch_metallicity);

        // Sternsystem generieren
        let system_type = Self::generate_system_type(&mut rng, &cosmic_epoch, unit_system);

        // Bewohnbarkeit bewerten
        let habitability_assessment =
            Self::assess_habitability(&system_type, &radiation_environment);

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
        }
    }

    fn generate_galactic_distance(rng: &mut ChaCha8Rng, unit_system: UnitSystem) -> Distance {
        let r: f64 = rng.r#gen();
        let distance_kpc = match r {
            x if x < 0.05 => rng.gen_range(0.0..1.0),   // 5% im Kern
            x if x < 0.15 => rng.gen_range(1.0..4.0),   // 10% innere Bulge
            x if x < 0.70 => rng.gen_range(4.0..10.0),  // 55% bewohnbare Zone
            x if x < 0.90 => rng.gen_range(10.0..20.0), // 20% äußere Scheibe
            _ => rng.gen_range(20.0..50.0),             // 10% Halo
        };

        match unit_system {
            UnitSystem::Astronomical => Distance::new(distance_kpc, unit_system), // kpc
            UnitSystem::SI => Distance::meters(distance_kpc * KILOPARSEC_TO_METERS),
        }
    }

    fn classify_galactic_region(distance: &Distance) -> GalacticRegion {
        let distance_kpc = match distance.system {
            UnitSystem::Astronomical => distance.value,
            UnitSystem::SI => distance.in_meters() / KILOPARSEC_TO_METERS,
        };

        match distance_kpc {
            d if d < 1.0 => GalacticRegion::Core,
            d if d < 4.0 => GalacticRegion::InnerBulge,
            d if d < 10.0 => GalacticRegion::HabitableZone,
            d if d < 20.0 => GalacticRegion::OuterDisk,
            _ => GalacticRegion::Halo,
        }
    }

    fn assess_radiation_environment(
        region: &GalacticRegion,
        age: f64,
        rng: &mut ChaCha8Rng,
    ) -> CosmicRadiationEnvironment {
        let age_factor = if age < 4.0 { 2.0 } else { 1.0 };

        match region {
            GalacticRegion::Core => CosmicRadiationEnvironment {
                agn_risk: 0.9 * age_factor,
                supernova_frequency: 0.8 * age_factor,
                grb_risk: 0.7 * age_factor,
                stellar_encounter_rate: 0.9,
            },
            GalacticRegion::HabitableZone => CosmicRadiationEnvironment {
                agn_risk: 0.2 * age_factor,
                supernova_frequency: 0.3 * age_factor,
                grb_risk: 0.3 * age_factor,
                stellar_encounter_rate: 0.2,
            },
            _ => CosmicRadiationEnvironment {
                agn_risk: 0.1 * age_factor,
                supernova_frequency: 0.1 * age_factor,
                grb_risk: 0.4 * age_factor,
                stellar_encounter_rate: 0.05,
            },
        }
    }

    fn calculate_elemental_abundance(metallicity: f64) -> ElementalAbundance {
        let metal_fraction = 10_f64.powf(metallicity) * 0.02;
        ElementalAbundance {
            hydrogen: 0.73 - metal_fraction * 0.5,
            helium: 0.25 - metal_fraction * 0.3,
            lithium: 1e-9,
            carbon: metal_fraction * 0.25,
            nitrogen: metal_fraction * 0.08,
            oxygen: metal_fraction * 0.45,
            heavy_metals: metal_fraction * 0.22,
        }
    }

    fn generate_system_type(
        rng: &mut ChaCha8Rng,
        cosmic_epoch: &CosmicEpoch,
        unit_system: UnitSystem,
    ) -> SystemType {
        let primary_mass_solar = Self::generate_stellar_mass(rng);
        let primary_mass = Mass::solar_masses(primary_mass_solar).to_system(unit_system);

        // Multiplizität basierend auf Sternmasse
        let multiplicity_probability = match primary_mass_solar {
            m if m > 15.0 => 0.8, // Massive Sterne oft in Multiples
            m if m > 1.5 => 0.6,  // Sonnenähnliche Sterne
            m if m > 0.5 => 0.4,  // K-Zwerge
            _ => 0.25,            // M-Zwerge selten in Multiples
        };

        let age = Time::years(cosmic_epoch.age_universe * 0.8).to_system(unit_system);

        if rng.r#gen::<f64>() < multiplicity_probability {
            let secondary_mass_solar = Self::generate_secondary_mass(rng, primary_mass_solar);
            let secondary_mass = Mass::solar_masses(secondary_mass_solar).to_system(unit_system);

            let primary =
                StellarProperties::new(primary_mass, age.clone(), cosmic_epoch.epoch_metallicity);
            let secondary =
                StellarProperties::new(secondary_mass, age.clone(), cosmic_epoch.epoch_metallicity);

            // Orbitale Parameter mit realistischen Verteilungen
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

            // Prüfung auf Mehrsternsysteme (> 2 Sterne)
            if rng.r#gen::<f64>() < 0.1 && primary_mass_solar > 2.0 {
                // Triplet-System
                let tertiary_mass =
                    Mass::solar_masses(Self::generate_secondary_mass(rng, secondary_mass_solar))
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
            let star = StellarProperties::new(primary_mass, age, cosmic_epoch.epoch_metallicity);
            SystemType::Single(star)
        }
    }

    fn generate_stellar_mass(rng: &mut ChaCha8Rng) -> f64 {
        // IMF (Initial Mass Function) - Salpeter-like
        let r: f64 = rng.r#gen();
        match r {
            x if x < 0.6 => rng.gen_range(0.1..0.5),   // M-Zwerge
            x if x < 0.85 => rng.gen_range(0.5..1.0),  // K-Zwerge
            x if x < 0.95 => rng.gen_range(1.0..2.0),  // G-F Zwerge
            x if x < 0.99 => rng.gen_range(2.0..10.0), // A-B Sterne
            _ => rng.gen_range(10.0..50.0),            // O-Sterne (sehr selten)
        }
    }

    fn generate_secondary_mass(rng: &mut ChaCha8Rng, primary_mass: f64) -> f64 {
        // Sekundäre Sterne sind meist leichter
        let mass_ratio = rng.gen_range(0.1..1.0);
        (primary_mass * mass_ratio).max(0.08) // Minimum für Wasserstofffusion
    }

    fn generate_binary_separation(
        rng: &mut ChaCha8Rng,
        primary_mass: f64,
        secondary_mass: f64,
    ) -> f64 {
        // Realistische Binärseparationen basierend auf Beobachtungen
        // Log-normale Verteilung um 30 AU
        let log_separation = rng.gen_range(0.0..4.0); // log10(AU)
        let separation_au = 10.0_f64.powf(log_separation);

        // Korrektur für Masse (massenreiche Sterne haben weitere Orbits)
        let mass_factor = (primary_mass + secondary_mass) / 2.0;
        separation_au * mass_factor.sqrt()
    }

    fn assess_habitability(
        system_type: &SystemType,
        radiation_env: &CosmicRadiationEnvironment,
    ) -> HabitabilityAssessment {
        match system_type {
            SystemType::Single(star) => {
                let hz = star.calculate_habitable_zone();
                let radiation_risks = RadiationRisks {
                    pre_main_sequence_hazard: match &star.evolutionary_stage {
                        crate::stellar_properties::EvolutionaryStage::PreMainSequence {
                            ..
                        } => 0.8,
                        _ => 0.1,
                    },
                    stellar_flare_risk: match star.mass.in_solar_masses() {
                        m if m < 0.5 => 0.7, // M-Zwerge haben viele Flares
                        _ => 0.2,
                    },
                    galactic_radiation_risk: (radiation_env.agn_risk
                        + radiation_env.supernova_frequency
                        + radiation_env.grb_risk)
                        / 3.0,
                };

                let overall_habitability =
                    Self::calculate_overall_habitability_single(star, &radiation_risks);
                let habitability_conditions = Self::assess_habitability_conditions_single(star);

                HabitabilityAssessment {
                    overall_habitability,
                    system_habitable_zone: hz,
                    radiation_risks,
                    habitability_conditions,
                }
            }
            SystemType::Binary {
                primary,
                secondary,
                orbital_properties,
            } => {
                let combined_hz = orbital_properties.combined_habitable_zone(primary, secondary);

                let radiation_risks = RadiationRisks {
                    pre_main_sequence_hazard: 0.3, // Binäre haben komplexere Evolution
                    stellar_flare_risk: 0.4,
                    galactic_radiation_risk: (radiation_env.agn_risk
                        + radiation_env.supernova_frequency
                        + radiation_env.grb_risk)
                        / 3.0,
                };

                let base_habitability =
                    (Self::calculate_overall_habitability_single(primary, &radiation_risks)
                        + Self::calculate_overall_habitability_single(secondary, &radiation_risks))
                        / 2.0;
                let binary_penalty =
                    if orbital_properties.orbital_elements.semimajor_axis.in_au() < 10.0 {
                        0.7
                    } else {
                        0.9
                    };
                let overall_habitability = base_habitability * binary_penalty;

                let mut habitability_conditions = vec![
                    "S-Type orbits around individual stars possible".to_string(),
                    "P-Type circumbinary orbits possible".to_string(),
                    "Complex orbital dynamics".to_string(),
                ];

                if orbital_properties.lagrange_system.is_some() {
                    habitability_conditions.push("Stable Lagrange points present".to_string());
                }

                HabitabilityAssessment {
                    overall_habitability,
                    system_habitable_zone: combined_hz,
                    radiation_risks,
                    habitability_conditions,
                }
            }
            SystemType::Multiple {
                components,
                hierarchy,
            } => {
                // Vereinfachte Multiple-System Bewertung
                let average_habitability = components
                    .iter()
                    .map(|star| {
                        let dummy_risks = RadiationRisks {
                            pre_main_sequence_hazard: 0.4,
                            stellar_flare_risk: 0.5,
                            galactic_radiation_risk: 0.3,
                        };
                        Self::calculate_overall_habitability_single(star, &dummy_risks)
                    })
                    .sum::<f64>()
                    / components.len() as f64;

                let stability_penalty = hierarchy.stability_factor;
                let overall_habitability = average_habitability * stability_penalty * 0.5; // Multiple sind schwieriger

                // Kombinierte HZ (vereinfacht)
                let total_luminosity: f64 = components.iter().map(|s| s.luminosity).sum();
                let combined_hz = crate::stellar_properties::HabitableZone {
                    inner_edge: Distance::new(
                        0.95 * total_luminosity.sqrt(),
                        components[0].unit_system,
                    ),
                    outer_edge: Distance::new(
                        1.37 * total_luminosity.sqrt(),
                        components[0].unit_system,
                    ),
                    optimistic_inner: Distance::new(
                        0.84 * total_luminosity.sqrt(),
                        components[0].unit_system,
                    ),
                    optimistic_outer: Distance::new(
                        1.67 * total_luminosity.sqrt(),
                        components[0].unit_system,
                    ),
                };

                HabitabilityAssessment {
                    overall_habitability,
                    system_habitable_zone: combined_hz,
                    radiation_risks: RadiationRisks {
                        pre_main_sequence_hazard: 0.5,
                        stellar_flare_risk: 0.6,
                        galactic_radiation_risk: 0.4,
                    },
                    habitability_conditions: vec![
                        format!("Complex {}-body system", components.len()),
                        format!(
                            "Stability factor: {:.1}%",
                            hierarchy.stability_factor * 100.0
                        ),
                        format!(
                            "Chaos timescale: {:.0} Myr",
                            hierarchy.chaos_timescale.in_years() / 1e6
                        ),
                    ],
                }
            }
        }
    }

    fn calculate_overall_habitability_single(
        star: &StellarProperties,
        radiation_risks: &RadiationRisks,
    ) -> f64 {
        let mut habitability = 1.0;

        // Sterntyp-basierte Bewertung
        habitability *= match star.mass.in_solar_masses() {
            m if m < 0.08 => 0.1, // Braune Zwerge
            m if m < 0.3 => 0.6,  // M-Zwerge (problematisch)
            m if m < 0.8 => 0.9,  // K-Zwerge (ideal)
            m if m < 1.4 => 1.0,  // G-Zwerge (gut)
            m if m < 2.0 => 0.7,  // F-Zwerge (kurz)
            _ => 0.1,             // Zu massiv
        };

        // Evolutionsstadium
        habitability *= match &star.evolutionary_stage {
            crate::stellar_properties::EvolutionaryStage::MainSequence { .. } => 1.0,
            crate::stellar_properties::EvolutionaryStage::PreMainSequence { .. } => 0.3,
            crate::stellar_properties::EvolutionaryStage::RedGiant => 0.2,
            _ => 0.05,
        };

        // Strahlungsrisiken
        habitability *= 1.0 - radiation_risks.pre_main_sequence_hazard * 0.5;
        habitability *= 1.0 - radiation_risks.stellar_flare_risk * 0.3;
        habitability *= 1.0 - radiation_risks.galactic_radiation_risk * 0.4;

        habitability.max(0.0).min(1.0)
    }

    fn assess_habitability_conditions_single(star: &StellarProperties) -> Vec<String> {
        let mut conditions = Vec::new();

        match star.mass.in_solar_masses() {
            m if m < 0.3 => {
                conditions.push("Planets likely tidal-locked".to_string());
                conditions.push("Strong stellar flares in youth".to_string());
                conditions.push("Very long habitable periods possible".to_string());
                conditions.push("Requires thick atmospheres".to_string());
            }
            m if m < 0.8 => {
                conditions.push("Optimal for long-term habitability".to_string());
                conditions.push("Stable radiation environment".to_string());
                conditions.push("Multiple billion year habitable periods".to_string());
            }
            m if m < 1.5 => {
                conditions.push("Earth-like radiation environment".to_string());
                conditions.push("Moderate habitable periods (1-10 Gyr)".to_string());
                conditions.push("Stable main sequence evolution".to_string());
            }
            _ => {
                conditions.push("Very short evolutionary timescales".to_string());
                conditions.push("High UV radiation levels".to_string());
                conditions.push("Complex life unlikely due to time constraints".to_string());
            }
        }

        conditions
    }

    /// Exportiert das System als RON-String
    pub fn to_ron_string(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
    }

    /// Lädt ein System aus einem RON-String  
    pub fn from_ron_string(s: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_orbit_stability() {
        let sun = StellarProperties::sun_like();
        let jupiter = StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6), 0.0);

        let binary_orbit = BinaryOrbit::new(
            &sun,
            &jupiter,
            Distance::au(5.2),
            0.048, // Jupiter's eccentricity
            0.0,
            0.0,
            0.0,
        );

        // S-Type Stabilität sollte existieren
        assert!(binary_orbit.s_type_stability.0.in_au() > 0.0);
        assert!(binary_orbit.s_type_stability.1.in_au() > 0.0);

        // P-Type sollte größer als Separation sein
        assert!(
            binary_orbit.p_type_stability.in_au()
                > binary_orbit.orbital_elements.semimajor_axis.in_au()
        );

        // Lagrange-System sollte stabil sein
        assert!(binary_orbit.lagrange_system.is_some());
    }

    #[test]
    fn test_hierarchy_stability() {
        let star1 = StellarProperties::sun_like();
        let star2 = StellarProperties::new(Mass::solar_masses(0.8), Time::years(4.6), 0.0);
        let star3 = StellarProperties::new(Mass::solar_masses(0.3), Time::years(4.6), 0.0);

        let components = vec![star1, star2, star3];
        let hierarchy = SystemHierarchy::new(&components);

        assert_eq!(hierarchy.hierarchy_levels.len(), 2); // n-1 levels
        assert!(hierarchy.stability_factor > 0.0);
        assert!(hierarchy.chaos_timescale.in_years() > 0.0);
    }

    #[test]
    fn test_system_generation() {
        for seed in [42, 1337, 9999] {
            let system = StarSystem::generate_from_seed(seed);

            // System sollte valide Parameter haben
            assert!(system.habitability_assessment.overall_habitability >= 0.0);
            assert!(system.habitability_assessment.overall_habitability <= 1.0);

            match &system.system_type {
                SystemType::Single(star) => {
                    assert!(star.mass.in_solar_masses() > 0.0);
                    assert!(star.age.in_years() > 0.0);
                }
                SystemType::Binary {
                    primary,
                    secondary,
                    orbital_properties,
                } => {
                    assert!(primary.mass.in_solar_masses() > 0.0);
                    assert!(secondary.mass.in_solar_masses() > 0.0);
                    assert!(orbital_properties.orbital_elements.semimajor_axis.in_au() > 0.0);
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

        // Gleicher Seed sollte äquivalente Systeme erzeugen
        assert_eq!(system_au.seed, system_si.seed);
    }
}
