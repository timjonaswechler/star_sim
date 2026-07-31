# Scientific basis for a first stellar-evolution block

This note defines a deliberately narrow, reproducible version-1 single-star evolution model. The central recommendation is to interpolate a published stellar-track grid. A single power law for lifetime, luminosity, radius, and temperature is not scientifically adequate over `0.08–100 M_sun`: the mass dependence changes, metallicity and mass loss matter, post-main-sequence phases are short and non-monotonic in the HR diagram, and compact-remnant formation is not a monotonic function of birth mass.

The recommended luminous-star backend is **MIST v1.2, non-rotating (`v/vcrit = 0`), solar-scaled**. MIST supplies tracks from pre-main sequence to central hydrogen depletion, white-dwarf cooling, or central carbon depletion depending on mass. Its published solar-scaled grid covers `0.1–300 M_sun`, `5 <= log10(age/yr) <= 10.3`, and principally `-2.0 <= [Z/H] <= +0.5`; the more metal-poor extension to `-4` does not cover every late phase ([Choi et al. 2016](https://doi.org/10.3847/0004-637X/823/2/102); [open manuscript](https://arxiv.org/abs/1604.08592); [official MIST site](https://mist.science/)). The live [official track interpolator](https://mist.science/interp_tracks.html) currently exposes `0.1–300 M_sun` and `-4 <= [Fe/H] <= +0.5`, but this wider input form must not be mistaken for uniform evolutionary-phase coverage.

This is a **single-star** model. The project's companions are evolved independently as coeval stars with shared initial composition. It does not model mass transfer, mergers, common envelopes, tidal effects, supernova disruption, rejuvenation, or chemically altered donor/accretor surfaces.

## Exact v1 interface and provenance

Keep initial and present-day properties distinct:

```text
StellarEvolutionInput {
    initial_mass_msun: f64,
    age_since_formation_gyr: f64,
    initial_global_metallicity_mh: f64,
    initial_hydrogen_x: f64,
    initial_helium_y: f64,
    initial_metals_z: f64,
}

StellarEvolutionOutput {
    model_id: "mist_v1_2_nonrotating_solar_scaled",
    source_initial_mass_msun: f64,
    source_metallicity_coordinate: f64,
    state: PreMainSequence | MainSequence | SubgiantAndRedGiantBranch
         | HeliumIgnitionTransition | CoreHeliumBurning
         | EarlyAsymptoticGiantBranch | ThermallyPulsingAsymptoticGiantBranch
         | PostAsymptoticGiantBranch | WolfRayet | AdvancedBurningTrackEnd
         | WhiteDwarf,
    raw_eep: f64?,
    raw_phase: i8?,
    zams_age_gyr: f64?,
    tams_age_gyr: f64?,
    main_sequence_duration_gyr: f64?,
    fractional_main_sequence_age: f64?,
    current_mass_msun: f64,
    luminosity_lsun: f64?,
    radius_rsun: f64?,
    effective_temperature_k: f64?,
    surface_gravity_log10_cgs: f64?,
    remnant_mass_msun: f64?,
    quality_flags: set<QualityFlag>,
}
```

Required flags and typed coverage results include `AlphaProjectedToSolarScaled`, `OutsideTrackGrid`, `IncompleteLowMetallicityPhases`, `WhiteDwarfCoolingNotBundled`, `RemnantPrescriptionDependent`, `UnsupportedCoreCollapse`, and `BinaryInteractionIgnored`. Preserve `initial_mass_msun`; never overwrite it with `current_mass_msun`.

The existing alpha-corrected `[M/H]` may be used as the solar-scaled MIST metallicity coordinate in v1, with `AlphaProjectedToSolarScaled` whenever `[alpha/Fe] != 0`. This is an explicit approximation, not an alpha-enhanced track. `X`, `Y`, and `Z` validate composition and remain provenance; they are not three extra independent interpolation axes. The chosen grid's mixture and helium-enrichment law own the evolution calculation.

Reject non-finite inputs, negative ages, `X <= 0`, `Y <= 0`, `Z <= 0`, or `|X+Y+Z-1|` outside the chemistry module's numerical tolerance. Do not silently clamp mass, age, or composition to a grid boundary.

## Equivalent evolutionary points and interpolation

MIST's Equivalent Evolutionary Points (EEPs) put physically corresponding stages at corresponding indices. Dotter's construction exists specifically because interpolation only by raw age or fractional lifetime can mix unlike phases and because the initial-mass/age relation can become non-monotonic ([Dotter 2016](https://doi.org/10.3847/0067-0049/222/1/8); [open manuscript](https://arxiv.org/abs/1601.05144)).

Use this order:

1. bracket the requested initial composition and mass;
2. interpolate ages and continuous quantities between the same EEPs on the bracketing tracks;
3. locate the requested chronological age on the resulting track;
4. interpolate `star_mass`, `log_L`, `log_Teff`, `log_R`, and `log_g` only within one continuous phase interval;
5. never average categorical phase codes or interpolate through a track termination.

Linear interpolation in `[M/H]` and initial mass is acceptable for a coarse prototype only after exact-node and convergence tests. Prefer logarithmic interpolation for positive quantities spanning orders of magnitude and monotone interpolation in age. A production interpolator should follow Dotter's EEP algorithm, including its handling of non-monotonic mass-age branches, instead of inventing a bilinear shortcut.

The official MIST reader documents the columns `star_age`, `star_mass`, core masses, `log_L`, `log_Teff`, `log_R`, `log_g`, central abundances, and `phase`, and gives these phase codes ([official reader example](https://mist.science/read_mist_models_demo.html)):

| MIST phase | Meaning | Broad v1 state |
|---:|---|---|
| `-1` | pre-main sequence | `PreMainSequence` |
| `0` | main sequence | `MainSequence` |
| `2` | subgiant + red-giant branch | `PostMainSequenceLuminous` |
| `3` | core-helium burning | `PostMainSequenceLuminous` |
| `4` | early AGB | `PostMainSequenceLuminous` |
| `5` | thermally pulsing AGB | `PostMainSequenceLuminous` |
| `6` | post-AGB, continuing toward WD cooling in eligible tracks | see WD handoff below |
| `9` | Wolf-Rayet | `PostMainSequenceLuminous` |

The primary EEPs in the v1.2 files are `1` first PMS, `202` ZAMS, `353` intermediate-age MS, `454` TAMS, `605` RGB tip, `631` zero-age core-He burning, `707` terminal core-He burning, and then low- and high-mass dependent endpoints. At exact EEP `454`, the downloaded files already report phase `2`; therefore the exact v1 main-sequence condition is `202 <= EEP < 454`, not `EEP <= 454`.

Because input age includes zero, the five states originally proposed by the project are not exhaustive: young stars can be pre-main-sequence. `PreMainSequence` must be added or the model must return an unsupported result. Calling all PMS stars main-sequence would be a physical classification error.

## Main-sequence lifetime

Expose three different quantities:

```text
zams_age       = track age at EEP 202
tams_age       = track age at EEP 454
ms_duration    = tams_age - zams_age
fractional_ms_age = (age - zams_age) / ms_duration   # only on MS
```

State classification compares the system's age since formation to `zams_age` and `tams_age`. A conventional formula such as `10 Gyr * M^-2.5` must not be used over the full mass interval or treated as chemistry-aware. If a later compact analytic backend is genuinely needed, Hurley, Pols & Tout provide a complete, separately versioned rapid single-star evolution model fitted to detailed tracks, including remnant phases; the official implementation states a domain of `0.1–100 M_sun` and `0.0001 <= Z <= 0.03` ([Hurley, Pols & Tout 2000](https://doi.org/10.1046/j.1365-8711.2000.03426.x); [open manuscript](https://arxiv.org/abs/astro-ph/0001295); [official SSE code/interface](https://astronomy.swin.edu.au/~jhurley/stellar.html)). Cherry-picking one SSE lifetime equation while using unrelated luminosity and radius power laws is not an internally consistent backend.

## Present luminosity, radius, and temperature

For PMS, MS, and luminous post-MS stars, return the interpolated values from the same MIST row/segment. Do not independently evaluate mass-luminosity, mass-radius, and temperature formulae: they will generally violate both the track and the Stefan-Boltzmann relation.

As an invariant,

\[
\frac{L}{L_\odot}=\left(\frac{R}{R_\odot}\right)^2
\left(\frac{T_{\rm eff}}{5772\,{\rm K}}\right)^4.
\]

The nominal solar effective temperature `5772 K`, luminosity, and radius are IAU nominal conversion constants ([IAU 2015 Resolution B3 / Prša et al. 2016](https://doi.org/10.3847/0004-6256/152/2/41); [open resolution text](https://arxiv.org/abs/1510.07674)). File precision determines the test tolerance.

Compact remnants are different:

- White-dwarf luminosity, radius, and temperature require a cooling age and a WD cooling grid.
- A neutron star's thermal luminosity and temperature require a cooling model, envelope composition, magnetic field, and age; a fixed `12 km` radius does not supply an HR-diagram luminosity.
- A black hole has no stellar photospheric radius or effective temperature. Return `None` for stellar `L`, `R`, and `Teff` and plot a categorical glyph. The Schwarzschild radius, if later exposed, must be named separately and must not masquerade as `stellar_radius_rsun`.

## White-dwarf handoff and initial-final mass relation

MIST phase `6` includes post-AGB evolution and cannot by itself distinguish a hot luminous post-AGB core from a cooling white dwarf. For a grid-only implementation, find the maximum `Teff` after the AGB departure on that same track: classify earlier phase-6 points as `PostMainSequenceLuminous` and subsequent points as `WhiteDwarf`. Preserve the exact handoff age and define `cooling_age = system_age - wd_handoff_age`.

For a separate empirical remnant mass, use the MIST-based, three-piece Cummings et al. initial-final mass relation only in its measured progenitor range:

\[
M_f/M_\odot =
\begin{cases}
0.080(M_i/M_\odot)+0.489, & 0.83 < M_i/M_\odot < 2.85,\\
0.187(M_i/M_\odot)+0.184, & 2.85 < M_i/M_\odot < 3.60,\\
0.107(M_i/M_\odot)+0.471, & 3.60 < M_i/M_\odot < 7.20.
\end{cases}
\]

The fitted coefficient uncertainties are respectively `(0.016, 0.030)`, `(0.061, 0.199)`, and `(0.016, 0.077)`, with correlated slope/intercept errors. The observed scatter about the relation is about `0.06 M_sun`. The paper's non-detection of a metallicity effect applies only over approximately `-0.15 < [Fe/H] < +0.15`, not the simulator's full chemistry support ([Cummings et al. 2018](https://doi.org/10.3847/1538-4357/aadfd6); [open manuscript](https://arxiv.org/abs/1809.01673)). Do not extrapolate this IFMR below `0.83` or above `7.2 M_sun`; low-mass single stars below the calibrated range have not had time to make ordinary WDs within the simulated `13.5 Gyr` anyway.

For WD observables, the recommended later backend is the Bédard et al. Montréal cooling grid. The official release provides 23 mass sequences spanning `0.2–1.3 M_sun`, with thick-H (`q_H=10^-4`) and thin-H (`q_H=10^-10`) envelopes and outputs including cooling age, luminosity, radius, and effective temperature ([Bédard et al. 2020](https://doi.org/10.3847/1538-4357/abafbe); [official downloadable cooling sequences](https://www.astro.umontreal.ca/~bergeron/CoolingModels/)). Choose and record the atmosphere/envelope model; v1 should default to thick-H only as a declared model choice. Do not extrapolate C/O-core tables to unsupported He-core or O/Ne WDs.

For the present MIST-v1.2 backend, prefer the current mass at the track's WD handoff over evaluating the Cummings IFMR for the same star. Mixing a track-predicted current mass with a second, empirical remnant mass would produce two incompatible values. The Cummings relation is useful only as an explicitly named fallback for a track that does not reach a WD handoff; such a fallback must carry `RemnantPrescriptionDependent` and cannot manufacture an exact cooling age from a missing post-AGB segment.

As of 2026, a more internally coherent future upgrade is available: the MIST III WD grid descends from the MIST-v2 progenitor calculations and supplies C/O-core, hydrogen-atmosphere tracks with realistic WD cooling physics, approximately `0.5–1.05 M_sun` in WD mass and `0.6 <= M_ZAMS < 7 M_sun` in progenitor mass. Its files expose both cooling age and total age. It also states that the artificial crossing between the end of the AGB and the start of the WD cooling sequence is omitted and lasts roughly `10^2–10^4 yr` in those models ([Bauer et al. 2026](https://doi.org/10.3847/1538-4365/ae401e); [open manuscript](https://arxiv.org/abs/2509.21717); [official data release](https://doi.org/10.5281/zenodo.15242047)). Adopting it should be an end-to-end `mist_v2_5_plus_mist_iii` backend, not an unmarked substitution inside `mist_v1_2_nonrotating_solar_scaled`. Montréal remains a valid separately versioned alternative and a useful cross-grid systematic comparison.

## Neutron stars and black holes

A rule such as `8–25 M_sun => neutron star; above 25 M_sun => black hole` is not a scientifically defensible deterministic classifier. Solar-metallicity calculations over `9–120 M_sun` find that exploding progenitors are not a simply connected sequence in initial mass. In that model the mean gravitational NS mass is near `1.4 M_sun`, while mean BH masses are about `9 M_sun` if only the helium core implodes or `14 M_sun` if the whole presupernova star collapses; these are population summaries, not constants to assign every remnant ([Sukhbold et al. 2016](https://doi.org/10.3847/0004-637X/821/1/38); [open manuscript](https://arxiv.org/abs/1510.04643)).

The exact safe v1 behavior when a massive MIST track ends at carbon depletion is therefore:

```text
return UnsupportedCoreCollapse {
    last_current_mass_msun,
    last_co_core_mass_msun,
    track_end_age_gyr,
}
```

The public enum may reserve `NeutronStar` and `BlackHole`, but must not emit them until a named explosion/remnant prescription is implemented. The recommended extension is `fryer2012_delayed`, evaluated from final presupernova mass and CO-core mass and tagged `RemnantPrescriptionDependent`. Fryer et al. publish analytic rapid and delayed prescriptions; the delayed engine yields a continuous compact-remnant distribution, whereas the rapid engine produces a sharp NS/BH transition and a mass gap. Above roughly `30 M_sun`, stellar-wind and metallicity choices dominate the uncertainty ([Fryer et al. 2012](https://doi.org/10.1088/0004-637X/749/1/91); [open manuscript](https://arxiv.org/abs/1110.1726)). This extension must be implemented and golden-tested as a complete named prescription, including baryonic-to-gravitational mass conversion and its stated boundary rules; it must not be reduced to a ZAMS-mass cutoff.

Thus a minimal implementation can scientifically classify PMS, MS, luminous post-MS, and WDs, while reporting a structured unsupported result for core collapse. Claiming all five requested terminal categories before the remnant engine exists would overstate the model.

## Post-main-sequence topology and safe v1 policy

The primary EEP definitions are physical, but the integer EEP positions in the distributed MIST v1.2 files also encode a branch. Dotter defines the post-MS primary EEPs as RGB tip, zero-age core-He burning, terminal-age core-He burning, and then either (a) TP-AGB followed by post-AGB and WD cooling for a WD progenitor or (b) the end of core-C burning for a massive star. The branch is selected from the track's terminal central-temperature behavior, not from a universal initial-mass cutoff. Processing stops at the first primary EEP that cannot be identified ([Dotter 2016, Section 2.1](https://doi.org/10.3847/0067-0049/222/1/8); [open manuscript](https://arxiv.org/abs/1601.05144)). Consequently, `initial_mass_msun` alone must never be used to infer which late EEPs exist.

The exact distributed numbering is:

| EEP | Physical definition | State ownership at the exact node |
|---:|---|---|
| `454` | TAMS, central H exhausted | post-MS (`phase=2`) |
| `605` | RGB tip, or its high-mass analogue | core-He transition (`phase=3`) |
| `631` | zero-age core-He burning | core-He burning (`phase=3`) |
| `707` | terminal-age core-He burning | early AGB / post-core-He luminous (`phase=4`) |
| `808` | TP-AGB onset on the WD-progenitor branch **or** terminal carbon-burning EEP on the massive branch | branch-dependent (`phase=5` in both sampled cases) |
| `1409` | post-AGB onset, WD-progenitor branch only | post-AGB (`phase=6`) |
| `1710` | WD cooling-sequence endpoint defined from central Coulomb coupling, WD-progenitor branch only | WD cooling (`phase=6`) |

The implementation should therefore persist `track_branch = WhiteDwarfProgenitor | MassiveBurning` and the ordered primary-EEP list from each source header. `raw_phase` remains provenance and a broad plotting aid; it is not sufficient to disambiguate EEP `808`, and phase `6` does not distinguish an expanding post-AGB envelope from a cooling WD.

For the detailed state enum, use these half-open intervals where the track contains the necessary endpoints:

```text
454 <= EEP < 605   SubgiantOrRedGiantBranch
605 <= EEP < 631   HeliumIgnitionTransition
631 <= EEP < 707   CoreHeliumBurning
707 <= EEP < 808   EarlyAsymptoticGiantBranch
808 <= EEP < 1409  ThermallyPulsingAsymptoticGiantBranch  # WD branch only
808                AdvancedBurningTrackEnd                 # massive branch endpoint
1409 <= EEP < knee PostAsymptoticGiantBranch               # WD branch only
knee <= age         WhiteDwarf                              # see cooling policy
```

Here `knee` is the maximum `log_Teff` after EEP `1409` on a track that actually contains points beyond `1409`. At equality, the WD owns the boundary and `cooling_age_gyr = age_gyr - knee_age_gyr`. Before the knee, keep the phase-6 object luminous and post-AGB. If no post-AGB continuation exists, return `PostAgbTrackIncomplete`; do not treat EEP `1409` itself as a WD. If the requested age is beyond a massive-branch track that reached EEP `808`, return `UnsupportedCoreCollapse` with its last mass, CO-core mass, and endpoint age. If a track terminates earlier, such as EEP `631`, return the distinct `TrackEndedBeforeExpectedEndpoint`; it did not reach a defensible core-collapse handoff.

The current milestone deliberately does **not** bundle WD cooling. At or after the knee it may return `WhiteDwarf` with track-derived remnant mass and non-negative cooling age, but `L`, `R`, and `Teff` must be absent and `WhiteDwarfCoolingNotBundled` must be set. Values from the old v1.2 phase-6 tail must not be presented as the promised long-term cooling model. A later, named MIST-III or Montréal backend owns those observables.

### Observed source-track topology

The following headers were extracted on 2026-07-31 from official MIST-v1.2, non-rotating, solar-composition files returned by the [official track interpolator](https://mist.science/interp_tracks.html). This is a regression fixture for this exact model/version, not a mass-threshold law:

| Initial mass (`M_sun`) | Header branch | Last EEP | Safe interpretation |
|---:|---|---:|---|
| `0.1`, `0.2`, `0.5` | low-mass | `454` | PMS–TAMS only; later evolution is absent, though it is unreachable within the simulator's `13.5 Gyr` age cap |
| `0.8`, `1.0`, `1.5`, `2.0` | low-mass | `1710` | complete listed route through post-AGB and the v1.2 WDCS endpoint |
| `3.0` | low-mass | `1409` | reaches post-AGB onset but no WD knee; return incomplete afterward |
| `5.0` | low-mass | `1710` | complete listed route through WDCS endpoint |
| `8.0` | low-mass | `808` | reaches TP-AGB onset only; no post-AGB/WD handoff |
| `15`, `25`, `100` | high-mass | `808` | massive branch; beyond the endpoint is unsupported core collapse |
| `40` | high-mass | `631` | terminates during the core-He portion; not a valid core-collapse handoff |

The non-monotonic `3`, `5`, `8 M_sun` completion pattern is direct evidence that a rectangular post-MS grid cannot be assumed. For interpolation, form only the common EEP prefix of all bracketing tracks and never interpolate through a termination. Better coverage requires adding nearby source-mass nodes and convergence tests; it cannot be repaired by copying the longer neighbor's endpoint.

### Post-MS golden fixtures

For the official solar `1 M_sun` track, exact source rows are:

| Point | Age (Gyr) | Current mass | `log10(L/L_sun)` | `log10(Teff/K)` | `log10(R/R_sun)` | Phase |
|---|---:|---:|---:|---:|---:|---:|
| EEP `605`, RGB tip | `11.33617629147941` | `0.953602324825483` | `3.377823235500291` | `3.487177816081699` | `2.237233137768862` | `3` |
| EEP `631`, ZACHeB | `11.33791245441244` | `0.953459656031981` | `1.673413573030812` | `3.663674414402668` | `1.032035109892185` | `3` |
| EEP `707`, TACHeB | `11.44758072378725` | `0.951041458287192` | `2.008938212854032` | `3.635733533546944` | `1.255679191515243` | `4` |
| EEP `808`, TP-AGB onset | `11.46157279689226` | `0.945573553775712` | `3.000571496594100` | `3.534271942758570` | `1.954419014962024` | `5` |
| EEP `1409`, post-AGB onset | `11.46292597195444` | `0.598925855986035` | `3.523760803090961` | `3.476429773670422` | `2.331698006386750` | `6` |
| maximum-`Teff` knee, EEP `1611` | `11.46295979000143` | `0.539831842052712` | `2.988117362419048` | `5.081053201909836` | `-1.145370570428035` | `6` |
| EEP `1710`, v1.2 WDCS endpoint | `11.46547147529703` | `0.539831758849139` | `0.199956172214389` | `4.677813369224881` | `-1.732971500160455` | `6` |

Required acceptance cases are: exact-node identity at every row above; state changes at EEPs `454`, `605`, `631`, `707`, `808`, `1409`, and the temperature knee without averaging enum values; a `1 M_sun`, solar-composition object just below/at/above `11.46295979000143 Gyr` changes from post-AGB to WD with cooling ages `<0` rejected, `0`, and positive respectively; a solar `3 M_sun` request after EEP `1409` yields `PostAgbTrackIncomplete`; a solar `15 M_sun` request after EEP `808` yields `UnsupportedCoreCollapse`; and a solar `40 M_sun` request after EEP `631` yields `TrackEndedBeforeExpectedEndpoint`.

## Official solar-track golden data

The following values were extracted on 2026-07-31 from theoretical `.track.eep` files returned by the official MIST web interpolator for MIST v1.2, `[Fe/H]=0`, `[alpha/Fe]=0`, and `v/vcrit=0`. Ages are since the beginning of the track, not durations since ZAMS. These values are source-data regression oracles, not observational measurements.

| Initial mass (`M_sun`) | ZAMS age at EEP 202 (Gyr) | TAMS age at EEP 454 (Gyr) |
|---:|---:|---:|
| `0.1` | `1.55417027` | `3256.98673` |
| `0.2` | `0.772525126` | `1140.71414` |
| `0.5` | `0.157391126` | `95.7971635` |
| `0.8` | `0.0668512972` | `22.4227013` |
| `1.0` | `0.0418734723` | `9.91942394` |
| `1.5` | `0.0139005411` | `2.39192079` |
| `2.0` | `0.00903714073` | `1.06277694` |
| `3.0` | `0.00348217122` | `0.359245629` |
| `5.0` | `0.00100293494` | `0.0999975776` |
| `8.0` | `0.000316163163` | `0.0356663394` |
| `15` | `0.0000858788620` | `0.0124396728` |
| `25` | `0.0000403856291` | `0.00693248179` |
| `40` | `0.0000239230921` | `0.00474408264` |
| `100` | `0.0000115901780` | `0.00297950559` |

TAMS values longer than the age of the Universe are valid model endpoints but unreachable in this simulator's `age <= 13.5 Gyr` domain.

Detailed `1 M_sun` solar-composition oracles are:

| Point | Age (Gyr) | Current mass | `log10(L/L_sun)` | `log10(Teff/K)` | `log10(R/R_sun)` | Phase |
|---|---:|---:|---:|---:|---:|---:|
| EEP 202, ZAMS | `0.0418734723299` | `0.999997374272` | `-0.127208577190` | `3.756412214260` | `-0.0537515649391` | `0` |
| age interpolation at 4.568 Gyr | `4.568` | `0.999839456` | `0.043819491` | `3.767025909` | `0.010535079` | `0` |
| EEP 454, TAMS | `9.91942394274` | `0.999438309638` | `0.358461324716` | `3.754652577290` | `0.192602659964` | `2` |

The 4.568-Gyr row is a linear-in-age interpolation between the adjacent official EEP rows 354 and 355 and should be reproduced only by an interpolator with that same local rule. It is useful as a regression fixture, not as a claim that MIST must reproduce nominal solar values exactly.

Useful exact IFMR arithmetic oracles are `M_f(1)=0.569`, `M_f(3)=0.745`, and `M_f(5)=1.006 M_sun`. The neighboring pieces meet to rounding at `2.85 M_sun` (`0.7170`) and `3.60 M_sun` (`0.8572` versus `0.8562`); code must define endpoint ownership explicitly rather than leave gaps.

## Validation and acceptance bounds

1. **Source-node identity:** every bundled golden node reproduces `star_age`, `star_mass`, `log_L`, `log_Teff`, `log_R`, and phase to the precision stored in the source file. Interpolation endpoints must be exact.
2. **Age and bounds:** all track ages are finite and strictly increasing after duplicate-point policy; outputs are finite and positive after exponentiating logarithms. Outside model coverage returns a typed unsupported result, never a boundary clamp.
3. **State boundaries:** test immediately below, at, and above EEP 202 and 454. EEP 202 is MS; EEP 454 is already post-MS. No averaged or fractional categorical phase is allowed.
4. **Lifetime ordering:** at solar composition, reachable-track lifetimes obey `t_TAMS(0.5) > t_TAMS(1) > t_TAMS(2) > t_TAMS(5)`. Compare against the table above with source-file precision, not a loose power-law tolerance.
5. **Observable closure:** `L`, `R`, and `Teff` satisfy the Stefan-Boltzmann identity to the rounding precision of the MIST columns. `current_mass <= initial_mass` within source precision.
6. **Chemistry:** exact grid nodes reproduce source rows. Interpolation at midpoint compositions lies between bracketing continuous values only where the chosen EEP branch is monotonic. Alpha-enhanced inputs always carry `AlphaProjectedToSolarScaled` under MIST v1.2.
7. **Grid convergence:** halve the mass and metallicity cell widths for representative PMS, middle-MS, TAMS, RGB, and AGB cases. Record the change in age, `log_L`, `log_Teff`, and `log_R`; this measured change, not an invented global percent, sets the v1 interpolation error budget.
8. **Cross-grid systematics:** compare selected `0.8, 1, 2, 5 M_sun` cases at at least two metallicities with PARSEC, whose tracks use different input physics and solar calibration ([Bressan et al. 2012](https://doi.org/10.1111/j.1365-2966.2012.21948.x); [official PARSEC/CMD service](https://stev.oapd.inaf.it/cgi-bin/cmd)). Differences are systematic-model envelopes, not failures that should be tuned away.
9. **White dwarfs:** require non-negative cooling age. If the empirical fallback is selected, enforce Cummings' progenitor domain; if Montréal is selected, enforce its `0.2–1.3 M_sun` cooling-grid domain. Reproduce every selected IFMR and cooling-grid node before interpolation tests. A track-derived MIST handoff does not also apply the empirical IFMR.
10. **Core collapse:** a test must demonstrate that no simple monotonic `M_init` threshold invariant is asserted. Until the named Fryer engine is complete, all post-carbon-depletion massive tracks return `UnsupportedCoreCollapse`.
11. **Determinism:** evolution is a deterministic transform of birth properties and model version. It consumes no random stream unless a future explicitly stochastic remnant or uncertain-physics model is selected.

## Hard validity limits and upgrade path

- MIST begins at `0.1 M_sun`, while the birth sampler begins at `0.08 M_sun`. The interval `0.08 <= M < 0.1` must return `OutsideTrackGrid`, use a separately cited very-low-mass grid, or be excluded from the evolved population. Silent extrapolation is forbidden.
- Solar-scaled MIST v1.2 cannot independently consume `[alpha/Fe]`. Using alpha-corrected `[M/H]` is a documented projection whose limits are already described in the chemistry note.
- Ages below `10^5 yr` lie outside the published isochrone-age range, though individual track files begin earlier. If the simulator promises uniform grid support, return unsupported below the actually bundled track minimum.
- Rotation can materially affect massive-star lifetimes and endpoints. V1 fixes `v/vcrit=0`; do not randomly mix rotating and non-rotating grids.
- Post-AGB, TP-AGB, massive-star winds, core collapse, and remnant cooling carry much larger model uncertainty than a smooth middle-main-sequence interpolation.
- Independent evolution of binary members becomes invalid after interaction. Until orbits and interaction criteria exist, every multiple-system result must retain `BinaryInteractionIgnored`.

After the bundled prototype below, the next upgrade should add convergence tests and the luminous post-main-sequence EEPs, followed by one explicitly named WD cooling backend. Prefer a coherent MIST-v2.5 + MIST-III upgrade; Montréal remains a separately versioned alternative. Only after that should the named Fryer delayed core-collapse prescription activate the `NeutronStar` and `BlackHole` outputs.

## Bundled prototype subset

The implementation now bundles the available PMS-through-terminal portions of official MIST v1.2 non-rotating, solar-scaled tracks in `config/stellar_evolution.ron`. It contains the composition coordinates `[M/H] = -2.0, -1.0, 0.0, +0.5` and the initial-mass nodes `0.1, 0.2, 0.5, 0.8, 1.0, 1.5, 2, 3, 5, 8, 15, 25, 40, 100 M_sun`. Every tenth EEP, every primary boundary, every raw phase transition, and all phase-6 rows are retained. Source branch, ordered primary EEPs, current mass, and C/O-core mass are persisted. The reduction is reproducible with `tools/reduce_mist_tracks.rs` when supplied with the official `.track.eep` directories.

The evaluator intersects the available EEPs of the bracketing mass/composition tracks, interpolates only those physically corresponding points, then locates chronological age within the resulting common track. Age is interpolated logarithmically across the source grid; current mass, core mass, and logarithmic MIST observables are interpolated linearly. Track branches are never averaged. This is the explicitly coarse prototype path described above, not yet the full Dotter isochrone algorithm.

The bundled states now cover PMS, main sequence, subgiant/RGB, helium ignition, core-helium burning, early AGB, TP-AGB, post-AGB, Wolf-Rayet, advanced-burning track endpoints, and the white-dwarf handoff. White dwarfs expose track-derived remnant mass and cooling age but deliberately return no luminosity, radius, or temperature until a named cooling backend is added. Distinct errors cover incomplete post-AGB tracks, unsupported core collapse, and tracks ending before a terminal handoff. Initial masses below `0.1 M_sun`, chemistry outside the four composition nodes, and ages before the first retained point remain unsupported and are never clamped. Alpha-enhanced input uses the existing alpha-corrected `[M/H]` projection and carries `AlphaProjectedToSolarScaled`.
