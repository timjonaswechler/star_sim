# Workspace layout

The repository root is a virtual Cargo workspace. Production functionality lives in reusable crates; executable packages compose those crates under `apps/`.

```text
assets/scientific_models/       versioned RON inputs for the scientific models
assets/shaders/                 Bevy shader assets
crates/simulation/              simulation crate
  src/core/                     Bevy-independent domain model and deterministic generation
  src/models.rs                 feature-gated loading of scientific model inputs
crates/physics/units/            reusable physical units
crates/utilities/name_generator/ optional naming experiment
crates/automation_control/       reusable Bevy automation-control plugin, protocol, and optional driver
apps/population_lab/             plots and statistical validation
apps/app/                        interactive Bevy application and visual examples
apps/star_sim_debug/             Debug Host and REPL for Star Sim Controlled Sessions
```

Dependencies point inward:

```text
population_lab ──> simulation [feature: models]
app ──────────────> simulation [core only]
app ──(automation-control feature)──> automation_control
star_sim_debug ──drives──> app [feature: automation-control]
```

`simulation::core` must not depend on Bevy, an application, or the RON loader. The `models` feature adds the RON adapter and bundled data only for consumers that request it. Applications should contain composition and presentation, not reusable simulation behavior.

## Commands

```bash
cargo check
cargo test -p simulation --features models
cargo run -p population_lab -- --seed 42
cargo run -p app
```

Optional development functionality is activated at the consuming edge. The Debug Host selects the app's internal controlled build; users do not pass its Cargo feature or child arguments.

```bash
cargo run -p star_sim_debug -- --mode logical
cargo run -p star_sim_debug -- --mode rendered
cargo run -p app --example name_generator_lab --features name-generation
```

The name generator remains in the repository but is excluded from workspace-wide builds. It is only compiled as the optional dependency of the feature-gated viewer example. This keeps its standalone experimental targets out of normal `cargo check`, `cargo test --workspace`, and application builds.

`population_lab` obtains the bundled inputs through `simulation::models`. Its generated catalog always covers the local 10-parsec sphere and it writes plots below `output/population_lab/`.

The official Montréal cooling sequences are not redistributed because their download page does not state a redistribution licence. Run `tools/fetch_montreal_cooling.sh` once to generate the ignored local file `assets/scientific_models/white_dwarf_cooling.local.ron`; `simulation::models` discovers it automatically.
