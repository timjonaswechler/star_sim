// Benötigte Typen aus dem neuen Einheitensystem importieren
use crate::physics::units_v2::*;

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

//================================================================================
// 1. Grundlegende Eigenschaften (als Komponenten, aber hier nur als Daten)
//    Diese sind nicht mehr nötig, da wir Ihre Typen verwenden.
//================================================================================
// -> Gelöscht und durch `use`-Statements oben ersetzt.

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveCore(pub bool);

//================================================================================
// 2. Orbitale Mechanik (angepasst an Ihr Einheitensystem)
//================================================================================

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Orbit {
    /// Die große Halbachse in Astronomischen Einheiten.
    pub semi_major_axis: Distance<AstronomicalUnit>,
    /// Die Exzentrizität (dimensionslos).
    pub eccentricity: f64,
    /// Die Bahnneigung in Radiant.
    pub inclination: Angle<Radian>,
    /// Die Länge des aufsteigenden Knotens in Radiant.
    pub longitude_of_ascending_node: Angle<Radian>,
    /// Das Argument der Periapsis in Radiant.
    pub argument_of_periapsis: Angle<Radian>,
    /// Die mittlere Anomalie zur Epoche in Radiant.
    pub mean_anomaly_at_epoch: Angle<Radian>,
}
impl Default for Orbit {
    fn default() -> Self {
        Orbit {
            semi_major_axis: Distance::<AstronomicalUnit>::new(1.0), // Standardwert 1 AU
            eccentricity: 0.0,
            inclination: Angle::<Radian>::new(0.0),
            longitude_of_ascending_node: Angle::<Radian>::new(0.0),
            argument_of_periapsis: Angle::<Radian>::new(0.0),
            mean_anomaly_at_epoch: Angle::<Radian>::new(0.0),
        }
    }
}

//================================================================================
// 3. Klassifizierung von Himmelskörpern (bleibt größtenteils gleich)
//================================================================================

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
// 4. Serializable Strukturen für die RON-Ausgabe (angepasst)
//================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct StarData {
    pub mass: Mass<SolarMass>,
    pub radius: Distance<SunRadius>,
    pub temperature: Temperature<Kelvin>,
    pub luminosity: Power<SolarLuminosity>,
    pub spectral_type: SpectralType,
    pub luminosity_class: LuminosityClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanetData {
    pub body_type: BodyType,
    pub mass: Mass<EarthMass>,
    pub radius: Distance<EarthRadius>,
    pub active_core: ActiveCore,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BodyKind {
    Star(StarData),
    Planet(PlanetData),
    Barycenter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableBody {
    pub name: String,
    pub kind: BodyKind,
    pub orbit: Option<Orbit>,
    pub satellites: Vec<SerializableBody>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableStellarSystem {
    pub name: String,
    pub age: Time<Gigayear>, // Verwende Time<Gigayear> statt Age(f64)
    pub roots: Vec<SerializableBody>,
}

//================================================================================
// 5. Generierungslogik (angepasst an die neuen Typen)
//================================================================================

pub fn generate_teacup_system() -> SerializableStellarSystem {
    let moon_ae_2 = SerializableBody {
        name: "Teacup Ae II".to_string(),
        kind: BodyKind::Planet(PlanetData {
            body_type: BodyType::Rocky,
            mass: Mass::<EarthMass>::new(0.004),
            radius: Distance::<EarthRadius>::new(0.18),
            active_core: ActiveCore(false),
        }),
        orbit: Some(Orbit {
            semi_major_axis: Distance::<AstronomicalUnit>::new(0.00167),
            eccentricity: 0.01,
            inclination: Angle::<Radian>::new(0.087),
            ..Default::default()
        }),
        satellites: vec![],
    };

    let planet_ae = SerializableBody {
        name: "Teacup Ae".to_string(),
        kind: BodyKind::Planet(PlanetData {
            body_type: BodyType::SuperEarth,
            mass: Mass::<EarthMass>::new(0.8),
            radius: Distance::<EarthRadius>::new(0.96),
            active_core: ActiveCore(true),
        }),
        orbit: Some(Orbit {
            semi_major_axis: Distance::<AstronomicalUnit>::new(0.45),
            eccentricity: 0.1,
            inclination: Angle::<Radian>::new(0.0),
            longitude_of_ascending_node: Angle::<Radian>::new(0.0),
            argument_of_periapsis: Angle::<Radian>::new(2.79), // ~160 Grad in Radiant
            mean_anomaly_at_epoch: Angle::<Radian>::new(2.09), // ~120 Grad in Radiant
        }),
        satellites: vec![moon_ae_2],
    };

    let star_a = SerializableBody {
        name: "Teacup A".to_string(),
        kind: BodyKind::Star(StarData {
            mass: Mass::<SolarMass>::new(0.7),
            radius: Distance::<SunRadius>::new(0.66),
            temperature: Temperature::<Kelvin>::new(4500.0),
            luminosity: Power::<SolarLuminosity>::new(0.15),
            spectral_type: SpectralType::K(5),
            luminosity_class: LuminosityClass::V,
        }),
        orbit: None,
        satellites: vec![planet_ae],
    };

    SerializableStellarSystem {
        name: "Teacup System".to_string(),
        age: Time::<Gigayear>::new(6.0), // 6 Milliarden Jahre
        roots: vec![star_a],
    }
}
