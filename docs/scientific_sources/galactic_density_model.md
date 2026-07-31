# Scientific basis for the prototype Galactic density model

This note records a defensible **first-pass, axisymmetric stellar number-density model**. It is not a unique fit to the Milky Way. Reported scale lengths and heights depend on the selected stellar tracer, age, chemistry, extinction treatment, unresolved binaries, and adopted functional form.

## Recommended prototype parameters

| Parameter | Prototype value | Meaning and provenance |
|---|---:|---|
| Solar Galactocentric radius, `R_sun` | 8.178 kpc | Geometric S2-orbit measurement: 8.178 ± 0.013 (stat.) ± 0.022 (sys.) kpc |
| Local stellar number density, `n_sun` | 0.0799 stars pc⁻³ | CNS5 25-pc census; this counts stars, not stellar systems or brown dwarfs |
| Thin-disc radial scale, `L_thin` | 2.6 kpc | SDSS late-type main-sequence star-count fit |
| Thin-disc vertical scale, `H_thin` | 0.300 kpc | Same fit |
| Thick-disc radial scale, `L_thick` | 3.6 kpc | Same, geometrically defined thick component |
| Thick-disc vertical scale, `H_thick` | 0.900 kpc | Same |
| Local thick/thin ratio | 0.12 | Number-density normalization at the Solar position |
| Stellar-halo flattening, `q_halo` | 0.64 | Oblate SDSS stellar-halo fit |
| Stellar-halo power-law slope, `p_halo` | 2.8 | `n ∝ m⁻²·⁸` for ellipsoidal radius `m` |
| Local stellar-halo/thin ratio | 0.005 | Number-density normalization at the Solar position |
| Bulge shape | α = 1.8, r₀ = 0.075 kpc, r_cut = 2.1 kpc, q = 0.5 | McMillan's convenient axisymmetric **mass-density** approximation; do not treat its amplitude as a measured star count |

The first eight structural values form a coherent engineering baseline from [Jurić et al. (2008)](https://arxiv.org/abs/astro-ph/0510520), anchored in absolute number at the Solar position by [Golovin et al. (2023), CNS5](https://doi.org/10.1051/0004-6361/202244250). The precise Solar radius comes from the [GRAVITY Collaboration (2019)](https://doi.org/10.1051/0004-6361/201935656).

Using the Jurić local ratios solely to split the CNS5 total gives the prototype normalization

```text
n_thin_sun  = 0.0799 / (1 + 0.12 + 0.005) = 0.07102 stars pc^-3
n_thick_sun = 0.12  * n_thin_sun          = 0.00852 stars pc^-3
n_halo_sun  = 0.005 * n_thin_sun          = 0.000355 stars pc^-3
```

This split is an engineering calibration, not a result fitted jointly by either paper: CNS5 samples the full nearby luminosity function, while Jurić et al. fit a colour-selected late-type main-sequence tracer and correct their result assuming a 35% binary fraction.

## Prototype equations

For each geometrical disc component `i`:

```text
n_i(R, z) = n_i_sun
            * exp(-(R - R_sun) / L_i)
            * exp(-abs(z) / H_i)
```

For the oblate stellar halo:

```text
m(R, z)    = sqrt(R^2 + (z / q_halo)^2)
n_halo     = n_halo_sun * (R_sun / m)^p_halo
```

The halo law must not be extrapolated to `m = 0`; it was inferred from halo tracers, not the Galactic centre. The prototype should either omit the stellar-halo term in the inner bulge region or apply an explicitly labelled numerical validity floor. Such a floor is a software safeguard, not an observed core radius.

Jurić et al. mapped about 48 million SDSS stars and obtained the bias-corrected values `H1 = 300 pc`, `L1 = 2600 pc`, `H2 = 900 pc`, `L2 = 3600 pc`, a local thick-disc normalization of 12%, and a stellar halo with `q = 0.64`, slope approximately 2.8, and local halo/thin normalization of 0.5%. They estimate errors no larger than roughly 20% for disc scales and roughly 10% for the thick-disc normalization. Their maps also contain real overdensities, so the smooth model is only a baseline. See [the paper and abstract](https://arxiv.org/abs/astro-ph/0510520) and [journal PDF](https://dash.harvard.edu/bitstream/1/33462111/1/Juric_2008_ApJ_673_864.pdf).

## Absolute local density: what is being counted

CNS5 reports `(7.99 ± 0.11) × 10⁻² stars pc⁻³` and, separately, `(1.07 ± 0.04) × 10⁻² brown dwarfs pc⁻³` within its 25-pc census ([Golovin et al. 2023](https://doi.org/10.1051/0004-6361/202244250)). The prototype therefore uses 0.0799 **stellar objects** pc⁻³ and excludes brown dwarfs initially.

This is not a stellar-**system** density. Unresolved companions, catalogue completeness, multiplicity, white dwarfs, and the star/substellar boundary affect what gets counted. Before generating systems, the model needs a multiplicity prescription that converts the stellar-object density into a system density without double-counting companions.

As an independent Gaia-era warning about selection, the Gaia EDR3 nearby-star catalogue estimates at least 92% completeness through spectral type M9 within 100 pc and discusses unresolved close binaries explicitly ([Gaia Collaboration 2021](https://doi.org/10.1051/0004-6361/202039498)).

## Thin/thick discs are a useful decomposition, not unique populations

The recommended two exponentials are a morphological approximation. Abundance-selected SEGUE populations span scale heights from about 0.2 to 1 kpc and radial scale lengths from more than 4.5 to about 2 kpc as chemistry and likely age change ([Bovy et al. 2012](https://doi.org/10.1088/0004-637X/753/2/148)). A related mass-weighted analysis finds a continuous distribution of scale heights rather than a unique thin/thick bimodality ([Bovy, Rix & Hogg 2012](https://doi.org/10.1088/0004-637X/751/2/131)).

Consequently, `thin` and `thick` should initially mean **geometrical model components**, not formation histories or chemically pure populations. Later age/metallicity sampling should replace or refine this two-component approximation.

## Bulge: shape is usable now, number normalization is not

A convenient axisymmetric approximation used by [McMillan (2017)](https://doi.org/10.1093/mnras/stw2759) is

```text
r' = sqrt(R^2 + (z/q)^2)
rho_b = rho_0,b / (1 + r'/r_0)^alpha * exp(-(r'/r_cut)^2)
```

with `alpha = 1.8`, `r_0 = 0.075 kpc`, `r_cut = 2.1 kpc`, `q = 0.5`, total bulge mass `8.9 × 10^9 M_sun`, and `rho_0,b = 9.93 × 10^10 M_sun kpc^-3`. These are **mass-density** quantities. They cannot be added directly to the disc/halo star-number densities above. Converting them requires a present-day mass function, remnants, and a multiplicity convention; a single guessed mean stellar mass would only be an explicit visualization assumption.

The real bulge is barred and box/peanut-shaped. VVV red-clump tracer counts find approximate axis ratios `10:6.3:2.6` and exponential scales `0.70:0.44:0.18 kpc` along the bar axes ([Wegg & Gerhard 2013](https://doi.org/10.1093/mnras/stt1376)). McMillan explicitly warns that an axisymmetric model cannot accurately represent the inner few kiloparsecs. Therefore:

- show the bulge as a separately labelled relative/mass-shape panel for now;
- do not include it in a quantitative total **number-density** map until the population-to-mass conversion exists;
- later replace it with a barred model if Galactic-centre regions matter.

Also, McMillan's halo component is a **dark-matter halo**. Its density must never be used as the stellar-halo density; the prototype stellar halo comes from Jurić et al.

## Scope and validity

Version 1 deliberately ignores spiral arms, warp, flare, streams, clusters, the nuclear stellar disc/cluster, dust-selection effects, and local vertical asymmetries. It is appropriate for testing deterministic sampling and large-scale plots, not precision prediction at an arbitrary Galactic coordinate.

The code/config should preserve provenance and semantics in parameter names, for example `stellar_number_density_per_pc3`, `bulge_mass_density_msun_per_pc3`, and `geometric_thick_disk`, so later work cannot silently mix units or interpretations.
