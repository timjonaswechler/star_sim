# Evidence models for satellites, rings, and Trojan populations

Research for the Wayfinder ticket **Choose evidence models for satellites, rings, and Trojan populations**. Sources were checked on 2026-08-02. This note uses primary papers only.

## Decision summary

The simulator can support all three structures, but it cannot assign a universally empirical occurrence probability to any of them. The evidence supports a layered model:

| Structure or property | Recommended evidence label | Decision |
|---|---|---|
| Frequency of *detectable Galilean-analog* systems around warm Kepler planets | `Empirical` in the surveyed domain only | Preserve the survey posterior and selection domain; do not interpret it as the frequency of all moons. |
| Generic regular moons around giant planets | `PhysicalProxy` | Generate through a circumplanetary-disc formation proxy with a moon-system mass budget and architecture distribution. |
| Large moons of terrestrial planets; captured or irregular moons | `Speculative` occurrence, `PhysicalProxy` dynamics | Permit rarely through named formation channels, then require Roche/Hill/tidal and system-wide checks. |
| Frequency of large rings around short-period transiting planets | empirical **upper constraint**, not an empirical draw rate | Choose a `PhysicalProxy` probability that respects the constraint in its applicable domain. |
| Ring dimensions, composition, orientation, and survival | `PhysicalProxy` | Derive from planetary radius/density, Roche/Hill limits, temperature, age, tides, and conflicts with moons. |
| Frequency and mass of extrasolar small-body Trojan swarms | `PhysicalProxy` or `Speculative` | There is no calibrated extrasolar occurrence rate. Tie the proxy to planetary architecture and reservoir history. |
| Trojan 1:1 resonance, L4/L5 placement, and dynamical survival | `PhysicalProxy` | Enforce analytical eligibility and a multi-body stability check; the label is not `Empirical` because it is a dynamics model. |

An empirical limit used to constrain a proxy does **not** turn the sampled property into `Empirical`. Provenance should therefore be stored per property: for example, a ring's `occurrence_probability` can be `PhysicalProxy`, its `occurrence_upper_bound` can be `Empirical`, and an unusual cosmetic colour variation can be `Speculative`.

## Natural-satellite systems

### What is observed

The best population constraint relevant to ordinary exoplanets remains narrow. A stacked search of 284 Kepler planet candidates, spanning Earth-to-Jupiter radii and roughly 0.1–1 au, inferred an occurrence of Galilean-analog moon systems of \(\eta=0.16^{+0.13}_{-0.10}\) at 68.3% credibility and \(\eta<0.38\) at 95% confidence; the authors found no strong population signal, and explicitly described a possible \(\sim0.5\,R_\oplus\), 5–10-planetary-radius feature as only a hint with Bayes factor 2 ([Teachey, Kipping & Schmitt 2018](https://doi.org/10.3847/1538-3881/aa93f2)). This can calibrate one explicitly named **Galilean-analog observational class**, but it does not measure the rate of smaller moons, moons outside the sample's separation range, or every kind of satellite system.

A later survey selected 70 usable cool, long-period giant-planet light curves from an initial 73 and reported one exomoon candidate, Kepler-1708 b-i; it did not establish a general moon-occurrence distribution ([Kipping et al. 2022](https://doi.org/10.1038/s41550-021-01539-1)). In July 2026, radial-velocity observations produced strong evidence for at least one planetary-mass satellite around the brown dwarf CD-35 2722 B, with a minimum mass of 0.743 Jupiter masses and a period of 169 days; the host is a substellar companion rather than an ordinary planet, and a single hierarchical system cannot supply an occurrence model for planetary moons ([Hoy et al. 2026](https://doi.org/10.1038/s41586-026-10751-w)).

**Inference for the simulator:** do not use “has moons = 16%.” Use the Teachey et al. posterior only when drawing the specifically defined, detectable Galilean-analog class inside its survey domain. Generic and low-mass moon systems require proxies.

### What formation and dynamics can support

For gas giants, a gas-starved circumplanetary-disc model balances continued solid inflow against loss through gas-driven orbital migration and produces a characteristic total regular-satellite mass fraction near \(10^{-4}\) of the planet mass ([Canup & Ward 2006](https://doi.org/10.1038/nature04860)). A later N-body population synthesis for a Jupiter-like planet found that 85% of its generated systems were less massive than the Galilean system's \(2\times10^{-4}M_J\); the paper also showed that the distributions depend strongly on assumed dust-to-gas ratio and solids-refilling timescale, so its outputs are model-conditioned rather than empirical frequencies ([Cilibrasi et al. 2021](https://arxiv.org/abs/2011.11513)). These results support a `PhysicalProxy` for regular moons of giant planets, with the circumplanetary solid inventory exposed as an assumption rather than hidden in a universal constant.

The same population synthesis found that, when counting only moons in the individual-Galilean mass regime (at least \(10^{-5}M_J\)), 60% of simulated systems formed no moon above that threshold and about 18% formed three to five; 1:2 and 2:3 were its most frequent resonant-pair configurations ([Cilibrasi et al. 2021](https://arxiv.org/abs/2011.11513)). Those figures are suitable as a replaceable architecture proxy for Jupiter analogues, not as observational occurrence rates and not for terrestrial or ice-giant hosts.

Orbital placement has defensible hard gates. Numerical integrations give an approximate outer stability limit, in Hill-radius units, of

\[
a_{\rm max,pro}\simeq0.4895R_H(1-1.0305e_p-0.2738e_s)
\]

for prograde moons and

\[
a_{\rm max,retro}\simeq0.9309R_H(1-1.0764e_p-0.9812e_s)
\]

for retrograde moons in the tested restricted three-body configurations ([Domingos, Winter & Yokoyama 2006](https://doi.org/10.1111/j.1365-2966.2006.11104.x)). These fits are a first-pass coverage model, not proof of stability in a real multi-planet or multiple-star system; a system-level validator must still reject perturbation-driven failures.

Tidal evolution can remove moons even when their instantaneous orbit lies inside the Hill-stable region. The survival limit depends on system age, planetary spin, tidal quality factor \(Q_p\), Love number, satellite mass, and whether migration is inward or outward; close-in, tidally braked giant planets can lose massive primordial moons much faster than distant giants ([Barnes & O'Brien 2002](https://doi.org/10.1086/341477)). Because \(Q_p\) and related interior parameters will often be inferred rather than observed, the tidal result remains a `PhysicalProxy` and should retain its assumption set.

### Recommended moon-generation contract

1. Select a named formation channel: `RegularCircumplanetaryDisk`, `GiantImpact`, or `Capture`. The regular gas-giant channel is `PhysicalProxy`; occurrence for giant-impact and capture channels is `Speculative` until a calibrated host-dependent rate is adopted.
2. Draw a total satellite mass budget before individual moons. For a Jupiter-like regular system, centre the proxy near the \(10^{-4}\)–\(2\times10^{-4}\) planet-mass scale, but retain broad model uncertainty and permit zero surviving material ([Canup & Ward 2006](https://doi.org/10.1038/nature04860); [Cilibrasi et al. 2021](https://arxiv.org/abs/2011.11513)).
3. Partition that budget into moons and orbital locations using a host-specific proxy. Never apply the Jupiter synthesis unchanged to terrestrial planets, ice giants, remnants, or strongly irradiated hot Jupiters.
4. Require each resolved moon to lie outside the applicable disruption/contact boundary and inside the eccentricity-dependent prograde or retrograde stability boundary, then run mutual and external-perturber checks ([Schlichting & Chang 2011](https://arxiv.org/abs/1104.3863); [Domingos, Winter & Yokoyama 2006](https://doi.org/10.1111/j.1365-2966.2006.11104.x)).
5. Apply tidal survival over the actual system age. A generated candidate that fails is absent in that seed, with the rejection reason preserved; it is not silently redrawn ([Barnes & O'Brien 2002](https://doi.org/10.1086/341477)).

## Ring systems

### What is observed

A systematic fit of ringed and ringless models to 168 high-signal-to-noise Kepler short-cadence planet candidates found no viable ring candidate after investigating false positives. Under the assumption that the ring plane is tidally aligned with the planetary orbit, the study constrained the occurrence of rings with outer radius greater than twice the planet radius to below 15%; most targets were short-period planets ([Aizawa et al. 2018](https://doi.org/10.3847/1538-3881/aab9a1)). This is an upper bound on one large, favourably modelled ring class—not a measured Bernoulli probability for all rings.

Physical ring models predict that warm planets inside the ice line should preferentially have rocky rather than icy rings, and that optically thick rings can persist for model-dependent lifetimes from a few million to a few billion years. The same work uses the Roche radius, radiation drag, tidal alignment and Laplace-plane warping to delimit plausible ring structure ([Schlichting & Chang 2011](https://arxiv.org/abs/1104.3863)). The range is too broad to justify an age-independent ring draw, but it supplies a `PhysicalProxy` for material, lifetime and geometry.

Massive moons can dynamically clear otherwise ring-stable regions. In integrations of a hypothetical extended Jovian ring, the Galilean satellites removed material across much of the tested 3–29 Jupiter-radius region over \(10^6\)–\(10^7\) years ([Kane & Li 2022](https://arxiv.org/abs/2207.06434)). Therefore rings and moons cannot be generated independently and merely attached to the same planet.

### Recommended ring-generation contract

1. The occurrence draw is `PhysicalProxy`. For short-period planets in the Aizawa et al. domain, the probability of a ring larger than \(2R_p\) must not exceed the empirical 15% upper constraint; smaller, misaligned, faint, and long-period rings remain observationally unresolved by that number ([Aizawa et al. 2018](https://doi.org/10.3847/1538-3881/aab9a1)).
2. Derive inner/outer extent from the planet surface, density-dependent Roche scale, Hill/stability scale, and surviving moons. Treat an exceptional ring outside the nominal Roche-dominated zone as `Speculative` and still require a finite survival mechanism ([Schlichting & Chang 2011](https://arxiv.org/abs/1104.3863); [Kane & Li 2022](https://arxiv.org/abs/2207.06434)).
3. Derive bulk material from thermal environment: rocky material is the default inside the ice line; icy material requires a sufficiently cold environment ([Schlichting & Chang 2011](https://arxiv.org/abs/1104.3863)). Decorative colours may be `Speculative`, but must be compatible with the selected material and temperature.
4. Check the ring's survival time against stellar-system age and conflicts with moons. If a short-lived ring is retained in an old system, the model must also record a recent replenishment/disruption event; otherwise reject it ([Schlichting & Chang 2011](https://arxiv.org/abs/1104.3863)).

## Trojan populations

### What is and is not constrained

The current extrasolar searches target **massive co-orbital companions**, not asteroid-like swarms. A radial-velocity-informed transit analysis of 95 planets around low-mass stars found one 3-sigma candidate and 25 systems merely compatible at 1 sigma, and concluded that existing data mainly rule out companions more massive than Saturn; the authors state that the data do not yet strongly constrain co-orbital occurrence ([Balsalobre-Ruza et al. 2024](https://doi.org/10.1051/0004-6361/202450717)). Those limits must not be converted into the occurrence or total mass of a `TrojanPopulation` made of planetesimals.

Solar-System observations demonstrate that small bodies can occupy a 1:1 mean-motion resonance around the leading and trailing triangular Lagrange regions. A Subaru survey detected 189 objects in Jupiter's trailing L5 swarm, selected an unbiased 87-object sample in the 2–10 km range, and found that the L4 and L5 size distributions agree over a wide size range while the estimated number ratio above 2 km is \(N_{L4}/N_{L5}=1.40\pm0.15\) ([Uehata et al. 2022](https://doi.org/10.3847/1538-3881/ac5b6d)). This is evidence for the possible architecture and a Solar-System template, but applying Jupiter's size distribution or L4/L5 asymmetry to every extrasolar planet would be a proxy rather than an empirical extrasolar model.

In the restricted three-body problem the triangular equilibrium is linearly stable for primary-secondary mass ratio \(\mu=m_2/(m_1+m_2)\lesssim0.0385\), but additional resonances and perturbing planets structure and erode the stable phase space ([Schwarz et al. 2014](https://doi.org/10.1093/mnras/stu1279)). The mass-ratio test is therefore an eligibility gate only. A generated swarm must also pass the simulator's full architecture check or an integration-based resonance/stability test.

### Recommended Trojan-generation contract

1. Keep `TrojanPopulation` as a statistical planetesimal population attached to a specific planet's 1:1 resonance. Do not represent planet-mass co-orbitals with this type; they need a separate orbital-architecture decision because current surveys constrain a different observable ([Balsalobre-Ruza et al. 2024](https://doi.org/10.1051/0004-6361/202450717)).
2. Draw occurrence through a `PhysicalProxy` tied to the existence and dynamical history of a planetesimal reservoir, planetary mass/growth and migration history. If those formation-history fields do not exist and a free probability is used, label the occurrence `Speculative`.
3. Place swarm centres around both L4 and L5, with libration amplitude, eccentricity and inclination rather than exact point locations. A Jupiter-derived size distribution or L4/L5 asymmetry is acceptable only as an explicitly identified Solar-System analogue proxy ([Uehata et al. 2022](https://doi.org/10.3847/1538-3881/ac5b6d)).
4. Apply the \(\mu\lesssim0.0385\) eligibility test, then test the 1:1 resonant angle and survival in the complete generated architecture. Reject the population if nearby planets, stellar companions or secular resonances erase the stable region ([Schwarz et al. 2014](https://doi.org/10.1093/mnras/stu1279)).
5. Materialise only notable Trojan bodies. Store the remaining swarm statistically as total mass, size-frequency parameters, L4/L5 partition and orbital-dispersion parameters.

## Cross-system plausibility rule

All three generators should emit **candidates**, not guaranteed decorations. The accepted order is:

```text
host-specific occurrence draw
-> formation-channel / morphology proxy
-> candidate bodies or statistical population
-> Roche, Hill, tidal, resonance and mutual-overlap checks
-> full stellar-system perturbation check
-> accept, or reject with a retained reason
```

The evidence label describes how a value was obtained; it never exempts that value from plausibility checks. A `Speculative` ring or captured moon can survive only if it obeys the same known physical and dynamical constraints as an empirically seeded candidate.

## Sharp follow-up decisions surfaced

1. **Choose satellite formation channels and host eligibility.** Decide which planet classes can invoke regular-disc, giant-impact, and capture channels, and which channels are in the first implemented slice.
2. **Choose proxy priors and uncertainty propagation.** Fix distributions for circumplanetary solid budget, planetary \(Q_p\), Love number, ring optical depth/lifetime, and unknown migration history; each materially controls survival.
3. **Define the small-body materialisation boundary.** Decide when a moon, ring moonlet, or Trojan becomes an individual body versus remaining part of a statistical population.
4. **Define the system-wide validation contract.** Specify integration duration/criterion, analytical fallbacks, rejection reasons, and whether bounded repositioning is allowed for proxy or speculative candidates.
5. **Separate planet-mass co-orbitals from Trojan populations.** Decide whether planet-mass 1:1 companions enter the destination at all and, if so, where they sit in the orbital hierarchy.

## Remaining fog

- A defensible general occurrence rate for small moons around each planet class does not yet exist in the sources above.
- Terrestrial-planet moon occurrence, captured-moon occurrence, and irregular-satellite population distributions need separate research after formation channels and supported host classes are chosen.
- Ring rates below \(2R_p\), for misaligned rings, and for long-period planets remain poorly constrained by the cited survey.
- Asteroid-scale extrasolar Trojan occurrence and mass distributions remain observationally unconstrained; they may become sharper only after the plan defines a shared planetesimal-reservoir and migration-history model.
