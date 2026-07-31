# Scientific basis for prototype stellar ages and metallicities

This note defines a defensible **first-pass** joint age–metallicity model for the geometrical thin disc, geometrical thick disc, and stellar halo. It is deliberately a phenomenological sampler, not a unique fit to the Milky Way. Survey ages, metallicities, and apparent population fractions depend strongly on the tracer, selection function, spatial footprint, age pipeline, and how a population is defined.

## The key semantic distinction

The density model's `thin_disk` and `thick_disk` are two **geometrical exponential components**. Observational archaeology papers often instead divide stars chemically (low/high `[alpha/Fe]`), kinematically, or by age. Those selections overlap but are not interchangeable.

Mackereth et al. show that mono-age populations form a continuum in thickness and that a geometrically thick disc can contain both old, intrinsically thick populations and younger flared populations ([Mackereth et al. 2017](https://doi.org/10.1093/mnras/stx1774)). Miglio et al.'s approximately 11-Gyr, nearly coeval result refers specifically to an **alpha-rich chemical population** in the Kepler field, not every star selected from a thick exponential ([Miglio et al. 2021](https://doi.org/10.1051/0004-6361/202038307)). Therefore the v1 distributions below are intentionally broader than a chemically selected sequence.

The eventual model should represent

```text
p(age, [Fe/H], [alpha/Fe] | R, z, latent formation population)
```

rather than treating a geometric component label as a complete formation history.

## Observational constraints

### Disc ages and their overlap

Using nearly 5,400 Kepler–APOGEE giants, Miglio et al. find their roughly 400 alpha-rich RGB stars compatible with an old population around 11 Gyr; their statistical model places 95% of its formation within about 1.5 Gyr. Plausible stellar-model changes move the inferred mean by about 1 Gyr, and apparently young alpha-rich stars can be products of binary interaction rather than genuinely young stars. The same sample contains old (8–9 Gyr), low-alpha, super-solar-metallicity stars, consistent with radial migration ([Miglio et al. 2021](https://arxiv.org/abs/2004.14806)). This is strong evidence for an old chemical thick disc, but also evidence that age, alpha enhancement, and geometry do not create clean partitions.

GALAH DR3 turnoff stars with Gaia EDR3 astrometry show an inner-disc minimum in the age distribution around 10 Gyr, near the transition from high to low alpha abundance and from hot to cool kinematics. The same work finds substantial radial structure: later inner-disc stars reach super-solar metallicity while outer-disc formation starts at least 0.5 dex lower ([Sahlholdt, Feltzing & Feuillet 2022](https://doi.org/10.1093/mnras/stab3681); [open preprint](https://arxiv.org/abs/2112.08218)).

The approximately 247,000 LAMOST subgiants analysed by Xiang & Rix split into two almost disjoint regions of age–metallicity space around 8 Gyr. Their old sequence enriches from `[Fe/H] < -1` near 13 Gyr to roughly `+0.5` near 7 Gyr, and most stars in that sequence formed near 11 Gyr during the Gaia–Sausage–Enceladus epoch ([Xiang & Rix 2022](https://doi.org/10.1038/s41586-022-04496-5); [open preprint](https://arxiv.org/abs/2203.12110)). That tight old sequence is useful future guidance for a latent chemical population, not a direct probability law for the geometric thick disc.

### Disc metallicity is spatial, not global

APOGEE maps of 69,919 red giants over `3 < R < 15 kpc` and `|z| < 2 kpc` show that the mid-plane metallicity-distribution peak moves outward to lower metallicity. The solar-annulus mid-plane MDF is roughly Gaussian and peaks near solar metallicity, while the inner and outer MDFs have opposite skewness; high above the plane the MDF is broadly peaked around `[Fe/H] ~ -0.4` across radius ([Hayden et al. 2015](https://doi.org/10.1088/0004-637X/808/2/132); [open preprint](https://arxiv.org/abs/1503.02110)). A single Galaxy-wide Gaussian for the thin disc is therefore indefensible.

An APOGEE–Gaia hierarchical age analysis measures a young mid-plane **`[M/H]`** gradient of `-0.059 +/- 0.010 dex/kpc` and finds that the age–metallicity relation changes with both radius and height ([Feuillet et al. 2019](https://doi.org/10.1093/mnras/stz2221); [open preprint](https://arxiv.org/abs/1908.02772)). This is evidence that the next model must be spatial, but `[M/H]` is not `[Fe/H]`, so the number must not silently become an iron-abundance gradient.

Sharma et al. forward-model the K2-HERMES selection and infer a thick-disc mean age near 9–10 Gyr. Their fitted metallicity parameter is `log(Z/Z_sun) = -0.16` with dispersion `0.17`, which is an alpha-aware total-metallicity quantity and **must not be relabelled `[Fe/H]`** ([Sharma et al. 2019](https://doi.org/10.1093/mnras/stz2861); [open preprint](https://arxiv.org/abs/1904.12444)). For the v1 `[Fe/H]` sampler below, the broader high-`|z|` APOGEE MDF is the safer geometric-component guide.

### Iron-specific radial gradient for the thin disc

The first spatial correction must be calibrated to **iron abundance**, not copied from a survey's `[M/H]` gradient. Classical Cepheids are useful here because they are young (roughly 20–400 Myr in the cited sample), remain close to their formation chemistry, and have accurate individual distances. Genovali et al. place 450 Cepheids on homogeneous distance and iron-abundance scales and fit

\[
[\mathrm{Fe/H}] = (0.57 \pm 0.02) - (0.060 \pm 0.002)\,R_\mathrm{GC}/\mathrm{kpc}.
\]

Their adopted solar Galactocentric radius is `7.94 kpc`; the fit samples approximately `5 <= R_GC <= 19 kpc` (individual objects extend to about 4 kpc). At the project's adopted `R_ref = 8.178 kpc`, the quoted Cepheid relation evaluates to about `[Fe/H] = +0.079 dex`. The same paper finds `-0.051 +/- 0.010 dex/kpc` for 44 open clusters younger than 3 Gyr and a shallower `-0.034 +/- 0.009 dex/kpc` for 23 clusters aged 3.6–9 Gyr. The older sample is small and shows larger scatter and outer-disc structure, so this difference is evidence for age/migration effects rather than a sufficiently precise universal age law ([Genovali et al. 2014](https://doi.org/10.1051/0004-6361/201323198)).

A later Gaia-verified compilation of 136 spectroscopic and 14 photometric open clusters finds a robust all-sample slope of `-0.058 +/- 0.004 dex/kpc` and `-0.063 +/- 0.005 dex/kpc` inside 12 kpc. Its overlapping age bins from about 0.25 to 1.32 Gyr give slopes from approximately `-0.054` to `-0.061 dex/kpc`, with no significant age trend within their uncertainties. It reports possible flattening beyond about 13 kpc, but explicitly cautions that this region contains few clusters and a strongly unequal age distribution. This independently supports `-0.060 dex/kpc` as a useful young/thin-disc first-pass slope while warning against treating the outer-disc continuation or age dependence as settled ([Netopil et al. 2022](https://doi.org/10.1093/mnras/stab2961)).

A homogenized compilation of high-resolution measurements for 251 open clusters fits the high-quality subsample of 175 with a **continuous broken line**: inner slope `-0.064 +/- 0.007 dex/kpc`, outer slope `-0.019 +/- 0.008 dex/kpc`, knee `12.1 +/- 1.1 kpc`, and intrinsic scatter `0.091 +/- 0.005 dex`. This is a more useful procedural shape than an unlimited straight line because the outer slope is significantly shallower but not exactly flat. The fit uses present cluster radii, and the authors explicitly warn that old clusters may have migrated away from their birth radii ([Spina, Magrini & Cunha 2022](https://doi.org/10.3390/universe8020087); [open preprint](https://arxiv.org/abs/2202.00463)).

The final Gaia-ESO sample independently covers 62 thin-disc open clusters from 0.1 to about 7 Gyr over `6 <= R_GC <= 21 kpc`. Its single-line fit is `-0.054 +/- 0.004 dex/kpc`; separate fits give `-0.081 +/- 0.008` inside 11.2 kpc and `-0.044 +/- 0.014 dex/kpc` outside. The age bins give `-0.038 +/- 0.004` at 0.1–1 Gyr, `-0.063 +/- 0.006` at 1–3 Gyr, and `-0.084 +/- 0.019 dex/kpc` above 3 Gyr, but the authors caution that abundance analysis of active young stars may bias the youngest bin. This supports radial structure and outer flattening, but not a precise v1 age law ([Magrini et al. 2023](https://doi.org/10.1051/0004-6361/202244957); [open preprint](https://arxiv.org/abs/2210.15525)).

#### Exact engineering v1 rule

The implemented v1 applies a linear radial **offset to the existing broad thin-disc mean**, rather than replacing that mean with the Cepheid intercept:

\[
\mu_\mathrm{thin}(R) = \mu_\mathrm{thin,local}
  + g_\mathrm{thin}\,[\operatorname{clamp}(R, R_\min, R_\max)-R_\mathrm{ref}].
\]

Recommended configurable values:

| Parameter | v1 value | Meaning |
|---|---:|---|
| `reference_radius_pc` | `8178.0` | Radius where the existing `thin_disk.metallicity.mean = -0.10 dex` remains unchanged |
| `dex_per_kpc` | `-0.060` | Young-tracer iron slope from Genovali et al. |
| observational slope uncertainty | `0.002` | Provenance/sensitivity metadata, not per-star random noise |
| `calibration_min_radius_pc` | `5000.0` | Conservative inner edge of the Cepheid calibration |
| `calibration_max_radius_pc` | `19000.0` | Outer edge of the Cepheid calibration |
| `outside_range_policy` | `ClampCorrection` | Hold the correction at the nearest calibrated edge; do not extrapolate to the Galactic centre or indefinitely outward |
| `thick_disk.slope_dex_per_kpc` | `0.0` | Explicit v1 omission |
| `stellar_halo.slope_dex_per_kpc` | `0.0` | Explicit v1 omission |

This makes the v1 thin-disc mean about `+0.091 dex` at 5 kpc, `-0.10 dex` at 8.178 kpc, and `-0.749 dex` at 19 kpc before the existing hard-support truncation is applied. These are consequences of the engineering rule, not three new measurements. The Cepheid fit itself would instead predict `+0.27`, `+0.079`, and `-0.57 dex` at those radii because it describes a young population whose solar-circle zero-point is not the same as the prototype's all-age geometrical thin-disc mean.

Do **not** add the gradient as a second full-Galaxy mean on top of the current Gaussian. The intended operation is a change in its mean, with the configured `0.25 dex` scatter retained. Spina et al.'s smaller `0.091 dex` cluster scatter describes a different selected tracer and should not be added in quadrature or silently replace the broad all-star prior. Do not draw a new slope independently for each star; observed parameter uncertainties describe uncertainty in a global model and belong in sensitivity runs. A continuous broken-line law such as Spina et al.'s is the documented next refinement if the outer-disc tail becomes important; it is deliberately not part of the linear v1.

Keep the slope age-independent in this first implementation. Applying an age-dependent slope to a star's **current** radius would conflate chemical evolution with radial migration. A later chemo-dynamical model should condition birth chemistry on `FormationRadius` and then migrate the star to `CurrentGalacticPosition`; only then should it use separate age-bin gradients and age-dependent scatter. The Netopil et al. result also shows that the precise age evolution is not observationally settled.

Keeping thick-disc and halo slopes at zero is a scope decision, not a claim that the true gradients vanish exactly. SEGUE old disc stars show the radial `[Fe/H]` gradient becoming flat above roughly `|z| = 1 kpc`, supporting zero as a defensible baseline for the geometrical thick component ([Cheng et al. 2012](https://doi.org/10.1088/0004-637X/746/2/149); [open preprint](https://arxiv.org/abs/1110.5933)). Conversely, SEGUE K giants show a modest but significant outward metallicity decrease in the outer stellar halo from 10 to 65 kpc, caused in their model by a changing mixture of intermediate and metal-poor components. That is not well represented by a single linear cylindrical-`R` correction, especially while the generator lacks halo substructure and accretion origins, so v1 should record the omission rather than encode the wrong law ([Xue et al. 2015](https://doi.org/10.1088/0004-637X/809/2/144); [open preprint](https://arxiv.org/abs/1506.06144)).

### Halo ages and metallicities

A colour–magnitude analysis of the local kinematically selected halo finds both its red/in-situ and blue/accreted sequences to be old, with a sharp young-age cutoff around 10 Gyr; the halo is composite even when its age distributions overlap ([Gallart et al. 2019](https://doi.org/10.1038/s41550-019-0829-5); [open preprint](https://arxiv.org/abs/1901.02900)). A BHB chronographic map independently finds a central concentration older than 12 Gyr and a mean age decrease of roughly 1–1.5 Gyr toward 45–50 kpc, so a spatially constant halo age law is only a baseline ([Carollo et al. 2016](https://doi.org/10.1038/nphys3874)).

For metallicity, the Pristine inner-halo main-sequence-turnoff sample peaks at `[Fe/H] = -1.6` and shows a substantial extremely metal-poor tail ([Youakim et al. 2020](https://doi.org/10.1093/mnras/stz3619); [open preprint](https://arxiv.org/abs/2001.04988)). SDSS Stripe 82 main-sequence fits place a one-component peak between `-1.55` and `-1.80`, depending on photometric calibration, with a width near `0.4 dex`; two-component fits place peaks near `-1.7` and `-2.3`, but the authors caution that the MDF alone does not establish a physically dual halo ([An et al. 2013](https://doi.org/10.1088/0004-637X/763/1/65); [open preprint](https://arxiv.org/abs/1211.7073)).

## Recommended engineering v1 sampler

These are exact **engineering prototype parameters chosen from the constraints above**, not quoted survey fits. They are intentionally compatible with the current sampler, which can draw one independent `TruncatedNormal` for age and one for `[Fe/H]`. They should live in configuration and retain a model-version/provenance field.

Every normal distribution below is truncated to its stated support and resampled, not clipped to a boundary.

| Geometrical component | Age law | `[Fe/H]` law | Hard support |
|---|---|---|---|
| Thin disc | `Normal(5.0, 3.0)` | `Normal(-0.10, 0.25)` | age 0–13.5 Gyr; `[Fe/H]` -1.2 to +0.6 |
| Thick disc | `Normal(10.5, 1.5)` | `Normal(-0.45, 0.30)` | age 7–13.5 Gyr; `[Fe/H]` -1.5 to +0.3 |
| Stellar halo | `Normal(12.0, 1.0)` | `Normal(-1.60, 0.40)` | age 8–13.5 Gyr; `[Fe/H]` -4.0 to 0.0 |

Interpretation and intentional limitations:

- The broad thin-disc normal is only a convenient one-component prior. It does **not** claim that the Milky Way star-formation history is Gaussian. The GALAH data contain multiple phases and recent structure that a one-component law cannot reproduce.
- The current interface accepts `GalacticPosition` and shifts the thin-disc `[Fe/H]` mean with cylindrical radius `R`; it does not use `z`. Age and `[Fe/H]` are still conditionally independent once population and position are fixed, while `[alpha/Fe]` is conditioned on population and sampled `[Fe/H]`. The model therefore still omits the age–metallicity covariance, changing MDF skewness, a vertical chemical law, formation radius, and radial migration. The active `SpatialIronAndAlphaV2` version records this expanded interface.
- Do not add a universal vertical metallicity term in v1. The changing mixture of populations with `|z|`, the geometric component selection, and intrinsic gradients are otherwise easily double-counted. Revisit this with a joint chemo-spatial fit.
- The thick-disc parameters deliberately span the 9–11 Gyr results and geometrical/chemical mismatch. Its `[Fe/H]` baseline is centred close to the high-`|z|` APOGEE MDF, not converted from Sharma et al.'s `[M/H]`; `[M/H]` and `[Fe/H]` must remain distinct fields if total metallicity is added later.
- The halo single Gaussian captures the main MDF peak but underproduces the extremely metal-poor tail and ignores accreted streams. An optional experiment is a mixture `0.8 * Normal(-1.6, 0.35) + 0.2 * Normal(-2.3, 0.35)`, following the scale of An et al.'s two-component fits, but it must be labelled a numerical mixture rather than proof of two physical haloes.
- At halo radii beyond roughly 15 kpc, a later version should reduce the mean age gradually by no more than about 1–1.5 Gyr by 45–50 kpc and introduce spatial substructure. Do not infer that detail from the smooth v1 density law.

## Cosmic age cap

The Planck base-Lambda-CDM fit gives a Universe age of about 13.8 Gyr ([Planck Collaboration 2020](https://doi.org/10.1051/0004-6361/201833910); [open preprint](https://arxiv.org/abs/1807.06209)). The generator should enforce `0 <= stellar_age <= 13.5 Gyr`. The 0.3-Gyr margin represents the interval before the first stars and avoids generating a star at the instant of the Big Bang; it is a modelling choice, not a new cosmological measurement.

Observed stellar posterior estimates can exceed 13.8 Gyr because of uncertainty and stellar-model systematics. Such catalogue values should not be silently clipped during validation. Compare posterior distributions or uncertainty-aware summaries, while keeping generated physical ages below the configured cosmic cap.

## Selection effects: what not to validate directly

Survey histograms are not intrinsic population probability distributions:

- Kepler/K2 asteroseismic samples select giants in narrow sky fields and have evolutionary-state and detectability selection; inferred ages also depend on mass loss, stellar tracks, and binary mass transfer.
- APOGEE is a magnitude-, colour-, and footprint-selected red-giant survey. Hayden et al. explicitly test its targeting effects, but its raw MDF is still not an all-star present-day mass function.
- GALAH/LAMOST turnoff and subgiant cuts preferentially retain stars whose ages can be measured well; quality cuts reshape the sample.
- BHB stars, white dwarfs, and main-sequence turnoff stars each trace different progenitor masses and evolutionary lifetimes.
- Metal-poor searches are designed to find rare tails and cannot supply an unweighted all-halo MDF without their selection function.

Consequently, do not tune the generator until its raw histogram matches a published tracer histogram. The correct comparison is a forward model: generate births, evolve stars, apply the survey's spatial/photometric/evolutionary selection, add measurement uncertainty, and only then compare to the catalogue.

## Validation expectations for the prototype

Use broad invariants rather than pretending the chosen parameters are uniquely measured:

1. **Bounds and determinism:** no generated age exceeds 13.5 Gyr; equal domain-separated seeds reproduce `(age, [Fe/H])` exactly.
2. **Ordering:** in large local samples, halo and thick-disc median ages exceed the thin-disc median; halo median `[Fe/H]` is at least about 0.8 dex below either disc component.
3. **Disc gradient:** a large thin-disc sample must recover approximately `-0.060 dex/kpc` between 5 and 19 kpc while keeping the configured mean unchanged at `R = 8.178 kpc`. Boundary tests must prove that the mean correction stays constant below 5 kpc and above 19 kpc. `[M/H]` and `[Fe/H]` validations must remain separate. A future broken-line version should be tested independently against the Spina et al. knee and outer slope rather than changing this v1 test silently.
4. **Overlap:** the model must produce some old thin-disc and metal-rich/metal-poor overlap; component classification must not be recoverable perfectly from age or `[Fe/H]` alone.
5. **Halo MDF:** a large simple-halo sample should peak near `-1.6` with width near `0.4 dex`; a test should explicitly note that the baseline Gaussian fails to reproduce the Pristine metal-poor tail.
6. **Sensitivity plots:** plot the joint age–`[Fe/H]` distribution separately by component and at multiple radii. Vary every provisional mean/dispersion by at least 20% to show which later predictions are fragile.
7. **No survey-count claim:** label all raw-generator comparisons as prior-predictive. Quantitative goodness-of-fit requires an explicit survey selection function and stellar-evolution weighting.

This v1 is suitable for deterministic procedural sampling and for revealing architectural mistakes. It is not yet suitable for predicting exact local fractions, planet occurrence as a function of chemistry, or the observed distribution of any specific survey.
