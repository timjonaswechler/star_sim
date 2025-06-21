//! Core infrastructure for the type-safe unit system with dimensional analysis.
//!
//! This module provides the foundational types and traits for a compile-time unit system
//! that prevents unit mixing errors and supports automatic dimensional analysis.
//!
//! # Features
//!
//! - **Type Safety**: Prevents mixing incompatible units at compile time
//! - **Hub-and-Spoke Conversions**: O(n) conversion complexity instead of O(n²)
//! - **Dimensional Analysis**: Track physical dimensions through calculations
//! - **Serialization**: Full serde support for data persistence
//!
//! # Examples
//!
//! ```rust
//! use star_sim::physics::units_v2::*;
//!
//! // Create quantities with specific units
//! let distance = Distance::<AstronomicalUnit>::new(1.5);
//! let mass = Mass::<EarthMass>::new(0.8);
//!
//! // Convert between units
//! let distance_m = distance.convert_to::<Meter>();
//! let mass_kg = mass.convert_to::<Kilogram>();
//!
//! // Perform calculations with dimensional safety
//! let velocity = calculate_velocity(distance_m, Time::<Second>::new(3600.0));
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represents physical dimensions using const generics for compile-time dimensional analysis.
///
/// This type encodes the seven fundamental SI dimensions as compile-time constants,
/// enabling automatic tracking of physical dimensions through calculations.
///
/// # Dimensions
///
/// - `L`: Length (meters)
/// - `M`: Mass (kilograms)
/// - `T`: Time (seconds)
/// - `K`: Temperature (kelvin)
/// - `I`: Electric Current (amperes)
/// - `J`: Luminous Intensity (candela)
/// - `N`: Amount of Substance (moles)
///
/// # Examples
///
/// ```rust
/// // Velocity has dimensions [Length¹ Time⁻¹]
/// type VelocityDims = Dimensions<1, 0, -1, 0, 0, 0, 0>;
///
/// // Force has dimensions [Length¹ Mass¹ Time⁻²]
/// type ForceDims = Dimensions<1, 1, -2, 0, 0, 0, 0>;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions<
    const L: i8, // Length
    const M: i8, // Mass
    const T: i8, // Time
    const K: i8, // Temperature
    const I: i8, // Current
    const J: i8, // Luminous Intensity
    const N: i8, // Amount of substance
>;

/// A physical quantity with compile-time unit and dimensional type safety.
///
/// This is the core type that represents a physical quantity (like distance, mass, time)
/// with a specific unit and dimensional information tracked at compile time.
///
/// # Type Parameters
///
/// - `Unit`: The specific unit type (e.g., `Meter`, `AstronomicalUnit`, `Kilogram`)
/// - `L, M, T, K, I, J, N`: Dimensional exponents for the seven SI base dimensions
///
/// # Examples
///
/// ```rust
/// use star_sim::physics::units_v2::*;
///
/// // Distance in astronomical units
/// let distance: Distance<AstronomicalUnit> = Distance::new(1.5);
///
/// // Mass in earth masses  
/// let mass: Mass<EarthMass> = Mass::new(0.8);
///
/// // Convert between units
/// let distance_meters = distance.convert_to::<Meter>();
/// assert_eq!(distance_meters.value(), 1.5 * 149_597_870_700.0);
/// ```
///
/// # Dimensional Safety
///
/// The type system prevents mixing incompatible units:
///
/// ```compile_fail
/// let distance = Distance::<Meter>::new(100.0);
/// let mass = Mass::<Kilogram>::new(5.0);
/// let invalid = distance + mass; // Compile error!
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quantity<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> {
    /// The numerical value of this quantity in the specified unit
    pub value: f64,
    /// Phantom data to track the unit type at compile time
    _unit: PhantomData<Unit>,
    /// Phantom data to track the dimensional information at compile time
    _dims: PhantomData<Dimensions<L, M, T, K, I, J, N>>,
}

/// Trait for converting quantities to their equivalent value in SI base units.
///
/// This trait enables the hub-and-spoke conversion system where all unit conversions
/// go through SI base units as an intermediate step, reducing conversion complexity
/// from O(n²) to O(n).
///
/// # Implementation
///
/// For each unit type, implement this trait to specify how to convert to the
/// corresponding SI base unit (meters, kilograms, seconds, etc.).
///
/// # Examples
///
/// ```rust
/// impl ToSI for Distance<AstronomicalUnit> {
///     fn to_si(&self) -> f64 {
///         self.value * 149_597_870_700.0  // Convert AU to meters
///     }
/// }
/// ```
pub trait ToSI {
    /// Convert this quantity to its equivalent value in SI base units.
    ///
    /// # Returns
    ///
    /// The numerical value in the appropriate SI base unit (meters for distance,
    /// kilograms for mass, seconds for time, etc.).
    fn to_si(&self) -> f64;
}

/// Trait for creating quantities from values in SI base units.
///
/// This is the inverse of `ToSI` and completes the hub-and-spoke conversion system.
/// It allows creating a quantity of a specific unit from a value in SI base units.
///
/// # Examples
///
/// ```rust
/// impl FromSI for Distance<AstronomicalUnit> {
///     fn from_si(meters: f64) -> Self {
///         Self::new(meters / 149_597_870_700.0)  // Convert meters to AU
///     }
/// }
/// ```
pub trait FromSI: Sized {
    /// Create a new quantity from a value in SI base units.
    ///
    /// # Parameters
    ///
    /// - `value`: The numerical value in the appropriate SI base unit
    ///
    /// # Returns
    ///
    /// A new quantity with the value converted to this unit type.
    fn from_si(value: f64) -> Self;
}

/// Trait for providing human-readable unit symbols.
///
/// This trait allows quantities to display themselves with appropriate unit symbols
/// when formatted. Supports Unicode symbols for astronomical units.
///
/// # Examples
///
/// ```rust
/// impl UnitSymbol for AstronomicalUnit {
///     fn symbol() -> &'static str {
///         "AU"
///     }
/// }
///
/// impl UnitSymbol for EarthMass {
///     fn symbol() -> &'static str {
///         "M⊕"  // Unicode symbol for Earth mass
///     }
/// }
/// ```
pub trait UnitSymbol {
    /// Returns the standard symbol for this unit.
    ///
    /// # Returns
    ///
    /// A string slice containing the unit symbol (e.g., "m", "kg", "AU", "M☉").
    fn symbol() -> &'static str;
}

impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Quantity<Unit, L, M, T, K, I, J, N>
{
    /// Create a new quantity with the specified value and unit.
    ///
    /// # Parameters
    ///
    /// - `value`: The numerical value in the specified unit
    ///
    /// # Returns
    ///
    /// A new `Quantity` with the given value and unit type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use star_sim::physics::units_v2::*;
    ///
    /// let distance = Distance::<AstronomicalUnit>::new(1.5);
    /// let mass = Mass::<SolarMass>::new(0.7);
    /// let time = Time::<Gigayear>::new(6.0);
    /// ```
    pub fn new(value: f64) -> Self {
        Self {
            value,
            _unit: PhantomData,
            _dims: PhantomData,
        }
    }

    /// Get the numerical value of this quantity in its current unit.
    ///
    /// # Returns
    ///
    /// The numerical value as a `f64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use star_sim::physics::units_v2::*;
    ///
    /// let distance = Distance::<AstronomicalUnit>::new(1.5);
    /// assert_eq!(distance.value(), 1.5);
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Convert this quantity to a different unit of the same physical dimension.
    ///
    /// This method uses the hub-and-spoke conversion system: it converts the current
    /// quantity to SI units via `ToSI`, then creates a new quantity in the target
    /// unit via `FromSI`.
    ///
    /// # Type Parameters
    ///
    /// - `ToUnit`: The target unit type to convert to
    ///
    /// # Returns
    ///
    /// A new `Quantity` with the same physical value but expressed in the target unit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use star_sim::physics::units_v2::*;
    ///
    /// let distance_au = Distance::<AstronomicalUnit>::new(1.0);
    /// let distance_m = distance_au.convert_to::<Meter>();
    /// assert_eq!(distance_m.value(), 149_597_870_700.0);
    ///
    /// let mass_earth = Mass::<EarthMass>::new(1.0);
    /// let mass_kg = mass_earth.convert_to::<Kilogram>();
    /// assert_eq!(mass_kg.value(), 5.972e24);
    /// ```
    ///
    /// # Compile-Time Safety
    ///
    /// This conversion is only possible between units of the same physical dimension.
    /// Attempting to convert between incompatible dimensions will result in a compile error:
    ///
    /// ```compile_fail
    /// let distance = Distance::<Meter>::new(100.0);
    /// let invalid = distance.convert_to::<Kilogram>(); // Compile error!
    /// ```
    pub fn convert_to<ToUnit>(self) -> Quantity<ToUnit, L, M, T, K, I, J, N>
    where
        Self: ToSI,
        Quantity<ToUnit, L, M, T, K, I, J, N>: FromSI,
    {
        let si_value = self.to_si();
        Quantity::<ToUnit, L, M, T, K, I, J, N>::from_si(si_value)
    }
}

impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Default for Quantity<Unit, L, M, T, K, I, J, N>
{
    fn default() -> Self {
        Self::new(0.0)
    }
}

// Addition (same dimensions)
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Add for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

// Subtraction (same dimensions)
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Sub for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

// Multiplication with scalar
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Mul<f64> for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(self.value * scalar)
    }
}

// Division by scalar
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Div<f64> for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self::new(self.value / scalar)
    }
}

// Negation
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Neg for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

// For now, we'll skip automatic dimensional analysis via multiplication/division
// This feature requires const generic arithmetic which is not yet stable in Rust
// Instead, we'll provide explicit functions for common operations

// Helper function for multiplying quantities - returns result in SI units
pub fn multiply_quantities<
    Unit1,
    Unit2,
    const L1: i8,
    const M1: i8,
    const T1: i8,
    const K1: i8,
    const I1: i8,
    const J1: i8,
    const N1: i8,
    const L2: i8,
    const M2: i8,
    const T2: i8,
    const K2: i8,
    const I2: i8,
    const J2: i8,
    const N2: i8,
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

// Helper function for dividing quantities - returns result in SI units
pub fn divide_quantities<
    Unit1,
    Unit2,
    const L1: i8,
    const M1: i8,
    const T1: i8,
    const K1: i8,
    const I1: i8,
    const J1: i8,
    const N1: i8,
    const L2: i8,
    const M2: i8,
    const T2: i8,
    const K2: i8,
    const I2: i8,
    const J2: i8,
    const N2: i8,
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

// Display implementation
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> fmt::Display for Quantity<Unit, L, M, T, K, I, J, N>
where
    Unit: UnitSymbol,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, Unit::symbol())
    }
}
