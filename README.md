# Stellar Population Simulation

A deterministic Rust simulation for generating scientifically traceable stellar catalogs and exploring them through statistical and visual applications.

## Workspace

This repository uses a virtual Cargo workspace:

- `crates/simulation` — Bevy-independent simulation with a `core` module and optional `models` loader
- `crates/physics/units` — reusable physical units
- `crates/agent_control` — optional control protocol and Bevy plugin
- `crates/utilities/name_generator` — isolated naming experiment
- `apps/population_lab` — statistical plots and model validation
- `apps/bevy_viewer` — interactive Bevy application and visual examples
- [`apps/agent_cli`](apps/agent_cli/README.md) — development and debugging tool for agent-controlled examples
- `assets/scientific_models` — versioned RON inputs

See [`WORKSPACE.md`](WORKSPACE.md) for dependency rules, features, and detailed commands.

## Setup

```bash
git clone <repository-url>
cd star_sim
./setup.sh
cargo check
cargo test -p simulation --features models
```

## Run the applications

```bash
cargo run -p population_lab -- --seed 42
cargo run -p bevy_viewer
```

Optional development integrations are disabled by default:

```bash
cargo run -p bevy_viewer --example agent_control_prototype \
  --features agent-control -- --agent

cargo run -p bevy_viewer --example name_generator_lab \
  --features name-generation
```

## Scientific model data

Versioned, redistributable model inputs live in `assets/scientific_models/` and are loaded through `simulation::models`. The optional Montréal white-dwarf cooling grid must be generated locally:

```bash
tools/fetch_montreal_cooling.sh
```

The generated `assets/scientific_models/white_dwarf_cooling.local.ron` remains ignored by Git.
