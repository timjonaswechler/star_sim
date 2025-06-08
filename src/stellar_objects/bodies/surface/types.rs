use serde::{Deserialize, Serialize};
use crate::physics::units::Distance;

/// Verschiedene Oberflächentypen aus dem Artikel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceType {
    /// Gesteins-Oberflächen
    Rock {
        rock_type: RockType,
        weathering_degree: f64, // 0.0 = frisch, 1.0 = stark verwittert
        albedo: f64,
    },
    /// Metall-Oberflächen (exponierte Kerne)
    Metal {
        metal_type: MetalType,
        oxidation_level: f64, // 0.0 = rein, 1.0 = vollständig oxidiert
        albedo: f64,
    },
    /// Regolith (loses Material)
    Regolith {
        source_material: String,
        grain_size: RegolithGrainSize,
        albedo: f64,
    },
    /// Eis-Oberflächen
    Ice {
        ice_type: IceType,
        thickness: Distance, // Dicke der Eisschicht
        surface_age: f64,    // 0.0 = frisch, 1.0 = alt/verschmutzt
        albedo: f64,
    },
    /// Kohlenstoff-Oberflächen
    Carbon {
        carbon_type: CarbonType,
        albedo: f64,
    },
    /// Schwefel-Oberflächen
    Sulfur {
        sulfur_type: SulfurType,
        temperature: f64, // Bestimmt Farbe
        albedo: f64,
    },
    /// Glas-Oberflächen (schnell abgekühlte Magma-Ozeane)
    Glass { composition: String, albedo: f64 },
    /// Vegetation (biologische Oberflächen)
    Vegetation {
        vegetation_type: VegetationType,
        coverage_density: f64, // 0.0-1.0
        seasonal_variation: f64,
        albedo: f64,
    },
    /// Lava-Oberflächen (geschmolzen)
    Lava {
        composition: LavaComposition,
        temperature: f64,
        viscosity: f64,
        albedo: f64,
    },
    /// Strange Matter (hochspekulative Physik)
    StrangeMatter {
        density: f64,   // g/cm³
        stability: f64, // 0.0-1.0
        albedo: f64,
    },
    /// Ozean-Oberfläche (wird in ocean_types.rs detaillierter behandelt)
    Ocean {
        liquid_type: String,
        depth: Distance,
        albedo: f64,
    },
}

/// Gesteinstypen aus dem Artikel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RockType {
    /// Chondritisch (ursprüngliches Material)
    Chondritic,
    /// Mafisch (basaltisch, dunkel)
    Mafic,
    /// Felsisch (granitisch, hell)
    Felsic,
    /// Ultramafisch (olivinreich)
    Ultramafic,
    /// Anorthositisch (feldspatreich)
    Anorthositic,
    /// Sedimentär
    Sedimentary,
    /// Metamorph
    Metamorphic,
}

/// Metalltypen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetalType {
    /// Eisen-Nickel (häufigste Kern-Zusammensetzung)
    IronNickel,
    /// Reines Eisen
    Iron,
    /// Alkalimetalle (Natrium, Kalium)
    AlkaliMetals,
    /// Sulfide (Pyrit, etc.)
    Sulfides,
    /// Oxide (Rost, etc.)
    Oxides,
}

/// Regolith-Korngrößen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegolithGrainSize {
    /// Ton-ähnlich (sehr fein, <2μm)
    Clay,
    /// Silt (2-50μm)
    Silt,
    /// Sand (50μm-2mm)
    Sand,
    /// Kies (2-64mm)
    Gravel,
    /// Geröll (>64mm)
    Boulder,
}

/// Eistypen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IceType {
    /// Wassereis (häufigster Typ)
    WaterIce,
    /// Methaneis
    MethaneIce,
    /// Stickstoffeis
    NitrogenIce,
    /// Ammoniakeis
    AmmoniaIce,
    /// CO2-Eis (Trockeneis)
    CarbonDioxideIce,
    /// Kohlenmonoxid-Eis
    CarbonMonoxideIce,
    /// Mischeis (mehrere Komponenten)
    MixedIce { components: Vec<String> },
}

/// Kohlenstoff-Oberflächentypen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarbonType {
    /// Graphit (sehr dunkel)
    Graphite,
    /// Diamant (sehr hell)
    Diamond,
    /// Carbide (Titanium-Carbid, etc.)
    Carbides,
    /// Organische Polymere (teerartig)
    OrganicPolymers,
    /// Tholins (komplexe organische Verbindungen)
    Tholins,
}

/// Schwefel-Oberflächentypen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SulfurType {
    /// Elementarer Schwefel
    ElementalSulfur,
    /// Schwefeldioxid-Eis
    SulfurDioxideIce,
    /// Sulfide (Pyrit, etc.)
    Sulfides,
    /// Sulfate (Gips, etc.)
    Sulfates,
    /// Schwefelsäure
    SulfuricAcid,
}

/// Vegetationstypen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VegetationType {
    /// Grüne Vegetation (Earth-like)
    GreenVegetation,
    /// Rote Vegetation (angepasst an andere Sterne)
    RedVegetation,
    /// Blaue Vegetation
    BlueVegetation,
    /// Violette Vegetation
    PurpleVegetation,
    /// Fluoreszierende Vegetation (UV-Schutz)
    FluorescentVegetation,
    /// Grasland
    Grassland,
    /// Wald
    Forest,
    /// Mikrobenmatten
    MicrobialMats,
}

/// Lava-Zusammensetzungen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LavaComposition {
    /// Silikat-Lava (gewöhnlich)
    Silicate,
    /// Eisenreiche Lava
    IronRich,
    /// Schwefel-Lava
    Sulfur,
    /// Alkalimetall-Lava
    AlkaliMetals,
    /// Karbonat-Lava (CO2-reich)
    Carbonate,
}

impl SurfaceType {
    /// Gibt die Albedo des Oberflächentyps zurück
    pub fn albedo(&self) -> f64 {
        match self {
            SurfaceType::Rock { albedo, .. } => *albedo,
            SurfaceType::Metal { albedo, .. } => *albedo,
            SurfaceType::Regolith { albedo, .. } => *albedo,
            SurfaceType::Ice { albedo, .. } => *albedo,
            SurfaceType::Carbon { albedo, .. } => *albedo,
            SurfaceType::Sulfur { albedo, .. } => *albedo,
            SurfaceType::Glass { albedo, .. } => *albedo,
            SurfaceType::Vegetation { albedo, .. } => *albedo,
            SurfaceType::Lava { albedo, .. } => *albedo,
            SurfaceType::StrangeMatter { albedo, .. } => *albedo,
            SurfaceType::Ocean { albedo, .. } => *albedo,
        }
    }

    /// Beschreibt die Farbe der Oberfläche
    pub fn color_description(&self) -> String {
        match self {
            SurfaceType::Rock { rock_type, .. } => match rock_type {
                RockType::Chondritic => "grau-braun".to_string(),
                RockType::Mafic => "dunkel grau bis schwarz".to_string(),
                RockType::Felsic => "hell grau bis weiß".to_string(),
                RockType::Ultramafic => "grün".to_string(),
                RockType::Anorthositic => "hell grau".to_string(),
                RockType::Sedimentary => "variabel, meist braun".to_string(),
                RockType::Metamorphic => "variabel".to_string(),
            },
            SurfaceType::Metal {
                metal_type,
                oxidation_level,
                ..
            } => {
                if *oxidation_level > 0.5 {
                    match metal_type {
                        MetalType::IronNickel => "rostrot".to_string(),
                        MetalType::Iron => "rostrot".to_string(),
                        _ => "oxidiert, dunkel".to_string(),
                    }
                } else {
                    "silbrig-metallisch".to_string()
                }
            }
            SurfaceType::Ice {
                ice_type,
                surface_age,
                ..
            } => {
                let base_color = match ice_type {
                    IceType::WaterIce => "weiß bis bläulich",
                    IceType::MethaneIce => "rosa bis rötlich",
                    IceType::NitrogenIce => "rosa",
                    IceType::AmmoniaIce => "gelblich",
                    IceType::CarbonDioxideIce => "weiß",
                    _ => "weiß",
                };
                if *surface_age > 0.5 {
                    format!("{}, verschmutzt grau-braun", base_color)
                } else {
                    base_color.to_string()
                }
            }
            SurfaceType::Carbon { carbon_type, .. } => match carbon_type {
                CarbonType::Graphite => "tiefschwarz".to_string(),
                CarbonType::Diamond => "kristallklar bis weiß".to_string(),
                CarbonType::Carbides => "dunkel metallisch".to_string(),
                CarbonType::OrganicPolymers => "braun bis schwarz, teerartig".to_string(),
                CarbonType::Tholins => "orange-rot bis braun".to_string(),
            },
            SurfaceType::Sulfur {
                sulfur_type,
                temperature,
                ..
            } => match sulfur_type {
                SulfurType::ElementalSulfur => {
                    if *temperature < 430.0 {
                        "gelb".to_string()
                    } else if *temperature < 470.0 {
                        "rot".to_string()
                    } else {
                        "schwarz".to_string()
                    }
                }
                SulfurType::SulfurDioxideIce => "weiß".to_string(),
                SulfurType::Sulfides => "gelblich metallisch".to_string(),
                SulfurType::Sulfates => "weiß bis gelblich".to_string(),
                SulfurType::SulfuricAcid => "klar bis gelblich".to_string(),
            },
            SurfaceType::Vegetation {
                vegetation_type, ..
            } => match vegetation_type {
                VegetationType::GreenVegetation => "grün".to_string(),
                VegetationType::RedVegetation => "rot".to_string(),
                VegetationType::BlueVegetation => "blau".to_string(),
                VegetationType::PurpleVegetation => "violett".to_string(),
                VegetationType::FluorescentVegetation => "leuchtend, variabel".to_string(),
                VegetationType::Grassland => "grün bis gelb".to_string(),
                VegetationType::Forest => "dunkelgrün".to_string(),
                VegetationType::MicrobialMats => "variabel, oft grün-braun".to_string(),
            },
            SurfaceType::Lava { temperature, .. } => {
                if *temperature > 1500.0 {
                    "hell glühend rot-orange".to_string()
                } else if *temperature > 1000.0 {
                    "dunkel glühend rot".to_string()
                } else {
                    "schwarze erstarrte Lava".to_string()
                }
            }
            _ => "unbekannt".to_string(),
        }
    }

    /// Gibt an ob die Oberfläche fest ist
    pub fn is_solid(&self) -> bool {
        !matches!(self, SurfaceType::Lava { temperature, .. } if *temperature > 1000.0)
            && !matches!(self, SurfaceType::Ocean { .. })
    }

    /// Gibt an ob die Oberfläche geologisch aktiv ist
    pub fn is_geologically_active(&self) -> bool {
        matches!(self, SurfaceType::Lava { .. }) || matches!(self, SurfaceType::Ice { .. }) // Cryovolcanism
    }
}
