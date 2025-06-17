# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

**Build and Check:**
- `cargo check` - Quick compilation check
- `cargo build` - Full build
- `cargo build --release` - Optimized build

**Testing:**
- `cargo test` - Run all tests
- `cargo test serialization` - Run specific test module
- `cargo test unit_serialization_test` - Run unit serialization tests

**Run:**
- `cargo run` - Execute main binary (generates teacup_system_typed.ron)

## Architecture

This is a **Rust library for scientifically accurate stellar system generation**, designed as a data generation backend for games and simulations. It does NOT include rendering, game mechanics, or user interfaces.

### Core Structure

**Two main modules:**
- `physics/` - Type-safe unit system with conversions
- `stellar_objects.rs` - Stellar system data structures and generation

### Physics Module (`src/physics/`)

**Custom unit system** similar to `uom` crate but implemented manually without macros:
- All quantities stored internally in SI base units
- Supports conversions between units on demand
- Each unit type has its own module: `distance/`, `mass/`, `time/`, `temperature/`, etc.
- Common pattern: `Distance<Meter>`, `Mass<SolarMass>`, `Temperature<Kelvin>`

**Unit module structure:**
- `mod.rs` - Main type definition and Default trait
- `convert.rs` - Unit conversion implementations
- `operations.rs` - Mathematical operations (Add, Sub, Mul, Div)
- `prefix.rs` - Metric prefixes (Kilo, Mega, etc.)
- `traits.rs` - Shared traits

### Stellar Objects Module

**Hierarchical system modeling:**
- `SerializableStellarSystem` - Top-level system with age and root bodies
- `SerializableBody` - Individual celestial objects (stars, planets, moons)
- `BodyKind` enum - Star, Planet, or Barycenter variants
- `Orbit` struct - Keplerian orbital elements using typed units

**Data structures use Bevy ECS components** (`#[derive(Component)]`) but can work independently.

### Key Dependencies

- `bevy` - ECS framework (components used for data structures)
- `serde` + `ron` - Serialization to RON format
- `rand` + `rand_chacha` - Reproducible random generation

### Generated Output

The main binary generates `teacup_system_typed.ron` containing a sample stellar system in RON format, demonstrating the complete data pipeline from generation to serialization.

### Testing Strategy

Unit serialization tests use a macro pattern to verify RON round-trip serialization for all physics unit types. Tests are located in `tests/serialization.rs`.