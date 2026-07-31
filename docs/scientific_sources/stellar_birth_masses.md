# Scientific basis for stellar birth masses and companion pairing

This note defines a reproducible version-1 birth-mass sampler over `0.08–100 M_sun`. It separates three quantities that are often accidentally conflated:

1. the **individual-star initial mass function** (IMF),
2. the number of companions attached to a system primary, and
3. the companion mass-ratio distribution conditional on that primary.

The recommended model is suitable for a procedural prototype. It is not a self-consistent reconstruction of the Galactic system-primary mass function, and it does not yet model orbital periods or pristine cluster multiplicity.

## Canonical Kroupa individual-star IMF

Kroupa defines

\[
\xi(m)\,dm = dN, \qquad \xi(m) \propto m^{-\alpha},
\]

so the IMF is a number density per **linear mass interval**, `dN/dm`, not per logarithmic interval and not a mass-weighted distribution. The canonical stellar segments are `alpha = 1.3 +/- 0.5` over `0.08 <= m/M_sun < 0.50` and `alpha = 2.3 +/- 0.3` above `0.50 M_sun`. The additional `alpha = 0.3` segment lies below the hydrogen-burning boundary and is outside this project's stellar-only v1. Kroupa's steeper alternative slopes after unresolved-binary corrections are discussed as a revised possibility, not the canonical law selected here ([Kroupa 2001](https://doi.org/10.1046/j.1365-8711.2001.04022.x); [open manuscript](https://arxiv.org/abs/astro-ph/0009005)).

Use this exact engineering configuration:

```text
model = canonical_kroupa_individual_star_v1
minimum_mass_msun = 0.08
maximum_mass_msun = 100.0

segments = [
  { min = 0.08, max = 0.50, alpha = 1.3, relative_amplitude = 1.0 },
  { min = 0.50, max = 100., alpha = 2.3, relative_amplitude = 0.5 },
]
```

The upper bound `100 M_sun` is a simulator truncation, not a canonical Kroupa break or a claim about the physical maximum stellar mass. The high-segment amplitude is `0.5`, because continuity at the break requires

\[
k_2 = k_1\,0.5^{\alpha_2-\alpha_1}=0.5k_1.
\]

For a power-law segment `[a,b]` with exponent `alpha != 1`, its unnormalized number weight is

\[
W = k\frac{b^{1-\alpha}-a^{1-\alpha}}{1-\alpha}.
\]

Select a segment in proportion to `W`, not by its width and not with equal probability. Conditional inverse-transform sampling is

\[
m=\left[a^{1-\alpha}+u\left(b^{1-\alpha}-a^{1-\alpha}\right)\right]^{1/(1-\alpha)},
\qquad u\sim U[0,1).
\]

For the exact v1 truncation, the expected number probabilities are:

| Mass interval (`M_sun`) | Number probability |
|---:|---:|
| `0.08–0.50` | `0.7607070903` |
| `0.50–100` | `0.2392929097` |

The corresponding mean birth mass is `0.5738648030 M_sun`; `P(m >= 1) = 0.0970379993`, `P(m >= 8) = 0.0062721786`, and `P(m >= 16) = 0.0024021757`. These values are analytic consequences of the configured truncation and are useful test oracles, not new observations.

## Primary mass is not the same distribution as component mass

Kroupa's canonical law above describes individual stars after correcting for unresolved systems. Drawing every **primary** from it and then adding companions necessarily changes the aggregate distribution of all components: it adds extra low-mass stars without removing any initial draw. The output therefore will not, in general, reproduce the canonical individual-star IMF.

For v1, use the Kroupa draw as an explicitly named `primary_mass_proxy`, then draw companions conditionally. This is acceptable for exercising the architecture, but plots must distinguish:

```text
configured primary proxy IMF
generated primary-mass distribution
generated all-component mass distribution
```

A scientifically closed later model must numerically calibrate a system-primary distribution such that primaries plus conditional companions recover the target individual-star IMF. Do not describe a Kroupa-primary-plus-companions sample as “a Kroupa population” until that closure test passes.

## Mass-dependent companion counts

Multiplicity fraction `MF` is the fraction of systems with at least one companion. Companion frequency `CF` is the mean number of companions per primary/system. `CF` may exceed one and must never be used as a Bernoulli probability.

Winters et al.'s volume-limited 25-pc survey of 1120 M-dwarf primaries reports corrected aggregate `MF = 0.268 +/- 0.014` and `CF = 0.324 +/- 0.014`. Its uncorrected primary-mass bins give `MF = 0.160`, `0.214`, and `0.282` over `0.075–0.15`, `0.15–0.30`, and `0.30–0.60 M_sun`; the authors warn that undetected companions particularly affect the lowest bin ([Winters et al. 2019](https://doi.org/10.3847/1538-3881/ab05dc); [open manuscript](https://arxiv.org/abs/1901.06364)).

For higher primary masses, Moe & Di Stefano construct a selection-corrected joint model over mass ratio and orbital period. For companions with `q > 0.1` and approximately `0.2 < log10(P/days) < 8`, their Table 13 gives `CF = 0.50, 0.84, 1.3, 1.6, 2.1` for solar, A/late-B, mid-B, early-B, and O-type primaries. They also give the corresponding single, binary, and higher-order fractions ([Moe & Di Stefano 2017](https://doi.org/10.3847/1538-4365/aa6fb6); [open manuscript](https://arxiv.org/abs/1606.05347)).

Use the following exact categorical v1 table. The M-dwarf rows preserve the Winters mass-bin `MF` values while borrowing the corrected aggregate M-dwarf high-order proportions. The higher-mass rows split Moe & Di Stefano's published `triple + quadruple` category into triples and quadruples so that the categorical distribution reproduces their published `CF` exactly.

| Primary mass (`M_sun`) | Single | Binary | Triple | Quadruple | Implied `MF` | Implied `CF` | Status |
|---:|---:|---:|---:|---:|---:|---:|---|
| `0.08–0.15` | `0.840000000` | `0.128955224` | `0.028656716` | `0.002388060` | `0.160` | `0.193432836` | engineered from Winters |
| `0.15–0.30` | `0.786000000` | `0.172477612` | `0.038328358` | `0.003194030` | `0.214` | `0.258716418` | engineered from Winters |
| `0.30–0.80` | `0.718000000` | `0.227283582` | `0.050507463` | `0.004208955` | `0.282` | `0.340925373` | Winters anchor ends at `0.60` |
| `0.80–2.00` | `0.60` | `0.30` | `0.10` | `0.00` | `0.40` | `0.50` | solar anchor is `0.8–1.2` |
| `2.00–5.00` | `0.41` | `0.37` | `0.19` | `0.03` | `0.59` | `0.84` | selection-corrected synthesis |
| `5.00–9.00` | `0.24` | `0.36` | `0.26` | `0.14` | `0.76` | `1.30` | selection-corrected synthesis |
| `9.00–16.0` | `0.16` | `0.32` | `0.28` | `0.24` | `0.84` | `1.60` | selection-corrected synthesis |
| `16.0–100` | `0.06` | `0.21` | `0.30` | `0.43` | `0.94` | `2.10` | selection-corrected synthesis |

The split of a published higher-order aggregate is not unique. It is an engineering device for a four-component cap, not a measured triple/quadruple breakdown. Independent primary surveys support the scale and strong mass trend: a nearby F6–K3 sample finds observed single/double/triple/higher fractions of `56/33/8/3%` ([Raghavan et al. 2010](https://doi.org/10.1088/0067-0049/190/1/1); [open manuscript](https://arxiv.org/abs/1007.0414)); the VAST A-star survey finds a lower-limit `56.4/32.1/9.0/1.9/0.6%` for orders one through five in its best-covered subset and estimates `CF = 0.689 +/- 0.070` after adding spectroscopic companions ([De Rosa et al. 2014](https://doi.org/10.1093/mnras/stt1932); [open manuscript](https://arxiv.org/abs/1311.7141)). Sana et al. infer an intrinsic close-binary fraction `0.69 +/- 0.09` for Galactic O stars, demonstrating that even close companions alone are common ([Sana et al. 2012](https://doi.org/10.1126/science.1223344); [open manuscript](https://arxiv.org/abs/1207.6397)).

## Companion masses: primary-constrained pairing

Use

```text
draw primary M1
draw component count conditional on M1
for each direct companion:
    draw q conditional on M1
    set M2 = q * M1
```

Do not draw `M1` and `M2` independently from the IMF and sort them afterward. A systematic comparison of pairing algorithms finds no observational or theoretical support for independent random pairing ([Kouwenhoven et al. 2009](https://doi.org/10.1051/0004-6361:200810234); [open manuscript](https://arxiv.org/abs/0811.2859)). A direct comparison with field M-, G-, and intermediate-mass samples rejects IMF random pairing and finds more near-equal-mass systems than it predicts ([Reggiani & Meyer 2011](https://doi.org/10.1088/0004-637X/738/1/60); [open manuscript](https://arxiv.org/abs/1106.3064)).

Moe & Di Stefano show why a fully mass-dependent `q` model cannot be cleanly separated from orbital period: at fixed primary mass the slopes, low-`q` behavior, and twin excess all change with period. De Rosa et al. likewise find an approximately flat A-star `q` distribution inside about 125 au but a distribution weighted toward smaller `q` at wider separation. Sana et al.'s close O-star sample finds an approximately flat intrinsic distribution, `f(q) proportional to q^(-0.1 +/- 0.6)`, over its measured range. A single mass-only exponent would mix incompatible orbital selections.

Until periods are part of the generated domain, use this deliberately simple marginal proxy:

```text
mass_ratio_model = power_law
gamma = +0.25
q_max = 1.0
q_min = max(0.10, 0.08 / primary_mass_msun)
```

Here `p(q) proportional to q^gamma`. The exponent comes from Reggiani & Meyer's maximum-likelihood re-fit of combined field samples, `gamma = 0.25 +/- 0.29` ([Reggiani & Meyer 2013](https://doi.org/10.1051/0004-6361/201321631); [open manuscript](https://arxiv.org/abs/1304.3459)). It is a period-marginal engineering proxy, not evidence that `q` is universal.

The `q >= 0.1` floor keeps this companion sampler semantically aligned with the Moe & Di Stefano frequencies. For `M1 < 0.8 M_sun`, the stellar-mass floor `0.08/M1` is already the larger bound, so the rule includes every possible stellar companion represented by the Winters counts. Above `0.8 M_sun`, it intentionally omits stellar companions with `q < 0.1`; the high-mass `CF` values make no claim about that poorly constrained population.

For a power law on `[q_min,q_max]`, use the same inverse transform as for an IMF segment. Reject configuration where `q_min > q_max`. The exact lower-bound primary `M1 = 0.08 M_sun` admits only an equal-mass stellar companion; this is a boundary case of measure zero for continuous primary sampling but requires an explicit numerical policy.

## Validation expectations

1. **Analytic IMF tests:** segment probabilities, mean mass, and tail probabilities must match the values above within numerical tolerance. A log-log density histogram must recover slopes `-1.3` and `-2.3`; a histogram in `log m` instead recovers slopes `1-alpha`, so tests must name their measure.
2. **Continuity:** estimated `dN/dm` immediately below and above `0.5 M_sun` must agree. Equal segment amplitudes are a regression failure.
3. **Bounds and finiteness:** every primary and companion must be finite and in `0.08–100 M_sun`; every companion must satisfy `M2 <= M1` and configured `q` support.
4. **Categorical closure:** for each primary bin, a large sample must recover every table probability, `MF`, and `CF`. In particular, `CF > 1` for massive stars must not be clipped to one.
5. **Conditional pairing:** in every generated system the recorded `q` must equal `M2/M1`. Companion draws must never promote a companion above the primary.
6. **Aggregate-IMF diagnostic:** always compare primary-only and all-component histograms with the canonical target. In v1 this is an expected diagnostic mismatch, not a reason to tune unrelated IMF slopes.
7. **Sensitivity:** repeat `q` plots with `gamma = -0.5`, `+0.25`, and `+0.5`; repeat multiplicity with published uncertainties. These are sensitivity cases, not posterior draws per star.
8. **Determinism:** domain-separated seeds for primary mass, component count, and each companion mass must make results reproducible and prevent a new companion draw from perturbing already generated properties.

## Explicit limitations and upgrade path

- The multiplicity anchors describe present-day field or main-sequence samples. They are not direct measurements of pristine birth multiplicity; wide systems can be disrupted and close systems can evolve.
- The bins extrapolate Winters from `0.60` to `0.80 M_sun` and the Moe & Di Stefano solar result from `1.2` to `2.0 M_sun`. Keep the source interval and configured interval as separate metadata.
- The four-component cap cannot represent higher-order massive systems exactly, and the engineered triple/quadruple split is non-unique.
- The simple `q` law omits its measured covariance with orbital period, separation, twin excess, hierarchy, eccentricity, age, environment, and metallicity.
- Each companion is treated as directly associated with the primary. Real triples and quadruples require a stable orbital hierarchy; drawing several independent `q` values is only a mass inventory.
- The sampler produces initial stellar masses. It must not overwrite them with present-day masses after winds, mass transfer, mergers, supernovae, or remnant formation.
- The rare massive tail makes small regions extremely noisy. Distribution tests need millions of draws or analytic integration; a single 10-pc realization is not a meaningful IMF fit.

The next scientific upgrade should add orbital period first and implement Moe & Di Stefano's joint `p(q, P, multiplicity | M1)` model. After that, calibrate the primary-mass proposal until the generated all-component birth population closes against the chosen individual-star IMF and total number/mass normalization.
