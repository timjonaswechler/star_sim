use rand::Rng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::physics::units::Distance;
use crate::stellar_objects::bodies::properties::PhysicalProperties;
use crate::stellar_objects::bodies::surface::{
    composition::SurfaceComposition,
    types::{
        CarbonType, IceType, LavaComposition, MetalType, RegolithGrainSize,
        RockType, SulfurType, SurfaceType, VegetationType,
    },
};

/// Planetenkomposition bestimmt Masse-Radius-Beziehung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanetComposition {
    /// Terrestrische Planeten (Gestein-Metall)
    Terrestrial {
        /// Massenanteil Gestein (Rest ist Metall)
        rock_fraction: f64,
    },
    /// Wasserwelten
    Waterworld {
        /// Massenanteil Wasser
        water_fraction: f64,
        /// Massenanteil Gestein (Rest ist Metall)
        rock_fraction: f64,
    },
    /// Gasriesen
    GasGiant {
        /// Gesamtmetallizität (schwere Elemente)
        bulk_metallicity: f64,
        /// Atmosphären-Metallizität
        atmosphere_metallicity: f64,
    },
    /// Eisriesen
    IceGiant {
        /// Anteil Volatiles (H2O, CH4, NH3)
        volatiles_fraction: f64,
    },
    /// Kohlenstoffplaneten
    CarbonPlanet {
        /// C/O Verhältnis
        carbon_oxygen_ratio: f64,
    },
}

impl SurfaceComposition {
    /// Bestimmt Oberflächentypen basierend auf planetaren Bedingungen
    pub fn determine_surface(
        properties: &PhysicalProperties,
        temperature: f64,
        atmospheric_pressure: Option<f64>,
        stellar_type: &str,
        rng: &mut ChaCha8Rng,
    ) -> Self {
        let mut surfaces = Vec::new();
        let mut primary_surface = SurfaceType::Rock {
            rock_type: RockType::Chondritic,
            weathering_degree: 0.0,
            albedo: 0.3,
        };

        // Temperatur-basierte Oberflächenbestimmung
        if temperature < 63.0 {
            // Sehr kalt: Stickstoffeis möglich
            primary_surface = Self::generate_ice_surface(temperature, rng);
        } else if temperature < 273.0 {
            // Kalt: Wassereis und andere Eis-Typen
            primary_surface = Self::generate_ice_surface(temperature, rng);
        } else if temperature < 373.0 {
            // Gemäßigt: Wasser flüssig, gesteinsreich
            primary_surface =
                Self::generate_temperate_surface(properties, atmospheric_pressure, rng);
        } else if temperature < 1000.0 {
            // Heiß: Gestein dominiert
            primary_surface = Self::generate_hot_surface(temperature, rng);
        } else if temperature < 1500.0 {
            // Sehr heiß: Beginnendes Schmelzen
            primary_surface = Self::generate_molten_surface(temperature, rng);
        } else {
            // Extrem heiß: Lava-Ozeane
            primary_surface = Self::generate_lava_surface(temperature, rng);
        }

        // Sekundäre Oberflächen basierend auf Komposition
        surfaces = Self::generate_secondary_surfaces(properties, temperature, rng);

        // Durchschnittliche Albedo berechnen
        let mut total_albedo = primary_surface.albedo();
        let mut total_coverage = 1.0;

        for (surface, coverage) in &surfaces {
            total_albedo += surface.albedo() * coverage;
            total_coverage += coverage;
        }
        let average_albedo = total_albedo / total_coverage;

        // Temperaturbereich basierend auf Oberfläche
        let temperature_range = Self::calculate_temperature_range(temperature, &primary_surface);

        // Geologische Aktivität
        let geological_activity = Self::estimate_geological_activity(properties, temperature);

        Self {
            primary_surface,
            secondary_surfaces: surfaces,
            average_albedo,
            temperature_range,
            geological_activity,
        }
    }

    /// Generiert Eis-Oberflächen für kalte Welten
    fn generate_ice_surface(temperature: f64, rng: &mut ChaCha8Rng) -> SurfaceType {
        let ice_type = if temperature < 63.0 {
            // Stickstoff kondensiert
            IceType::NitrogenIce
        } else if temperature < 90.0 {
            // Methan kondensiert
            IceType::MethaneIce
        } else if temperature < 195.0 {
            // Ammoniak kondensiert
            IceType::AmmoniaIce
        } else {
            // Wassereis
            IceType::WaterIce
        };

        let thickness = Distance::meters(rng.gen_range(1.0..10000.0));
        let surface_age = rng.gen_range(0.0..1.0);

        // Albedo abhängig von Alter und Typ
        let base_albedo = match ice_type {
            IceType::WaterIce => 0.6,
            IceType::MethaneIce => 0.7,
            IceType::NitrogenIce => 0.8,
            IceType::AmmoniaIce => 0.5,
            _ => 0.6,
        };

        let albedo = base_albedo * (1.0 - surface_age * 0.4); // Altes Eis ist dunkler

        SurfaceType::Ice {
            ice_type,
            thickness,
            surface_age,
            albedo,
        }
    }

    /// Generiert gemäßigte Oberflächen (Earth-like)
    fn generate_temperate_surface(
        properties: &PhysicalProperties,
        atmospheric_pressure: Option<f64>,
        rng: &mut ChaCha8Rng,
    ) -> SurfaceType {
        let has_atmosphere = atmospheric_pressure.unwrap_or(0.0) > 0.1;

        if has_atmosphere && rng.gen_bool(0.3) {
            // Vegetation möglich bei Atmosphäre
            let vegetation_type = match rng.gen_range(0..5) {
                0 => VegetationType::GreenVegetation,
                1 => VegetationType::RedVegetation,
                2 => VegetationType::Forest,
                3 => VegetationType::Grassland,
                _ => VegetationType::MicrobialMats,
            };

            SurfaceType::Vegetation {
                vegetation_type,
                coverage_density: rng.gen_range(0.3..0.9),
                seasonal_variation: rng.gen_range(0.0..0.5),
                albedo: rng.gen_range(0.1..0.25),
            }
        } else {
            // Gesteins-Oberfläche
            let rock_type = if has_atmosphere {
                RockType::Felsic // Kontinentale Kruste
            } else {
                RockType::Mafic // Basaltisch
            };

            SurfaceType::Rock {
                rock_type,
                weathering_degree: if has_atmosphere { 0.6 } else { 0.1 },
                albedo: if has_atmosphere { 0.3 } else { 0.15 },
            }
        }
    }

    /// Generiert heiße Oberflächen
    fn generate_hot_surface(temperature: f64, rng: &mut ChaCha8Rng) -> SurfaceType {
        if temperature > 800.0 && rng.gen_bool(0.2) {
            // Schwefel-Oberflächen bei hohen Temperaturen
            let sulfur_type = if temperature > 900.0 {
                SulfurType::ElementalSulfur
            } else {
                SulfurType::Sulfides
            };

            SurfaceType::Sulfur {
                sulfur_type,
                temperature,
                albedo: 0.2,
            }
        } else {
            // Meist Gestein
            SurfaceType::Rock {
                rock_type: RockType::Mafic,
                weathering_degree: 0.0, // Wenig Verwitterung bei Hitze
                albedo: 0.1,
            }
        }
    }

    /// Generiert geschmolzene Oberflächen
    fn generate_molten_surface(temperature: f64, rng: &mut ChaCha8Rng) -> SurfaceType {
        if rng.gen_bool(0.3) {
            // Glas durch schnelle Abkühlung
            SurfaceType::Glass {
                composition: "Silicate glass".to_string(),
                albedo: 0.1,
            }
        } else {
            // Teilweise geschmolzenes Gestein
            SurfaceType::Rock {
                rock_type: RockType::Mafic,
                weathering_degree: 0.0,
                albedo: 0.08,
            }
        }
    }

    /// Generiert Lava-Oberflächen
    fn generate_lava_surface(temperature: f64, rng: &mut ChaCha8Rng) -> SurfaceType {
        let composition = if temperature > 2000.0 {
            LavaComposition::IronRich
        } else {
            LavaComposition::Silicate
        };

        SurfaceType::Lava {
            composition,
            temperature,
            viscosity: rng.gen_range(1e2..1e6), // Pa⋅s
            albedo: 0.05,                       // Sehr dunkel, aber glüht selbst
        }
    }

    /// Generiert sekundäre Oberflächentypen
    fn generate_secondary_surfaces(
        properties: &PhysicalProperties,
        temperature: f64,
        rng: &mut ChaCha8Rng,
    ) -> Vec<(SurfaceType, f64)> {
        let mut surfaces = Vec::new();

        // Regolith ist fast überall möglich
        if rng.gen_bool(0.7) {
            let grain_size = match temperature {
                t if t < 273.0 => RegolithGrainSize::Sand,
                t if t < 373.0 => RegolithGrainSize::Silt,
                _ => RegolithGrainSize::Clay,
            };

            surfaces.push((
                SurfaceType::Regolith {
                    source_material: "Impact debris".to_string(),
                    grain_size,
                    albedo: rng.gen_range(0.15..0.4),
                },
                rng.gen_range(0.1..0.3),
            ));
        }

        // Kohlenstoff-Oberflächen bei hohem C/O-Verhältnis
        if let PlanetComposition::CarbonPlanet {
            carbon_oxygen_ratio,
        } = &properties.composition
        {
            if *carbon_oxygen_ratio > 0.8 {
                let carbon_type = if temperature > 1000.0 {
                    CarbonType::Graphite
                } else {
                    CarbonType::Diamond
                };

                surfaces.push((
                    SurfaceType::Carbon {
                        carbon_type: carbon_type.clone(),
                        albedo: if matches!(carbon_type, CarbonType::Diamond) {
                            0.8
                        } else {
                            0.04
                        },
                    },
                    rng.gen_range(0.2..0.6),
                ));
            }
        }

        // Metall-Oberflächen für sehr hohe Temperaturen oder spezielle Körper
        if temperature > 1200.0 && rng.gen_bool(0.1) {
            surfaces.push((
                SurfaceType::Metal {
                    metal_type: MetalType::IronNickel,
                    oxidation_level: 0.5,
                    albedo: 0.15,
                },
                rng.gen_range(0.05..0.2),
            ));
        }

        surfaces
    }

    /// Berechnet Temperaturbereich der Oberfläche
    fn calculate_temperature_range(base_temp: f64, surface: &SurfaceType) -> (f64, f64) {
        let variation = match surface {
            SurfaceType::Ice { .. } => 20.0,        // Eis stabilisiert
            SurfaceType::Ocean { .. } => 10.0,      // Wasser puffert
            SurfaceType::Vegetation { .. } => 15.0, // Biologische Pufferung
            SurfaceType::Rock { .. } => 50.0,       // Standard Variation
            SurfaceType::Lava { .. } => 200.0,      // Große Variation
            _ => 30.0,
        };

        (base_temp - variation, base_temp + variation)
    }

    /// Schätzt geologische Aktivität
    fn estimate_geological_activity(properties: &PhysicalProperties, temperature: f64) -> f64 {
        let mass_factor = (properties.mass.in_earth_masses() / 5.0).min(1.0); // Größere Planeten aktiver
        let temp_factor = if temperature > 1000.0 { 1.0 } else { 0.3 };
        let density_factor = (properties.density / 5.0).min(1.0); // Dichtere Planeten aktiver

        (mass_factor * temp_factor * density_factor).min(1.0)
    }
}
