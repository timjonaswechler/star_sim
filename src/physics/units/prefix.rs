use core::marker::PhantomData;

/// Maßeinheiten-Vorsatz (SI-Prefix o. Ä.)
pub trait Prefix {
    /// Multiplikator auf die Basiseinheit ­(z. B. 1_000 für „kilo“)
    const FACTOR: f64;
    /// Kurzzeichen des Präfixes ­(z. B. "k")
    fn symbol() -> String;
}

#[rustfmt::skip]
pub struct Yotta;
pub struct Zetta;
pub struct Exa;
pub struct Peta;
pub struct Tera;
pub struct Giga;
pub struct Mega;
pub struct Kilo;
pub struct Hecto;
pub struct Deca;
pub struct Deci;
pub struct Centi;
pub struct Milli;
pub struct Micro;
pub struct Nano;
pub struct Pico;
pub struct Femto;
pub struct Atto;
pub struct Zepto;
pub struct Yocto;

impl Prefix for Yotta {
    const FACTOR: f64 = 1_000_000_000_000_000_000_000.0;
    fn symbol() -> String {
        "Y".into()
    }
}
impl Prefix for Zetta {
    const FACTOR: f64 = 1_000_000_000_000_000_000.0;
    fn symbol() -> String {
        "Z".into()
    }
}
impl Prefix for Exa {
    const FACTOR: f64 = 1_000_000_000_000_000.0;
    fn symbol() -> String {
        "E".into()
    }
}
impl Prefix for Peta {
    const FACTOR: f64 = 1_000_000_000_000.0;
    fn symbol() -> String {
        "P".into()
    }
}
impl Prefix for Tera {
    const FACTOR: f64 = 1_000_000_000.0;
    fn symbol() -> String {
        "T".into()
    }
}
impl Prefix for Giga {
    const FACTOR: f64 = 1_000_000.0;
    fn symbol() -> String {
        "G".into()
    }
}
impl Prefix for Mega {
    const FACTOR: f64 = 1_000_000.0;
    fn symbol() -> String {
        "M".into()
    }
}
impl Prefix for Kilo {
    const FACTOR: f64 = 1_000.0;
    fn symbol() -> String {
        "k".into()
    }
}
impl Prefix for Milli {
    const FACTOR: f64 = 0.001;
    fn symbol() -> String {
        "m".into()
    }
}
impl Prefix for Micro {
    const FACTOR: f64 = 0.000_001;
    fn symbol() -> String {
        "µ".into()
    }
}
impl Prefix for Nano {
    const FACTOR: f64 = 0.000_000_001;
    fn symbol() -> String {
        "n".into()
    }
}
impl Prefix for Pico {
    const FACTOR: f64 = 0.000_000_000_001;
    fn symbol() -> String {
        "p".into()
    }
}
impl Prefix for Femto {
    const FACTOR: f64 = 0.000_000_000_000_001;
    fn symbol() -> String {
        "f".into()
    }
}
impl Prefix for Atto {
    const FACTOR: f64 = 0.000_000_000_000_000_001;
    fn symbol() -> String {
        "a".into()
    }
}
impl Prefix for Zepto {
    const FACTOR: f64 = 0.000_000_000_000_000_000_001;
    fn symbol() -> String {
        "z".into()
    }
}
impl Prefix for Yocto {
    const FACTOR: f64 = 0.000_000_000_000_000_000_000_001;
    fn symbol() -> String {
        "y".into()
    }
}
