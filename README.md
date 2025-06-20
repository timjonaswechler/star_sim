# Stellar Properties Generator

A scientifically accurate Rust framework for generating realistic stellar system properties for games and simulations.

## 🌟 Overview

This codebase provides a comprehensive **property generation system** for stellar systems, planets, and cosmic environments. It serves as a scientific foundation for games requiring realistic exoplanet systems, delivering physically accurate parameters that can be consumed by game engines, procedural generation systems, or scientific simulations.

**Not included:** Rendering, game mechanics, or user interfaces. This is purely a **data generation library**.

## 🎯 Purpose

Designed for game developers who need:
- **Scientifically accurate** stellar and planetary properties
- **Reproducible** system generation via seeds
- **Habitability assessment** for world-building
- **Real astrophysics** as foundation for gameplay

## 🚀 Key Features

### 🌟 Stellar Evolution
- Complete stellar lifecycle modeling (Pre-MS → Main Sequence → Post-MS)
- Realistic mass-luminosity relationships
- Spectral classification (O through Y-type stars)
- Habitability zone calculations
- Tidal locking analysis

### 🪐 Planetary Systems
- **Multiple planet types**: Terrestrial, Water worlds, Gas giants, Ice giants, Carbon planets
- **Surface composition**: 11+ surface types based on temperature and composition
- **Physical properties**: Mass, radius, density, gravity, escape velocity
- **Atmospheric considerations**: Pressure, composition, greenhouse effects

### 🔬 Scientific Accuracy
- Based on peer-reviewed exoplanet research
- Realistic mass-radius relationships from observational data
- Proper stellar evolution timescales
- Accurate habitability metrics

### 🌌 Cosmic Context
- **Galactic regions**: Core, Bulge, Habitable Zone, Outer Disk, Halo
- **Cosmic epochs**: From Big Bang to Black Hole Era
- **Elemental abundance**: Chemical evolution over cosmic time
- **Radiation environment**: Supernovae, GRBs, stellar flares

### 🎲 Advanced Systems
- **Binary/Multiple star systems** with orbital stability
- **Trojan objects** at Lagrange points with libration dynamics
- **System hierarchy** analysis for complex systems
- **Long-term stability** assessment (million-year timescales)





## 🏗️ Architecture

```
stellar_objects/
├── universe/           # Cosmic time, galaxies, cosmic environment  
├── cosmic_environment/ # Galactic regions, radiation, elemental abundance
├── stars/             # Stellar properties, evolution, habitability zones
├── planets/           # Planetary composition, mass-radius relations
├── bodies/            # Physical properties, surfaces, atmospheres, habitability
├── moons/             # Satellite systems (framework ready)
├── stellar_systems/   # Binary/multiple systems, stability, hierarchy
└── trojans_asteroid/  # Lagrange points, trojan dynamics
```

## Setup

```bash
git clone <repository-url>
cd star_sim
./setup.sh
cargo check
```

## Builder Usage

The library exposes builders for creating planets, stars, moons and star systems.
They offer sensible defaults and allow customization via method chaining.

```rust
use star_sim::stellar_objects::planets::builder::PlanetBuilder;
use star_sim::physics::units::Mass;

let planet = PlanetBuilder::new()
    .with_seed(42)
    .with_mass(Mass::earth_masses(1.0))
    .build();
```

Every module features its own `*Builder` type following this pattern.

The crate also provides an optional typed unit system located in `physics::unit_system`.
It works similarly to the [`uom`](https://crates.io/crates/uom) crate but is
implemented manually without macros. Quantities are stored internally in SI base
units and can be converted to any supported unit on demand.

```rust
use star_sim::physics::unit_system::{Mass, MassUnit};

let mass = Mass::new(1.0, MassUnit::EarthMass);
let in_kg = mass.get(MassUnit::Kilogram);
```
