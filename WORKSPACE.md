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

`population_lab` reads the versioned scientific inputs from `config/`, including the reduced MIST stellar-evolution grid, the static stellar-orbital hierarchy model, the empirical S-type planetary-stability model, the empirical planet-occurrence model, and the explicit-planet realization model. Its generated catalog always covers the local 10-parsec sphere. It writes `output/population_lab/stellar-evolution.png` for stellar evolution, `output/population_lab/stellar-orbital-hierarchy.png` for companion scales and hierarchy coverage, `output/population_lab/planetary-stability-zones.png` for circumstellar critical semimajor axes, and `output/population_lab/explicit-planets.png` for accepted and rejected explicit candidates.

The official Montréal cooling sequences are not redistributed because their download page does not state a redistribution licence. Run `tools/fetch_montreal_cooling.sh` once to generate the ignored local file `config/white_dwarf_cooling.local.ron`; `population_lab` loads it automatically.

The legacy package can be addressed explicitly with `-p star_sim`. It is not expected to compile until its old unit types are either restored or migrated.
