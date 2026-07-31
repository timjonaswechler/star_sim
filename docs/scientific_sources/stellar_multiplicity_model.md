# Scientific basis for the prototype stellar-multiplicity model

> Historical v1 note: the active population-lab path now samples primary mass first and uses mass-conditioned multiplicity from `stellar_birth_masses.ron`. This fixed nearby-M-dwarf model remains preserved for comparison. See `stellar_birth_masses.md` for the replacement and its limitations.

This note defines a defensible **first-pass conversion from stellar-object number density to stellar-system number density**. It is deliberately a local-field approximation for the prototype, not a universal law of stellar multiplicity.

## Recommendation for version 1

Use the corrected M-dwarf statistics from the volume-limited 25 pc survey by [Winters et al. (2019)](https://doi.org/10.3847/1538-3881/ab05dc):

```text
multiplicity fraction MF = 0.268 +/- 0.014
companion frequency   CF = 0.324 +/- 0.014 companions per primary/system
mean stellar components per system = 1 + CF = 1.324
```

The study searched 1120 M-dwarf primaries with trigonometric distances, combined several companion-detection methods, and corrected the incomplete close-companion census using the much better studied 10 pc subsample. Its correction raises the observed 265 multiple systems and 310 stellar companions to estimates of 300 multiples and 363 companions. Systems having only a brown-dwarf companion were counted as stellar singles, which matches the prototype density's exclusion of brown dwarfs. See the authors' [published paper](https://www.astro.gsu.edu/RECONS/published45.pdf), especially Sections 5.1–5.2.

For drawing a categorical multiplicity before masses exist, use this engineered distribution:

| Stellar components in system | Probability |
|---:|---:|
| 1 (single) | 0.732 |
| 2 (binary) | 0.216 |
| 3 (triple) | 0.048 |
| 4 (quadruple) | 0.004 |

These probabilities are **not a directly published corrected breakdown**. They are a v1 reconstruction that exactly reproduces the two corrected aggregate constraints:

```text
MF = 0.216 + 0.048 + 0.004 = 0.268
CF = 1*0.216 + 2*0.048 + 3*0.004 = 0.324
```

The split of the high-order tail is anchored approximately to Winters et al.'s observed, uncorrected counts of 856 singles, 223 doubles, 37 triples, and 3 higher-order systems. Version 1 caps the tail at four stellar components; this is a sampling simplification, not a claim that higher orders do not exist.

## Definitions and the exact density conversion

Let `N_k` be the number of systems containing exactly `k` stellar components and let

```text
N_system = sum(k >= 1) N_k
```

Then

```text
MF = sum(k >= 2) N_k / N_system

CF = sum(k >= 2) (k - 1) * N_k / N_system

mean_stars_per_system
   = sum(k >= 1) k * N_k / N_system
   = 1 + CF
```

Winters et al. use the names *multiplicity rate* and *companion rate* for the same two quantities. The essential distinction is that a triple contributes once to `MF` but twice to `CF`. Consequently, `MF` alone is insufficient for converting object density to system density unless every multiple is assumed to be binary.

If `n_star` counts individual stellar components over the same population and companion limits as the multiplicity model, the exact conversion is

```text
n_system = n_star / (1 + CF)
         = n_star / 1.324
         = 0.75529 * n_star                 [v1]
```

Propagating only the reported statistical uncertainty in `CF` gives

```text
sigma(1 / (1 + CF)) = sigma_CF / (1 + CF)^2
                     = 0.00799

n_system / n_star = 0.755 +/- 0.008          [statistical only]
```

For the current CNS5 normalization `n_star = 0.0799 stars pc^-3`, the numerical v1 result is

```text
n_system = 0.06034 systems pc^-3
         = 0.0603 +/- 0.0010 systems pc^-3   [reported statistical errors combined]
```

The second uncertainty combines the CNS5 `n_star = 0.0799 +/- 0.0011 stars pc^-3` uncertainty and the Winters et al. `CF` uncertainty in quadrature, assuming independence. It excludes catalogue incompleteness, unresolved companions, the mismatched target populations, and Galactic-population variation.

At any other Galactic position the same v1 factor may multiply the local stellar-object density. This assumes multiplicity does not change with population, age, metallicity, or environment; that is a temporary software assumption and its systematic uncertainty is not contained in `+/- 0.008`.

## Why MF is not “the probability that a random star is binary”

The denominator of `MF` is systems (or target primaries), not individual stars. Applying `MF` independently to every object in a star-count field would select companions again as new primaries and double-count systems. For the recommended distribution, 26.8% of **systems** are multiple, while the fraction of individual stellar components residing in multiple systems is

```text
(2*0.216 + 3*0.048 + 4*0.004) / 1.324 = 0.447
```

Thus about 44.7% of component stars in this particular v1 realization belong to multiple systems even though `MF` is only 26.8%.

## Observational support and mass dependence

The v1 choice is useful because nearby stellar number counts are dominated by low-mass stars, but multiplicity is strongly dependent on primary mass:

- Within the M-dwarf regime, Winters et al. measure uncorrected multiplicity rates of `28.2 +/- 2.1%`, `21.4 +/- 2.0%`, and `16.0 +/- 2.5%` for primary-mass intervals `0.30–0.60`, `0.15–0.30`, and `0.075–0.15 M_sun`, respectively. The authors caution that undetected companions especially affect the lowest bin ([Winters et al. 2019](https://doi.org/10.3847/1538-3881/ab05dc), Section 6.1.3).
- As an independent nearby-M-dwarf check, the volume-limited 15 pc POKEMON sample reports an erratum-corrected `MF = 23.3 +/- 2.0%` and `CF = 28.6 +/- 2.1%` for 455 M0–M9 primaries ([Clark et al. 2024](https://doi.org/10.3847/1538-3881/ad81fa)). This supports the overall scale but also shows that the Winters statistical error alone does not represent cross-survey systematic uncertainty.
- A volume-limited survey of 454 nearby F6–K3 solar-type targets finds observed single/double/triple/higher-order fractions of `56 +/- 2%`, `33 +/- 2%`, `8 +/- 1%`, and `3 +/- 1%`; its completeness analysis estimates an intrinsic multiplicity fraction of `46 +/- 2%` ([Raghavan et al. 2010](https://doi.org/10.1088/0067-0049/190/1/1)).
- An independent, much larger distance-limited sample of 4847 F/G dwarfs finds `MF = 0.46`, a high-order fraction of `0.13 +/- 0.01`, and approximate component-count fractions `54:33:8:4:1` for orders 1, 2, 3, 4, and 5 ([Tokovinin 2014a](https://doi.org/10.1088/0004-6256/147/4/86); [Tokovinin 2014b](https://doi.org/10.1088/0004-6256/147/4/87)).
- The quantitative synthesis by [Moe & Di Stefano (2017)](https://doi.org/10.3847/1538-4365/aa6fb6) finds the mean number of stellar companions with `q > 0.1` and `log10(P/days) < 8` rising from `0.50 +/- 0.04` for solar-type main-sequence primaries to `2.1 +/- 0.3` for O-type primaries. It also shows that mass ratio, period, eccentricity, and multiplicity cannot be treated as mutually independent distributions.

[Duchêne & Kraus (2013)](https://doi.org/10.1146/annurev-astro-081710-102602) is a useful review of the primary literature and reaches the same broad conclusion: field-star multiplicity and the width of the orbital-period distribution increase steeply with primary mass. The original surveys above should remain the numerical sources for the implementation.

## What cannot yet be inferred without primary masses

A scientifically stronger conversion requires a system-primary present-day mass function `Psi(M1)` and a mass-dependent companion frequency `CF(M1)`:

```text
n_system = integral Psi(M1) dM1

n_star = integral Psi(M1) * (1 + CF(M1)) dM1
```

The object-level present-day mass function is not interchangeable with `Psi(M1)`: it already contains secondaries. Drawing every primary from the object mass function and then adding companions would overproduce low-mass stars and would no longer reproduce the input CNS5 number density or mass function. The later model must infer or construct a primary/system mass function such that primaries plus sampled companions recover the target object-level present-day mass function.

Therefore, before masses are sampled, the model cannot defensibly provide:

- a different multiplicity law for thin disc, thick disc, and halo;
- the correct contribution of rare F/G/A/B/O systems at a selected position;
- companion masses or mass ratios;
- orbital-period, separation, and eccentricity distributions;
- remnant-containing systems, because the CNS5 stellar density includes white dwarfs whereas the Winters M-dwarf target sample excluded systems with a white-dwarf component;
- a universal Galactic conversion factor.

CNS5 contains 5230 stars within 25 pc—4946 main-sequence stars, 20 red giants, and 264 white dwarfs—and finds that about 72% of its stars are M dwarfs. It reports catalogue component/system metadata, but it is a nearby-object catalogue rather than a companion-completeness-corrected multiplicity survey ([Golovin et al. 2023](https://doi.org/10.1051/0004-6361/202244250)). Its raw system labels should not be used as an exact global multiplicity correction. The 72% M-dwarf dominance makes an M-dwarf baseline reasonable for a prototype, but does not remove the primary/component selection mismatch.

## Uncertainty policy for the prototype

Store the observed aggregate inputs (`MF`, `CF`) separately from the derived categorical probabilities. For plots and tests:

1. Use `CF = 0.324` and the `73.2:21.6:4.8:0.4` distribution as the named `nearby_m_dwarf_v1` baseline.
2. Propagate `CF +/- 0.014` when showing statistical uncertainty in expected system counts.
3. Treat the lack of primary-mass conditioning as a larger, separate systematic. A useful sensitivity run is `CF = 0.28–0.50`, spanning approximately the uncorrected M-dwarf result and the solar-type result; this is an engineering envelope, not a confidence interval.
4. Label all generated counts as expected local-field counts. Do not apply this prescription to young clusters, star-forming regions, the Galactic bulge, or massive-star-selected populations.

## Upgrade path

Version 2 should sample a system primary mass first, then draw multiplicity from a primary-mass-dependent model and draw companion mass ratios and orbital hierarchy conditionally. The generated component population must be validated back against the target present-day mass function and total stellar-object density. Only after that closure test should the single global `0.75529` conversion be retired.
