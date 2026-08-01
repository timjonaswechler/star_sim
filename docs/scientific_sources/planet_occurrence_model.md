# Scientific basis for a first planet-occurrence block

This note defines a deliberately narrow, reproducible version-1 occurrence model. It generates a **planet-population summary** for an eligible stellar member: counts in a few observed planet-radius/period domains and, separately, whether at least one Doppler-detectable giant planet is present. It does not generate orbital elements, dynamically stable architectures, planet masses, compositions, or habitability.

Occurrence is always tied to an explicit observational domain. A statement such as “this star has a 30% planet probability” is not meaningful without planet size or mass, period or semimajor-axis limits, host selection, and survey completeness. In particular, the average number of planets per star may exceed one and is not a Bernoulli probability.

The recommended v1 combines three empirical calibrations rather than pretending that one survey uniformly covers every planet and host type:

1. Kepler warm small planets around FGK dwarfs from the California-Kepler Survey (CKS);
2. Kepler small planets around early M dwarfs from an injection/recovery analysis of the full four-year data set;
3. California Planet Survey giant-planet host fractions as a function of stellar mass and `[Fe/H]`.

All relations below use stellar iron abundance `[Fe/H]`, not alpha-corrected global `[M/H]`. The chemistry module must preserve the birth `[Fe/H]` value separately so this model does not silently substitute one for the other.

## Exact v1 output and semantics

Keep a population summary separate from any later concrete orbital hierarchy:

```text
PlanetOccurrenceInput {
    host_initial_mass_msun,
    host_current_mass_msun,
    host_effective_temperature_k,
    host_surface_gravity_log10_cgs,
    host_age_gyr,
    host_evolutionary_state,
    host_birth_fe_h,
    multiplicity_environment:
        Single | KnownWide | KnownClose { semimajor_axis_au } | SeparationUnknown,
}

PlanetPopulationSummary {
    model_id: "empirical_occurrence_v1",
    small_planets: Result<
        FgkWarm {
            warm_super_earth_count,    # 1.0-1.7 R_earth, 10-100 d
            warm_sub_neptune_count,    # 1.7-4.0 R_earth, 10-100 d
        }
      | MDwarfAggregate {
            small_planet_count,        # 1.0-4.0 R_earth, P < 200 d
        },
        SmallPlanetCoverageError,
    >,
    giant_planets: Result<
        HasAtLeastOneCpsGiant(bool),    # K > 20 m/s, a <= about 2.5 AU
        GiantPlanetCoverageError,
    >,
    quality_flags,
}
```

Only one small-planet host calibration owns a star. The M-dwarf count is an aggregate bin and must not also be populated through the two CKS FGK bins. The giant result is a separate Bernoulli channel and can coexist with either small-planet channel. Keep per-channel coverage results: a cool K dwarf outside both small-planet source selections can still have a calibrated CPS giant result.

The output describes the empirical domain only. It must not contain invented planet radius, mass, period, eccentricity, inclination, or semimajor axis. Those require a separately sourced distribution and, for multi-planet systems, a model for correlated architecture and stability.

## FGK dwarf small-planet channel

Petigura et al. selected Kepler stars with approximately `4700 <= Teff <= 6500 K` and `3.9 <= log(g) <= 5.0`, and measured occurrence after correcting for transit geometry and detection completeness. Their named warm domains are:

| Class | Radius | Period | Recommended solar-metallicity mean |
|---|---:|---:|---:|
| Warm super-Earth | `1.0-1.7 R_earth` | `10-100 d` | `lambda = 0.20 planets/star` |
| Warm sub-Neptune | `1.7-4.0 R_earth` | `10-100 d` | `lambda = 0.2828427 planets/star` |

The paper reports about 20 warm super-Earths per 100 stars across `-0.4 <= [Fe/H] <= +0.4`; its fitted metallicity exponent is `beta = -0.3 +/- 0.2`, which is weak enough that v1 should use **no metallicity scaling** for that class instead of turning a marginal fitted slope into a strong simulation effect.

For warm sub-Neptunes, the reported occurrence rises from about 20 to 40 planets per 100 stars between `[Fe/H] = -0.4` and `+0.4`. A compact v1 interpolation is

```text
lambda_warm_super_earth = 0.20

beta_warm_sub_neptune = log10(0.40 / 0.20) / 0.8
                         = 0.3762874946
lambda_warm_sub_neptune = sqrt(0.20 * 0.40)
                         * 10^(beta_warm_sub_neptune * [Fe/H])
```

This sub-Neptune equation is an **engineering log-linear interpolation between the two published endpoints**, not a fit quoted by the paper. It gives `lambda = 0.2828427` at solar metallicity and must only be evaluated over `-0.4 <= [Fe/H] <= +0.4`. Do not clamp or extrapolate outside that range; return a typed coverage result.

Draw the count in each bin as `Poisson(lambda)`. The Poisson choice converts an observed mean number per star into a minimal count sampler, but it is not an observational claim that planet counts are independent. Kepler multi-planet systems show that planets are clustered by host; a later architecture model must replace these independent draws.

Source: [Petigura et al. (2018), CKS IV](https://doi.org/10.3847/1538-3881/aaa54c); [open manuscript](https://arxiv.org/abs/1712.04042).

## Early-M-dwarf small-planet channel

Dressing & Charbonneau searched the full four-year Kepler data set with their own pipeline and injected 2,000 trial transits into each target light curve to measure completeness. For 2,543 small target stars they inferred

```text
lambda_m_dwarf_small = 2.5 planets/star
```

for `1-4 R_earth` planets with periods shorter than `200 d`, with a quoted uncertainty of `+/- 0.2 planets/star`. The source sample began with `Teff < 4000 K` and `log(g) > 3`; after its quality cuts the 2,543 retained stars span `2661-3999 K`. The exact implementable v1 eligibility is therefore `2661 <= Teff <= 3999 K`, `log(g) > 3`, and `MainSequence`. Store these cuts in configuration rather than equating the word “M dwarf” with an arbitrary mass boundary. They reproduce the source domain more directly than a spectral-type label, though they do not reproduce all of the survey's light-curve quality selections.

Do not apply the FGK metallicity scaling to this M-dwarf rate. This study did not calibrate such a dependence, and M-dwarf abundance scales have additional systematics. The aggregate `1-4 R_earth` count also cannot be subdivided into super-Earths and sub-Neptunes without importing the paper's full radius-period occurrence table.

Source: [Dressing & Charbonneau (2015)](https://doi.org/10.1088/0004-637X/807/1/45); [open manuscript](https://arxiv.org/abs/1501.01623).

## Giant-planet channel: host mass and metallicity

Johnson et al. analyzed 1,194 California Planet Survey stars spanning `0.2 < M_star/M_sun < about 2.0` and `-1.0 < [Fe/H] < +0.55`. Their selected planets have Doppler semi-amplitude `K > 20 m/s` and semimajor axis `a <= about 2.5 AU`. The fitted fraction of stars with at least one such giant planet is

```text
p_giant = 0.07
          * (host_current_mass_msun)^1.0
          * 10^(1.2 * host_birth_fe_h)
```

with marginalized parameter estimates

```text
C     = 0.07  (68.2% interval 0.06-0.08)
alpha = 1.0   (0.70-1.30)
beta  = 1.2   (1.0-1.4)
```

This is a Bernoulli probability for **one or more detectable giants**, not a mean giant count. V1 draws `has_cps_giant_planet ~ Bernoulli(p_giant)`. Evaluate it only inside the measured stellar mass and `[Fe/H]` rectangle and only when `0 <= p_giant <= 1`; outside it, return `OutsideGiantOccurrenceCalibration` rather than clipping to one or extrapolating. On the main sequence, `current_mass_msun` is the quantity closest to the survey's inferred present stellar mass. Preserve initial mass as provenance but do not silently use it in this empirical equation.

The relation includes M dwarfs, FGK dwarfs, and intermediate-mass subgiants in its source sample, but it does not identify planet radii, exact masses, periods, or multiplicity. A true giant-planet population generator will need a conditional mass-period distribution after this occurrence gate.

Source: [Johnson et al. (2010)](https://doi.org/10.1086/655775); [open manuscript](https://arxiv.org/abs/1005.3084).

## Multiplicity correction

Planet occurrence cannot be conditioned scientifically on companion count alone. The relevant observed dependence is strongly separation-dependent. From high-resolution imaging of 382 Kepler Objects of Interest, Kraus et al. fitted a step model in which binaries inside

```text
a_cut = 47 (+59/-23) AU
```

have only

```text
S_close = 0.34 (+0.14/-0.15)
```

times the planet occurrence of wider binaries or single stars. Their planet-host sample shows a `4.6 sigma` deficit of companions at projected separations below `50 AU` and mass ratio `q > 0.4`.

The minimal application is

```text
multiplicity_factor = 1.0   for Single or KnownWide
multiplicity_factor = 0.34  for KnownClose with a < 47 AU
```

Multiply every Poisson mean and the giant-planet Bernoulli probability by this factor. Store `47 AU` and `0.34` together as one named `kraus2016_step` prescription, because their uncertainties are correlated. The factor is a coarse population correction, not a dynamical stability calculation.

For `SeparationUnknown`, return `MultiplicitySeparationRequired`. Do not infer “close” from member count or mass ratio and do not apply `0.34` to every binary. The current stellar multiplicity model therefore needs either a static separation category or an explicit unsupported result before this correction can be used. Sampling that category is a later multiplicity extension; it does not require integrating orbits.

This calibration is for Kepler-like planet systems around solar-type targets and is not demonstrated to be universal across M dwarfs, evolved stars, or the CPS giant-planet domain. Applying it to all three channels is a declared v1 approximation and must set `MultiplicitySuppressionExtrapolated` outside the solar-type dwarf calibration.

Source: [Kraus et al. (2016)](https://doi.org/10.3847/0004-6256/152/1/8); [open manuscript](https://arxiv.org/abs/1604.05744).

## Evolutionary state and age

The default v1 small-planet channels support only `MainSequence` hosts inside the source surveys' `Teff` and `log(g)` cuts. Return typed unsupported results for `PreMainSequence`, all post-main-sequence states, white dwarfs, and compact remnants.

This is not a claim that evolved or remnant stars cannot host planets. It prevents two different questions from being conflated:

1. **formation inventory:** what formed around the star;
2. **present-day survivors:** what remains after stellar radius evolution, tides, engulfment, mass loss, binary interaction, and orbital expansion or instability.

The current stellar-evolution snapshot supplies the host state but no planet-survival calculation. Consequently, copying a main-sequence occurrence rate onto an RGB, AGB, or white-dwarf host would manufacture a present-day population. The Johnson giant relation used subgiants to estimate correlations with stellar mass and metallicity, but it is not a phase-by-phase survival prescription across the project's detailed EEP states.

Within the supported main-sequence sample, v1 has no explicit age factor. The Kepler field-star age distribution is implicit in its measured occurrence rates, and these sources do not establish a universal separable multiplier `f(age)` for all planet classes. Preserve age in the input/provenance and set `HostAgeDependenceNotModeled`; do not invent a monotonic decay law.

## Deterministic sampling and system ownership

Use a seed derived from stable identities, for example

```text
planet_seed = hash(global_seed, system_id, member_id, model_id)
```

not from the member's vector index. Adding an unrelated system must not reshuffle existing planet outcomes. The planet summary belongs to one stellar member, while multiplicity context belongs to the parent stellar system.

Apply the host eligibility and multiplicity checks before consuming random numbers. This makes typed coverage failures deterministic and prevents future supported branches from changing the random stream of existing ones.

Suggested per-channel result shape:

```text
small_planets: Result<SmallPlanetOccurrence, SmallPlanetCoverageError>
giant_planets: Result<GiantPlanetOccurrence, GiantPlanetCoverageError>

Shared coverage variants include:
    UnsupportedEvolutionaryState
  | OutsideHostCalibration
  | OutsideMetallicityCalibration
  | MultiplicitySeparationRequired
  | MissingStellarObservable
```

Quality flags should include `PoissonIndependenceApproximation`, `HostAgeDependenceNotModeled`, `MultiplicitySuppressionExtrapolated`, and `PlanetPropertiesNotGenerated`.

## What v1 may and may not claim

V1 can defensibly answer:

- how many warm `1.0-1.7 R_earth` and `1.7-4.0 R_earth` planets are drawn around an eligible FGK dwarf;
- how many `1-4 R_earth`, `P < 200 d` planets are drawn around an eligible early M dwarf;
- whether an eligible `0.2-2.0 M_sun` host has at least one CPS-domain giant planet;
- whether a known close stellar companion suppresses those rates under the declared step approximation.

V1 cannot claim:

- a complete planetary system, total planet count at all orbital distances, or Solar-System analog rate;
- individual radius, mass, composition, period, semimajor axis, eccentricity, inclination, or resonance;
- dynamical stability, circumbinary versus circumstellar ownership, or which component of an unresolved pair hosts a detected planet;
- planet survival around post-main-sequence stars or compact remnants;
- habitability;
- calibrated occurrence for O/B/A dwarfs, late-K hosts between source selections, very young stars, halo chemistry outside the measured `[Fe/H]` ranges, or alpha-enhanced populations.

The next scientifically coherent extension after this summary is not random orbital elements. It is a sourced radius-period grid with correlated system multiplicity, followed by a separately named architecture/stability module. Post-main-sequence survival should be another independent transformation from birth inventory to present-day inventory.
