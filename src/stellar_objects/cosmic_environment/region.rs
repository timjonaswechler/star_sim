use serde::{Deserialize, Serialize};

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
    pub fn get_distance_from_center(&self) -> &Distance {
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
                    0.2 // Zu viele störende Einflüsse
                } else {
                    0.4 // Moderate Bedingungen
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
