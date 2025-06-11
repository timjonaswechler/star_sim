use crate::physics::units::Float;
use serde::{Deserialize, Serialize};

/// Maßeinheiten-Vorsatz (SI-Prefix o. Ä.)
pub trait Prefix {
    /// Multiplikator auf die Basiseinheit ­(z. B. 1_000 für „kilo“)
    const FACTOR: Float;
    /// Kurzzeichen des Präfixes ­(z. B. "k")
    fn symbol() -> String;
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)] 

pub struct Yotta;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Zetta;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Exa;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Peta;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Tera;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Giga;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Mega;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Kilo; // Hier ist es wichtig
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Hecto;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Deca;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Deci;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Centi;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Milli;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Micro;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Nano;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Pico;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Femto;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Atto;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Zepto;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Yocto;

impl Prefix for Yotta {
    const FACTOR: Float = 1_000_000_000_000_000_000_000.0;
    fn symbol() -> String {
        "Y".into()
    }
}
impl Prefix for Zetta {
    const FACTOR: Float = 1_000_000_000_000_000_000.0;
    fn symbol() -> String {
        "Z".into()
    }
}
impl Prefix for Exa {
    const FACTOR: Float = 1_000_000_000_000_000.0;
    fn symbol() -> String {
        "E".into()
    }
}
impl Prefix for Peta {
    const FACTOR: Float = 1_000_000_000_000.0;
    fn symbol() -> String {
        "P".into()
    }
}
impl Prefix for Tera {
    const FACTOR: Float = 1_000_000_000.0;
    fn symbol() -> String {
        "T".into()
    }
}
impl Prefix for Giga {
    const FACTOR: Float = 1_000_000.0;
    fn symbol() -> String {
        "G".into()
    }
}
impl Prefix for Mega {
    const FACTOR: Float = 1_000_000.0;
    fn symbol() -> String {
        "M".into()
    }
}
impl Prefix for Kilo {
    const FACTOR: Float = 1_000.0;
    fn symbol() -> String {
        "k".into()
    }
}
impl Prefix for Milli {
    const FACTOR: Float = 0.001;
    fn symbol() -> String {
        "m".into()
    }
}
impl Prefix for Micro {
    const FACTOR: Float = 0.000_001;
    fn symbol() -> String {
        "µ".into()
    }
}
impl Prefix for Nano {
    const FACTOR: Float = 0.000_000_001;
    fn symbol() -> String {
        "n".into()
    }
}
impl Prefix for Pico {
    const FACTOR: Float = 0.000_000_000_001;
    fn symbol() -> String {
        "p".into()
    }
}
impl Prefix for Femto {
    const FACTOR: Float = 0.000_000_000_000_001;
    fn symbol() -> String {
        "f".into()
    }
}
impl Prefix for Atto {
    const FACTOR: Float = 0.000_000_000_000_000_001;
    fn symbol() -> String {
        "a".into()
    }
}
impl Prefix for Zepto {
    const FACTOR: Float = 0.000_000_000_000_000_000_001;
    fn symbol() -> String {
        "z".into()
    }
}
impl Prefix for Yocto {
    const FACTOR: Float = 0.000_000_000_000_000_000_000_001;
    fn symbol() -> String {
        "y".into()
    }
}
