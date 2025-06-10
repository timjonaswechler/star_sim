use super::types::*;
use crate::physics::constants::*;
use crate::physics::units::*;
use crate::physics::astrophysics::orbit::elements::EscapeVelocity;
use crate::stellar_objects::bodies::habitability::HabitableZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TidalLockingAnalysis {
    /// Wahrscheinlichkeit für Tidal Locking (0.0-1.0)
    pub tidal_lock_probability: f64,
    /// Synchronisationszeit in Jahren
    pub synchronization_timescale: Time,
    /// Mögliche Spin-Orbit Resonanzen (z.B. 3:2, 2:1)
    pub possible_resonances: Vec<(u32, u32)>,
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
    pub units: UnitSystem,
}

impl StellarProperties {
    /// Erstellt einen neuen Stern mit gegebenem Einheitensystem
    pub fn new(mass: Mass, age: Time, metallicity: f64) -> Self {
        let units = mass.units();

        let mut star = StellarProperties {
            mass: mass.clone(),
            age: age.clone(),
            metallicity,
            units,
            luminosity: 0.0,
            effective_temperature: 0.0,
            radius: Distance::new(0.0, units),
            spectral_type: SpectralType::G(2), // Default, wird überschrieben
            luminosity_class: LuminosityClass::V, // Default, wird überschrieben
            evolutionary_stage: EvolutionaryStage::MainSequence {
                // Default, wird überschrieben
                fraction_complete: 0.0,
            },
            main_sequence_lifetime: Time::new(0.0, units), // Default, wird berechnet
        };

        star.calculate_properties();
        star
    }

    /// Erstellt sonnenähnlichen Stern (Kompatibilität mit alter API)
    pub fn sun_like() -> Self {
        Self::new(Mass::solar_masses(1.0), Time::years(4.6e9), 0.0) // Korrigiertes Alter in Jahren
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
                // TODO: Detailliertere Post-MS Modelle hinzufügen
                self.calculate_post_main_sequence_properties();
            }
        }

        // Spektraltyp und Leuchtkraftklasse bestimmen
        self.spectral_type = self.determine_spectral_type();
        self.luminosity_class = self.determine_luminosity_class();
    }

    /// Mass-Luminosity Relation für ZAMS (Zero-Age Main Sequence)
    /// Gibt die Leuchtkraft in L☉ zurück.
    fn calculate_luminosity_from_mass(&self) -> f64 {
        let m = self.mass.in_solar_masses();
        // Basierend auf Standard-Mass-Luminosity-Beziehungen, angepasst für ZAMS
        if m < 0.43 {
            0.23 * m.powf(2.3)
        } else if (m - 1.0).abs() < 1e-6 {
            // Speziell für 1 M☉
            0.69 // Literaturwert für ZAMS Sonne (ca. 0.69 - 0.7 L☉)
        } else if m < 2.0 {
            m.powf(4.0) * 0.69 // Skaliert mit M^4 von der ZAMS-Sonne
        } else if m < 55.0 {
            // Erweiterter Bereich für massereichere Sterne
            1.4 * m.powf(3.5)
        } else {
            32000.0 * m // Sehr massive Sterne (vereinfacht)
        }
    }

    /// Mass-Temperature Relation für Hauptreihensterne
    fn calculate_temperature_from_mass(&self) -> f64 {
        // T ~ M^0.5 bis M^0.8 für Hauptreihensterne. M^0.5 ist eine gängige Vereinfachung.
        // Für eine genauere Anpassung, besonders um 1 M☉:
        let m = self.mass.in_solar_masses();
        if (m - 1.0).abs() < 1e-2 {
            // Näherungsweise 1 M☉
            SOLAR_TEMPERATURE // Sollte 5778K für die Sonne ergeben
        } else if m < 2.0 {
            SOLAR_TEMPERATURE * m.powf(0.6) // Etwas steiler für sonnenähnliche
        } else {
            SOLAR_TEMPERATURE * m.powf(0.5)
        }
    }

    /// Stefan-Boltzmann Gesetz: R = sqrt(L / (T/T☉)^4) (in Sonneneinheiten)
    fn calculate_radius_from_luminosity_temperature(&self) -> Distance {
        // self.luminosity ist bereits in L☉
        // self.effective_temperature ist in K
        let radius_solar =
            (self.luminosity / (self.effective_temperature / SOLAR_TEMPERATURE).powf(4.0)).sqrt();

        // Sicherstellen, dass der Radius nicht NaN oder unendlich wird, falls L=0 oder T=0
        if radius_solar.is_nan() || radius_solar.is_infinite() || radius_solar <= 0.0 {
            // Fallback auf eine Mass-Radius-Beziehung für Hauptreihensterne als Notlösung
            // R ~ M^0.8 (für M < 1 M☉) oder R ~ M^0.57 (für M > 1 M☉)
            let mass_solar = self.mass.in_solar_masses();
            let fallback_radius_solar = if mass_solar < 1.0 {
                mass_solar.powf(0.8)
            } else {
                mass_solar.powf(0.57)
            };
            match self.units {
                UnitSystem::Astronomical => {
                    Distance::new(fallback_radius_solar.max(0.01), UnitSystem::Astronomical)
                }
                UnitSystem::SI => Distance::meters(fallback_radius_solar.max(0.01) * SOLAR_RADIUS),
            }
        } else {
            match self.units {
                UnitSystem::Astronomical => Distance::new(radius_solar, UnitSystem::Astronomical),
                UnitSystem::SI => Distance::meters(radius_solar * SOLAR_RADIUS),
            }
        }
    }

    /// Hauptreihen-Lebensdauer basierend auf Masse
    fn calculate_main_sequence_lifetime(&self) -> Time {
        let m_solar = self.mass.in_solar_masses();
        if m_solar <= 0.0 {
            return Time::years(0.0);
        }

        let l_zams = self.calculate_luminosity_from_mass();
        if l_zams <= 0.0 {
            return Time::years(0.0);
        }

        // L_TAMS / L_ZAMS Ratio variiert, ca. 1.5-2.0 für < 2M☉, höher für massereichere
        let l_tams_ratio = if m_solar < 0.5 {
            1.5
        } else if m_solar < 2.0 {
            2.0
        } else if m_solar < 10.0 {
            3.0
        } else {
            5.0 // Grobe Schätzung für sehr massive Sterne
        };

        let avg_luminosity_factor = 1.0 + (l_tams_ratio - 1.0) / 2.0; // Lineare Annahme der Leuchtkraftsteigerung
        let avg_luminosity = l_zams * avg_luminosity_factor;

        // t_MS ≈ 10^10 Jahre * (M/M☉) / (L_avg/L☉)
        let lifetime_years = 1e10 * m_solar / avg_luminosity.max(1e-9); // .max(1e-9) um robuste Division zu gewährleisten
        Time::years(lifetime_years)
    }

    /// Bestimmt das aktuelle Evolutionsstadium
    fn determine_evolutionary_stage(&self) -> EvolutionaryStage {
        let age_years = self.age.in_years();
        let pre_ms_years = self.calculate_pre_main_sequence_time().in_years();
        let ms_lifetime_years = self.main_sequence_lifetime.in_years();

        if age_years < pre_ms_years {
            EvolutionaryStage::PreMainSequence { age: age_years }
        } else if age_years < pre_ms_years + ms_lifetime_years {
            let ms_age = age_years - pre_ms_years;
            let fraction_complete = if ms_lifetime_years > 0.0 {
                (ms_age / ms_lifetime_years).max(0.0).min(1.0)
            } else {
                1.0
            };
            EvolutionaryStage::MainSequence { fraction_complete }
        } else {
            match self.mass.in_solar_masses() {
                m if m < 0.08 => EvolutionaryStage::MainSequence {
                    fraction_complete: 1.0,
                },
                m if m < 0.25 => EvolutionaryStage::BlueDwarf,
                m if m < 8.0 => EvolutionaryStage::RedGiant,
                m if m < 30.0 => EvolutionaryStage::NeutronStar,
                _ => EvolutionaryStage::BlackHole,
            }
        }
    }

    /// Pre-Main Sequence Zeit (Kontraktionszeit bis zur ZAMS)
    fn calculate_pre_main_sequence_time(&self) -> Time {
        let m_solar = self.mass.in_solar_masses();
        let pre_ms_myr = if m_solar >= 1.0 {
            30.0 * m_solar.powf(-2.5)
        } else if m_solar >= 0.5 {
            150.0 - 240.0 * (m_solar - 0.5)
        } else {
            500.0 - 875.0 * (m_solar - 0.1)
        };
        Time::years(pre_ms_myr.max(0.1) * 1e6)
    }

    /// Berechnet Eigenschaften während der Hauptreihe
    fn calculate_main_sequence_properties(&mut self, fraction_complete: f64) {
        let l_zams = self.calculate_luminosity_from_mass();

        let m_solar = self.mass.in_solar_masses();
        let l_tams_ratio = if m_solar < 0.5 {
            1.5
        } else if m_solar < 2.0 {
            2.0
        } else if m_solar < 10.0 {
            3.0
        } else {
            5.0
        };

        self.luminosity = l_zams * (1.0 + (l_tams_ratio - 1.0) * fraction_complete);

        let t_zams = self.calculate_temperature_from_mass();
        let t_tams_factor =
            if self.mass.in_solar_masses() >= 0.8 && self.mass.in_solar_masses() <= 1.2 {
                1.01
            } else {
                1.05
            };
        self.effective_temperature =
            t_zams * (1.0 + (t_tams_factor - 1.0) * fraction_complete.max(0.0).min(1.0));

        self.radius = self.calculate_radius_from_luminosity_temperature();
    }

    /// Pre-Main Sequence Eigenschaften (vereinfacht)
    fn calculate_pre_main_sequence_properties(&mut self) {
        let l_zams = self.calculate_luminosity_from_mass();
        let t_zams = self.calculate_temperature_from_mass();

        self.luminosity = l_zams * 2.0;
        self.effective_temperature = t_zams * 0.8;

        self.radius = self.calculate_radius_from_luminosity_temperature();
    }

    /// Post-Main Sequence Eigenschaften (stark vereinfacht)
    fn calculate_post_main_sequence_properties(&mut self) {
        let l_ms_avg = self.calculate_luminosity_from_mass() * 1.5;

        match &self.evolutionary_stage {
            EvolutionaryStage::RedGiant => {
                self.luminosity = l_ms_avg * 100.0;
                self.effective_temperature = 3500.0;
            }
            EvolutionaryStage::BlueDwarf => {
                self.luminosity = l_ms_avg * 0.5;
                self.effective_temperature = self.calculate_temperature_from_mass() * 1.2;
            }
            _ => {
                self.luminosity = l_ms_avg;
                self.effective_temperature = self.calculate_temperature_from_mass();
            }
        }
        self.radius = self.calculate_radius_from_luminosity_temperature();
    }

    /// Bestimmt Spektraltyp basierend auf Temperatur
    fn determine_spectral_type(&self) -> SpectralType {
        let t = self.effective_temperature;
        if t >= 30000.0 {
            SpectralType::O(((45000.0 - t.min(45000.0)) / 1500.0).floor().min(9.0) as u8)
        } else if t >= 10000.0 {
            SpectralType::B(((30000.0 - t.min(30000.0)) / 2000.0).floor().min(9.0) as u8)
        } else if t >= 7500.0 {
            SpectralType::A(((10000.0 - t.min(10000.0)) / 250.0).floor().min(9.0) as u8)
        } else if t >= 6000.0 {
            SpectralType::F(((7500.0 - t.min(7500.0)) / 150.0).floor().min(9.0) as u8)
        } else if t >= 5200.0 {
            SpectralType::G(((6000.0 - t.min(6000.0)) / 80.0).floor().min(9.0) as u8)
        } else if t >= 3700.0 {
            SpectralType::K(((5200.0 - t.min(5200.0)) / 150.0).floor().min(9.0) as u8)
        } else if t >= 2400.0 {
            SpectralType::M(((3700.0 - t.min(3700.0)) / 130.0).floor().min(9.0) as u8)
        } else if t >= 1300.0 {
            SpectralType::L(((2400.0 - t.min(2400.0)) / 110.0).floor().min(9.0) as u8)
        } else if t >= 500.0 {
            SpectralType::T(((1300.0 - t.min(1300.0)) / 80.0).floor().min(9.0) as u8)
        } else {
            SpectralType::Y(((500.0 - t.min(500.0).max(0.0)) / 25.0).floor().min(9.0) as u8)
        }
    }

    /// Bestimmt Leuchtkraftklasse (vereinfacht)
    fn determine_luminosity_class(&self) -> LuminosityClass {
        match &self.evolutionary_stage {
            EvolutionaryStage::PreMainSequence { .. } => LuminosityClass::V,
            EvolutionaryStage::MainSequence { .. } => LuminosityClass::V,
            EvolutionaryStage::RedGiant | EvolutionaryStage::AsymptoticGiantBranch => {
                LuminosityClass::III
            }
            EvolutionaryStage::HorizontalBranch => LuminosityClass::III,
            EvolutionaryStage::WhiteDwarf { .. } => LuminosityClass::VII,
            _ => LuminosityClass::V,
        }
    }

    /// Berechnet bewohnbare Zone (erweitert mit Einheiten)
    pub fn calculate_habitable_zone(&self) -> HabitableZone {
        let sqrt_l = self.luminosity.sqrt();
        if sqrt_l <= 0.0 || sqrt_l.is_nan() {
            return HabitableZone {
                inner_edge: Distance::new(0.0, self.units),
                outer_edge: Distance::new(0.0, self.units),
                optimistic_inner: Distance::new(0.0, self.units),
                optimistic_outer: Distance::new(0.0, self.units),
            };
        }

        HabitableZone {
            inner_edge: Distance::new(0.95 * sqrt_l, self.units),
            outer_edge: Distance::new(1.37 * sqrt_l, self.units),
            optimistic_inner: Distance::new(0.84 * sqrt_l, self.units),
            optimistic_outer: Distance::new(1.67 * sqrt_l, self.units),
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

        let distance_factor = (0.1 / distance_au.max(0.01)).powf(2.0).min(5.0);
        let tidal_lock_probability = (base_probability * distance_factor).min(1.0);

        let synchronization_years = 1e9 * distance_au.powf(6.0) / (mass_solar.powf(2.0)).max(1e-6);
        let synchronization_timescale = Time::years(synchronization_years);

        let mut possible_resonances = Vec::new();
        if tidal_lock_probability > 0.8 {
            possible_resonances.push((1, 1));
        } else if tidal_lock_probability > 0.3 {
            possible_resonances.push((1, 1));
            possible_resonances.push((3, 2));
            possible_resonances.push((2, 1));
        }

        TidalLockingAnalysis {
            tidal_lock_probability,
            synchronization_timescale,
            possible_resonances,
        }
    }

    /// Escape Velocity von der Sternoberfläche
    pub fn surface_escape_velocity(&self) -> Velocity {
        let radius_in_meters_obj = match self.radius.system {
            UnitSystem::Astronomical => Distance::meters(self.radius.value * SOLAR_RADIUS),
            UnitSystem::SI => self.radius.clone(),
        };
        EscapeVelocity::from_surface(&self.mass, &radius_in_meters_obj)
    }

    /// Orbitale Geschwindigkeit an gegebener Entfernung
    pub fn orbital_velocity_at_distance(&self, distance: &Distance) -> Velocity {
        let gm = G * self.mass.in_kg();
        let r = distance.in_meters();
        if r <= 0.0 {
            return Velocity::meters_per_second(0.0);
        }
        let velocity = (gm / r).sqrt();
        Velocity::meters_per_second(velocity)
    }

    /// Konvertiert zu anderem Einheitensystem
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if target == self.units {
            return self.clone();
        }
        Self {
            mass: self.mass.to_system(target),
            age: self.age.to_system(target),
            main_sequence_lifetime: self.main_sequence_lifetime.to_system(target),
            radius: self.radius.to_system(target),
            units: target,
            luminosity: self.luminosity,
            effective_temperature: self.effective_temperature,
            spectral_type: self.spectral_type.clone(),
            luminosity_class: self.luminosity_class.clone(),
            evolutionary_stage: self.evolutionary_stage.clone(),
            metallicity: self.metallicity,
        }
    }
}
