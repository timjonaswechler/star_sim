//! Macros for generating unit systems with minimal boilerplate.
//!
//! This module provides powerful macros that automatically generate all the necessary
//! boilerplate code for defining new unit systems. The macros implement the hub-and-spoke
//! conversion pattern, eliminating the O(n²) conversion complexity of traditional approaches.

/// Generates a complete unit system for a specific physical dimension.
///
/// This macro creates unit marker types, implements conversion traits, provides unit symbols,
/// and generates convenience constructors - all with minimal boilerplate. It implements
/// the hub-and-spoke conversion pattern where all conversions go through SI base units.
///
/// # Parameters
///
/// - `dimension`: The name of the quantity type (e.g., `Distance`, `Mass`, `Time`)
/// - `base_unit`: The SI base unit for this dimension
/// - `units`: A list of unit types and their conversion factors to the base unit
/// - `symbols`: Human-readable symbols for each unit type
///
/// # Generated Code
///
/// For each unit specified, this macro generates:
/// - A unit marker struct (e.g., `struct AstronomicalUnit;`)
/// - `UnitSymbol` implementation with the provided symbol
/// - `ToSI` implementation for converting to SI base units
/// - `FromSI` implementation for converting from SI base units
/// - Convenience constructor methods on the base unit type
///
/// # Examples
///
/// ```rust
/// use star_sim::physics::units_v2::*;
/// use star_sim::{define_unit_dimension, define_quantity};
///
/// // First define the quantity type
/// define_quantity!(Distance, 1, 0, 0, 0, 0, 0, 0);
///
/// // Then define the unit system
/// define_unit_dimension! {
///     dimension Distance {
///         base_unit: Meter = 1.0,
///         units: {
///             Meter = 1.0,
///             Kilometer = 1000.0,
///             AstronomicalUnit = 149_597_870_700.0,
///         },
///         symbols: {
///             Meter = "m",
///             Kilometer = "km", 
///             AstronomicalUnit = "AU",
///         }
///     }
/// }
/// ```
///
/// # Conversion Complexity
///
/// Traditional unit systems require O(n²) conversion implementations (every unit to every
/// other unit). This macro generates O(n) conversions by using SI units as a hub:
///
/// - **Traditional**: 6 units × 6 units = 36 conversion functions
/// - **Hub-and-spoke**: 6 units × 2 conversions each = 12 conversion functions
///
/// Adding a new unit requires only 2 additional conversions instead of 2n conversions.
#[macro_export]
macro_rules! define_unit_dimension {
    (
        dimension $dim_name:ident {
            base_unit: $base_unit:ident = $base_value:expr,
            units: {
                $($unit:ident = $conversion:expr),+ $(,)*
            },
            symbols: {
                $($symbol_unit:ident = $symbol:expr),+ $(,)*
            }
        }
    ) => {
        // Define unit marker structs
        $(
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
            pub struct $unit;
        )+
        
        // Implement UnitSymbol trait for each unit
        $(
            impl UnitSymbol for $symbol_unit {
                fn symbol() -> &'static str {
                    $symbol
                }
            }
        )+
        
        // Implement ToSI for each unit (convert to base SI unit)
        $(
            impl ToSI for $dim_name<$unit> {
                fn to_si(&self) -> f64 {
                    self.value * $conversion
                }
            }
        )+
        
        // Implement FromSI for each unit (convert from base SI unit)
        $(
            impl FromSI for $dim_name<$unit> {
                fn from_si(value: f64) -> Self {
                    Self::new(value / $conversion)
                }
            }
        )+
        
        // Note: Prefixed unit implementations are automatically available
        // through the generic Prefixed<P, U> type and its ToSI/FromSI implementations
        
        // Convenience constructors
        impl $dim_name<$base_unit> {
            $(
                pub fn $unit(value: f64) -> $dim_name<$unit> {
                    $dim_name::<$unit>::new(value)
                }
            )+
        }
    };
}

/// Creates a new quantity type with specific dimensional exponents.
///
/// This macro defines a type alias for `Quantity` with specific dimensional exponents,
/// making it easy to create new physical quantity types with compile-time dimensional safety.
///
/// # Parameters
///
/// - `$name`: The name of the new quantity type
/// - `$l, $m, $t, $k, $i, $j, $n`: The dimensional exponents for length, mass, time,
///   temperature, current, luminous intensity, and amount of substance respectively
///
/// # Examples
///
/// ```rust
/// use star_sim::physics::units_v2::*;
/// use star_sim::define_quantity;
///
/// // Define basic quantities
/// define_quantity!(Distance, 1, 0, 0, 0, 0, 0, 0);    // Length¹
/// define_quantity!(Mass, 0, 1, 0, 0, 0, 0, 0);        // Mass¹
/// define_quantity!(Time, 0, 0, 1, 0, 0, 0, 0);        // Time¹
///
/// // Define derived quantities
/// define_quantity!(Area, 2, 0, 0, 0, 0, 0, 0);        // Length²
/// define_quantity!(Volume, 3, 0, 0, 0, 0, 0, 0);      // Length³
/// define_quantity!(Velocity, 1, 0, -1, 0, 0, 0, 0);   // Length¹Time⁻¹
/// define_quantity!(Force, 1, 1, -2, 0, 0, 0, 0);      // Length¹Mass¹Time⁻²
/// ```
///
/// # Usage
///
/// Once defined, you can use these quantity types with any compatible unit:
///
/// ```rust
/// // Distance can use any length unit
/// let d1 = Distance::<Meter>::new(100.0);
/// let d2 = Distance::<AstronomicalUnit>::new(1.5);
///
/// // Velocity can use any length/time unit combination
/// let v = Velocity::<Meter>::new(30.0); // 30 m/s in SI base units
/// ```
#[macro_export]
macro_rules! define_quantity {
    ($name:ident, $l:expr, $m:expr, $t:expr, $k:expr, $i:expr, $j:expr, $n:expr) => {
        pub type $name<Unit> = Quantity<Unit, $l, $m, $t, $k, $i, $j, $n>;
    };
}

// Removed problematic macro - will use direct function definitions instead