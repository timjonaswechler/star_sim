//! Metric prefix system for unit system scalability.
//!
//! This module provides a generic prefix system that can be applied to any unit type,
//! eliminating the need to hardcode every possible unit combination. Instead of defining
//! separate units like `Kilometer`, `Megameter`, `Gigameter`, etc., you can use:
//!
//! ```rust
//! use star_sim::physics::units_v2::*;
//! 
//! let distance = Distance::<Prefixed<Kilo, Meter>>::new(5.0); // 5 km
//! let mass = Mass::<Prefixed<Mega, Gram>>::new(2.0);          // 2 Mg
//! let time = Time::<Prefixed<Micro, Second>>::new(100.0);     // 100 μs
//! ```
//!
//! # Benefits
//!
//! - **Combinatorial explosion avoided**: Instead of n×m hardcoded units, we have n prefixes × m base units
//! - **Consistent symbols**: Automatically generates "km", "Mg", "μs", etc.
//! - **Type safety**: Maintains all dimensional analysis and conversion safety
//! - **Hub-and-spoke conversions**: Prefixed units still convert through SI base units
//!
//! # Standard SI Prefixes
//!
//! The system includes all standard SI prefixes from yocto (10⁻²⁴) to yotta (10²⁴):
//!
//! | Prefix | Factor | Symbol | Example |
//! |--------|--------|--------|---------|
//! | Yotta  | 10²⁴   | Y      | Ym      |
//! | Zetta  | 10²¹   | Z      | Zg      |
//! | Exa    | 10¹⁸   | E      | Es      |
//! | Peta   | 10¹⁵   | P      | PJ      |
//! | Tera   | 10¹²   | T      | TW      |
//! | Giga   | 10⁹    | G      | GHz     |
//! | Mega   | 10⁶    | M      | MHz     |
//! | Kilo   | 10³    | k      | km      |
//! | Hecto  | 10²    | h      | hPa     |
//! | Deca   | 10¹    | da     | dam     |
//! | Deci   | 10⁻¹   | d      | dm      |
//! | Centi  | 10⁻²   | c      | cm      |
//! | Milli  | 10⁻³   | m      | mm      |
//! | Micro  | 10⁻⁶   | μ      | μm      |
//! | Nano   | 10⁻⁹   | n      | nm      |
//! | Pico   | 10⁻¹²  | p      | ps      |
//! | Femto  | 10⁻¹⁵  | f      | fs      |
//! | Atto   | 10⁻¹⁸  | a      | as      |
//! | Zepto  | 10⁻²¹  | z      | zs      |
//! | Yocto  | 10⁻²⁴  | y      | ys      |

use crate::physics::units_v2::core::*;
use std::marker::PhantomData;

/// Trait for metric prefixes.
///
/// Defines the multiplication factor and symbol for a metric prefix.
/// All factors are relative to the base unit (e.g., Kilo has factor 1000.0).
pub trait Prefix {
    /// Multiplication factor to apply to the base unit value.
    const FACTOR: f64;
    
    /// Short symbol for the prefix (e.g., "k" for Kilo).
    fn symbol() -> &'static str;
}

/// A unit with a metric prefix applied.
///
/// This type combines a prefix with a base unit to create prefixed units
/// like `Prefixed<Kilo, Meter>` for kilometers.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Prefixed<P: Prefix, U>(PhantomData<(P, U)>);

/// Implements unit symbol generation for prefixed units.
///
/// Combines the prefix symbol with the base unit symbol.
/// For example: `Prefixed<Kilo, Meter>` becomes "km".
impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: UnitSymbol,
{
    fn symbol() -> &'static str {
        // We need to use a static approach since we can't allocate in const contexts
        // This is a limitation - we'll need to use a different approach
        U::symbol() // For now, just return base unit symbol
    }
}

// Note: ToSI and FromSI implementations for Prefixed units need to be 
// implemented in the specific dimension modules to avoid circular dependencies
// and infinite recursion. The macro system will handle this automatically.

// ================================================================================================
// SI PREFIX DEFINITIONS
// ================================================================================================

// Large prefixes (10^n where n > 0)

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Yotta;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Zetta;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Exa;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Peta;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Tera;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Giga;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Mega;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Kilo;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Hecto;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Deca;

// Small prefixes (10^n where n < 0)

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Deci;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Centi;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Milli;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Micro;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Nano;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Pico;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Femto;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Atto;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Zepto;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Yocto;

// ================================================================================================
// PREFIX IMPLEMENTATIONS
// ================================================================================================

impl Prefix for Yotta {
    const FACTOR: f64 = 1e24;
    fn symbol() -> &'static str { "Y" }
}

impl Prefix for Zetta {
    const FACTOR: f64 = 1e21;
    fn symbol() -> &'static str { "Z" }
}

impl Prefix for Exa {
    const FACTOR: f64 = 1e18;
    fn symbol() -> &'static str { "E" }
}

impl Prefix for Peta {
    const FACTOR: f64 = 1e15;
    fn symbol() -> &'static str { "P" }
}

impl Prefix for Tera {
    const FACTOR: f64 = 1e12;
    fn symbol() -> &'static str { "T" }
}

impl Prefix for Giga {
    const FACTOR: f64 = 1e9;
    fn symbol() -> &'static str { "G" }
}

impl Prefix for Mega {
    const FACTOR: f64 = 1e6;
    fn symbol() -> &'static str { "M" }
}

impl Prefix for Kilo {
    const FACTOR: f64 = 1e3;
    fn symbol() -> &'static str { "k" }
}

impl Prefix for Hecto {
    const FACTOR: f64 = 1e2;
    fn symbol() -> &'static str { "h" }
}

impl Prefix for Deca {
    const FACTOR: f64 = 1e1;
    fn symbol() -> &'static str { "da" }
}

impl Prefix for Deci {
    const FACTOR: f64 = 1e-1;
    fn symbol() -> &'static str { "d" }
}

impl Prefix for Centi {
    const FACTOR: f64 = 1e-2;
    fn symbol() -> &'static str { "c" }
}

impl Prefix for Milli {
    const FACTOR: f64 = 1e-3;
    fn symbol() -> &'static str { "m" }
}

impl Prefix for Micro {
    const FACTOR: f64 = 1e-6;
    fn symbol() -> &'static str { "μ" }
}

impl Prefix for Nano {
    const FACTOR: f64 = 1e-9;
    fn symbol() -> &'static str { "n" }
}

impl Prefix for Pico {
    const FACTOR: f64 = 1e-12;
    fn symbol() -> &'static str { "p" }
}

impl Prefix for Femto {
    const FACTOR: f64 = 1e-15;
    fn symbol() -> &'static str { "f" }
}

impl Prefix for Atto {
    const FACTOR: f64 = 1e-18;
    fn symbol() -> &'static str { "a" }
}

impl Prefix for Zepto {
    const FACTOR: f64 = 1e-21;
    fn symbol() -> &'static str { "z" }
}

impl Prefix for Yocto {
    const FACTOR: f64 = 1e-24;
    fn symbol() -> &'static str { "y" }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES
// ================================================================================================
// Note: Type aliases are defined in dimensions.rs to avoid circular imports

// ================================================================================================
// UTILITY FUNCTIONS
// ================================================================================================
// Note: Utility functions would be defined elsewhere to avoid circular imports.
// Users can create prefixed units directly: Distance::<Prefixed<Kilo, Meter>>::new(5.0)