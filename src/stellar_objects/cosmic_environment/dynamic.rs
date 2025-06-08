use serde::{Deserialize, Serialize};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::physics::constants::{KILOPARSEC_IN_METERS, PI};
use crate::physics::units::{Distance, UnitSystem};
use crate::stellar_objects::cosmic_environment::region::{GalacticRegion, SpiralArmContext, GasDistribution};

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

impl GalacticDynamics {
    /// Berechnet galaktische Dynamik für gegebene Position
    pub fn calculate_for_position(
        region: &GalacticRegion,
        age_gyr: f64,
        rng: &mut ChaCha8Rng,
    ) -> Self {
        let distance_kpc = match region.get_distance_from_center().system {
            UnitSystem::Astronomical => region.get_distance_from_center().value,
            UnitSystem::SI => region.get_distance_from_center().in_meters() / KILOPARSEC_IN_METERS,
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
        let orbital_period = 2.0 * PI * distance_kpc * KILOPARSEC_IN_METERS
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
