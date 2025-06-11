use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

//================================================================================
// 1. Grundlegende Einheiten und physikalische Eigenschaften
//    Jetzt mit `Serialize` und `Deserialize` für RON-Kompatibilität.
//================================================================================

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Mass(pub f64);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Radius(pub f64);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Temperature(pub f64);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Luminosity(pub f64);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Age(pub f64);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveCore(pub bool);

//================================================================================
// 2. Orbitale Mechanik
//================================================================================

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Orbit {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub longitude_of_ascending_node: f64,
    pub argument_of_periapsis: f64,
    pub mean_anomaly_at_epoch: f64,
}

//================================================================================
// 3. Klassifizierung von Himmelskörpern
//================================================================================

// --- Stern-spezifische Eigenschaften ---
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpectralType {
    O(u8),
    B(u8),
    A(u8),
    F(u8),
    G(u8),
    K(u8),
    M(u8),
    L,
    T,
    Y,
    D,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LuminosityClass {
    Ia,
    Ib,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

// --- Planeten- und Mond-spezifische Eigenschaften ---
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyType {
    Rocky,
    SuperEarth,
    WaterWorld,
    IceWorld,
    MiniNeptune,
    IceGiant,
    GasGiant,
    Cthonian,
}

//================================================================================
// 4. Serializable Strukturen für die RON-Ausgabe
//    Diese Strukturen sind für die hierarchische Darstellung in der Datei.
//================================================================================

/// Enthält die spezifischen Daten für einen Stern.
#[derive(Debug, Serialize, Deserialize)]
pub struct StarData {
    pub mass: Mass,
    pub radius: Radius,
    pub temperature: Temperature,
    pub luminosity: Luminosity,
    pub spectral_type: SpectralType,
    pub luminosity_class: LuminosityClass,
}

/// Enthält die spezifischen Daten für einen Planeten oder Mond.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlanetData {
    pub body_type: BodyType,
    pub mass: Mass,
    pub radius: Radius,
    pub active_core: ActiveCore,
}

/// Unterscheidet zwischen den Arten von Himmelskörpern.
#[derive(Debug, Serialize, Deserialize)]
pub enum BodyKind {
    Star(StarData),
    Planet(PlanetData),
    Barycenter, // Für Mehrfachsternsysteme
}

/// Eine rekursive Struktur, die einen Himmelskörper und seine Satelliten repräsentiert.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableBody {
    pub name: String,
    pub kind: BodyKind,
    /// Die Umlaufbahn dieses Körpers um seinen Parent. `None` für den zentralen Körper.
    pub orbit: Option<Orbit>,
    /// Eine Liste von Körpern, die diesen Körper umkreisen.
    pub satellites: Vec<SerializableBody>,
}

/// Die Wurzelstruktur für das gesamte Sternensystem.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableStellarSystem {
    pub name: String,
    pub age: Age,
    /// Die zentralen Körper des Systems (normalerweise ein Stern, könnte aber auch ein Baryzentrum sein).
    pub roots: Vec<SerializableBody>,
}

//================================================================================
// 5. Generierungslogik, die die serializable Struktur erzeugt.
//================================================================================

pub fn generate_teacup_system() -> SerializableStellarSystem {
    // Baue die Hierarchie von innen nach außen auf (Mond -> Planet -> Stern).

    // Mond von Teacup Ae
    let moon_ae_2 = SerializableBody {
        name: "Teacup Ae II".to_string(),
        kind: BodyKind::Planet(PlanetData {
            body_type: BodyType::Rocky,
            mass: Mass(0.004),
            radius: Radius(0.18),
            active_core: ActiveCore(false),
        }),
        orbit: Some(Orbit {
            semi_major_axis: 0.00167,
            eccentricity: 0.01,
            inclination: 0.087,
            ..default()
        }),
        satellites: vec![], // Monde haben hier keine weiteren Satelliten
    };

    // Planet Teacup Ae
    let planet_ae = SerializableBody {
        name: "Teacup Ae".to_string(),
        kind: BodyKind::Planet(PlanetData {
            body_type: BodyType::SuperEarth,
            mass: Mass(0.8),
            radius: Radius(0.96),
            active_core: ActiveCore(true),
        }),
        orbit: Some(Orbit {
            semi_major_axis: 0.45,
            eccentricity: 0.1,
            inclination: 0.0,
            longitude_of_ascending_node: 0.0,
            argument_of_periapsis: 2.79,
            mean_anomaly_at_epoch: 2.09,
        }),
        satellites: vec![moon_ae_2], // Füge den Mond als Satelliten hinzu
    };

    // Stern Teacup A
    let star_a = SerializableBody {
        name: "Teacup A".to_string(),
        kind: BodyKind::Star(StarData {
            mass: Mass(0.7),
            radius: Radius(0.66),
            temperature: Temperature(4500.0),
            luminosity: Luminosity(0.15),
            spectral_type: SpectralType::K(5),
            luminosity_class: LuminosityClass::V,
        }),
        orbit: None,                 // Der zentrale Stern hat keine Umlaufbahn
        satellites: vec![planet_ae], // Füge den Planeten als Satelliten hinzu
    };

    // Das gesamte System
    SerializableStellarSystem {
        name: "Teacup System".to_string(),
        age: Age(6.0),
        roots: vec![star_a],
    }
}

// Dummy-Default für die Orbit-Komponente, um das Bundling zu erleichtern.
// Wird auch hier für eine saubere Initialisierung verwendet.
impl Default for Orbit {
    fn default() -> Self {
        Self {
            semi_major_axis: 0.0,
            eccentricity: 0.0,
            inclination: 0.0,
            longitude_of_ascending_node: 0.0,
            argument_of_periapsis: 0.0,
            mean_anomaly_at_epoch: 0.0,
        }
    }
}
