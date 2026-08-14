//! Stellar-population simulation and optional scientific model data.

pub mod core;

#[cfg(feature = "models")]
pub mod models;

pub use core::*;
