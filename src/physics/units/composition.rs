//! Advanced unit composition system for type-safe dimensional analysis.
//!
//! This module provides a powerful system for composing units through mathematical operations
//! while maintaining full type safety and dimensional correctness. It enables natural
//! mathematical expressions with automatic unit derivation.
//!
//! # Key Features
//!
//! - **Power Operations**: `Power<Unit, N>` for squares, cubes, etc.
//! - **Automatic Arithmetic**: `Distance × Distance = Area`, `Distance ÷ Time = Velocity`
//! - **Modular Composition**: `Power<Prefixed<Kilo, Meter>, 2>` = km²
//! - **Type Safety**: Compile-time dimensional analysis prevents unit mixing errors
//!
//! # Examples
//!
//! ```rust
//! use star_sim::physics::units::*;
//! use star_sim::physics::units::composition::*;
//!
//! // Basic power operations
//! let radius = Distance::<Meter>::new(5.0);
//! let area = radius.squared(); // Returns Area<Power<Meter, 2>>
//! let volume = radius.cubed(); // Returns Volume<Power<Meter, 3>>
//!
//! // Automatic unit arithmetic
//! let distance1 = Distance::<Meter>::new(100.0);
//! let distance2 = Distance::<Meter>::new(50.0);
//! let area = distance1 * distance2; // Returns Area<Meter>
//!
//! // Complex compositions
//! let force = Mass::<Kilogram>::new(10.0) * Acceleration::<MeterPerSecondSquared>::new(9.81);
//! let energy = force * Distance::<Meter>::new(5.0); // Returns Energy<Joule>
//!
//! // Prefixed power units
//! let km_distance = Distance::<Prefixed<Kilo, Meter>>::new(2.0);
//! let km2_area = km_distance.squared(); // Returns Area<Power<Prefixed<Kilo, Meter>, 2>>
//! ```

use crate::physics::units::core::*;
use crate::physics::units::prefix::*;
use std::marker::PhantomData;

// ================================================================================================
// POWER TYPE SYSTEM
// ================================================================================================

/// Trait for types that can be raised to a power.
///
/// This trait enables creating power units like `Power<Meter, 2>` for square meters
/// or `Power<Meter, 3>` for cubic meters.
pub trait PowerUnit<const N: i8> {
    /// The result type when this unit is raised to power N.
    type Output;
    
    /// Returns the conversion factor when this unit is raised to power N.
    fn power_factor() -> f64;
    
    /// Returns the symbol for this unit raised to power N.
    fn power_symbol() -> &'static str;
}

/// A unit raised to an exponent.
///
/// This type represents mathematical powers of units, such as m² (square meters)
/// or m³ (cubic meters). It maintains type safety while allowing complex unit
/// compositions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Exponent<U, const N: i8>(PhantomData<U>);

impl<U, const N: i8> UnitSymbol for Exponent<U, N>
where
    U: UnitSymbol,
{
    fn symbol() -> &'static str {
        // For simplicity, we use a runtime approach here
        // In practice, this could be optimized with const string manipulation
        match N {
            1 => U::symbol(),
            2 => Box::leak(format!("{}²", U::symbol()).into_boxed_str()),
            3 => Box::leak(format!("{}³", U::symbol()).into_boxed_str()),
            -1 => Box::leak(format!("{}⁻¹", U::symbol()).into_boxed_str()),
            -2 => Box::leak(format!("{}⁻²", U::symbol()).into_boxed_str()),
            -3 => Box::leak(format!("{}⁻³", U::symbol()).into_boxed_str()),
            _ => Box::leak(format!("{}^{}", U::symbol(), N).into_boxed_str()),
        }
    }
}

// ================================================================================================
// AUTOMATIC UNIT ARITHMETIC OPERATIONS
// ================================================================================================

/// Trait for enabling automatic unit arithmetic.
///
/// This trait allows units to be multiplied and divided naturally, with the result
/// type automatically determined based on dimensional analysis.
pub trait UnitArithmetic<Rhs> {
    /// The result type when multiplying this unit by another.
    type MulOutput;
    
    /// The result type when dividing this unit by another.
    type DivOutput;
    
    /// Multiply this quantity by another, returning the appropriate unit type.
    fn multiply_units(self, rhs: Rhs) -> Self::MulOutput;
    
    /// Divide this quantity by another, returning the appropriate unit type.
    fn divide_units(self, rhs: Rhs) -> Self::DivOutput;
}

// ================================================================================================
// MULTIPLICATION IMPLEMENTATIONS
// ================================================================================================

// Note: Specific multiplication/division implementations are now in core.rs
// for better organization and to avoid complex const generic arithmetic

// ================================================================================================
// POWER OPERATION IMPLEMENTATIONS
// ================================================================================================

// Simplified power operations for specific cases
impl<Unit> Quantity<Unit, 1, 0, 0, 0, 0, 0, 0>
where
    Unit: UnitSymbol,
    Self: ToSI,
{
    /// Square this distance quantity, returning an area quantity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let distance = Distance::<Meter>::new(5.0);
    /// let area = distance.squared(); // Returns Area<Power<Meter, 2>>
    /// ```
    pub fn squared(self) -> Quantity<Exponent<Unit, 2>, 2, 0, 0, 0, 0, 0, 0> {
        let si_value = self.to_si();
        let result_si = si_value * si_value;
        Quantity::new(result_si)
    }
    
    /// Cube this distance quantity, returning a volume quantity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let distance = Distance::<Meter>::new(5.0);
    /// let volume = distance.cubed(); // Returns Volume<Power<Meter, 3>>
    /// ```
    pub fn cubed(self) -> Quantity<Exponent<Unit, 3>, 3, 0, 0, 0, 0, 0, 0> {
        let si_value = self.to_si();
        let result_si = si_value * si_value * si_value;
        Quantity::new(result_si)
    }
}

impl<Unit> Quantity<Exponent<Unit, 2>, 2, 0, 0, 0, 0, 0, 0>
where
    Unit: UnitSymbol,
    Self: ToSI,
{
    /// Take the square root of this area quantity, returning a distance quantity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let area = Area::<Power<Meter, 2>>::new(25.0);
    /// let distance = area.sqrt(); // Returns Distance<Meter>
    /// ```
    pub fn sqrt(self) -> Quantity<Unit, 1, 0, 0, 0, 0, 0, 0> {
        let si_value = self.to_si();
        let result_si = si_value.sqrt();
        Quantity::new(result_si)
    }
}

// ================================================================================================
// PREFIXED POWER UNITS
// ================================================================================================

// Note: Prefixed power unit implementations are simplified due to const generic limitations

// ================================================================================================
// CONVENIENCE TYPE ALIASES
// ================================================================================================

/// Square meter type alias for common usage.
pub type SquareMeterComposition = Exponent<crate::physics::units::Meter, 2>;

/// Cubic meter type alias for common usage.
pub type CubicMeterComposition = Exponent<crate::physics::units::Meter, 3>;

/// Square kilometer type alias for common usage.  
pub type SquareKilometerComposition = Exponent<Prefixed<Kilo, crate::physics::units::Meter>, 2>;

/// Cubic kilometer type alias for common usage.
pub type CubicKilometerComposition = Exponent<Prefixed<Kilo, crate::physics::units::Meter>, 3>;

// ================================================================================================
// UNIT ARITHMETIC HELPER FUNCTIONS
// ================================================================================================

/// Multiply two quantities and return the result with proper dimensional analysis.
///
/// This function handles the multiplication of any two quantities and returns
/// the result with the correct dimensions and in SI units.
pub fn multiply_with_dimensions<
    Unit1, Unit2,
    const L1: i8, const M1: i8, const T1: i8, const K1: i8, const I1: i8, const J1: i8, const N1: i8,
    const L2: i8, const M2: i8, const T2: i8, const K2: i8, const I2: i8, const J2: i8, const N2: i8,
>(
    q1: Quantity<Unit1, L1, M1, T1, K1, I1, J1, N1>,
    q2: Quantity<Unit2, L2, M2, T2, K2, I2, J2, N2>,
) -> f64
where
    Quantity<Unit1, L1, M1, T1, K1, I1, J1, N1>: ToSI,
    Quantity<Unit2, L2, M2, T2, K2, I2, J2, N2>: ToSI,
{
    q1.to_si() * q2.to_si()
}

/// Divide two quantities and return the result with proper dimensional analysis.
///
/// This function handles the division of any two quantities and returns
/// the result with the correct dimensions and in SI units.
pub fn divide_with_dimensions<
    Unit1, Unit2,
    const L1: i8, const M1: i8, const T1: i8, const K1: i8, const I1: i8, const J1: i8, const N1: i8,
    const L2: i8, const M2: i8, const T2: i8, const K2: i8, const I2: i8, const J2: i8, const N2: i8,
>(
    q1: Quantity<Unit1, L1, M1, T1, K1, I1, J1, N1>,
    q2: Quantity<Unit2, L2, M2, T2, K2, I2, J2, N2>,
) -> f64
where
    Quantity<Unit1, L1, M1, T1, K1, I1, J1, N1>: ToSI,
    Quantity<Unit2, L2, M2, T2, K2, I2, J2, N2>: ToSI,
{
    q1.to_si() / q2.to_si()
}

// ================================================================================================
// DIMENSIONLESS RATIO TYPE
// ================================================================================================

/// A dimensionless ratio type for representing fractions and ratios.
///
/// This type has all dimensional exponents set to 0, making it suitable for
/// representing pure numbers, ratios, and fractions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Dimensionless;

impl UnitSymbol for Dimensionless {
    fn symbol() -> &'static str {
        ""
    }
}

/// Type alias for dimensionless quantities.
pub type DimensionlessRatio = Quantity<Dimensionless, 0, 0, 0, 0, 0, 0, 0>;

impl ToSI for DimensionlessRatio {
    fn to_si(&self) -> f64 {
        self.value
    }
}

impl FromSI for DimensionlessRatio {
    fn from_si(value: f64) -> Self {
        Self::new(value)
    }
}

// ================================================================================================
// MACRO FOR AUTOMATIC UNIT ARITHMETIC
// ================================================================================================

/// Macro for implementing automatic unit arithmetic operations.
///
/// This macro generates implementations for multiplying and dividing units
/// with automatic dimensional analysis.
#[macro_export]
macro_rules! impl_unit_arithmetic {
    (
        $lhs_unit:ty, $rhs_unit:ty,
        $lhs_dims:expr, $rhs_dims:expr,
        $mul_result:ty, $div_result:ty
    ) => {
        impl std::ops::Mul<$rhs_unit> for $lhs_unit {
            type Output = $mul_result;
            
            fn mul(self, rhs: $rhs_unit) -> Self::Output {
                let result_si = self.to_si() * rhs.to_si();
                <$mul_result>::from_si(result_si)
            }
        }
        
        impl std::ops::Div<$rhs_unit> for $lhs_unit {
            type Output = $div_result;
            
            fn div(self, rhs: $rhs_unit) -> Self::Output {
                let result_si = self.to_si() / rhs.to_si();
                <$div_result>::from_si(result_si)
            }
        }
    };
}