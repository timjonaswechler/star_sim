//! Physics engine for scientifically accurate stellar system simulation.
//!
//! This module provides a comprehensive physics framework designed specifically for
//! stellar system generation and simulation. It includes type-safe unit systems,
//! physical constants, and mechanics calculations optimized for astronomical scales.
//!
//! # Module Structure
//!
//! ## Core Components
//!
//! - **[`units`]** - Type-safe unit system with dimensional analysis
//! - **[`constants`]** - Physical and astronomical constants
//! - **[`mechanics`]** - Classical mechanics for orbital dynamics
//! - **[`statics`]** - Static equilibrium and stability calculations
//! - **[`thermodynamics`]** - Thermal processes and stellar atmospheres
//!
//! # Design Philosophy
//!
//! ## Scientific Accuracy
//! - All calculations use real physical constants and equations
//! - Units prevent dimensional analysis errors at compile time
//! - Mathematical models based on established astrophysics literature
//!
//! ## Performance
//! - Zero-cost abstractions for unit conversions
//! - Optimized for f64 precision suitable for astronomical calculations
//! - Minimal runtime overhead while maintaining type safety
//!
//! ## Extensibility
//! - Modular design allows selective use of components
//! - Easy to add new unit types and physical quantities
//! - Supports both analytical and numerical approaches
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use star_sim::physics::{units::*, constants::*};
//!
//! // Create a stellar system with type-safe units
//! let star_mass = Mass::<SolarMass>::new(1.2);
//! let planet_distance = Distance::<AstronomicalUnit>::new(0.8);
//! let system_age = Time::<Year>::new(4.6e9);
//!
//! println!("System age: {}", system_age);
//! println!("Star mass: {}", star_mass);
//! println!("Planet distance: {}", planet_distance);
//! ```
//!
//! # Safety Features
//!
//! - **Compile-time dimensional analysis** - Prevents unit mixing errors
//! - **Type-safe conversions** - Hub-and-spoke conversion system
//! - **Overflow protection** - Appropriate numeric types for astronomical scales
//! - **Validated constants** - All physical constants from authoritative sources
//!
//! # Performance Characteristics
//!
//! - Unit conversions: O(1) multiplication/division
//! - Dimensional checking: Zero runtime cost
//! - Memory usage: Same as raw f64 values
//! - SIMD friendly: Works well with vectorized operations

// pub mod astrophysics;
pub mod constants;
pub mod mechanics;
pub mod statics;
pub mod thermodynamics;

// Units are now provided by the external physics-units crate
// pub mod units;
