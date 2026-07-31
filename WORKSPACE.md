# Workspace layout

The existing root package remains intact as an archive of the early Bevy and astronomy experiments. Useful ideas can be migrated individually instead of being lost in a bulk rewrite. The package currently refers to an older interface of the physical-units experiment, so it is preserved but excluded from the default build.

```text
star_sim/                    existing experimental Bevy package
crates/simulation_core/     new Bevy-independent scientific model
crates/physics/units/       reusable physical units
crates/utilities/name_generator/
                             reusable IPA and language generator
apps/population_lab/        plots and statistical validation
apps/bevy_viewer/           interactive view of generated regions
```

Dependencies point in one direction: both applications may depend on `simulation_core`, while `simulation_core` must not depend on Bevy or either application.

## Commands

```bash
cargo check
cargo test -p simulation_core
cargo run -p population_lab -- --seed 42
cargo run -p bevy_viewer
```

`population_lab` currently reads the versioned scientific inputs from `config/`, including the reduced MIST stellar-evolution grid through luminous post-main-sequence phases and the white-dwarf handoff. It writes `output/population_lab/stellar-evolution.png` with the present-day HR plane, a solar-composition reference track, initial-to-current mass comparison, and explicit coverage outcomes.

The legacy package can be addressed explicitly with `-p star_sim`. It is not expected to compile until its old unit types are either restored or migrated.
