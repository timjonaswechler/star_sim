# Evidence models for planetesimal reservoirs and debris disks

## Research question

Which primary observational and physical models can support occurrence,
placement, evolution, and observables for `Asteroid Belt`, `Outer Planetesimal
Belt`, `Comet Reservoir`, notable `Dwarf Planet` members, and `Debris Disk`, and
where must the simulator use marked physical proxies or decorative variance?

## Recommended decision

Keep `Planetesimal Reservoir` and `Debris Disk` as separate, causally linked
domain objects. A reservoir is a latent population of solid bodies; a debris
disk is the current dust observable produced from one or more reservoirs.
Far-infrared detection fractions can calibrate a *survey-detectable debris-disk
claim*, but cannot directly calibrate universal reservoir occurrence: Herschel
DEBRIS detected excess around 47 of 275 F--K stars
(`17.1% +2.6/-2.3%`), while its M-star subsample detected 2 of 94
(`2.1% +2.7/-0.7%`) and was roughly ten times shallower in fractional-luminosity
versus radius space than the FGK sample.[Sibthorpe et al. 2018](https://doi.org/10.1093/mnras/stx3188)
[Lestrade et al. 2025](https://arxiv.org/abs/2502.04441)

Adopt this evidence boundary:

| Generated claim | Evidence level | Recommended model |
|---|---|---|
| A cold dust excess would be detectable in the DEBRIS survey domain | `Empirical` | Sample a survey-specific detection channel by host class, preserving wavelength, threshold, and completeness provenance. The measured rates are at least `24 +/- 5%` at 100 micrometres for A stars and `17.1% +2.6/-2.3%` across F--K stars.[Thureau et al. 2014](https://doi.org/10.1093/mnras/stu1864) [Sibthorpe et al. 2018](https://doi.org/10.1093/mnras/stx3188) |
| A warm/habitable-zone dust signal and its level | `Empirical` only inside HOSTS coverage | Draw the observable in zodis from a fitted luminosity distribution, not an `Asteroid Belt` occurrence. The completed 38-star HOSTS survey had 10 significant excesses; its Sun-like subsample had a best-fit median of 3 zodis and about 20% were substantially dustier.[Ertel et al. 2020](https://doi.org/10.3847/1538-3881/ab7817) |
| Existence and initial mass of an unseen planetesimal reservoir | `PhysicalProxy` | Draw a latent reservoir, evolve it, derive dust, and forward-model detectability. Do not relabel a non-detection as absence: the DEBRIS M-star comparison demonstrates that raw detection rates change with physical sensitivity.[Lestrade et al. 2025](https://arxiv.org/abs/2502.04441) |
| Radius of a bright, resolved outer belt | `Empirical` conditional model, otherwise `PhysicalProxy` | Use the resolved-belt relation `R = 73 (+6/-6) au * (L_star/L_sun)^(0.19 +/- 0.04)` with about 17% intrinsic scatter only for the selected population it describes; attach the source domain and selection warning.[Matrà et al. 2018](https://doi.org/10.3847/1538-4357/aabcc4) |
| Belt width, vertical thickness, and dynamical excitation | `Empirical` conditional model, otherwise `PhysicalProxy` | Sample from the resolved REASONS population rather than assuming every belt is a narrow ring. Its 74 resolved belts are selection-biased; most are broad, and 24 measured vertical aspect ratios imply inclination dispersions of roughly 1--20 degrees.[Matrà et al. 2025](https://arxiv.org/abs/2501.09058) |
| Present dust mass or fractional luminosity from age | `PhysicalProxy` | Use a collisional-cascade evolution model. In the Wyatt model, mass stays near its initial value until the largest bodies enter collisional equilibrium and then falls approximately as `1/t`; REASONS independently finds faster depletion for smaller-radius belts.[Wyatt et al. 2007](https://doi.org/10.1086/518404) [Matrà et al. 2025](https://arxiv.org/abs/2501.09058) |
| Asteroid-belt inventory and size-frequency distribution | `PhysicalProxy` (Solar-System calibrated) | Store a statistical distribution, not individual asteroids. The main-belt distribution is shaped by collisions, resonant leakage, chaotic diffusion, and Yarkovsky delivery, so a static single power law is not a scientifically faithful universal model.[Bottke et al. 2005a](https://doi.org/10.1016/j.icarus.2004.10.026) [Bottke et al. 2005b](https://doi.org/10.1016/j.icarus.2005.04.001) |
| Outer-belt inventory and upper-tail bodies | `PhysicalProxy` (Solar-System calibrated) | Use a debiased TNO absolute-magnitude/size distribution and materialise only its notable upper tail. OSSOS estimates about 30,000 non-resonant main-belt objects with `H_r < 8.3` and a total main-belt mass near `0.014 M_earth`; those numbers describe the Solar System, not exosystem occurrence.[Petit et al. 2023](https://doi.org/10.3847/2041-8213/acc525) |
| Scattered comet-supplying population | `PhysicalProxy` | Derive it from an outer reservoir plus scattering planets. The debiased scattering-TNO distribution requires a knee or divot and is large enough in the studied models to supply Jupiter-family comets.[Shankman et al. 2016](https://arxiv.org/abs/1511.02896) |
| Distant Oort-cloud analogue | `PhysicalProxy` only with formation context; otherwise `Speculative` or `Unsupported` | Require a scattering architecture and a modelled birth/field environment. Solar-System integrations show that cluster environment changes inner-cloud loading and outer-cloud trapping, while late-instability models still struggle to reproduce the observed Oort-cloud/scattered-disc population ratio.[Kaib & Quinn 2008](https://doi.org/10.1016/j.icarus.2008.03.020) [Brasser & Morbidelli 2013](https://doi.org/10.1016/j.icarus.2013.03.012) |
| A notable member is called a `Dwarf Planet` | `PhysicalProxy` taxonomy | Draw it from the reservoir's upper tail, then evaluate roundness and lack of dynamical dominance. The IAU 2006 definition is explicitly Solar-System wording; an exosystem use is therefore an extension, while Margot supplies a quantitative orbit-clearing test based on host mass, body mass, and period.[IAU Resolution 5A (2006)](https://www.iau.org/static/resolutions/Resolution_GA26-5-6.pdf) [Margot 2015](https://doi.org/10.1088/0004-6256/150/6/185) |

The simulator may use `Speculative` decorative variation for fine morphology
that the evidence does not fix--for example a bounded clump phase, minor radial
substructure, a notable body's rotation phase, or albedo within a
composition-compatible interval. Decorative draws must not decide whether a
major reservoir exists, inflate its mass beyond the selected proxy, create dust
without a parent reservoir, or bypass stability. Resolved systems show genuine
diversity, but unresolved photometry has a radius--temperature degeneracy and
even resolved samples are selected for bright emission; diversity supports
bounded variation, not an empirical universal distribution.[Booth et al. 2013](https://arxiv.org/abs/1210.0547)
[Matrà et al. 2025](https://arxiv.org/abs/2501.09058)

## Evidence by model responsibility

### 1. Occurrence is an observable-selection problem

The unbiased DEBRIS A-star sample found 21/86 excesses at 100 micrometres
(`24 +/- 5%`), and the F--K analysis found 47/275 (`17.1% +2.6/-2.3%`). Both
are detection rates at particular wavelengths and sensitivities rather than
complete inventories of planetesimals.[Thureau et al. 2014](https://doi.org/10.1093/mnras/stu1864)
[Sibthorpe et al. 2018](https://doi.org/10.1093/mnras/stx3188)

The M-star DEBRIS result is the decisive counterexample to treating those
percentages as universal reservoir occurrence: only two disks were detected,
but degrading the K-star sample to the same physical sensitivity made its rate
statistically consistent with the M-star result.[Lestrade et al. 2025](https://arxiv.org/abs/2502.04441)
The model should therefore preserve at least:

- `observable_kind` and observing wavelength;
- source survey and host-star coverage;
- detection threshold/completeness domain;
- measured detection state separately from latent reservoir presence.

HOSTS constrains a distinct warm-dust observable. It measured 38 nearby main
sequence stars with a median sensitivity of 23 zodis for early types and 48
zodis for Sun-like stars, found 10 significant excesses, and found a clear
association with already-known cold dust.[Ertel et al. 2020](https://doi.org/10.3847/1538-3881/ab7817)
It therefore supports a correlated warm-dust channel, but it does not establish
that each warm excess is a bounded rocky `Asteroid Belt`.

### 2. Placement follows formation proxies, then whole-system stability

For an outer belt with matching source coverage, Matrà et al. fitted 26
millimetre-resolved systems with
`R = 73 (+6/-6) au * (L_star/L_sun)^(0.19 +/- 0.04)` and about 17% intrinsic
scatter.[Matrà et al. 2018](https://doi.org/10.3847/1538-4357/aabcc4)
The paper explicitly tested selection effects, so the relation should retain a
coverage marker rather than become a hard universal snow-line law.[Matrà et al. 2018](https://doi.org/10.3847/1538-4357/aabcc4)

The larger 74-belt REASONS sample is not an occurrence sample: it combines a
flux-limited follow-up with archival resolved targets. Only 9/74 belts have
radii below 60 au, and the analysis reports selection effects that generally
favour smaller and/or more massive belts.[Matrà et al. 2025](https://arxiv.org/abs/2501.09058)
It is nevertheless the better conditional source for belt width, aspect ratio,
inclination dispersion, and radius-dependent collisional evolution.

Placement must be clipped or rejected against the accepted stellar and
planetary architecture rather than merely against a sampled mean radius.
Holman and Wiegert's test-particle integrations provide bounded S-type and
P-type critical semimajor axes over their stated binary eccentricity and mass
ratio domains; their integrations cover `10^4` binary periods and do not prove
permanent stability.[Holman & Wiegert 1999](https://doi.org/10.1086/300695)
Observed debris statistics also show a lack of detected disks for binary
separations around 25--135 au, while radius inference, orbital projection, and
stability-boundary uncertainty make individual claims less certain.[Yelverton et al. 2019](https://doi.org/10.1093/mnras/stz1927)

Recommended placement algorithm:

1. Obtain stable circumstellar or circumbinary intervals from the accepted
   `Stellar Orbital Hierarchy` and `Planetary Architecture`.
2. Draw an evidence-conditioned candidate centre and width.
3. Subtract planet-clearing, resonance, and already-occupied exclusion zones.
4. Accept a clipped interval only when the source model permits truncation;
   otherwise reject the candidate and retain the rejection reason.
5. Never resample an empirical observable silently until a belt fits.

### 3. Evolve bodies first and derive dust second

Wyatt et al.'s analytical steady-state model reproduces A-star Spitzer trends
with a collisional population in which disk mass eventually declines as
approximately `1/t`; it also identifies systems too luminous for the model as
possible transient or atypical cases.[Wyatt et al. 2007](https://doi.org/10.1086/518404)
Sibthorpe et al. similarly fit the F--K excess population with steady-state
evolution but could not explain the unusually hot and brightest outliers with
that model.[Sibthorpe et al. 2018](https://doi.org/10.1093/mnras/stx3188)

REASONS adds resolved evidence that dust mass depletes with age in a
radius-dependent way and that most observed belts are broad rather than narrow
rings.[Matrà et al. 2025](https://arxiv.org/abs/2501.09058)
The first implementation should therefore store model parameters, not only a
final opacity:

- initial and current reservoir mass;
- inner/outer radius or centre plus width;
- size-frequency model and maximum body scale;
- eccentricity/inclination dispersion or vertical aspect ratio;
- collisional age and stirring state;
- current dust mass, fractional luminosity, temperature, and modified-blackbody
  parameters;
- a link from every `Debris Disk` component to its parent reservoir(s).

The modified-blackbody observable should keep blackbody radius distinct from
physical radius. Herschel A-star modelling uses one or two modified-blackbody
components, and resolved A-star disks show physical-to-blackbody radius ratios
between roughly 1 and 2.5 because grain properties affect temperature.[Thureau et al. 2014](https://doi.org/10.1093/mnras/stu1864)
[Booth et al. 2013](https://arxiv.org/abs/1210.0547)

### 4. Asteroid belts, outer belts, and notable dwarf planets are statistical populations

The Solar System provides resolved inventories but not exosystem occurrence
rates. Main-belt models require collisional disruption plus dynamical loss;
Bottke et al. constrain them against the observed asteroid size distribution,
asteroid families, Vesta, meteorite exposure ages, and cratering records.[Bottke et al. 2005b](https://doi.org/10.1016/j.icarus.2005.04.001)
This supports a rocky-reservoir size-frequency proxy, with an explicit
`solar_system_transfer` provenance tag, rather than individually materialising
millions of asteroids.

For an outer reservoir, the OSSOS debiased model is a stronger basis for the
large-body tail. It distinguishes dynamically hot and cold components and finds
large bodies in the hot population that are absent from the cold population at
the bright end.[Petit et al. 2023](https://doi.org/10.3847/2041-8213/acc525)
Consequently, `Notable Dwarf Planets` should be sampled *after* reservoir mass,
dynamical component, and size distribution; they must not be an independent
Bernoulli decoration.

The IAU dwarf-planet definition requires near-roundness, failure to clear the
orbital neighbourhood, and non-satellite status, but is worded for bodies
orbiting the Sun.[IAU Resolution 5A (2006)](https://www.iau.org/static/resolutions/Resolution_GA26-5-6.pdf)
An extrasolar implementation should mark its generalized classification as a
proxy and can use Margot's quantitative clearing metric where host mass, body
mass, and period are available.[Margot 2015](https://doi.org/10.1088/0004-6256/150/6/185)
Roundness remains composition-dependent, so an unmeasured composition must not
produce an unqualified empirical dwarf-planet label.

### 5. Comet reservoirs need two different evidence levels

A scattering/outer-disk `Comet Reservoir` can use the Solar System's debiased
scattering-TNO population as a proxy. Shankman et al. reject a single-slope
absolute-magnitude distribution in favour of a knee or divot and obtain a
population capable of supplying the Jupiter-family comet flux in their tested
model.[Shankman et al. 2016](https://arxiv.org/abs/1511.02896)
Transient Ca II absorption supplies direct evidence that star-grazing exocomets
occur in some selected A/B debris-disk systems, but the targeted sample cannot
provide a general occurrence rate.[Welsh & Montgomery 2015](https://doi.org/10.1155/2015/980323)

A distant Oort-cloud analogue is more weakly constrained. Kaib and Quinn's
4.5-Gyr integrations show that the first 100 Myr in a cluster preferentially
loads the inner cloud and changes trapping efficiency.[Kaib & Quinn 2008](https://doi.org/10.1016/j.icarus.2008.03.020)
Brasser and Morbidelli's late-instability calculation finds that simultaneous
Solar-System Oort-cloud and scattered-disk models can underproduce their
inferred population ratio.[Brasser & Morbidelli 2013](https://doi.org/10.1016/j.icarus.2013.03.012)
Thus:

- a scattered comet reservoir may be `PhysicalProxy` when an outer belt and
  scattering planets exist;
- a distant comet cloud may be `PhysicalProxy` only when birth environment,
  Galactic tide, stellar encounters, and giant-planet scattering are represented;
- before those inputs exist, a rare bounded cloud may be `Speculative`, or the
  result should be typed `Unsupported`; it must not be called empirical.

## Whole-system generation contract

Use one deterministic generation pass with per-property evidence, in this
order:

```text
accepted stellar + planetary architecture
  -> available stable orbital domains
  -> latent reservoir candidates
  -> bounded placement and whole-system rejection
  -> reservoir mass + statistical size distribution
  -> age/stirring/collisional evolution
  -> notable upper-tail bodies
  -> dust production and thermal observable
  -> survey-specific detectability claims
```

An object can legitimately mix evidence levels: an outer belt's observable
excess may be `Empirical`, its latent initial mass `PhysicalProxy`, and a minor
azimuthal clump `Speculative`. The summary evidence level is the weakest level
among its displayed claims, while the source of truth remains per property.

Every candidate must retain:

- the seed stream/key that produced it;
- source and calibrated input domain;
- evidence level per property;
- accepted, rejected, or unsupported status;
- violated stability/coverage constraints;
- bounded-attempt count for proxy or decorative placement.

## Unsupported coverage that must remain explicit

- There is no primary survey here that measures universal occurrence of
  asteroid belts, outer planetesimal reservoirs, or Oort-cloud analogues;
  far-infrared and interferometric studies measure dust observables with
  selection functions.[Sibthorpe et al. 2018](https://doi.org/10.1093/mnras/stx3188)
  [Ertel et al. 2020](https://doi.org/10.3847/1538-3881/ab7817)
- The REASONS sample is appropriate for conditional morphology, not
  unconditional occurrence, because it is assembled from detected,
  millimetre-resolved targets and flux-limited follow-up.[Matrà et al. 2025](https://arxiv.org/abs/2501.09058)
- Solar-System size distributions do not establish how reservoir mass or
  dynamical components scale with host type, metallicity, stellar multiplicity,
  or planetary architecture.[Bottke et al. 2005a](https://doi.org/10.1016/j.icarus.2004.10.026)
  [Petit et al. 2023](https://doi.org/10.3847/2041-8213/acc525)
- The cited debris-disk occurrence channels mainly cover main-sequence A--M
  stars. Applying them to giants or stellar remnants would be unsupported by
  these sources.[Thureau et al. 2014](https://doi.org/10.1093/mnras/stu1864)
  [Lestrade et al. 2025](https://arxiv.org/abs/2502.04441)

## Newly surfaced decisions

These questions are sharp enough for new Wayfinder tickets:

1. **Choose the latent-reservoir calibration and forward selection function.**
   Decide whether V1 draws survey-detectable dust first and backfills a proxy
   parent reservoir, or fits a latent mass/radius distribution whose synthetic
   observables reproduce DEBRIS and HOSTS detection statistics.
2. **Choose the reservoir stability and truncation contract.** Decide how
   finite-width circumstellar/circumbinary belts interact with stellar critical
   semimajor axes, planet-clearing zones, resonances, clipping, and rejection.
3. **Choose the first collisional-evolution fidelity.** Decide between the
   Wyatt analytical steady-state proxy and a more detailed size-bin cascade,
   including which transient-bright systems remain unsupported.
4. **Choose the notable-body materialisation rule and extrasolar dwarf-planet
   taxonomy.** Decide the mass/size threshold, maximum materialised count,
   roundness proxy, and quantitative dynamical-dominance test.
5. **Choose whether distant comet clouds wait for birth-environment modelling.**
   Decide whether V1 returns `Unsupported` or permits a rare `Speculative`
   cloud behind scattering-architecture and stability gates.

The following remain fog rather than sharp tickets until the planetary
architecture and chemical-composition decisions advance:

- how belt chemistry and volatile fractions inherit the protoplanetary snow
  lines and stellar chemistry;
- how migration history and giant-planet instability correlate the inner,
  outer, and comet reservoirs;
- which decorative surface, colour, and naming properties notable bodies expose
  in visualisation.

## Primary sources

- Booth et al. (2013), *Resolved Debris Discs Around A Stars in the Herschel
  DEBRIS Survey*, [arXiv:1210.0547](https://arxiv.org/abs/1210.0547).
- Bottke et al. (2005a), *The fossilized size distribution of the main asteroid
  belt*, [doi:10.1016/j.icarus.2004.10.026](https://doi.org/10.1016/j.icarus.2004.10.026).
- Bottke et al. (2005b), *The collisional and dynamical evolution of the
  main-belt and NEA size distributions*,
  [doi:10.1016/j.icarus.2005.04.001](https://doi.org/10.1016/j.icarus.2005.04.001).
- Brasser & Morbidelli (2013), *Oort cloud and Scattered Disc formation during
  a late dynamical instability in the Solar System*,
  [doi:10.1016/j.icarus.2013.03.012](https://doi.org/10.1016/j.icarus.2013.03.012).
- Ertel et al. (2020), *The HOSTS survey for exozodiacal dust: Observational
  results from the complete survey*,
  [doi:10.3847/1538-3881/ab7817](https://doi.org/10.3847/1538-3881/ab7817).
- Holman & Wiegert (1999), *Long-Term Stability of Planets in Binary Systems*,
  [doi:10.1086/300695](https://doi.org/10.1086/300695).
- International Astronomical Union (2006), *Resolution 5A: Definition of a
  Planet in the Solar System*,
  [official resolution PDF](https://www.iau.org/static/resolutions/Resolution_GA26-5-6.pdf).
- Kaib & Quinn (2008), *The formation of the Oort cloud in open cluster
  environments*,
  [doi:10.1016/j.icarus.2008.03.020](https://doi.org/10.1016/j.icarus.2008.03.020).
- Lestrade et al. (2025), *Debris disks around M dwarfs: The Herschel DEBRIS
  survey*, [arXiv:2502.04441](https://arxiv.org/abs/2502.04441).
- Margot (2015), *A Quantitative Criterion for Defining Planets*,
  [doi:10.1088/0004-6256/150/6/185](https://doi.org/10.1088/0004-6256/150/6/185).
- Matrà et al. (2018), *An Empirical Planetesimal Belt Radius--Stellar
  Luminosity Relation*,
  [doi:10.3847/1538-4357/aabcc4](https://doi.org/10.3847/1538-4357/aabcc4).
- Matrà et al. (2025), *REsolved ALMA and SMA Observations of Nearby Stars
  (REASONS): A population of 74 resolved planetesimal belts at millimetre
  wavelengths*, [arXiv:2501.09058](https://arxiv.org/abs/2501.09058).
- Petit et al. (2023), *The hot main Kuiper belt size distribution from OSSOS*,
  [doi:10.3847/2041-8213/acc525](https://doi.org/10.3847/2041-8213/acc525).
- Shankman et al. (2016), *OSSOS II: A Sharp Transition in the Absolute
  Magnitude Distribution of the Kuiper Belt's Scattering Population*,
  [arXiv:1511.02896](https://arxiv.org/abs/1511.02896).
- Sibthorpe et al. (2018), *Analysis of the Herschel DEBRIS Sun-like star
  sample*, [doi:10.1093/mnras/stx3188](https://doi.org/10.1093/mnras/stx3188).
- Thureau et al. (2014), *An unbiased study of debris discs around A-type stars
  with Herschel*, [doi:10.1093/mnras/stu1864](https://doi.org/10.1093/mnras/stu1864).
- Welsh & Montgomery (2015), *The Appearance and Disappearance of Exocomet Gas
  Absorption*, [doi:10.1155/2015/980323](https://doi.org/10.1155/2015/980323).
- Wyatt et al. (2007), *Steady-state evolution of debris disks around A stars*,
  [doi:10.1086/518404](https://doi.org/10.1086/518404).
- Yelverton et al. (2019), *A statistically significant lack of debris discs in
  medium separation binary systems*,
  [doi:10.1093/mnras/stz1927](https://doi.org/10.1093/mnras/stz1927).
