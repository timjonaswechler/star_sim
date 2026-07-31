# Scientific basis for prototype stellar chemistry

This note defines a deterministic **v1 chemistry projection** for the procedural Milky Way generator. It starts from the already sampled iron abundance `[Fe/H]`, draws one population-conditioned composite `[alpha/Fe]`, derives a global metallicity `[M/H]`, and converts that logarithmic abundance into initial hydrogen, helium, and metal mass fractions `(X, Y, Z)`.

It is not a chemical-evolution simulation and it does not predict individual elemental abundances.

## Quantities must remain distinct

- `[Fe/H]` is the logarithmic iron-to-hydrogen number ratio relative to the Sun.
- `[alpha/Fe]` is a logarithmic abundance ratio for alpha-capture elements relative to iron. In observational catalogues the reported composite alpha abundance depends on which elements and spectral features enter the pipeline. Vincenzo et al. explicitly infer separate conditional distributions for Mg, O, Si, S, and Ca and find different separations between their low- and high-alpha sequences ([Vincenzo et al. 2021](https://doi.org/10.1093/mnras/stab2899); [open preprint](https://arxiv.org/abs/2101.04488)).
- `[M/H]` is a logarithmic global-metallicity proxy. It is not interchangeable with `[Fe/H]` when the mixture is alpha enhanced.
- `Z` is the mass fraction in elements heavier than helium. `X`, `Y`, and `Z` are mass fractions and must sum to one.

The v1 field named `alpha_enhancement_alpha_fe` should therefore be documented as a **single composite alpha proxy**, not as a complete abundance vector and not as H-alpha emission.

## Observational constraint on the disc alpha sequences

APOGEE DR16 red giants show an intrinsic low-alpha/high-alpha bimodality near the Solar circle at sub-solar `[Fe/H]`. Vincenzo et al. model `p([alpha/Fe] | [Fe/H], R)` as two Gaussians and infer typical intrinsic dispersions near `0.04 dex`; the high-alpha population becomes more prominent toward smaller Galactocentric radius, lower `[Fe/H]`, and larger `|z|` ([Vincenzo et al. 2021](https://doi.org/10.1093/mnras/stab2899); [open preprint](https://arxiv.org/abs/2101.04488)).

For their `[Mg/Fe]` fit at `7 <= R < 9 kpc`, their Table 1 gives low-alpha means from about `0.111` to `0.031 dex` and high-alpha means from about `0.282` to `0.124 dex` as the bin lower edge increases from `[Fe/H] = -0.5` to `+0.1`; the intrinsic dispersions are mostly `0.03–0.05 dex` ([Vincenzo et al. 2021](https://doi.org/10.1093/mnras/stab2899)). The fitted table only covers approximately `-0.5 <= [Fe/H] <= +0.2`, so extending either sequence across the generator's full iron support is a modelling extrapolation, not a measured relation.

The density model's geometrical thin and thick discs are not identical to APOGEE's chemical low- and high-alpha sequences. Consequently v1 uses those sequences as conditional priors for the geometrical components, while retaining overlap. This is an engineering correspondence, not an observational classification theorem.

### Exact engineering v1 alpha rules

Draw a truncated normal after `[Fe/H]` has been generated. The conditional mean is evaluated from that same system's iron abundance:

\[
\mu_{\alpha,\mathrm{thin}} = \operatorname{clamp}(0.035 - 0.16[\mathrm{Fe/H}],\ 0.03,\ 0.11),
\]

\[
\mu_{\alpha,\mathrm{thick}} = \operatorname{clamp}(0.160 - 0.27[\mathrm{Fe/H}],\ 0.12,\ 0.30).
\]

| Geometrical component | Conditional law | Hard support | Status |
|---|---|---|---|
| Thin disc | `Normal(mu_thin([Fe/H]), 0.04)` | `[-0.05, +0.25] dex` | Engineering approximation to the Solar-annulus low-alpha sequence |
| Thick disc | `Normal(mu_thick([Fe/H]), 0.04)` | `[+0.05, +0.45] dex` | Engineering approximation to the Solar-annulus high-alpha sequence |
| Stellar halo | `Normal(+0.25, 0.12)` | `[-0.10, +0.60] dex` | Deliberately broad engineering baseline |

The two disc slopes and clamps are compact approximations chosen from Vincenzo et al.'s tabulated Solar-annulus `[Mg/Fe]` means; they are not published regression coefficients. The `0.04 dex` residual width is observationally motivated, but the hard supports are safety bounds. The halo rule is broader because APOGEE finds chemically distinct high-Mg and low-Mg metal-poor populations with different kinematics, so one narrow Gaussian would conceal known accreted/in-situ structure ([Hayes et al. 2018](https://doi.org/10.3847/1538-4357/aa9cec); [open preprint](https://arxiv.org/abs/1711.05781)). A later halo model should use latent formation origins or a mixture rather than merely tuning this Gaussian.

Clamping the **mean** outside the disc fit interval avoids unlimited linear extrapolation, but it does not make the tails observationally calibrated. Samples with disc `[Fe/H] < -0.5` or `> +0.2` must be labelled prior-predictive.

## Converting iron and alpha abundance to global metallicity

For a uniformly alpha-enhanced mixture, Salaris, Chieffi, and Straniero give the global-metallicity correction

\[
[\mathrm{M/H}] = [\mathrm{Fe/H}] +
\log_{10}\left(0.638\,10^{[\alpha/\mathrm{Fe}]} + 0.362\right).
\]

The original work shows that low-mass, metal-poor alpha-enhanced stellar models can be approximated by solar-scaled models at the corrected global metallicity ([Salaris, Chieffi & Straniero 1993](https://doi.org/10.1086/173105)); the equation is reproduced explicitly in [Salaris & Cassisi 1997](https://doi.org/10.1093/mnras/289.2.406). At `[alpha/Fe] = 0`, the correction is zero; at `[alpha/Fe] = +0.3`, it is about `+0.214 dex`.

This is a useful v1 projection, not a replacement for alpha-enhanced stellar tracks. It assumes a scaled alpha mixture, while APOGEE shows that Mg, O, Si, S, and Ca do not follow identical sequences ([Vincenzo et al. 2021](https://doi.org/10.1093/mnras/stab2899)). It also should not be presented as a precision formula for high metallicity, unusual abundance patterns, carbon-enhanced stars, or chemically peculiar stars, because these were outside the purpose of the original low-metallicity isochrone equivalence ([Salaris, Chieffi & Straniero 1993](https://doi.org/10.1086/173105)).

## Converting `[M/H]` to initial mass fractions

Asplund et al. give protosolar mass fractions `X_sun = 0.7154`, `Y_sun = 0.2703`, and `Z_sun = 0.0142`, distinct from the present-day photospheric values because diffusion changes the surface composition over the Sun's lifetime ([Asplund et al. 2009](https://doi.org/10.1146/annurev.astro.46.060407.145222); [open preprint](https://arxiv.org/abs/0909.0948)). MIST adopts `Y_p = 0.249`, `Z_sun,initial = 0.0142`, and a linear helium enrichment slope `Delta Y / Delta Z = 1.5` in its solar-scaled stellar models ([Choi et al. 2016](https://doi.org/10.3847/0004-637X/823/2/102); [open preprint](https://arxiv.org/abs/1604.08592)). These are stellar-grid choices anchored to cosmological and solar constraints, not universal constants.

The engineering v1 adopts the MIST-compatible values:

| Parameter | v1 value |
|---|---:|
| `protosolar_x` | `0.7154` |
| `protosolar_y` | `0.2703` |
| `protosolar_z` | `0.0142` |
| `primordial_helium_y_p` | `0.249` |
| `helium_enrichment_dy_dz` | `1.5` |

To keep the logarithmic `Z/X` definition and the helium law internally consistent, use

\[
r_\odot = Z_\odot/X_\odot,
\qquad q = r_\odot 10^{[\mathrm{M/H}]},
\]

\[
Z = \frac{q(1-Y_p)}{1 + q(1 + \Delta Y/\Delta Z)},
\]

\[
Y = Y_p + (\Delta Y/\Delta Z)Z,
\qquad X = 1-Y-Z.
\]

The expression for `Z` is algebraically derived from `Z/X = q`, `Y = Y_p + (Delta Y/Delta Z) Z`, and `X + Y + Z = 1`; it is an implementation consequence, not an additional empirical fit. At `[M/H] = 0` it recovers the adopted protosolar composition to rounding. Using only `Z = Z_sun * 10^[M/H]` is close over modest ranges but does not exactly preserve both `Z/X` and the helium enrichment law.

## Data ownership and sampling order

Chemistry describes the birth material shared by a coeval stellar system. In v1 it should therefore be stored once on the generated system and inherited by all stellar components, rather than redrawn independently for binary companions:

```text
population + current-position proxy + system seed
    -> age and [Fe/H]
    -> conditional [alpha/Fe]
    -> [M/H]
    -> initial (X, Y, Z)
```

Real binaries can show surface-abundance changes from diffusion, dredge-up, rotation, or mass transfer, but those are later stellar-evolution/surface-state effects and should not overwrite the shared initial composition.

Use a separate deterministic random-number domain for the alpha residual. Adding chemistry must not perturb the previously generated system positions, multiplicities, ages, or iron abundances.

## Validity and intentional omissions

- The two disc conditional relations are data-guided only over roughly `-0.5 <= [Fe/H] <= +0.2` near the Solar annulus; their clamps are explicit extrapolation policy.
- The Salaris projection is safest for a uniformly alpha-enhanced, low-metallicity mixture. It cannot encode element-by-element variations.
- MIST's published solar-scaled grid covers `-2.0 <= [Z/H] <= +0.5`, with additional tracks down to `-4.0` for a more limited set of phases ([Choi et al. 2016](https://doi.org/10.3847/0004-637X/823/2/102)). A later evolution interpolator must check its chosen grid's actual coverage instead of assuming every generated chemistry has a valid track.
- The geometrical halo is chemically composite. Accreted streams, carbon-enhanced metal-poor stars, globular-cluster abundance anomalies, and distinct in-situ/accreted alpha sequences are omitted.
- `Y = Y_p + 1.5 Z` assigns a population mean initial helium abundance. It omits intrinsic helium spreads such as those found in some globular clusters.
- No abundance currently changes planet occurrence, opacity, lifetime, colour, or luminosity by itself. Those couplings belong to later planet-formation and stellar-evolution modules.

## Validation expectations

1. Equal `(seed, system_id, population, [Fe/H])` inputs reproduce the same complete chemistry.
2. Adding the chemistry sampler leaves all existing random streams unchanged.
3. At fixed `[Fe/H]` near the Solar circle, a large thick-disc sample has a higher median `[alpha/Fe]` than a thin-disc sample, while their hard supports overlap.
4. At `[alpha/Fe] = 0`, `[M/H] == [Fe/H]` to floating-point tolerance.
5. At `[Fe/H] = 0` and `[alpha/Fe] = +0.3`, `[M/H]` is approximately `+0.214`.
6. Every generated composition is finite, has `X > 0`, `Y > 0`, `Z > 0`, and satisfies `X + Y + Z = 1` within floating-point tolerance.
7. At `[M/H] = 0`, the conversion recovers approximately `(0.7154, 0.2703, 0.0142)`.
8. All stellar members of one generated system initially receive identical chemistry.
9. Visualize `[alpha/Fe]` against `[Fe/H]` by geometrical component. The diagram must be labelled as a prior-predictive chemistry plot, not an HR diagram or a direct survey fit.
