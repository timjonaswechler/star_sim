//! A type-safe, high-performance unit system with dimensional analysis.
//!
//! This module provides a completely redesigned unit system that solves the major
//! problems of traditional approaches while maintaining full type safety and adding
//! dimensional analysis capabilities.
//!
//! # Key Improvements Over Traditional Unit Systems
//!
//! ## 🚀 Hub-and-Spoke Conversions (O(n) complexity)
//!
//! Traditional unit systems require O(n²) conversion implementations. This system
//! uses SI units as a conversion hub, reducing complexity to O(n):
//!
//! - **Traditional**: 6 units × 6 units = 36 conversion functions
//! - **Hub-and-spoke**: 6 units × 2 conversions each = 12 conversion functions
//! - **Adding units**: O(1) instead of O(n) new conversions required
//!
//! ## 🛡️ Compile-Time Dimensional Safety
//!
//! The system tracks physical dimensions at compile time, preventing unit mixing errors:
//!
//! ```compile_fail
//! let distance = Distance::<Meter>::new(100.0);
//! let mass = Mass::<Kilogram>::new(5.0);
//! let invalid = distance + mass; // Compile error!
//! ```
//!
//! ## 🏭 Macro-Generated Boilerplate
//!
//! Adding new units requires minimal code thanks to powerful macros:
//!
//! ```rust
//! define_unit_dimension! {
//!     dimension Distance {
//!         base_unit: Meter = 1.0,
//!         units: {
//!             Meter = 1.0,
//!             AstronomicalUnit = 149_597_870_700.0,
//!             LightYear = 9.461e15,
//!         },
//!         symbols: {
//!             Meter = "m",
//!             AstronomicalUnit = "AU",
//!             LightYear = "ly",
//!         }
//!     }
//! }
//! ```
//!
//! ## 🎯 Astronomy-Focused Design
//!
//! Built specifically for stellar simulation with astronomical units:
//! - **Distance**: AU, parsecs, light-years, Earth/Solar radii
//! - **Mass**: Earth masses, solar masses
//! - **Time**: Gigayears for stellar evolution
//! - **Power**: Solar luminosity for stellar output
//! - **Unicode symbols**: R⊕, M☉, R☉, L☉
//!
//! # Quick Start
//!
//! ```rust
//! use star_sim::physics::units_v2::*;
//!
//! // Create quantities with specific units
//! let distance = Distance::<AstronomicalUnit>::new(1.5);
//! let mass = Mass::<SolarMass>::new(0.7);
//! let age = Time::<Gigayear>::new(6.0);
//!
//! // Convert between units (hub-and-spoke)
//! let distance_m = distance.convert_to::<Meter>();
//! let mass_earth = mass.convert_to::<EarthMass>();
//!
//! // Type-safe arithmetic
//! let total_distance = distance + Distance::<AstronomicalUnit>::new(0.5);
//!
//! // Display with proper symbols
//! println!("Distance: {}", distance); // "1.5 AU"
//! println!("Mass: {}", mass);         // "0.7 M☉"
//! ```
//!
//! # Architecture
//!
//! The system is built around several key components:
//!
//! - **[`core`]**: Core types (`Quantity`, `Dimensions`) and traits (`ToSI`, `FromSI`)
//! - **[`constants`]**: Centralized physical constants for conversions
//! - **[`macros`]**: Code generation macros for unit systems
//! - **[`dimensions`]**: Pre-defined quantity types and unit systems
//!
//! # Performance
//!
//! The system is designed for high performance:
//! - **Zero-cost abstractions**: Unit information is compile-time only
//! - **Minimal runtime overhead**: Conversions are simple multiplications
//! - **Efficient serialization**: Direct serde support without boxing
//! - **SIMD-friendly**: f64 values work well with vectorization
//!
//! # Adding New Units
//!
//! See the [`HOW_TO_ADD_UNITS.md`] guide for detailed instructions on extending
//! the system with new units and physical dimensions.
//!
//! # Examples
//!
//! See [`examples/units_v2_examples.rs`] for comprehensive usage examples including:
//! - Basic unit operations
//! - Stellar system modeling
//! - Serialization workflows
//! - Performance comparisons

pub mod constants;
pub mod core;
pub mod dimensions;
pub mod macros;
pub mod prefix;

pub use constants::*;
pub use core::*;
pub use dimensions::*;
pub use prefix::*;
