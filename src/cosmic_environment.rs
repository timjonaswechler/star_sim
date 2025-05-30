// cosmic_environment.rs - Kosmische Umgebung und galaktische Einflüsse

use rand::Rng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::units::*;

/// Kosmische Epoche und Zeitrahmen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicEpoch {
    /// Alter des Universums in Milliarden Jahren
    pub age_universe: f64,
    /// Kosmische Ära (String-Beschreibung)
    pub era: String,
    /// Sternentstehungsrate relativ zu heute
    pub star_formation_rate: f64,
    /// Durchschnittliche Metallizität in dieser Epoche
    pub epoch_metallicity: f64,
    /// Rotverschiebung (z)
    pub redshift: f64,
    /// Hubble-Parameter H(z)
    pub hubble_parameter: f64,
}

/// Galaktische Regionen und ihre Eigenschaften
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GalacticRegion {
    /// Galaktisches Zentrum (0-1 kpc)
    Core {
        distance_from_center: Distance,
        supermassive_black_hole_influence: f64,
    },
    /// Innere Bulge (1-4 kpc)
    InnerBulge {
        distance_from_center: Distance,
        stellar_density: f64,
    },
    /// Galaktische bewohnbare Zone (4-10 kpc)
    HabitableZone {
        distance_from_center: Distance,
        metallicity_gradient: f64,
    },
    /// Äußere Scheibe (10-20 kpc)
    OuterDisk {
        distance_from_center: Distance,
        gas_density: f64,
    },
    /// Galaktischer Halo (>20 kpc)
    Halo {
        distance_from_center: Distance,
        dark_matter_density: f64,
    },
}

/// Kosmische Strahlungsumgebung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicRadiationEnvironment {
    /// Risiko durch aktive galaktische Kerne (AGN)
    pub agn_risk: f64,
    /// Supernova-Häufigkeit in der Umgebung
    pub supernova_frequency: f64,
    /// Gamma-Ray Burst Risiko
    pub grb_risk: f64,
    /// Rate stellarer Begegnungen
    pub stellar_encounter_rate: f64,
    /// Kosmische Strahlung (GeV/cm²/s)
    pub cosmic_ray_flux: f64,
    /// UV-Hintergrundstrahlung
    pub uv_background: f64,
    /// Gravitationswellen-Aktivität
    pub gravitational_wave_activity: f64,
}

/// Elementhäufigkeiten in der kosmischen Umgebung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalAbundance {
    /// Wasserstoff (Massenanteil)
    pub hydrogen: f64,
    /// Helium (Massenanteil)
    pub helium: f64,
    /// Lithium (Massenanteil)
    pub lithium: f64,
    /// Kohlenstoff (Massenanteil)
    pub carbon: f64,
    /// Stickstoff (Massenanteil)
    pub nitrogen: f64,
    /// Sauerstoff (Massenanteil)
    pub oxygen: f64,
    /// Schwere Metalle (Z > 8, Massenanteil)
    pub heavy_metals: f64,
    /// Alpha-Elemente (O, Ne, Mg, Si, S, Ar, Ca, Ti)
    pub alpha_elements: f64,
    /// Eisengruppe (Fe, Co, Ni)
    pub iron_group: f64,
    /// s-Prozess Elemente
    pub s_process_elements: f64,
    /// r-Prozess Elemente
    pub r_process_elements: f64,
}

/// Galaktische Dynamik und Struktur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalacticDynamics {
    /// Rotationsgeschwindigkeit an dieser Position (km/s)
    pub rotation_velocity: f64,
    /// Galaktische Umlaufperiode (Myr)
    pub orbital_period: f64,
    /// Vertikale Oszillation durch die galaktische Scheibe
    pub vertical_oscillation: VerticalOscillation,
    /// Spiralarm-Struktur
    pub spiral_arm_context: SpiralArmContext,
    /// Lokale Sternendichte (Sterne/pc³)
    pub local_stellar_density: f64,
    /// Gasverteilung
    pub gas_distribution: GasDistribution,
}

/// Vertikale Oszillation durch die galaktische Scheibe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalOscillation {
    /// Amplitude der Oszillation (pc)
    pub amplitude: f64,
    /// Periode der Oszillation (Myr)
    pub period: f64,
    /// Aktuelle Phase (0.0-1.0)
    pub current_phase: f64,
    /// Geschwindigkeit durch die galaktische Ebene (km/s)
    pub velocity_z: f64,
}

/// Kontext zu Spiralarmen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpiralArmContext {
    /// Im Spiralarm
    InArm {
        arm_name: String,
        position_in_arm: f64, // 0.0 = Zentrum, 1.0 = Rand
    },
    /// Zwischen Spiralarmen
    InterArm {
        distance_to_nearest_arm: Distance,
        nearest_arm_name: String,
    },
    /// Spiralarm-Korotationsresonanz
    CorotationResonance,
    /// Lindblad-Resonanz
    LindBladResonance,
}

/// Gasverteilung in der galaktischen Umgebung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasDistribution {
    /// HI (neutraler Wasserstoff) Dichte (cm⁻³)
    pub hi_density: f64,
    /// H₂ (molekularer Wasserstoff) Dichte (cm⁻³)
    pub h2_density: f64,
    /// Ionisierter Wasserstoff Dichte (cm⁻³)
    pub hii_density: f64,
    /// Helium Dichte (cm⁻³)
    pub helium_density: f64,
    /// Staub-zu-Gas Verhältnis
    pub dust_to_gas_ratio: f64,
    /// Turbulenz-Geschwindigkeit (km/s)
    pub turbulence_velocity: f64,
}

impl CosmicEpoch {
    /// Erstellt eine kosmische Epoche für gegebenes Universumsalter
    pub fn from_age(age_gyr: f64) -> Self {
        let era = match age_gyr {
            age if age < 0.5 => "Primordial Era".to_string(),
            age if age < 2.0 => "Early Universe".to_string(),
            age if age < 6.0 => "Peak Star Formation".to_string(),
            age if age < 10.0 => "Stellar Era".to_string(),
            age if age < 13.0 => "Mature Universe".to_string(),
            _ => "Late Universe".to_string(),
        };

        // Vereinfachte Sternentstehungsrate (relativ zu heute)
        let star_formation_rate = match age_gyr {
            age if age < 1.0 => 0.1,
            age if age < 3.0 => 10.0, // Peak bei z~2-3
            age if age < 8.0 => 3.0,
            age if age < 11.0 => 1.0,
            _ => 0.3,
        };

        // Metallizität entwickelt sich mit der Zeit
        let epoch_metallicity = match age_gyr {
            age if age < 0.5 => -3.0, // Sehr metall-arm
            age if age < 2.0 => -1.5,
            age if age < 6.0 => -0.5,
            age if age < 10.0 => 0.0,
            _ => 0.2, // Leicht metall-reich
        };

        // Redshift approximation
        let redshift = ((13.8 / age_gyr) - 1.0).max(0.0);

        // Hubble parameter (vereinfacht)
        let hubble_parameter = 70.0 * (1.0 + redshift).sqrt(); // km/s/Mpc

        Self {
            age_universe: age_gyr,
            era,
            star_formation_rate,
            epoch_metallicity,
            redshift,
            hubble_parameter,
        }
    }

    /// Ist das Universum alt genug für komplexe Chemie?
    pub fn allows_complex_chemistry(&self) -> bool {
        self.age_universe > 1.0 && self.epoch_metallicity > -2.0
    }

    /// Ist das Universum alt genug für langlebige Sterne?
    pub fn allows_long_lived_stars(&self) -> bool {
        self.age_universe > 0.5
    }
}

impl GalacticRegion {
    /// Generiert eine zufällige galaktische Region
    pub fn generate_random(rng: &mut ChaCha8Rng, unit_system: UnitSystem) -> Self {
        let r: f64 = rng.r#gen();
        let distance_kpc = match r {
            x if x < 0.05 => rng.gen_range(0.0..1.0),   // 5% Core
            x if x < 0.15 => rng.gen_range(1.0..4.0),   // 10% Inner Bulge
            x if x < 0.70 => rng.gen_range(4.0..10.0),  // 55% Habitable Zone
            x if x < 0.90 => rng.gen_range(10.0..20.0), // 20% Outer Disk
            _ => rng.gen_range(20.0..50.0),             // 10% Halo
        };

        let distance = match unit_system {
            UnitSystem::Astronomical => Distance::new(distance_kpc, unit_system),
            UnitSystem::SI => Distance::meters(distance_kpc * KILOPARSEC_TO_METERS),
        };

        match distance_kpc {
            d if d < 1.0 => Self::Core {
                distance_from_center: distance,
                supermassive_black_hole_influence: rng.gen_range(0.5..1.0),
            },
            d if d < 4.0 => Self::InnerBulge {
                distance_from_center: distance,
                stellar_density: rng.gen_range(100.0..1000.0), // stars/pc³
            },
            d if d < 10.0 => Self::HabitableZone {
                distance_from_center: distance,
                metallicity_gradient: rng.gen_range(-0.1..0.1),
            },
            d if d < 20.0 => Self::OuterDisk {
                distance_from_center: distance,
                gas_density: rng.gen_range(0.1..1.0), // atoms/cm³
            },
            _ => Self::Halo {
                distance_from_center: distance,
                dark_matter_density: rng.gen_range(0.01..0.1),
            },
        }
    }

    /// Gibt die Entfernung vom galaktischen Zentrum zurück
    pub fn distance_from_center(&self) -> &Distance {
        match self {
            Self::Core {
                distance_from_center,
                ..
            } => distance_from_center,
            Self::InnerBulge {
                distance_from_center,
                ..
            } => distance_from_center,
            Self::HabitableZone {
                distance_from_center,
                ..
            } => distance_from_center,
            Self::OuterDisk {
                distance_from_center,
                ..
            } => distance_from_center,
            Self::Halo {
                distance_from_center,
                ..
            } => distance_from_center,
        }
    }

    /// Bewertet das Bewohnbarkeitspotenzial dieser galaktischen Region
    pub fn habitability_factor(&self) -> f64 {
        match self {
            Self::Core {
                supermassive_black_hole_influence,
                ..
            } => {
                (1.0 - supermassive_black_hole_influence) * 0.1 // Sehr gefährlich
            }
            Self::InnerBulge {
                stellar_density, ..
            } => {
                if *stellar_density > 500.0 {
                    0.3 // Zu viele störende Einflüsse
                } else {
                    0.6 // Moderate Bedingungen
                }
            }
            Self::HabitableZone {
                metallicity_gradient,
                ..
            } => {
                0.9 + metallicity_gradient * 0.1 // Optimal für Leben
            }
            Self::OuterDisk { gas_density, .. } => {
                if *gas_density < 0.3 {
                    0.7 // Wenig Gas, aber stabil
                } else {
                    0.5 // Viel Gas, aber mehr Störungen
                }
            }
            Self::Halo { .. } => 0.2, // Wenig Ressourcen
        }
    }
}

impl CosmicRadiationEnvironment {
    /// Erstellt Strahlungsumgebung basierend auf galaktischer Region und Epoche
    pub fn from_region_and_epoch(
        region: &GalacticRegion,
        epoch: &CosmicEpoch,
        rng: &mut ChaCha8Rng,
    ) -> Self {
        let age_factor = if epoch.age_universe < 4.0 { 2.0 } else { 1.0 };
        let distance_kpc = match region.distance_from_center().system {
            UnitSystem::Astronomical => region.distance_from_center().value,
            UnitSystem::SI => region.distance_from_center().in_meters() / KILOPARSEC_TO_METERS,
        };

        let (base_agn, base_sn, base_grb) = match region {
            GalacticRegion::Core { .. } => (0.9, 0.8, 0.7),
            GalacticRegion::InnerBulge { .. } => (0.6, 0.6, 0.5),
            GalacticRegion::HabitableZone { .. } => (0.2, 0.3, 0.3),
            GalacticRegion::OuterDisk { .. } => (0.1, 0.2, 0.4),
            GalacticRegion::Halo { .. } => (0.1, 0.1, 0.5),
        };

        let agn_risk: f64 = base_agn * age_factor * rng.gen_range(0.5..1.5);
        let supernova_frequency: f64 = base_sn * age_factor * rng.gen_range(0.5..1.5);
        let grb_risk: f64 = base_grb * rng.gen_range(0.3..1.0);

        // Stellar encounter rate (abhängig von Sternendichte)
        let stellar_encounter_rate = match region {
            GalacticRegion::Core { .. } => 0.9,
            GalacticRegion::InnerBulge {
                stellar_density, ..
            } => (stellar_density / 1000.0).min(1.0),
            _ => 0.1,
        };

        // Kosmische Strahlung (vereinfacht)
        let cosmic_ray_flux = (10.0 / distance_kpc).min(100.0); // GeV/cm²/s

        // UV-Hintergrund
        let uv_background: f64 = match epoch.age_universe {
            age if age < 2.0 => rng.gen_range(5.0..10.0),
            age if age < 6.0 => rng.gen_range(2.0..5.0),
            _ => rng.gen_range(1.0..2.0),
        };

        // Gravitationswellen (sehr vereinfacht)
        let gravitational_wave_activity = rng.gen_range(0.1..0.5);

        Self {
            agn_risk: agn_risk.min(1.0),
            supernova_frequency: supernova_frequency.min(1.0),
            grb_risk: grb_risk.min(1.0),
            stellar_encounter_rate,
            cosmic_ray_flux,
            uv_background,
            gravitational_wave_activity,
        }
    }

    /// Berechnet den Gesamtstrahlungsrisiko-Faktor
    pub fn total_radiation_risk(&self) -> f64 {
        (self.agn_risk
            + self.supernova_frequency
            + self.grb_risk
            + self.cosmic_ray_flux / 100.0
            + self.uv_background / 10.0)
            / 5.0
    }

    /// Ist die Umgebung sicher genug für Leben?
    pub fn is_life_friendly(&self) -> bool {
        self.total_radiation_risk() < 0.5
    }
}

impl ElementalAbundance {
    /// Erstellt Elementhäufigkeiten basierend auf Metallizität und kosmischer Epoche
    pub fn from_metallicity_and_epoch(metallicity: f64, epoch: &CosmicEpoch) -> Self {
        // Big Bang Nukleosynthese
        let primordial_h = 0.75;
        let primordial_he = 0.25;

        // Metallizität entwickelt sich mit der Zeit
        let metal_fraction = 10_f64.powf(metallicity) * 0.02 * (epoch.age_universe / 13.8);

        // Typ Ia vs Typ II Supernova Beiträge
        let alpha_enhancement = if epoch.age_universe < 3.0 {
            0.3 // Früh dominieren Type II SNe
        } else {
            0.0 // Später ausbalanciert durch Type Ia
        };

        // Grundverteilung der schweren Elemente
        let carbon = metal_fraction * 0.20;
        let nitrogen = metal_fraction * 0.05;
        let oxygen = metal_fraction * 0.45 * (1.0 + alpha_enhancement);
        let alpha_elements = metal_fraction * 0.25 * (1.0 + alpha_enhancement);
        let iron_group = metal_fraction * 0.15;

        // s- und r-Prozess Elemente (abhängig von Sternentstehungsgeschichte)
        let s_process_elements = metal_fraction * 0.05 * (epoch.age_universe / 10.0);
        let r_process_elements = metal_fraction * 0.03;

        let heavy_metals = metal_fraction - carbon - nitrogen - oxygen;

        Self {
            hydrogen: primordial_h - metal_fraction * 0.5,
            helium: primordial_he - metal_fraction * 0.3
                + primordial_he * 0.01 * epoch.age_universe / 13.8,
            lithium: 1e-9 * (1.0 + epoch.age_universe / 10.0), // Lithium production
            carbon,
            nitrogen,
            oxygen,
            heavy_metals: heavy_metals.max(0.0),
            alpha_elements,
            iron_group,
            s_process_elements,
            r_process_elements,
        }
    }

    /// Berechnet das C/O Verhältnis (wichtig für Planetenbildung)
    pub fn carbon_to_oxygen_ratio(&self) -> f64 {
        if self.oxygen > 0.0 {
            self.carbon / self.oxygen
        } else {
            0.0
        }
    }

    /// Ist genug Material für terrestrische Planeten vorhanden?
    pub fn supports_terrestrial_planets(&self) -> bool {
        self.oxygen + self.alpha_elements + self.iron_group > 0.01
    }

    /// Bewertung für Astrobiologie (0.0-1.0)
    pub fn astrobiological_potential(&self) -> f64 {
        let cnos = self.carbon + self.nitrogen + self.oxygen + self.heavy_metals * 0.1;
        cnos.min(0.1) * 10.0 // Normiert auf 0.0-1.0
    }
}

impl GalacticDynamics {
    /// Berechnet galaktische Dynamik für gegebene Position
    pub fn calculate_for_position(
        region: &GalacticRegion,
        age_gyr: f64,
        rng: &mut ChaCha8Rng,
    ) -> Self {
        let distance_kpc = match region.distance_from_center().system {
            UnitSystem::Astronomical => region.distance_from_center().value,
            UnitSystem::SI => region.distance_from_center().in_meters() / KILOPARSEC_TO_METERS,
        };

        // Rotationskurve der Milchstraße (vereinfacht)
        let rotation_velocity = if distance_kpc < 1.0 {
            200.0 * distance_kpc // Solid body rotation
        } else if distance_kpc < 10.0 {
            220.0 // Flat rotation curve
        } else {
            220.0 * (10.0 / distance_kpc).sqrt() // Keplerian falloff
        };

        // Umlaufperiode
        let orbital_period = 2.0 * PI * distance_kpc * KILOPARSEC_TO_METERS
            / (rotation_velocity * 1000.0)
            / (1e6 * 365.25 * 24.0 * 3600.0); // Myr

        // Vertikale Oszillation
        let vertical_oscillation = VerticalOscillation {
            amplitude: rng.gen_range(50.0..200.0), // pc
            period: rng.gen_range(60.0..100.0),    // Myr
            current_phase: rng.gen_range(0.0..1.0),
            velocity_z: rng.gen_range(-20.0..20.0), // km/s
        };

        // Spiralarm-Kontext
        let spiral_arm_context = Self::determine_spiral_arm_context(distance_kpc, age_gyr, rng);

        // Lokale Sternendichte
        let local_stellar_density = match region {
            GalacticRegion::Core { .. } => rng.gen_range(1000.0..10000.0),
            GalacticRegion::InnerBulge {
                stellar_density, ..
            } => *stellar_density,
            GalacticRegion::HabitableZone { .. } => rng.gen_range(0.1..1.0),
            GalacticRegion::OuterDisk { .. } => rng.gen_range(0.01..0.1),
            GalacticRegion::Halo { .. } => rng.gen_range(0.001..0.01),
        };

        // Gasverteilung
        let gas_distribution = GasDistribution {
            hi_density: rng.gen_range(0.1..2.0),      // cm⁻³
            h2_density: rng.gen_range(1.0..100.0),    // cm⁻³
            hii_density: rng.gen_range(0.01..0.1),    // cm⁻³
            helium_density: rng.gen_range(0.01..0.2), // cm⁻³
            dust_to_gas_ratio: rng.gen_range(0.001..0.01),
            turbulence_velocity: rng.gen_range(5.0..15.0), // km/s
        };

        Self {
            rotation_velocity,
            orbital_period,
            vertical_oscillation,
            spiral_arm_context,
            local_stellar_density,
            gas_distribution,
        }
    }

    fn determine_spiral_arm_context(
        distance_kpc: f64,
        _age_gyr: f64,
        rng: &mut ChaCha8Rng,
    ) -> SpiralArmContext {
        // Vereinfachte Spiralarm-Logik
        if distance_kpc < 3.0 || distance_kpc > 15.0 {
            return SpiralArmContext::InterArm {
                distance_to_nearest_arm: Distance::new(
                    rng.gen_range(0.5..2.0),
                    UnitSystem::Astronomical,
                ),
                nearest_arm_name: "Perseus Arm".to_string(),
            };
        }

        let arm_probability = rng.r#gen::<f64>();
        if arm_probability < 0.3 {
            let arms = ["Perseus Arm", "Sagittarius Arm", "Norma Arm", "Scutum Arm"];
            SpiralArmContext::InArm {
                arm_name: arms[rng.gen_range(0..arms.len())].to_string(),
                position_in_arm: rng.gen_range(0.0..1.0),
            }
        } else if arm_probability < 0.9 {
            SpiralArmContext::InterArm {
                distance_to_nearest_arm: Distance::new(
                    rng.gen_range(0.2..1.0),
                    UnitSystem::Astronomical,
                ),
                nearest_arm_name: "Perseus Arm".to_string(),
            }
        } else if arm_probability < 0.95 {
            SpiralArmContext::CorotationResonance
        } else {
            SpiralArmContext::LindBladResonance
        }
    }

    /// Bewertung der Stabilität der galaktischen Umgebung
    pub fn environmental_stability(&self) -> f64 {
        let mut stability = 1.0;

        // Spiralarm-Einfluss
        match &self.spiral_arm_context {
            SpiralArmContext::InArm { .. } => stability *= 0.7, // Mehr Störungen
            SpiralArmContext::InterArm { .. } => stability *= 1.0, // Stabiler
            SpiralArmContext::CorotationResonance => stability *= 0.8,
            SpiralArmContext::LindBladResonance => stability *= 0.6,
        }

        // Stellare Dichte
        if self.local_stellar_density > 10.0 {
            stability *= 0.8; // Zu viele Störungen
        } else if self.local_stellar_density < 0.01 {
            stability *= 0.9; // Zu isoliert
        }

        // Turbulenz
        if self.gas_distribution.turbulence_velocity > 20.0 {
            stability *= 0.7; // Hohe Turbulenz
        }

        stability
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_cosmic_epoch() {
        let early_epoch = CosmicEpoch::from_age(1.0);
        let modern_epoch = CosmicEpoch::from_age(13.8);

        assert_eq!(early_epoch.era, "Early Universe");
        assert_eq!(modern_epoch.era, "Late Universe");

        assert!(early_epoch.star_formation_rate > modern_epoch.star_formation_rate);
        assert!(early_epoch.epoch_metallicity < modern_epoch.epoch_metallicity);

        assert!(!early_epoch.allows_complex_chemistry());
        assert!(modern_epoch.allows_complex_chemistry());
    }

    #[test]
    fn test_galactic_region_habitability() {
        let core = GalacticRegion::Core {
            distance_from_center: Distance::new(0.5, UnitSystem::Astronomical),
            supermassive_black_hole_influence: 0.8,
        };

        let habitable_zone = GalacticRegion::HabitableZone {
            distance_from_center: Distance::new(8.0, UnitSystem::Astronomical),
            metallicity_gradient: 0.0,
        };

        assert!(core.habitability_factor() < 0.5);
        assert!(habitable_zone.habitability_factor() > 0.8);
    }

    #[test]
    fn test_elemental_abundance() {
        let early_epoch = CosmicEpoch::from_age(2.0);
        let modern_epoch = CosmicEpoch::from_age(13.8);

        let early_abundance = ElementalAbundance::from_metallicity_and_epoch(0.0, &early_epoch);
        let modern_abundance = ElementalAbundance::from_metallicity_and_epoch(0.0, &modern_epoch);

        // Moderne Epoche sollte mehr schwere Elemente haben
        assert!(modern_abundance.oxygen > early_abundance.oxygen);
        assert!(modern_abundance.iron_group > early_abundance.iron_group);

        // Wasserstoff/Helium sollten abnehmen
        assert!(early_abundance.hydrogen > modern_abundance.hydrogen);
    }

    #[test]
    fn test_radiation_environment() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let epoch = CosmicEpoch::from_age(10.0);

        let core_region = GalacticRegion::Core {
            distance_from_center: Distance::new(0.5, UnitSystem::Astronomical),
            supermassive_black_hole_influence: 0.8,
        };

        let hz_region = GalacticRegion::HabitableZone {
            distance_from_center: Distance::new(8.0, UnitSystem::Astronomical),
            metallicity_gradient: 0.0,
        };

        let core_radiation =
            CosmicRadiationEnvironment::from_region_and_epoch(&core_region, &epoch, &mut rng);
        let hz_radiation =
            CosmicRadiationEnvironment::from_region_and_epoch(&hz_region, &epoch, &mut rng);

        assert!(core_radiation.total_radiation_risk() > hz_radiation.total_radiation_risk());
        assert!(!core_radiation.is_life_friendly());
        assert!(hz_radiation.is_life_friendly());
    }
}
