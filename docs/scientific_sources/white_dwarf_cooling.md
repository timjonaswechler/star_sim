# White-dwarf cooling backend after the MIST handoff

## Recommendation

The minimal implementable backend should be **`montreal_bedard2020_co_core_thick_h_v1`**, using the Bédard et al. (2020) Montréal sequences with a thick hydrogen layer. It consumes exactly the quantities already produced at the project's MIST-v1.2 white-dwarf handoff—the track-derived remnant mass and non-negative cooling age—and supplies the currently missing white-dwarf luminosity, radius, effective temperature, and surface gravity.

This is a deliberately named hybrid: MIST v1.2 supplies the progenitor history and handoff, while a separate Montréal grid supplies cooling observables. It must not be presented as a continuous MIST track. A future end-to-end `mist_v2_5_plus_mist_iii` backend would be more internally coherent, but its official release is much larger and changes the progenitor model as well as the cooling model. It is not the smallest next step.

The first implementation should support only the thick-H grid:

```text
WhiteDwarfCoolingInput {
    remnant_mass_msun,
    cooling_age_gyr,
    envelope_model: ThickHydrogen,  # q_He = 1e-2, q_H = 1e-4
}

WhiteDwarfCoolingOutput {
    model_id: "montreal_bedard2020_co_core_thick_h_v1",
    effective_temperature_k,
    surface_gravity_log10_cgs,
    radius_rsun,
    luminosity_lsun,
    source_mass_bracket_msun,
    quality_flags,
}
```

Do not recompute or replace `remnant_mass_msun`; it remains the MIST handoff mass and is the mass coordinate used to query the cooling grid.

## Primary sources and downloadable data

Bédard et al. construct the current Montréal sequences with STELUM, initialized from MESA pre-white-dwarf structures. The published grid uses a homogeneous, equal-mass carbon/oxygen core (`X_C = X_O = 0.5`), a helium mantle, and either a thick or thin hydrogen layer. It covers 23 masses from `0.20` through `1.30 M_sun` in `0.05 M_sun` steps ([Bédard et al. 2020, Section 3.2](https://doi.org/10.3847/1538-4357/abafbe); [author-hosted paper](https://www.astro.umontreal.ca/~bergeron/CoolingModels/Bedard2020.pdf)).

The official [Montréal cooling-model page](https://www.astro.umontreal.ca/~bergeron/CoolingModels/) exposes all 46 thick- and thin-H sequences. The exact downloadable resources needed for v1 are:

- [all sequences archive](https://www.astro.umontreal.ca/~bergeron/CoolingModels/CoolingModels/AllSequences.tar.gz), approximately 573 kB;
- individual files named `seq_MMM_thick.txt`, for example the official [`0.60 M_sun` thick-H sequence](https://www.astro.umontreal.ca/~bergeron/CoolingModels/CoolingModels/seq_060_thick.txt).

The official page requests acknowledgement of the site and the relevant publications for its colour tables. It does not display an explicit data/software licence for the cooling-sequence archive. Public download and scientific use are clearly intended, but redistribution of the source tables inside this repository is not explicitly licensed on the page. Therefore either (a) keep a deterministic downloader/reducer and require users to fetch the archive, or (b) obtain written permission before committing derived/full tables. In all outputs and documentation, cite Bédard et al. (2020) and the official site.

This repository follows option (a). Run `tools/fetch_montreal_cooling.sh` to download the official archive and generate the ignored local file `assets/scientific_models/white_dwarf_cooling.local.ron`. `population_lab` discovers that file automatically; without it, white dwarfs retain the explicit `WhiteDwarfCoolingNotBundled` quality flag.

## File format and units

Each sequence is a chronological table. The official site specifies these fields, in this order:

| Column | Meaning | Source unit/encoding | Minimal v1 use |
|---|---|---|---|
| `#Mod` | row index | integer | provenance/testing |
| `Teff` | effective temperature | K | return directly |
| `Log(g)` | base-10 surface gravity | cgs | return directly |
| `R` | radius | cm | convert to `R_sun` |
| `Age` | cooling age | yr | input coordinate; convert from Gyr |
| `L` | photon luminosity | erg s⁻¹ | convert to `L_sun` |
| `Log(Tc)` | central temperature | base-10 K | not needed in v1 |
| `Log(Pc)` | central pressure | base-10 cgs | not needed in v1 |
| `Log(rhoc)` | central density | base-10 cgs | not needed in v1 |
| `Mx/M` | crystallized mass fraction | dimensionless | optional diagnostic |
| `Log(qx)` | crystallization-front coordinate | logarithmic fraction | optional diagnostic |
| `Lnu` | neutrino luminosity | erg s⁻¹ | optional diagnostic |
| `Log(H/*)` ... `Log(O/*)` | total elemental mass fractions | base-10 | provenance only |

These meanings and the thick/thin envelope definitions are stated on the [official sequence page](https://www.astro.umontreal.ca/~bergeron/CoolingModels/#EvolutionarySequences). The v1 conversions should use the same nominal solar units already used by the stellar-evolution module; do not introduce a second set of solar constants.

The source ages begin at `0 yr`. The end age is mass-dependent rather than rectangular: inspection of the official thick-H files gives about `15.6 Gyr` at `0.60 M_sun`, `10.45 Gyr` at `1.10 M_sun`, and `5.62 Gyr` at `1.30 M_sun`. The last temperatures are roughly `1500 K`. Consequently, coverage must be checked against both bracketing mass tracks at the requested cooling age; a global maximum age is invalid.

An exact official-file fixture for `seq_060_thick.txt` is useful for parser and node-identity tests:

| Point | `Teff` (K) | `log g` | `R` (cm) | age (yr) | `L` (erg s⁻¹) |
|---|---:|---:|---:|---:|---:|
| first row | `98454.6021` | `7.39369788` | `1.793422E+09` | `0` | `2.153481E+35` |
| last row | `1495.3692` | `8.04048163` | `8.517078E+08` | `1.560565E+10` | `2.584681E+27` |

## Physical coverage and declared approximations

The selected grid includes gravitational contraction at the hot start, neutrino cooling, thermal cooling, and crystallization. Bédard et al. describe the new sequences as extending most models above `100,000 K`; low-mass sequences start cooler. They also report that updated conductive opacities change cooling ages by as much as roughly ten percent relative to their older sequences, illustrating that cooling age is model-dependent rather than an exact clock ([Bédard et al. 2020, Section 3.2](https://doi.org/10.3847/1538-4357/abafbe)).

Important limitations must be represented explicitly:

- The fixed C/O-core model is not physically appropriate across the full nominal mass axis. The authors caution that white dwarfs below approximately `0.45 M_sun` are expected to have helium cores and those above approximately `1.1 M_sun` oxygen/neon cores. V1 should therefore define its scientifically supported domain as `0.45 <= M_WD/M_sun <= 1.10`. The tabulated `0.20–0.40` and `1.15–1.30 M_sun` C/O sequences may be retained as data but must not be silently used as ordinary single-star C/O models.
- The sequences keep a prescribed chemical profile, assume diffusive equilibrium, turn transport processes off, and ignore residual nuclear burning. The grid does not use the progenitor's `[Fe/H]` or `[alpha/Fe]` as interpolation axes.
- Thick H (`q_He=10^-2`, `q_H=10^-4`) is a model choice, not an inferred atmosphere for every simulated white dwarf. The official release also provides thin H (`q_H=10^-10`), but mixing the two without a population prescription would add an uncalibrated random choice.
- Bédard et al. warn that very short cooling ages, `log10(age/yr) <= 5`, are sensitive to the zero point of the initial model. In this project that issue is compounded by joining two independently defined handoffs. Return observables, but attach a flag such as `YoungWhiteDwarfCoolingZeroPointUncertain` for ages up to `10^5 yr`; do not claim that a MIST knee and the Montréal zero-age row are exactly continuous.
- Magnetic white dwarfs, merger products, accreting white dwarfs, atmosphere changes, He-core white dwarfs, and O/Ne-core white dwarfs are outside this v1 backend.

No input should be clamped into the supported range. Suggested typed failures are `OutsideWhiteDwarfCoolingMassGrid`, `OutsideWhiteDwarfCoolingAgeGrid`, and `UnsupportedWhiteDwarfCoreComposition`.

## Interpolation policy

The grid is rectangular in mass nodes but not in cooling-age endpoints. A safe minimal interpolator is:

1. Validate finite mass and non-negative cooling age.
2. Select the thick-H table only.
3. Find the exact or two bracketing mass sequences.
4. On each sequence independently, bracket the requested chronological cooling age. Refuse the query unless both tracks cover it.
5. Interpolate within each track, then interpolate the two results linearly in mass.
6. Interpolate positive observables (`Teff`, `R`, and `L`) in their logarithms; interpolate `log g` directly. Use piecewise linear interpolation, not an unconstrained cubic that can overshoot at rapid neutrino-cooling or crystallization features.
7. Reproduce source nodes exactly and derive one observable tuple from one consistent interpolation path.

Because age zero cannot be logged, use linear age on the first interval beginning at zero. For positive ages, interpolation in `log10(cooling_age)` is a reasonable compact-grid policy, but it is an implementation choice rather than a prescription stated by Bédard et al. It must be convergence-tested against interpolation in linear age and against a denser retained table. Preserve the original age samples in the reduced data until those tests justify downsampling.

Required invariants and acceptance tests are:

- every bundled mass node and age node reproduces the official table within parsing/conversion precision;
- `L/L_sun = (R/R_sun)^2 (Teff/5772 K)^4` within table precision;
- no interpolation crosses the end of either mass-bracketing sequence;
- age `0` is accepted, a negative age is rejected, and the young-zero-point flag is present through `10^5 yr`;
- masses below `0.45` or above `1.10 M_sun` return the typed unsupported-core result in the scientifically supported v1 mode;
- the old `WhiteDwarfCoolingNotBundled` flag disappears only when a cooling lookup succeeds; it remains present, or is replaced by a typed cooling failure, when lookup fails.

## Why not use MIST III for this minimal milestone?

The official [MIST III white-dwarf data release](https://doi.org/10.5281/zenodo.15242047) is the preferred future coherent upgrade. Its track files expose `log_cool_age`, `log_tot_age`, `log_L`, `log_Teff`, `log_R`, `log_g`, hydrogen/helium layer masses, convection-zone mass, central temperature, and central density. Constant-cooling-age contours span `log10(age/yr)=6.0–10.3` in `0.05` dex steps. The release includes progenitor metallicity and alpha-enhancement grids, non-rotating and default rotating variants, bolometric corrections, example code, and the complete MESA setup ([Bauer et al. 2026](https://doi.org/10.3847/1538-4365/ae401e); [model data](https://zenodo.org/records/15242047); [MESA setup](https://doi.org/10.5281/zenodo.15196933)).

The Zenodo record marks the data open under `CC BY 4.0`. Its smallest broad archive is the `177.1 MB` default, solar-scaled set; the full and non-rotating sets are about `879–881 MB`. More importantly, these white dwarfs descend from MIST-v2 progenitors. Substituting their cooling tail behind a MIST-v1.2 handoff would still be a hybrid while adding more axes and a much larger reduction task. Adopt MIST III later as a complete, separately versioned MIST-v2.5 progenitor-plus-cooling backend, not as an unmarked data replacement.

## Implementation boundary

The cooling backend should be a pure table evaluator, independent of catalog generation and rendering. The catalog/evolution pipeline owns when an object becomes a white dwarf and passes its MIST-derived mass and cooling age into this evaluator. On success, it merges the four photospheric observables into the existing `StellarEvolutionSnapshot`; on failure, it preserves the white-dwarf state and remnant/cooling ages while returning a typed coverage result. This keeps the scientific model replaceable and allows Montréal and future MIST-III results to be compared without changing the galaxy or population model.
