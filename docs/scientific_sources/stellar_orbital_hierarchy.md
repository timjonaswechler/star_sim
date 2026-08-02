# Scientific basis for a static stellar-orbital hierarchy

The narrowly bounded radius fallback used for contact rejection below the bundled MIST mass grid is documented separately in [low_mass_contact_radius.md](low_mass_contact_radius.md). It does not create a stellar-evolution snapshot.

This note defines a deliberately narrow version-1 model that turns the already generated members of one stellar system into a **latent physical hierarchy of Keplerian relative orbits**. Its immediate consumer is the planet-occurrence model: every supported member receives the semimajor axis of its nearest stellar companion. Version 1 does not integrate motion, predict an instantaneous position, model binary interaction, or synthesize an observing instrument.

The recommended implementation keeps orbit generation separate from the existing component-count and mass-ratio sampler. That separation is an engineering boundary, not a claim of statistical independence: the selection-corrected synthesis of Moe & Di Stefano shows that period, mass ratio, eccentricity, and primary mass are covariant. A later joint sampler should replace this staged approximation ([Moe & Di Stefano 2017](https://doi.org/10.3847/1538-4365/aa6fb6); [open manuscript](https://arxiv.org/abs/1606.05347)).

## Exact v1 domain model

Represent a system as a binary tree. Leaves reference stable stellar-member IDs; internal nodes own one relative orbit between the barycentres of their two children:

```text
StellarOrbitalHierarchy {
    model_id: "static_field_hierarchy_v1",
    root: OrbitNode,
    quality_flags,
}

OrbitNode = Member(member_id)
          | RelativeOrbit {
                left: OrbitNode,
                right: OrbitNode,
                semimajor_axis_au,
                period_days,
                eccentricity,
                mass_used_msun,
                provenance,
            }
```

`semimajor_axis_au` is the semimajor axis of the **relative orbit between the two child barycentres**. It is neither a component's barycentric radius nor an instantaneous separation. This distinction is required by the two-body form of Kepler's third law and by the stability test below.

Use present-day masses only when every member is on the main sequence and has a valid current mass and radius. The empirical field-star orbit samples describe present-day systems, while the current simulator does not evolve orbits through wind mass loss, mass transfer, common envelopes, supernovae, kicks, mergers, or disruption. Return `OrbitalEvolutionNotModeled` for a system containing an evolved star or remnant instead of attaching a present-day orbit to birth masses ([Moe & Di Stefano 2017](https://doi.org/10.3847/1538-4365/aa6fb6)).

## Binary scale distributions

Use two explicitly named source regimes. Do not silently interpolate across the gap or extrapolate to massive primaries.

### M-dwarf regime: `0.20 <= M1/M_sun < 0.70`

Sample a relative semimajor axis directly:

```text
log10(a / AU) ~ Normal(mu = 1.68, sigma = 0.97)
a <= 10_000 AU
```

Susemiehl & Meyer fitted this log-normal shape to binary-fraction measurements from four M-dwarf surveys spanning complementary semimajor-axis intervals. Their fitted parameters are `mu = 1.68 (+0.14/-0.16)` and `sigma = 0.97 +/- 0.19`; the well-constrained combined domain was `0.60 <= q <= 1.00` and `a <= 10,000 AU`. Version 1 uses the **normalized separation shape only**, because component counts and `q` have already been sampled elsewhere. Set `MSeparationShapeDecoupledFromMassRatio`, and additionally `LowMassExtrapolation` for `M1 < 0.20 M_sun`; the input surveys do not calibrate that lowest-mass interval ([Susemiehl & Meyer 2022](https://doi.org/10.1051/0004-6361/202038582); [open manuscript](https://arxiv.org/abs/2109.05951)).

This model's peak is about `48 AU`. Winters et al.'s independent volume-limited 25-pc census instead finds a broad projected-separation peak at `4-20 AU`, with a weak trend toward smaller separations for lower-mass primaries. The difference is a useful validation envelope, not a reason to tune the log-normal ad hoc ([Winters et al. 2019](https://doi.org/10.3847/1538-3881/ab05dc); [open manuscript](https://arxiv.org/abs/1901.06364)).

### F/G/early-K regime: `0.70 <= M1/M_sun <= 1.30`

Sample an orbital period:

```text
x = log10(P / day)
x ~ Normal(mu = 5.03, sigma = 2.28)
accept only -0.3 < x < 10.0
```

Raghavan et al.'s volume-limited survey of 454 nearby F6-K3 primaries fitted the observed orbital-period distribution with `mu = 5.03` and `sigma = 2.28` in `log10(days)`. The finite `-0.3 < log10(P/day) < 10` sampling bounds follow Tokovinin's explicit recursive F/G hierarchy experiment; they are numerical/model bounds rather than completeness limits of the Raghavan survey ([Raghavan et al. 2010](https://doi.org/10.1088/0067-0049/190/1/1); [open manuscript](https://arxiv.org/abs/1007.0414); [Tokovinin 2014](https://doi.org/10.1088/0004-6256/147/4/87); [open manuscript](https://arxiv.org/abs/1401.6827)).

Set `SolarPeriodShapeProxy` outside the narrower `0.8-1.2 M_sun` solar-type anchor used by the mass-dependent synthesis of Moe & Di Stefano. The wider `0.70-1.30 M_sun` software interval is justified by the F6-K3 target selection, but is not a statement that the distribution is constant across that interval ([Raghavan et al. 2010](https://doi.org/10.1088/0067-0049/190/1/1); [Moe & Di Stefano 2017](https://doi.org/10.3847/1538-4365/aa6fb6)).

### Typed gaps

Return `OutsideOrbitalScaleCalibration` for `0.08-0.20`, `0.60-0.70` only if the declared M-dwarf low-mass proxy is disabled, and always for `M1 > 1.30 M_sun` in strict mode. VAST constrains wide A-star companions but is incomplete at close separation, and Sana et al. constrain close O-star periods but not the full wide population; joining either to the solar log-normal would manufacture an unmeasured distribution ([De Rosa et al. 2014](https://doi.org/10.1093/mnras/stt1932); [open manuscript](https://arxiv.org/abs/1311.7141); [Sana et al. 2012](https://doi.org/10.1126/science.1223344); [open manuscript](https://arxiv.org/abs/1207.6397)).

## Kepler conversion and contact rejection

For a relative orbit with child-subtree masses `M_left` and `M_right`, use

```text
M_total_msun = M_left + M_right
a_AU = cbrt(M_total_msun * (P_days / 365.25)^2)
P_days = 365.25 * sqrt(a_AU^3 / M_total_msun)
```

These are Kepler's two-body relations in solar-mass, AU, and Julian-year units. For an orbit whose child is itself a binary, its mass is the sum of the leaf masses under that child.

Reject a leaf-leaf candidate if its periastron would overlap the two photospheres:

```text
a_AU * (1 - e) > (R_left + R_right) * R_sun_in_AU
R_sun_in_AU = 0.0046504673
```

This is only a geometric non-contact filter. It does not calculate Roche lobes, tides, pre-main-sequence radii, or later interaction.

## Eccentricity prescription

For the F/G/early-K regime, use:

```text
P <= 12 days: e = 0
P > 12 days:  e ~ Uniform(0, e_max)
e_max = min(0.99, 1 - (P / 2 days)^(-2/3))
```

Raghavan et al. find a tidal circularization period of about `12 d` and an approximately flat eccentricity distribution at longer periods. Moe & Di Stefano define the displayed `e_max(P)` envelope to avoid near-Roche-filling periastra and fit a period- and mass-dependent `p(e) proportional to e^eta`; the uniform v1 draw deliberately omits that more detailed covariance ([Raghavan et al. 2010](https://doi.org/10.1088/0067-0049/190/1/1); [Moe & Di Stefano 2017](https://doi.org/10.3847/1538-4365/aa6fb6)).

The cited M-dwarf separation fits do not supply a comparably complete eccentricity distribution. Version 1 may reuse the solar prescription only with `SolarEccentricityProxyForMDwarf`; strict mode instead returns `OutsideEccentricityCalibration`. The proxy is useful for producing a stability-screened hierarchy, but must not be presented as a measured M-dwarf eccentricity law.

## Triple and quadruple topology

A stable triple is represented as `(A,B)+C`: one inner binary orbited by a tertiary. A quadruple must represent one of the two distinct binary-tree topologies:

```text
2+2: (A,B) + (C,D)
3+1: ((A,B),C) + D
```

For nearby F/G systems, Tokovinin's completeness model predicts that about `74%` of quadruples are `2+2`; the observed value is `67%`, and 9 of 11 quadruples in the 25-pc subset are `2+2`. Use `p(2+2)=0.74`, `p(3+1)=0.26` as the named `tokovinin2014_fg_quad_topology` option. Set `TopologyMassExtrapolation` outside the F/G regime; this conditional split is not calibrated as universal ([Tokovinin 2014](https://doi.org/10.1088/0004-6256/147/4/87); [open manuscript](https://arxiv.org/abs/1401.6827)).

The existing mass inventory does not say which members form an inner pair. For deterministic v1 assignment, sort companions by mass: a triple uses the primary and most massive companion as `(A,B)`; a `3+1` quadruple then attaches the next and least massive companions outward; a `2+2` quadruple pairs the primary with the most massive companion and pairs the two remaining members. Mark `HierarchyPairingEngineered`. Observed hierarchy pairing is correlated with mass ratio and inner period, so this rule is software policy, not an empirical draw ([Tokovinin 2008](https://doi.org/10.1111/j.1365-2966.2008.13613.x); [open manuscript](https://arxiv.org/abs/0806.3263); [Tokovinin 2014](https://doi.org/10.1088/0004-6256/147/4/87)).

For each required orbit, draw independently from the applicable scale distribution, sort candidates from inner to outer, and reject the whole candidate if any nested level fails contact or stability. This follows the basic recursive Monte Carlo experiment of Tokovinin, but it ignores the observed excess of short inner periods and the covariance between hierarchy level, `q`, and period. Set `IndependentOrbitScaleDraws` ([Tokovinin 2014](https://doi.org/10.1088/0004-6256/147/4/87); [Moe & Di Stefano 2017](https://doi.org/10.3847/1538-4365/aa6fb6)).

## Stability screen

Apply the Mardling-Aarseth empirical boundary to every nested triple:

```text
a_out / a_in
  > 2.8
    * (1 + q_out)^(2/5)
    * (1 + e_out)^(2/5)
    / (1 - e_out)^(6/5)
    * (1 - 0.3 * i_mut / pi)

q_out = M_outer_child / M_inner_subtree
```

For v1 use `i_mut = 0` (coplanar prograde) only for the acceptance screen; no physical orientation is generated. This is more restrictive than the criterion's inclination correction for inclined or retrograde configurations, but it is not proof of long-term secular stability. The original criterion is an empirical chaos boundary, and its inclination factor is approximate ([Mardling & Aarseth 2001](https://doi.org/10.1046/j.1365-8711.2001.03974.x)).

Apply it as follows:

- triple: test inner `(A,B)` against outer `AB-C`;
- `3+1`: test `(A,B)` against `AB-C`, then treat `ABC` as the inner subtree and test the `ABC-D` orbit against the `AB-C` orbit;
- `2+2`: test the common outer orbit against each inner binary separately, using the other binary's total mass as the outer-child mass.

Use at most `128` deterministic rejection attempts per system and return `StableHierarchySamplingExhausted` on failure. The attempt cap is an engineering guard. A later backend can use the stability fit of Vynatheya et al., which adds stronger inner-eccentricity and inclination dependence and outperforms the original formula over its numerical training domain ([Vynatheya et al. 2022](https://doi.org/10.1093/mnras/stac2566); [open manuscript](https://arxiv.org/abs/2207.03151)).

## Feeding planet occurrence

For each stellar member, traverse from its leaf to the root and report the minimum semimajor axis among ancestral orbit nodes whose opposite child contains at least one star:

```text
nearest_companion_semimajor_axis_au(member)
    = min(semimajor axes on member-to-root path)
```

For a binary this is its only orbit. For either member of an inner binary it is normally the inner orbit; for the outer tertiary of a triple it is the outer orbit relative to the inner binary's barycentre. Feed this value into `KnownCompanionSeparation { semimajor_axis_au }`. The current Kraus et al. planet-occurrence step then applies its `<47 AU` suppression consistently; that calibration itself remains a coarse occurrence correction rather than a planetary stability calculation ([Kraus et al. 2016](https://doi.org/10.3847/0004-6256/152/1/8); [open manuscript](https://arxiv.org/abs/1604.05744)).

If hierarchy generation is outside source coverage, contains evolved members, or exhausts stability sampling, preserve `SeparationUnknown` and its existing `MultiplicitySeparationRequired` result. Do not substitute a system-member count, projected separation, or random fallback.

## Unresolved and projected companions

“Unresolved” is an observing-state label, not a physical orbit state. A procedurally generated latent binary always has a physical `a` even if a hypothetical telescope would blend it. Conversely, an observed projected separation `rho` is not the same quantity as `a`; the often used statistical conversion factor (about `1.26`) applies to ensembles under orientation/orbit assumptions and must not be used as an exact per-system conversion. Store imported observations as `ProjectedSeparationOnly` unless an orbital solution or an explicitly probabilistic inference layer supplies `a` ([Winters et al. 2019](https://doi.org/10.3847/1538-3881/ab05dc); [Susemiehl & Meyer 2022](https://doi.org/10.1051/0004-6361/202038582)).

No orientation, mean anomaly, instantaneous three-dimensional separation, projected separation, or angular separation is needed by v1. If an observer layer is added later, it should sample isotropic orientations and orbital phase and keep `a`, instantaneous `r`, projected `rho`, and sky angle as distinct types.

## Recommended configuration

```text
model_id = "static_field_hierarchy_v1"

m_dwarf_scale:
  configured_primary_mass_msun = [0.08, 0.70)
  source_primary_mass_msun = approximately [0.20, 0.67]
  log10_a_au_mean = 1.68
  log10_a_au_sigma = 0.97
  maximum_a_au = 10_000

solar_scale:
  configured_primary_mass_msun = [0.70, 1.30]
  log10_period_days_mean = 5.03
  log10_period_days_sigma = 2.28
  minimum_log10_period_days = -0.3
  maximum_log10_period_days = 10.0

eccentricity:
  circularization_period_days = 12.0
  long_period_power = 0.0       # uniform p(e)
  absolute_maximum = 0.99
  m_dwarf_uses_solar_proxy = true

quadruple_topology:
  probability_2_plus_2 = 0.74
  probability_3_plus_1 = 0.26

stability:
  coefficient = 2.8
  mutual_inclination_rad = 0.0
  maximum_sampling_attempts = 128
```

Every random draw must use a domain-separated seed derived from stable system, node, and attempt identities. Adding another unrelated system or changing plot order must not perturb existing hierarchies.

## What v1 may and may not claim

V1 can defensibly provide a deterministic latent relative semimajor axis for main-sequence M-dwarf and nearby solar-type field systems, distinguish triple/quadruple topology, reject obviously unstable nested configurations, and provide the nearest companion scale required by the planet-occurrence correction.

V1 cannot claim a jointly calibrated `p(q,P,e | M1)`, calibrated A/B/O-star full orbits, orbital evolution, tidal circularization histories, mass transfer, compact-object kicks, disruption of very wide pairs, secular stability, phase-space coordinates, eclipses, resolvability, circumbinary/circumstellar planet stability zones, or planet survival. Its M-dwarf eccentricities and non-F/G quadruple topology are declared proxies. The next scientific upgrade should implement the full Moe & Di Stefano joint model and replace the solar-proxy eccentricity and independent hierarchy draws.

## Implementation status

The active catalog implements this v1 as a deterministic binary tree of relative orbits. It preserves the existing stellar-member IDs, rejects photospheric contact and unstable nested candidates, records all declared proxy flags, and supplies each supported member's nearest companion semimajor axis to the planet-occurrence model. Unsupported masses, unavailable evolution snapshots, evolved stars, and exhausted stability sampling preserve `SeparationUnknown`; they are never filled by clamping or a fallback distribution.
