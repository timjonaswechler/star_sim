pub mod prefix;
pub mod time;
pub mod velocity;
pub mod acceleration;
pub mod energy;
pub mod force;
pub mod length;
pub mod mass;
pub mod power;
pub mod pressure;
pub mod temperature;

/// Trait for returning the short symbol of a unit.
pub trait UnitSymbol {
    fn symbol() -> String;
}

pub use acceleration::*;
pub use energy::*;
pub use force::*;
pub use length::*;
pub use mass::*;
pub use power::*;
pub use pressure::*;
pub use temperature::*;
pub use time::*;
pub use velocity::*;
pub use prefix::Atto;
pub use prefix::Centi;
pub use prefix::Deca;
pub use prefix::Deci;
pub use prefix::Exa;
pub use prefix::Femto;
pub use prefix::Giga;
pub use prefix::Hecto;
pub use prefix::Kilo;
pub use prefix::Mega;
pub use prefix::Micro;
pub use prefix::Milli;
pub use prefix::Nano;
pub use prefix::Peta;
pub use prefix::Pico;
pub use prefix::Prefix;
pub use prefix::Tera;
pub use prefix::Yocto;
pub use prefix::Yotta;
pub use prefix::Zepto;
pub use prefix::Zetta;
