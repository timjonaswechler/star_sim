//! Modular unit composition system for natural physics notation.
//!
//! This module provides an intuitive way to compose units using natural physics notation.
//! It allows for expressions like `Area::<Meter, Squared>` and `Acceleration::<Meter, Per<Second, Squared>>`.
//!
//! # Core Concepts
//!
//! - **Marker Types**: `Squared`, `Cubed` for common exponents
//! - **Per System**: `Per<Numerator, Denominator>` for fractions
//! - **Flexible Composition**: Mix prefixes, exponents, and divisions naturally
//!
//! # Examples
//!
//! ```rust
//! use star_sim::physics::units::*;
//! use star_sim::physics::units::modular::*;
//!
//! // Simple squared units
//! let area = Area::<Meter, Squared>::new(25.0);
//! let volume = Volume::<Meter, Cubed>::new(125.0);
//!
//! // Per notation for fractions
//! let velocity = Velocity::<Meter, Per<Second>>::new(10.0);
//! let acceleration = Acceleration::<Meter, Per<Second, Squared>>::new(9.81);
//!
//! // Complex compositions with prefixes
//! let force = Force::<Kilogram, Per<Meter, Per<Second, Squared>>>::new(98.1);
//! let prefixed_area = Area::<Prefixed<Kilo, Meter>, Squared>::new(4.0); // 4 km²
//!
//! // Using Exponent directly for arbitrary powers
//! let quartic = Quantity::<Meter, Exponent<4>>::new(625.0); // m⁴
//! ```

use crate::physics::units::core::*;
use crate::physics::units::prefix::*;
use crate::physics::units::composition::Exponent;
use std::marker::PhantomData;

// ================================================================================================
// MARKER TYPES FOR COMMON EXPONENTS
// ================================================================================================

/// Marker type representing squared (²) exponent.
///
/// This provides a more readable syntax for squared units:
/// `Area::<Meter, Squared>` instead of `Area::<Exponent<Meter, 2>>`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Squared;

/// Marker type representing cubed (³) exponent.
///
/// This provides a more readable syntax for cubed units:
/// `Volume::<Meter, Cubed>` instead of `Volume::<Exponent<Meter, 3>>`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Cubed;

/// Marker type representing fourth power (⁴) exponent.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Fourth;

/// Marker type representing inverse (-1) exponent.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Inverse;

// ================================================================================================
// PER SYSTEM FOR FRACTIONS
// ================================================================================================

/// Represents division between units using natural notation.
///
/// `Per<Numerator, Denominator>` creates fractional units like m/s, kg/m³, etc.
/// The denominator can be a simple unit or another composition.
///
/// # Examples
///
/// ```rust
/// // Velocity: m/s
/// type VelocityUnit = Per<Meter, Second>;
///
/// // Acceleration: m/s²
/// type AccelerationUnit = Per<Meter, Exponent<Second, 2>>;
/// // or more readable:
/// type AccelerationUnit2 = Per<Meter, Per<Second, Squared>>;
///
/// // Density: kg/m³
/// type DensityUnit = Per<Kilogram, Exponent<Meter, 3>>;
/// // or:
/// type DensityUnit2 = Per<Kilogram, Per<Meter, Cubed>>;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Per<Numerator, Denominator = ()>(PhantomData<(Numerator, Denominator)>);

// ================================================================================================
// COMPOSITE UNIT TYPE
// ================================================================================================

/// A composite unit that can handle both simple and complex unit compositions.
///
/// This type supports flexible unit composition with optional exponents and denominators.
/// It serves as the foundation for the modular unit syntax.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct CompositeUnit<Base, Modifier = ()>(PhantomData<(Base, Modifier)>);

// ================================================================================================
// UNIT SYMBOL IMPLEMENTATIONS
// ================================================================================================

impl UnitSymbol for Squared {
    fn symbol() -> &'static str {
        "²"
    }
}

impl UnitSymbol for Cubed {
    fn symbol() -> &'static str {
        "³"
    }
}

impl UnitSymbol for Fourth {
    fn symbol() -> &'static str {
        "⁴"
    }
}

impl UnitSymbol for Inverse {
    fn symbol() -> &'static str {
        "⁻¹"
    }
}

impl<Num, Den> UnitSymbol for Per<Num, Den>
where
    Num: UnitSymbol,
    Den: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}/{}", Num::symbol(), Den::symbol()).into_boxed_str())
    }
}

impl<Num> UnitSymbol for Per<Num, ()>
where
    Num: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("/{}", Num::symbol()).into_boxed_str())
    }
}

impl<Base, Modifier> UnitSymbol for CompositeUnit<Base, Modifier>
where
    Base: UnitSymbol,
    Modifier: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}{}", Base::symbol(), Modifier::symbol()).into_boxed_str())
    }
}

impl<Base> UnitSymbol for CompositeUnit<Base, ()>
where
    Base: UnitSymbol,
{
    fn symbol() -> &'static str {
        Base::symbol()
    }
}

// ================================================================================================
// QUANTITY ALIASES FOR MODULAR SYNTAX
// ================================================================================================

/// Area quantity with modular unit syntax.
///
/// # Examples
///
/// ```rust
/// let area1 = AreaModular::<Meter, Squared>::new(25.0);          // 25 m²
/// let area2 = AreaModular::<Prefixed<Kilo, Meter>, Squared>::new(2.5); // 2.5 km²
/// ```
pub type AreaModular<Base, Modifier> = Quantity<CompositeUnit<Base, Modifier>, 2, 0, 0, 0, 0, 0, 0>;

/// Volume quantity with modular unit syntax.
///
/// # Examples
///
/// ```rust
/// let volume1 = VolumeModular::<Meter, Cubed>::new(125.0);        // 125 m³
/// let volume2 = VolumeModular::<Prefixed<Centi, Meter>, Cubed>::new(1000.0); // 1000 cm³
/// ```
pub type VolumeModular<Base, Modifier> = Quantity<CompositeUnit<Base, Modifier>, 3, 0, 0, 0, 0, 0, 0>;

/// Velocity quantity with modular unit syntax.
///
/// # Examples
///
/// ```rust
/// let velocity1 = VelocityModular::<Meter, Per<Second>>::new(10.0);     // 10 m/s
/// let velocity2 = VelocityModular::<Prefixed<Kilo, Meter>, Per<Hour>>::new(60.0); // 60 km/h
/// ```
pub type VelocityModular<Base, Modifier> = Quantity<CompositeUnit<Base, Modifier>, 1, 0, -1, 0, 0, 0, 0>;

/// Acceleration quantity with modular unit syntax.
///
/// # Examples
///
/// ```rust
/// let accel1 = AccelerationModular::<Meter, Per<Second, Squared>>::new(9.81);     // 9.81 m/s²
/// let accel2 = AccelerationModular::<Meter, Per<Exponent<Second, 2>>>::new(9.81); // Alternative syntax
/// ```
pub type AccelerationModular<Base, Modifier> = Quantity<CompositeUnit<Base, Modifier>, 1, 0, -2, 0, 0, 0, 0>;

/// Force quantity with modular unit syntax.
///
/// # Examples
///
/// ```rust
/// // Newton: kg⋅m/s²
/// let force = ForceModular::<Kilogram, Per<Meter, Per<Second, Squared>>>::new(98.1);
/// ```
pub type ForceModular<Base, Modifier> = Quantity<CompositeUnit<Base, Modifier>, 1, 1, -2, 0, 0, 0, 0>;

/// Density quantity with modular unit syntax.
///
/// # Examples
///
/// ```rust
/// let density1 = DensityModular::<Kilogram, Per<Meter, Cubed>>::new(1000.0);     // 1000 kg/m³
/// let density2 = DensityModular::<Gram, Per<Prefixed<Centi, Meter>, Cubed>>::new(1.0); // 1 g/cm³
/// ```
pub type DensityModular<Base, Modifier> = Quantity<CompositeUnit<Base, Modifier>, -3, 1, 0, 0, 0, 0, 0>;

// ================================================================================================
// CONVERSION IMPLEMENTATIONS
// ================================================================================================

// Squared units - convert to Exponent<Unit, 2>
impl<Base> ToSI for AreaModular<Base, Squared>
where
    Quantity<Exponent<Base, 2>, 2, 0, 0, 0, 0, 0, 0>: ToSI,
{
    fn to_si(&self) -> f64 {
        let exp_quantity = Quantity::<Exponent<Base, 2>, 2, 0, 0, 0, 0, 0, 0>::new(self.value);
        exp_quantity.to_si()
    }
}

impl<Base> FromSI for AreaModular<Base, Squared>
where
    Quantity<Exponent<Base, 2>, 2, 0, 0, 0, 0, 0, 0>: FromSI,
{
    fn from_si(si_value: f64) -> Self {
        let exp_quantity = Quantity::<Exponent<Base, 2>, 2, 0, 0, 0, 0, 0, 0>::from_si(si_value);
        Self::new(exp_quantity.value)
    }
}

// Cubed units - convert to Exponent<Unit, 3>
impl<Base> ToSI for VolumeModular<Base, Cubed>
where
    Quantity<Exponent<Base, 3>, 3, 0, 0, 0, 0, 0, 0>: ToSI,
{
    fn to_si(&self) -> f64 {
        let exp_quantity = Quantity::<Exponent<Base, 3>, 3, 0, 0, 0, 0, 0, 0>::new(self.value);
        exp_quantity.to_si()
    }
}

impl<Base> FromSI for VolumeModular<Base, Cubed>
where
    Quantity<Exponent<Base, 3>, 3, 0, 0, 0, 0, 0, 0>: FromSI,
{
    fn from_si(si_value: f64) -> Self {
        let exp_quantity = Quantity::<Exponent<Base, 3>, 3, 0, 0, 0, 0, 0, 0>::from_si(si_value);
        Self::new(exp_quantity.value)
    }
}

// ================================================================================================
// CONVENIENCE MACROS
// ================================================================================================

/// Macro for creating modular unit types more easily.
///
/// # Examples
///
/// ```rust
/// // Create custom unit types
/// modular_unit!(Energy, Kilogram, Per<Meter, Squared, Per<Second, Squared>>); // kg⋅m²/s²
/// modular_unit!(Pressure, Newton, Per<Meter, Squared>); // N/m²
/// ```
#[macro_export]
macro_rules! modular_unit {
    ($name:ident, $base:ty, $modifier:ty, $dims:expr) => {
        pub type $name = Quantity<CompositeUnit<$base, $modifier>, { $dims[0] }, { $dims[1] }, { $dims[2] }, { $dims[3] }, { $dims[4] }, { $dims[5] }, { $dims[6] }>;
    };
}

// ================================================================================================
// HELPER FUNCTIONS
// ================================================================================================

/// Convert a modular unit to its equivalent exponent form.
///
/// This is useful for compatibility with existing code that uses `Exponent<Unit, N>`.
pub fn to_exponent_form<Base, Modifier>(
    modular: Quantity<CompositeUnit<Base, Modifier>, 2, 0, 0, 0, 0, 0, 0>
) -> Quantity<Exponent<Base, 2>, 2, 0, 0, 0, 0, 0, 0>
where
    Base: UnitSymbol,
{
    Quantity::new(modular.value)
}

/// Convert an exponent form to modular unit syntax.
///
/// This is useful for transitioning existing code to the new modular syntax.
pub fn from_exponent_form<Base>(
    exponent: Quantity<Exponent<Base, 2>, 2, 0, 0, 0, 0, 0, 0>
) -> Quantity<CompositeUnit<Base, Squared>, 2, 0, 0, 0, 0, 0, 0>
where
    Base: UnitSymbol,
{
    Quantity::new(exponent.value)
}

// ================================================================================================
// TESTING UTILITIES
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::units::quantities::distance::Meter;
    use crate::physics::units::quantities::time::Second;
    use crate::physics::units::quantities::mass::Kilogram;

    #[test]
    fn test_modular_area_syntax() {
        let area = AreaModular::<Meter, Squared>::new(25.0);
        assert_eq!(area.value(), 25.0);
    }

    #[test]
    fn test_modular_velocity_syntax() {
        let velocity = VelocityModular::<Meter, Per<Second>>::new(10.0);
        assert_eq!(velocity.value(), 10.0);
    }

    #[test]
    fn test_unit_symbols() {
        assert_eq!(Squared::symbol(), "²");
        assert_eq!(Cubed::symbol(), "³");
        // Note: Actual symbol concatenation would need more complex implementation
    }
}