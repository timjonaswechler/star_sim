// stellar_properties.rs - Erweiterte stellare Eigenschaften

use crate::constants::*;
use crate::orbital_mechanics::*;
use crate::units::*;
use serde::{Deserialize, Serialize};

pub type CosmicTime = f64;
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
    /// Masse in Sonnenmassen oder kg
    pub mass: Mass,
    /// Leuchtkraft in Sonnenleuchtkraft oder Watt
    pub luminosity: f64,
    /// Effektivtemperatur in Kelvin
    pub effective_temperature: f64,
    /// Radius in Sonnenradien oder Metern
    pub radius: Distance,
    /// Spektraltyp und Subklasse
    pub spectral_type: SpectralType,
    /// Leuchtkraftklasse
    pub luminosity_class: LuminosityClass,
    /// Aktuelles Evolutionsstadium
    pub evolutionary_stage: EvolutionaryStage,
    /// Hauptreihen-Lebensdauer in Jahren
    pub main_sequence_lifetime: Time,
    /// Aktuelles Alter in Jahren
    pub age: Time,
    /// Metallizität [Fe/H]
    pub metallicity: f64,
    /// Einheitensystem
    pub unit_system: UnitSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitableZone {
    /// Innere Grenze der bewohnbaren Zone
    pub inner_edge: Distance,
    /// Äußere Grenze der bewohnbaren Zone
    pub outer_edge: Distance,
    /// Optimistische innere Grenze
    pub optimistic_inner: Distance,
    /// Optimistische äußere Grenze
    pub optimistic_outer: Distance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TidalLockingAnalysis {
    /// Wahrscheinlichkeit für Tidal Locking (0.0-1.0)
    pub tidal_lock_probability: f64,
    /// Synchronisationszeit in Jahren
    pub synchronization_timescale: Time,
    /// Mögliche Spin-Orbit Resonanzen (z.B. 3:2, 2:1)
    pub possible_resonances: Vec<(u32, u32)>,
}

impl StellarProperties {
    /// Erstellt einen neuen Stern mit gegebenem Einheitensystem
    pub fn new(mass: Mass, age: Time, metallicity: f64) -> Self {
        let unit_system = mass.system;

        let mut star = StellarProperties {
            mass: mass.clone(),
            age: age.clone(),
            metallicity,
            unit_system,
            luminosity: 0.0,
            effective_temperature: 0.0,
            radius: Distance::new(0.0, unit_system),
            spectral_type: SpectralType::G(2),
            luminosity_class: LuminosityClass::V,
            evolutionary_stage: EvolutionaryStage::MainSequence {
                fraction_complete: 0.0,
            },
            main_sequence_lifetime: Time::new(0.0, unit_system),
        };

        star.calculate_properties();
        star
    }

    /// Erstellt sonnenähnlichen Stern (Kompatibilität mit alter API)
    pub fn sun_like() -> Self {
        Self::new(Mass::solar_masses(1.0), Time::years(4.6), 0.0)
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
        let mass_value = self.mass.in_solar_masses();
        match mass_value {
            m if m < 0.43 => 0.23 * m.powf(2.3),
            m if m < 2.0 => m.powf(4.0),
            m if m < 20.0 => 1.4 * m.powf(3.5),
            m => 32000.0 * m, // Sehr massive Sterne
        }
    }

    /// Mass-Temperature Relation  
    fn calculate_temperature_from_mass(&self) -> f64 {
        SOLAR_TEMPERATURE * self.mass.in_solar_masses().powf(0.5)
    }

    /// Stefan-Boltzmann Gesetz: R = sqrt(L / T^4) (in Sonneneinheiten)
    fn calculate_radius_from_luminosity_temperature(&self) -> Distance {
        let radius_solar =
            (self.luminosity / (self.effective_temperature / SOLAR_TEMPERATURE).powf(4.0)).sqrt();
        match self.unit_system {
            UnitSystem::Astronomical => Distance::new(radius_solar, UnitSystem::Astronomical), // Sonnenradien
            UnitSystem::SI => Distance::meters(radius_solar * SOLAR_RADIUS),
        }
    }

    /// Hauptreihen-Lebensdauer basierend auf Masse
    fn calculate_main_sequence_lifetime(&self) -> Time {
        let base_luminosity = self.calculate_luminosity_from_mass();
        let lifetime_years = 10.0 * self.mass.in_solar_masses() / base_luminosity;
        Time::years(lifetime_years)
    }

    /// Bestimmt das aktuelle Evolutionsstadium
    fn determine_evolutionary_stage(&self) -> EvolutionaryStage {
        let pre_ms_time = self.calculate_pre_main_sequence_time();
        let age_years = self.age.in_years();
        let pre_ms_years = pre_ms_time.in_years();
        let ms_lifetime_years = self.main_sequence_lifetime.in_years();

        if age_years < pre_ms_years {
            EvolutionaryStage::PreMainSequence { age: age_years }
        } else if age_years < pre_ms_years + ms_lifetime_years {
            let ms_age = age_years - pre_ms_years;
            let fraction_complete = ms_age / ms_lifetime_years;
            EvolutionaryStage::MainSequence { fraction_complete }
        } else {
            // Post-Main Sequence basierend auf Masse
            match self.mass.in_solar_masses() {
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
    fn calculate_pre_main_sequence_time(&self) -> Time {
        let mass_solar = self.mass.in_solar_masses();
        let pre_ms_years = match mass_solar {
            m if m < 0.1 => 10.0, // 10 Milliarden Jahre für kleinste Sterne
            m if m < 0.5 => 1.0,  // 1 Milliarde Jahre
            m if m < 1.0 => 0.1,  // 100 Millionen Jahre
            _ => 0.01,            // 10 Millionen Jahre für größere Sterne
        };
        Time::years(pre_ms_years)
    }

    /// Berechnet Eigenschaften während der Hauptreihe
    fn calculate_main_sequence_properties(&mut self, fraction_complete: f64) {
        let l_zams = self.calculate_luminosity_from_mass() * 0.5;
        let l_tams = self.calculate_luminosity_from_mass();

        self.luminosity = l_zams + (l_tams - l_zams) * fraction_complete;
        self.effective_temperature = self.calculate_temperature_from_mass();
        self.radius = self.calculate_radius_from_luminosity_temperature();
    }

    /// Pre-Main Sequence Eigenschaften
    fn calculate_pre_main_sequence_properties(&mut self) {
        self.luminosity = self.calculate_luminosity_from_mass() * 2.0;
        self.effective_temperature = self.calculate_temperature_from_mass() * 0.8;
        self.radius = self.calculate_radius_from_luminosity_temperature();
    }

    /// Post-Main Sequence Eigenschaften (vereinfacht)
    fn calculate_post_main_sequence_properties(&mut self) {
        match &self.evolutionary_stage {
            EvolutionaryStage::BlueDwarf => {
                self.luminosity = self.calculate_luminosity_from_mass() * 3.0;
                self.effective_temperature = 10000.0;
                self.radius = self.calculate_radius_from_luminosity_temperature();
            }
            EvolutionaryStage::RedGiant => {
                self.luminosity = self.calculate_luminosity_from_mass() * 100.0;
                self.effective_temperature = 3500.0;
                self.radius = self.calculate_radius_from_luminosity_temperature();
            }
            _ => {
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

    /// Berechnet bewohnbare Zone (erweitert mit Einheiten)
    pub fn calculate_habitable_zone(&self) -> HabitableZone {
        let sqrt_l = self.luminosity.sqrt();

        HabitableZone {
            inner_edge: Distance::new(0.95 * sqrt_l, self.unit_system),
            outer_edge: Distance::new(1.37 * sqrt_l, self.unit_system),
            optimistic_inner: Distance::new(0.84 * sqrt_l, self.unit_system),
            optimistic_outer: Distance::new(1.67 * sqrt_l, self.unit_system),
        }
    }

    /// Analysiert Tidal Locking für gegebene Orbitalentfernung (erweitert)
    pub fn analyze_tidal_locking(&self, orbital_distance: &Distance) -> TidalLockingAnalysis {
        let mass_solar = self.mass.in_solar_masses();
        let distance_au = orbital_distance.in_au();

        let base_probability = match mass_solar {
            m if m < 0.3 => 0.9,
            m if m < 0.6 => 0.5,
            m if m < 1.0 => 0.1,
            _ => 0.01,
        };

        let distance_factor = (1.0 / distance_au).min(1.0);
        let tidal_lock_probability = (base_probability * distance_factor).min(1.0);

        let synchronization_years = distance_au.powf(6.0) / mass_solar.powf(2.0);
        let synchronization_timescale = Time::years(synchronization_years);

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

    /// Escape Velocity von der Sternoberfläche
    pub fn surface_escape_velocity(&self) -> Velocity {
        EscapeVelocity::from_surface(&self.mass, &self.radius)
    }

    /// Orbitale Geschwindigkeit an gegebener Entfernung
    pub fn orbital_velocity_at_distance(&self, distance: &Distance) -> Velocity {
        let gm = G * self.mass.in_kg();
        let r = distance.in_meters();
        let velocity = (gm / r).sqrt();
        Velocity::meters_per_second(velocity)
    }

    /// Konvertiert zu anderem Einheitensystem
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if target == self.unit_system {
            return self.clone();
        }

        Self {
            mass: self.mass.to_system(target),
            age: self.age.to_system(target),
            main_sequence_lifetime: self.main_sequence_lifetime.to_system(target),
            radius: self.radius.to_system(target),
            unit_system: target,
            ..self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sun_like_star() {
        let sun = StellarProperties::sun_like();

        // Sollte etwa sonnenähnliche Eigenschaften haben
        assert!((sun.effective_temperature - SOLAR_TEMPERATURE).abs() < 500.0);
        assert!((sun.luminosity - 1.0).abs() < 0.5);
        assert_eq!(sun.unit_system, UnitSystem::Astronomical);
    }

    #[test]
    fn test_stellar_properties_si() {
        let star = StellarProperties::new(
            Mass::kilograms(SOLAR_MASS),
            Time::seconds(4.6 * SECONDS_PER_YEAR * 1e9).clone(),
            0.0,
        );

        assert_eq!(star.unit_system, UnitSystem::SI);
        assert!((star.mass.in_solar_masses() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_habitable_zone_units() {
        let sun = StellarProperties::sun_like();
        let hz = sun.calculate_habitable_zone();

        // HZ sollte in astronomischen Einheiten sein
        assert_eq!(hz.inner_edge.system, UnitSystem::Astronomical);
        assert!(hz.inner_edge.in_au() > 0.8);
        assert!(hz.outer_edge.in_au() < 1.5);
    }

    #[test]
    fn test_escape_velocity() {
        let sun = StellarProperties::sun_like();
        let escape_vel = sun.surface_escape_velocity();

        // Sonnen-Escape Velocity sollte etwa 617 km/s sein
        assert!((escape_vel.in_kms() - 617.0).abs() < 50.0);
    }
}
