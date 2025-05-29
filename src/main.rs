// Cargo.toml ergänzen:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// ron = "0.8"
// rand = "0.8"
// rand_chacha = "0.3"

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Kosmische Zeit in Milliarden Jahren nach dem Big Bang
pub type CosmicTime = f64;
/// Distanz in Kiloparsec (kpc)
pub type Distance = f64;
/// Metallizität als [Fe/H] (logarithmisch relativ zur Sonne)
pub type Metallicity = f64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpectralType {
    // Hauptreihe O, B, A, F, G, K, M mit Subklassen 0-9
    O(u8),
    B(u8),
    A(u8),
    F(u8),
    G(u8),
    K(u8),
    M(u8),
    // Braune Zwerge
    L(u8),
    T(u8),
    Y(u8),
    // Post-Main Sequence
    WolfRayet,  // W-type
    WhiteDwarf, // D-type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LuminosityClass {
    /// Hypergiants
    Zero,
    /// Bright supergiants
    Ia,
    /// Supergiants
    Ib,
    /// Bright giants
    II,
    /// Giants
    III,
    /// Subgiants
    IV,
    /// Main sequence (dwarfs)
    V,
    /// Subdwarfs
    VI,
    /// White dwarfs
    VII,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionaryStage {
    /// Pre-Main Sequence (Contraction phase)
    PreMainSequence {
        age: f64,
    },
    /// Zero Age Main Sequence
    ZAMS,
    /// Main Sequence
    MainSequence {
        fraction_complete: f64,
    },
    /// Terminal Age Main Sequence
    TAMS,
    /// Post-Main Sequence stages
    RedGiant,
    HorizontalBranch,
    AsymptoticGiantBranch,
    BlueDwarf,
    WhiteDwarf {
        cooling_age: f64,
    },
    NeutronStar,
    BlackHole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StellarProperties {
    /// Masse in Sonnenmassen
    pub mass: f64,
    /// Leuchtkraft in Sonnenleuchtkraft  
    pub luminosity: f64,
    /// Effektivtemperatur in Kelvin
    pub effective_temperature: f64,
    /// Radius in Sonnenradien
    pub radius: f64,
    /// Spektraltyp und Subklasse
    pub spectral_type: SpectralType,
    /// Leuchtkraftklasse
    pub luminosity_class: LuminosityClass,
    /// Aktuelles Evolutionsstadium
    pub evolutionary_stage: EvolutionaryStage,
    /// Hauptreihen-Lebensdauer in Milliarden Jahren
    pub main_sequence_lifetime: f64,
    /// Aktuelles Alter in Milliarden Jahren
    pub age: f64,
    /// Metallizität [Fe/H]
    pub metallicity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitableZone {
    /// Innere Grenze der bewohnbaren Zone (AU)
    pub inner_edge: f64,
    /// Äußere Grenze der bewohnbaren Zone (AU)  
    pub outer_edge: f64,
    /// Optimistische innere Grenze (AU)
    pub optimistic_inner: f64,
    /// Optimistische äußere Grenze (AU)
    pub optimistic_outer: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TidalLockingAnalysis {
    /// Wahrscheinlichkeit für Tidal Locking (0.0-1.0)
    pub tidal_lock_probability: f64,
    /// Synchronisationszeit in Milliarden Jahren
    pub synchronization_timescale: f64,
    /// Mögliche Spin-Orbit Resonanzen (z.B. 3:2, 2:1)
    pub possible_resonances: Vec<(u32, u32)>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOrbit {
    /// Semimajor axis in AU
    pub semimajor_axis: f64,
    /// Exzentrizität (0.0-1.0)
    pub eccentricity: f64,
    /// Orbitalperiode in Jahren
    pub orbital_period: f64,
    /// Barycenter position (fraction from primary)
    pub barycenter_position: f64,
    /// S-Type Stabilitätsgrenze für Planeten
    pub s_type_stability: (f64, f64), // (primary, secondary)
    /// P-Type Stabilitätsgrenze für Planeten
    pub p_type_stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHierarchy {
    /// Hierarchische Struktur für Mehrsternsysteme
    pub hierarchy_levels: Vec<BinaryOrbit>,
}

impl StellarProperties {
    /// Erstellt einen neuen Stern basierend auf Masse und Alter
    pub fn new(mass: f64, age: f64, metallicity: f64) -> Self {
        let mut star = StellarProperties {
            mass,
            age,
            metallicity,
            luminosity: 0.0,
            effective_temperature: 0.0,
            radius: 0.0,
            spectral_type: SpectralType::G(2),
            luminosity_class: LuminosityClass::V,
            evolutionary_stage: EvolutionaryStage::MainSequence {
                fraction_complete: 0.0,
            },
            main_sequence_lifetime: 0.0,
        };

        star.calculate_properties();
        star
    }

    /// Berechnet alle stellaren Eigenschaften basierend auf Masse und Alter
    fn calculate_properties(&mut self) {
        // Hauptreihen-Lebensdauer berechnen
        self.main_sequence_lifetime = self.calculate_main_sequence_lifetime();

        // Evolutionsstadium bestimmen
        self.evolutionary_stage = self.determine_evolutionary_stage();

        // Physikalische Eigenschaften berechnen
        match &self.evolutionary_stage {
            EvolutionaryStage::MainSequence { fraction_complete } => {
                self.calculate_main_sequence_properties(*fraction_complete);
            }
            EvolutionaryStage::PreMainSequence { .. } => {
                self.calculate_pre_main_sequence_properties();
            }
            _ => {
                self.calculate_post_main_sequence_properties();
            }
        }

        // Spektraltyp und Leuchtkraftklasse bestimmen
        self.spectral_type = self.determine_spectral_type();
        self.luminosity_class = self.determine_luminosity_class();
    }

    /// Mass-Luminosity Relation (verschiedene Formeln für verschiedene Massenbereiche)
    fn calculate_luminosity_from_mass(&self) -> f64 {
        match self.mass {
            m if m < 0.43 => 0.23 * m.powf(2.3),
            m if m < 2.0 => m.powf(4.0),
            m if m < 20.0 => 1.4 * m.powf(3.5),
            m => 32000.0 * m, // Sehr massive Sterne
        }
    }

    /// Mass-Temperature Relation  
    fn calculate_temperature_from_mass(&self) -> f64 {
        5778.0 * self.mass.powf(0.5) // Vereinfachte Relation
    }

    /// Stefan-Boltzmann Gesetz: R = sqrt(L / T^4) (in Sonneneinheiten)
    fn calculate_radius_from_luminosity_temperature(&self) -> f64 {
        (self.luminosity / (self.effective_temperature / 5778.0).powf(4.0)).sqrt()
    }

    /// Hauptreihen-Lebensdauer basierend auf Masse
    fn calculate_main_sequence_lifetime(&self) -> f64 {
        // t_MS = M / L (vereinfacht, in Milliarden Jahren)
        let base_luminosity = self.calculate_luminosity_from_mass();
        10.0 * self.mass / base_luminosity
    }

    /// Bestimmt das aktuelle Evolutionsstadium
    fn determine_evolutionary_stage(&self) -> EvolutionaryStage {
        let pre_ms_time = self.calculate_pre_main_sequence_time();

        if self.age < pre_ms_time {
            EvolutionaryStage::PreMainSequence { age: self.age }
        } else if self.age < pre_ms_time + self.main_sequence_lifetime {
            let ms_age = self.age - pre_ms_time;
            let fraction_complete = ms_age / self.main_sequence_lifetime;
            EvolutionaryStage::MainSequence { fraction_complete }
        } else {
            // Post-Main Sequence basierend auf Masse
            match self.mass {
                m if m < 0.08 => EvolutionaryStage::MainSequence {
                    fraction_complete: 1.0,
                }, // Braune Zwerge
                m if m < 0.25 => EvolutionaryStage::BlueDwarf,
                m if m < 8.0 => EvolutionaryStage::RedGiant,
                m if m < 30.0 => EvolutionaryStage::NeutronStar,
                _ => EvolutionaryStage::BlackHole,
            }
        }
    }

    /// Pre-Main Sequence Zeit (Kontraktion)
    fn calculate_pre_main_sequence_time(&self) -> f64 {
        // Für M-Zwerge kann das Milliarden Jahre dauern
        match self.mass {
            m if m < 0.1 => 10.0, // 10 Milliarden Jahre für kleinste Sterne
            m if m < 0.5 => 1.0,  // 1 Milliarde Jahre
            m if m < 1.0 => 0.1,  // 100 Millionen Jahre
            _ => 0.01,            // 10 Millionen Jahre für größere Sterne
        }
    }

    /// Berechnet Eigenschaften während der Hauptreihe
    fn calculate_main_sequence_properties(&mut self, fraction_complete: f64) {
        // ZAMS (Zero Age Main Sequence) Werte
        let l_zams = self.calculate_luminosity_from_mass() * 0.5; // Etwa die Hälfte der TAMS Luminosität
        let l_tams = self.calculate_luminosity_from_mass();

        // Linear interpolation zwischen ZAMS und TAMS
        self.luminosity = l_zams + (l_tams - l_zams) * fraction_complete;

        // Temperatur bleibt relativ konstant
        self.effective_temperature = self.calculate_temperature_from_mass();

        // Radius aus Stefan-Boltzmann
        self.radius = self.calculate_radius_from_luminosity_temperature();
    }

    /// Pre-Main Sequence Eigenschaften
    fn calculate_pre_main_sequence_properties(&mut self) {
        // Höhere Luminosität während Kontraktion
        self.luminosity = self.calculate_luminosity_from_mass() * 2.0;
        self.effective_temperature = self.calculate_temperature_from_mass() * 0.8;
        self.radius = self.calculate_radius_from_luminosity_temperature();
    }

    /// Post-Main Sequence Eigenschaften (vereinfacht)
    fn calculate_post_main_sequence_properties(&mut self) {
        match &self.evolutionary_stage {
            EvolutionaryStage::BlueDwarf => {
                self.luminosity = self.calculate_luminosity_from_mass() * 3.0;
                self.effective_temperature = 10000.0; // Blaue Zwerge sind heißer
                self.radius = self.calculate_radius_from_luminosity_temperature();
            }
            EvolutionaryStage::RedGiant => {
                self.luminosity = self.calculate_luminosity_from_mass() * 100.0;
                self.effective_temperature = 3500.0; // Kühler
                self.radius = self.calculate_radius_from_luminosity_temperature();
            }
            _ => {
                // Standard Main Sequence für andere Stadien
                self.luminosity = self.calculate_luminosity_from_mass();
                self.effective_temperature = self.calculate_temperature_from_mass();
                self.radius = self.calculate_radius_from_luminosity_temperature();
            }
        }
    }

    /// Bestimmt Spektraltyp basierend auf Temperatur
    fn determine_spectral_type(&self) -> SpectralType {
        match self.effective_temperature as u32 {
            t if t >= 30000 => SpectralType::O(5),
            t if t >= 10000 => SpectralType::B(5),
            t if t >= 7500 => SpectralType::A(5),
            t if t >= 6000 => SpectralType::F(5),
            t if t >= 5200 => SpectralType::G(5),
            t if t >= 3700 => SpectralType::K(5),
            t if t >= 2400 => SpectralType::M(5),
            t if t >= 1300 => SpectralType::L(5),
            t if t >= 500 => SpectralType::T(5),
            _ => SpectralType::Y(5),
        }
    }

    /// Bestimmt Leuchtkraftklasse
    fn determine_luminosity_class(&self) -> LuminosityClass {
        match &self.evolutionary_stage {
            EvolutionaryStage::MainSequence { .. } | EvolutionaryStage::PreMainSequence { .. } => {
                LuminosityClass::V
            }
            EvolutionaryStage::RedGiant => LuminosityClass::III,
            EvolutionaryStage::WhiteDwarf { .. } => LuminosityClass::VII,
            _ => LuminosityClass::V,
        }
    }

    /// Berechnet bewohnbare Zone
    pub fn calculate_habitable_zone(&self) -> HabitableZone {
        // Vereinfachte HZ Berechnung basierend auf Luminosität
        let sqrt_l = self.luminosity.sqrt();

        HabitableZone {
            inner_edge: 0.95 * sqrt_l,
            outer_edge: 1.37 * sqrt_l,
            optimistic_inner: 0.84 * sqrt_l,
            optimistic_outer: 1.67 * sqrt_l,
        }
    }

    /// Analysiert Tidal Locking für gegebene Orbitalentfernung
    pub fn analyze_tidal_locking(&self, orbital_distance: f64) -> TidalLockingAnalysis {
        // Tidal Locking wahrscheinlicher bei kleineren, kühleren Sternen
        let base_probability = match self.mass {
            m if m < 0.3 => 0.9,
            m if m < 0.6 => 0.5,
            m if m < 1.0 => 0.1,
            _ => 0.01,
        };

        // Nähere Planeten werden eher tidal locked
        let distance_factor = (1.0 / orbital_distance).min(1.0);
        let tidal_lock_probability = (base_probability * distance_factor).min(1.0);

        // Synchronisationszeit (sehr vereinfacht)
        let synchronization_timescale = orbital_distance.powf(6.0) / self.mass.powf(2.0);

        // Mögliche Resonanzen
        let possible_resonances = if tidal_lock_probability > 0.8 {
            vec![(1, 1)] // 1:1 synchronous
        } else if tidal_lock_probability > 0.3 {
            vec![(3, 2), (2, 1), (1, 1)] // Mercury-like oder synchronous
        } else {
            vec![] // Keine starke Resonanz
        };

        TidalLockingAnalysis {
            tidal_lock_probability,
            synchronization_timescale,
            possible_resonances,
        }
    }
}

impl BinaryOrbit {
    /// Erstellt neue Binärbahn basierend auf Stellarparametern
    pub fn new(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        separation: f64,
        eccentricity: f64,
    ) -> Self {
        let total_mass = primary.mass + secondary.mass;

        // Kepler's 3rd Law: P² = a³ / (M₁ + M₂)
        let orbital_period = (separation.powf(3.0) / total_mass).sqrt();

        // Barycenter Position
        let barycenter_position = secondary.mass / total_mass;

        // Stabilitätsgrenzen berechnen
        let mu_primary = secondary.mass / total_mass;
        let mu_secondary = primary.mass / total_mass;

        // S-Type Stabilität (vereinfacht)
        let s_type_primary = separation * (0.464 - 0.380 * mu_primary - 0.631 * eccentricity);
        let s_type_secondary = separation * (0.464 - 0.380 * mu_secondary - 0.631 * eccentricity);

        // P-Type Stabilität
        let mu_min = primary.mass.min(secondary.mass) / total_mass;
        let p_type_stability = separation * (1.60 + 4.12 * mu_min + 4.27 * eccentricity);

        BinaryOrbit {
            semimajor_axis: separation,
            eccentricity,
            orbital_period,
            barycenter_position,
            s_type_stability: (s_type_primary, s_type_secondary),
            p_type_stability,
        }
    }

    /// Berechnet minimale und maximale Entfernung zwischen Sternen
    pub fn distance_range(&self) -> (f64, f64) {
        let periapsis = self.semimajor_axis * (1.0 - self.eccentricity);
        let apoapsis = self.semimajor_axis * (1.0 + self.eccentricity);
        (periapsis, apoapsis)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    /// Eindeutige Seed für Reproduzierbarkeit
    pub seed: u64,
    /// Kosmische Epoche der Entstehung
    pub cosmic_epoch: CosmicEpoch,
    /// Galaktische Position und Umgebung
    pub galactic_distance: Distance,
    pub galactic_region: GalacticRegion,
    pub radiation_environment: CosmicRadiationEnvironment,
    /// Sternsystem Konfiguration
    pub system_type: SystemType,
    /// Elementhäufigkeiten
    pub elemental_abundance: ElementalAbundance,
    /// Gesamte Bewohnbarkeitsbewertung
    pub habitability_assessment: HabitabilityAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitabilityAssessment {
    /// Gesamter Bewohnbarkeitsfaktor (0.0-1.0)
    pub overall_habitability: f64,
    /// Bewohnbare Zone des Systems
    pub system_habitable_zone: HabitableZone,
    /// Strahlungsrisiken für Leben
    pub radiation_risks: RadiationRisks,
    /// Bewohnbarkeitsbedingungen für Planeten
    pub habitability_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationRisks {
    /// UV/XUV Strahlung während Pre-MS Phase
    pub pre_main_sequence_hazard: f64,
    /// Flare-Aktivität Risiko
    pub stellar_flare_risk: f64,
    /// Galaktische Strahlungsrisiken
    pub galactic_radiation_risk: f64,
}

// Kosmische Strukturen aus Teil I (vereinfacht für Kompatibilität)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicEpoch {
    pub age_universe: CosmicTime,
    pub era: String,
    pub star_formation_rate: f64,
    pub epoch_metallicity: Metallicity,
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

impl StarSystem {
    /// Generiert ein komplettes Sternsystem aus einem Seed
    pub fn generate_from_seed(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Kosmische Parameter aus Teil I
        let age_universe = rng.gen_range(3.0..13.8);
        let cosmic_epoch = CosmicEpoch {
            age_universe,
            era: format!("Stellar Era ({:.1} Gyr)", age_universe),
            star_formation_rate: 1.0,
            epoch_metallicity: rng.gen_range(-0.5..0.5),
        };

        // Galaktische Position
        let galactic_distance = Self::generate_galactic_distance(&mut rng);
        let galactic_region = Self::classify_galactic_region(galactic_distance);
        let radiation_environment =
            Self::assess_radiation_environment(&galactic_region, age_universe, &mut rng);

        // Elementhäufigkeiten basierend auf Metallizität
        let elemental_abundance =
            Self::calculate_elemental_abundance(cosmic_epoch.epoch_metallicity);

        // Sternsystem generieren
        let system_type = Self::generate_system_type(&mut rng, &cosmic_epoch);

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
        }
    }

    fn generate_galactic_distance(rng: &mut ChaCha8Rng) -> Distance {
        let r: f64 = rng.r#gen();
        match r {
            x if x < 0.05 => rng.gen_range(0.0..1.0),   // 5% im Kern
            x if x < 0.15 => rng.gen_range(1.0..4.0),   // 10% innere Bulge
            x if x < 0.70 => rng.gen_range(4.0..10.0),  // 55% bewohnbare Zone
            x if x < 0.90 => rng.gen_range(10.0..20.0), // 20% äußere Scheibe
            _ => rng.gen_range(20.0..50.0),             // 10% Halo
        }
    }

    fn classify_galactic_region(distance: Distance) -> GalacticRegion {
        match distance {
            d if d < 1.0 => GalacticRegion::Core,
            d if d < 4.0 => GalacticRegion::InnerBulge,
            d if d < 10.0 => GalacticRegion::HabitableZone,
            d if d < 20.0 => GalacticRegion::OuterDisk,
            _ => GalacticRegion::Halo,
        }
    }

    fn assess_radiation_environment(
        region: &GalacticRegion,
        age: CosmicTime,
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

    fn generate_system_type(rng: &mut ChaCha8Rng, cosmic_epoch: &CosmicEpoch) -> SystemType {
        // Multiplizität basierend auf Sternmasse
        let primary_mass = Self::generate_stellar_mass(rng);
        let multiplicity_probability = match primary_mass {
            m if m > 15.0 => 0.8, // Massive Sterne sind oft in Multiples
            m if m > 1.5 => 0.6,  // Sonnenähnliche Sterne
            m if m > 0.5 => 0.4,  // K-Zwerge
            _ => 0.25,            // M-Zwerge selten in Multiples
        };

        if rng.r#gen::<f64>() < multiplicity_probability {
            // Binärsystem generieren
            let primary = StellarProperties::new(
                primary_mass,
                cosmic_epoch.age_universe * 0.8,
                cosmic_epoch.epoch_metallicity,
            );
            let secondary_mass = Self::generate_secondary_mass(rng, primary_mass);
            let secondary = StellarProperties::new(
                secondary_mass,
                cosmic_epoch.age_universe * 0.8,
                cosmic_epoch.epoch_metallicity,
            );

            // Orbitale Parameter
            let separation = rng.gen_range(1.0..1000.0); // AU
            let eccentricity = rng.gen_range(0.0..0.8);

            let orbital_properties =
                BinaryOrbit::new(&primary, &secondary, separation, eccentricity);

            SystemType::Binary {
                primary,
                secondary,
                orbital_properties,
            }
        } else {
            // Einzelstern
            let star = StellarProperties::new(
                primary_mass,
                cosmic_epoch.age_universe * 0.8,
                cosmic_epoch.epoch_metallicity,
            );
            SystemType::Single(star)
        }
    }

    fn generate_stellar_mass(rng: &mut ChaCha8Rng) -> f64 {
        // IMF (Initial Mass Function) - mehr kleine Sterne
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

    fn assess_habitability(
        system_type: &SystemType,
        radiation_env: &CosmicRadiationEnvironment,
    ) -> HabitabilityAssessment {
        match system_type {
            SystemType::Single(star) => {
                let hz = star.calculate_habitable_zone();
                let radiation_risks = RadiationRisks {
                    pre_main_sequence_hazard: match &star.evolutionary_stage {
                        EvolutionaryStage::PreMainSequence { .. } => 0.8,
                        _ => 0.1,
                    },
                    stellar_flare_risk: match star.mass {
                        m if m < 0.5 => 0.7, // M-Zwerge haben viele Flares
                        _ => 0.2,
                    },
                    galactic_radiation_risk: (radiation_env.agn_risk
                        + radiation_env.supernova_frequency
                        + radiation_env.grb_risk)
                        / 3.0,
                };

                let overall_habitability =
                    Self::calculate_overall_habitability(star, &radiation_risks);
                let habitability_conditions = Self::assess_habitability_conditions(star);

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
                // Vereinfachte Binary-Bewertung
                let combined_luminosity = primary.luminosity + secondary.luminosity;
                let effective_star = StellarProperties::new(
                    primary.mass + secondary.mass * 0.5, // Gewichteter Durchschnitt
                    primary.age,
                    primary.metallicity,
                );

                // HZ für binäre Systeme ist komplexer, hier vereinfacht
                let mut hz = effective_star.calculate_habitable_zone();
                hz.inner_edge *= combined_luminosity.sqrt();
                hz.outer_edge *= combined_luminosity.sqrt();

                let radiation_risks = RadiationRisks {
                    pre_main_sequence_hazard: 0.3, // Binäre haben komplexere Evolution
                    stellar_flare_risk: 0.4,
                    galactic_radiation_risk: (radiation_env.agn_risk
                        + radiation_env.supernova_frequency
                        + radiation_env.grb_risk)
                        / 3.0,
                };

                let overall_habitability =
                    Self::calculate_overall_habitability(&effective_star, &radiation_risks) * 0.8; // Binäre etwas schwieriger
                let habitability_conditions = vec![
                    "S-Type orbits around individual stars possible".to_string(),
                    "P-Type circumbinary orbits possible".to_string(),
                    "Complex orbital dynamics".to_string(),
                    "Potential Milankovitch-like climate cycles".to_string(),
                ];

                HabitabilityAssessment {
                    overall_habitability,
                    system_habitable_zone: hz,
                    radiation_risks,
                    habitability_conditions,
                }
            }
            SystemType::Multiple { .. } => {
                // Placeholder für komplexe Mehrsternsysteme
                HabitabilityAssessment {
                    overall_habitability: 0.3,
                    system_habitable_zone: HabitableZone {
                        inner_edge: 0.5,
                        outer_edge: 2.0,
                        optimistic_inner: 0.3,
                        optimistic_outer: 3.0,
                    },
                    radiation_risks: RadiationRisks {
                        pre_main_sequence_hazard: 0.5,
                        stellar_flare_risk: 0.6,
                        galactic_radiation_risk: 0.4,
                    },
                    habitability_conditions: vec!["Highly complex orbital mechanics".to_string()],
                }
            }
        }
    }

    fn assess_habitability_conditions(star: &StellarProperties) -> Vec<String> {
        let mut conditions = Vec::new();

        match star.mass {
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

        // Evolutionsstadium-spezifische Bedingungen
        match &star.evolutionary_stage {
            EvolutionaryStage::PreMainSequence { .. } => {
                conditions.push("High luminosity variability".to_string());
                conditions.push("Intense stellar activity phase".to_string());
            }
            EvolutionaryStage::RedGiant => {
                conditions.push("Expanding habitable zone".to_string());
                conditions.push("Intense stellar winds".to_string());
            }
            _ => {}
        }

        conditions
    }

    fn calculate_overall_habitability(
        star: &StellarProperties,
        radiation_risks: &RadiationRisks,
    ) -> f64 {
        let mut habitability = 1.0;

        // Sterntyp-basierte Bewertung
        habitability *= match star.mass {
            m if m < 0.08 => 0.1, // Braune Zwerge
            m if m < 0.3 => 0.6,  // M-Zwerge (problematisch)
            m if m < 0.8 => 0.9,  // K-Zwerge (ideal)
            m if m < 1.4 => 1.0,  // G-Zwerge (gut)
            m if m < 2.0 => 0.7,  // F-Zwerge (kurz)
            _ => 0.1,             // Zu massiv
        };

        // Evolutionsstadium
        habitability *= match &star.evolutionary_stage {
            EvolutionaryStage::MainSequence { .. } => 1.0,
            EvolutionaryStage::PreMainSequence { .. } => 0.3,
            EvolutionaryStage::RedGiant => 0.2,
            _ => 0.05,
        };

        // Strahlungsrisiken
        habitability *= (1.0 - radiation_risks.pre_main_sequence_hazard * 0.5);
        habitability *= (1.0 - radiation_risks.stellar_flare_risk * 0.3);
        habitability *= (1.0 - radiation_risks.galactic_radiation_risk * 0.4);

        habitability.max(0.0).min(1.0)
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
    fn test_stellar_properties() {
        let star = StellarProperties::new(1.0, 4.6, 0.0); // Sonnenähnlich
        assert!((star.effective_temperature - 5778.0).abs() < 500.0);
        assert!((star.luminosity - 1.0).abs() < 0.5);
        println!("Sun-like star: {:?}", star);
    }

    #[test]
    fn test_binary_system() {
        let primary = StellarProperties::new(1.0, 4.6, 0.0);
        let secondary = StellarProperties::new(0.5, 4.6, 0.0);
        let orbit = BinaryOrbit::new(&primary, &secondary, 50.0, 0.2);

        println!("Binary orbit: {:?}", orbit);
        assert!(orbit.orbital_period > 0.0);
        assert!(orbit.s_type_stability.0 > 0.0);
        assert!(orbit.p_type_stability > orbit.semimajor_axis);
    }

    #[test]
    fn test_system_generation() {
        for seed in [42, 1337, 9999] {
            let system = StarSystem::generate_from_seed(seed);
            println!("\n=== System {} ===", seed);
            println!("Type: {:?}", system.system_type);
            println!(
                "Habitability: {:.2}",
                system.habitability_assessment.overall_habitability
            );
            println!(
                "Habitable Zone: {:.2}-{:.2} AU",
                system
                    .habitability_assessment
                    .system_habitable_zone
                    .inner_edge,
                system
                    .habitability_assessment
                    .system_habitable_zone
                    .outer_edge
            );
        }
    }

    #[test]
    fn test_tidal_locking() {
        let m_dwarf = StellarProperties::new(0.3, 5.0, 0.0);
        let tidal_analysis = m_dwarf.analyze_tidal_locking(0.1); // 0.1 AU orbit

        println!("Tidal locking analysis: {:?}", tidal_analysis);
        assert!(tidal_analysis.tidal_lock_probability > 0.5); // Should be high for close M-dwarf
    }
}

fn main() {
    println!("=== Wissenschaftlicher Sternsystem Generator ===\n");

    // Verschiedene Systemtypen demonstrieren
    for seed in [42, 1337, 2024, 9999] {
        let system = StarSystem::generate_from_seed(seed);

        println!("Seed: {}", seed);
        println!(
            "Galaktische Position: {:.1} kpc ({:?})",
            system.galactic_distance, system.galactic_region
        );

        match &system.system_type {
            SystemType::Single(star) => {
                println!(
                    "Einzelstern: {:?} {:?}",
                    star.spectral_type, star.luminosity_class
                );
                println!(
                    "  Masse: {:.2} M☉, Leuchtkraft: {:.3} L☉, Temperatur: {:.0} K",
                    star.mass, star.luminosity, star.effective_temperature
                );
                println!("  Evolutionsstadium: {:?}", star.evolutionary_stage);
                println!(
                    "  Hauptreihen-Lebensdauer: {:.1} Gyr",
                    star.main_sequence_lifetime
                );

                let hz = star.calculate_habitable_zone();
                println!(
                    "  Bewohnbare Zone: {:.2}-{:.2} AU",
                    hz.inner_edge, hz.outer_edge
                );

                // Tidal Locking für verschiedene Entfernungen testen
                if star.mass < 0.6 {
                    let tidal = star.analyze_tidal_locking(hz.inner_edge);
                    println!(
                        "  Tidal Lock Wahrscheinlichkeit @ innere HZ: {:.1}%",
                        tidal.tidal_lock_probability * 100.0
                    );
                }
            }
            SystemType::Binary {
                primary,
                secondary,
                orbital_properties,
            } => {
                println!("Binärsystem:");
                println!(
                    "  Primär: {:?} ({:.2} M☉)",
                    primary.spectral_type, primary.mass
                );
                println!(
                    "  Sekundär: {:?} ({:.2} M☉)",
                    secondary.spectral_type, secondary.mass
                );
                println!(
                    "  Orbitale Trennung: {:.1} AU, Exzentrizität: {:.2}",
                    orbital_properties.semimajor_axis, orbital_properties.eccentricity
                );
                println!(
                    "  Orbitalperiode: {:.0} Jahre",
                    orbital_properties.orbital_period
                );
                println!(
                    "  S-Type Stabilität: {:.1}/{:.1} AU",
                    orbital_properties.s_type_stability.0, orbital_properties.s_type_stability.1
                );
                println!(
                    "  P-Type Stabilität: > {:.1} AU",
                    orbital_properties.p_type_stability
                );

                let (peri, apo) = orbital_properties.distance_range();
                println!("  Entfernungsbereich: {:.1}-{:.1} AU", peri, apo);
            }
            SystemType::Multiple { .. } => {
                println!("Mehrsternsystem (komplex)");
            }
        }

        println!(
            "Bewohnbarkeit: {:.1}% - Bedingungen: {:?}",
            system.habitability_assessment.overall_habitability * 100.0,
            system.habitability_assessment.habitability_conditions
        );

        // Elementhäufigkeiten zeigen
        let elem = &system.elemental_abundance;
        if elem.carbon > 0.01 {
            println!(
                "Schwere Elemente: C={:.2}%, N={:.2}%, O={:.2}%, Metalle={:.2}%",
                elem.carbon * 100.0,
                elem.nitrogen * 100.0,
                elem.oxygen * 100.0,
                elem.heavy_metals * 100.0
            );
        }

        println!("---\n");
    }

    // RON Export Demonstration
    let example_system = StarSystem::generate_from_seed(2024);
    match example_system.to_ron_string() {
        Ok(ron_data) => {
            println!(
                "RON Export erfolgreich generiert ({} Zeichen)",
                ron_data.len()
            );
            // Nur erste paar Zeilen zeigen
            for (i, line) in ron_data.lines().take(10).enumerate() {
                println!("{:2}: {}", i + 1, line);
            }
            if ron_data.lines().count() > 10 {
                println!("    ... ({} weitere Zeilen)", ron_data.lines().count() - 10);
            }
        }
        Err(e) => println!("RON Export Fehler: {}", e),
    }
}
