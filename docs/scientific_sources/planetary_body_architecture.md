# Scientific basis for explicit planets and small-body reservoirs

This note defines a staged path from the existing **occurrence summary** to explicit bodies. The central distinction is between an empirically constrained population and a physically complete inventory: transit, radial-velocity, microlensing, and infrared surveys observe different slices of parameter space. A generated catalogue must not silently interpret “not detected” as “does not exist”, or an occurrence measurement inside one radius-period box as a complete planetary system.

## What a present-day orbital position means

A planet's current semimajor axis is not, by itself, a composition or formation-location label. The water-ice snow line belongs to the evolving protoplanetary disc; it moves as accretion heating and the pre-main-sequence star evolve. Kennedy & Kenyon explicitly model that time dependence and find that the region able to form giant-planet cores depends on stellar mass, disc mass, and the available formation time ([Kennedy & Kenyon 2008](https://arxiv.org/abs/0710.1065)). Planet-disc migration, scattering, and tidal migration can subsequently move planets. Therefore V1 must sample **present-day observed orbits** and may report a separately named formation diagnostic, but must not decide “rocky inside the snow line, gas giant outside it”.

The repository currently has neither a protoplanetary-disc state nor a formation time. A numerical `snow_line_au` derived only from the star's present luminosity would be a different quantity and is not a defensible birth-location boundary. Snow-line history and migration remain unsupported until a disc/formation module exists.

## Planet classes are probabilistic

Radius is an observable size, not a unique composition. Rogers' hierarchical analysis found that, for close-in planets (`P < about 50 d`), most planets at `1.6 R_earth` are already too low-density to consist of iron and silicates alone ([Rogers 2015](https://arxiv.org/abs/1407.4457)). This supports a probability such as `rocky_probability`, within that calibration, rather than a hard universal boundary. A body above that transition is “volatile-rich / envelope-bearing candidate”, not automatically an ice giant or gas giant.

For V1 use observational categories:

- `SmallPlanet { radius_rearth }` with an optional calibrated `rocky_probability`;
- `GiantPlanet { minimum_mass_mjup }` in the Doppler calibration domain;
- no hard “terrestrial”, “ocean”, “ice giant”, or atmospheric-composition tag unless mass and a sourced composition model are also present.

The project's existing FGK bins (`1.0-1.7 R_earth`, `1.7-4.0 R_earth`, `10-100 d`) and M-dwarf aggregate (`1-4 R_earth`, `P < 200 d`) remain valid occurrence summaries. M dwarfs additionally receive an independently drawn sub-Earth population (`0.5-1.0 R_earth`, `0.5-18.2 d`) from the three Table 4 cells that have point estimates. The M-dwarf aggregate is **not sufficient by itself** to sample a unique radius and period: explicit candidates should use the source's completeness-corrected radius-period grid rather than an invented uniform distribution ([Dressing & Charbonneau 2015](https://arxiv.org/abs/1501.01623)). A log-uniform draw within the FGK summary bins is usable only as a clearly flagged engineering approximation, not as a measured within-bin distribution.

The implementation now represents the revised-stellar-radius point estimates from Dressing & Charbonneau Table 4 as configured radius-period cells. Cells reported only as upper limits are omitted from the conditional placement distribution. The remaining cell weights are normalized to the separately drawn `1-4 R_earth`, `P < 200 d` aggregate count, and every resulting candidate carries `MDwarfOccurrenceGridRenormalizedToAggregateCount`. Radius and period are log-uniform inside the selected cell with a separate within-bin approximation flag.

The three supported sub-Earth cells sum to `0.3039 planets/star`; this sum is sampled as its own Poisson mean and the cell weights place those additional candidates. Each carries `MDwarfSubEarthOccurrenceLimitedToMeasuredCells`. The two longer-period cells without point estimates remain explicitly ungenerated, so the sub-Earth channel is not a claim about total occurrence out to `200 d`.

## Gas giants

The existing Johnson et al. gate answers whether a host has at least one giant in its calibrated Doppler domain; it does not provide the giant's exact mass, period, or multiplicity. For FGK hosts, Cumming et al. measured

```text
dN = C M^alpha P^beta dln(M) dln(P)
alpha = -0.31 +/- 0.20
beta  =  0.26 +/- 0.10
0.3 <= M sin(i) / M_J <= 10
2 <= P / day <= 2000
```

and normalized the domain to `10.5%` of solar-type stars ([Cumming et al. 2008](https://arxiv.org/abs/0803.3357)). V1 may conditionally draw one giant from this mass-period density only where the host and occurrence gate overlap the FGK source domain. It must preserve the sampled quantity as `minimum_mass_mjup` (`M sin i`) unless an inclination model is added. The current gate means “at least one”; it does not justify drawing multiple giants. For M dwarfs and non-FGK hosts covered by the Johnson gate but not by Cumming's distribution, retain the positive summary as `properties_unresolved` rather than borrowing the FGK orbit distribution.

The semimajor axis is then derived from period and present host mass with Kepler's third law and tested against the existing S-type stability boundary. A rejected draw must be retained in generation diagnostics; repeatedly resampling until it fits would change the occurrence model conditional on binary separation.

## Multi-planet architecture

Planets in one system should not be independent points. Kepler multis show correlated sizes and regular spacing; in the CKS sample essentially no adjacent period ratios were below `1.2`, `93%` of adjacent pairs were separated by at least ten mutual Hill radii, and about twenty mutual Hill radii was most common ([Weiss et al. 2018](https://arxiv.org/abs/1706.06204)). A forward model using clustered periods and radii fits Kepler multiplicities better than independent draws ([He, Ford & Ragozzine 2019](https://arxiv.org/abs/1907.07773)).

Consequently, independent within-bin draws are acceptable only as candidate generation. Before becoming an accepted architecture, candidates require ordered periods, a sourced mass-radius draw, mutual-Hill spacing, and the stellar-companion stability screen. Resonances and Gyr stability still require later dynamical validation.

## Asteroid belts and debris discs

An “asteroid belt” is not observationally interchangeable with an infrared debris-disc detection. Infrared surveys see dust maintained by collisions among a much larger, mostly unseen planetesimal reservoir. In the unbiased Herschel DEBRIS F-K sample, excess emission was detected for `47/275`, or `17.1% (+2.6/-2.3%)`; detected discs were concentrated near fractional luminosity `~10^-5` and blackbody radii `7-40 AU` ([Sibthorpe et al. 2018](https://arxiv.org/abs/1803.00072)). This is a **bright-disc detection fraction**, not the fraction of systems containing any asteroids or planetesimals.

Resolved cold-disc radii show large dispersion and no significant luminosity trend; that argues against setting every belt radius directly from an ice line. Dust temperature also does not map to radius as a perfect blackbody because grain properties matter ([Pawellek et al. 2014](https://arxiv.org/abs/1407.4579)).

A defensible later belt object is therefore a statistical reservoir, not millions of explicit asteroids:

```text
DebrisBeltProxy {
    host_member_id,
    blackbody_radius_au,
    fractional_dust_luminosity,
    source_domain,
    quality_flags: [InfraredDetectableDustProxy, PhysicalRadiusUnresolved],
}
```

It may initially be generated only for supported F-K main-sequence hosts from the joint empirical distribution in the DEBRIS analysis. Its radius must also fit within the stellar stability zone and avoid already occupied planet-crossing regions under a later planet-belt dynamics rule. V1 should **not** infer a belt from a gap between planets, force a belt at the snow line, generate individual asteroid sizes, or treat the `17.1%` detection fraction as total planetesimal incidence. M-dwarf debris belts require a separate sensitivity-aware calibration.

## Dwarf planets, comets, moons, and other bodies

Dwarf planets and comets are members of latent planetesimal reservoirs. Current exoplanet surveys do not supply a general per-star occurrence and orbital distribution that can populate them individually. They should remain aggregate `PlanetesimalReservoir` metadata until a formation/evolution model is chosen.

Exomoons also remain unsupported in V1. A moon requires planet mass, planet radius, the planet's Hill region, tidal history, and a satellite occurrence model; the current stellar S-type boundary does not establish satellite stability. Rings, Trojans, interstellar captures, rogue planets, and post-main-sequence survivors likewise need separate models and must not be added as decorative random classes.

## Recommended staged implementation

### Explicit planets V1

1. Preserve the existing per-host occurrence summary as the authoritative gate.
2. Generate stable IDs from global seed, system ID, host-member ID, channel, and candidate index.
3. FGK small planets: draw radius and period in the existing bins, initially log-uniform with `WithinBinDistributionApproximation`, or preferably import the published occurrence grid.
4. M-dwarf small planets: do not expand the aggregate count until the radius-period grid is represented in configuration.
5. FGK giants: if the existing gate is positive, draw one `M sin i` and period from the Cumming density in its exact domain; otherwise keep properties unresolved.
6. Derive semimajor axis by Kepler's law.
7. Reject candidates outside the conservative S-type policy boundary without replacement and record the reason.
8. Store radius-based class probabilities and provenance; do not assert bulk composition.

### Architecture V2

Add probabilistic masses, correlated period/radius clusters, mutual-Hill spacing, eccentricity/inclination distributions, and optional N-body validation. This is where an independent candidate list becomes a coherent multi-planet system.

### Small bodies V3

Add an observable `DebrisBeltProxy` for a narrowly calibrated F-K domain. Keep unseen asteroid/comet inventories, individual dwarf planets, and moons unsupported until dedicated models exist.

## V1 non-claims

Explicit-planets V1 does not claim a complete system, formation positions, migration histories, atmospheric chemistry, surface type, habitability, individual small bodies, moons, or long-term N-body stability. It produces a reproducible present-day realization inside explicitly observed parameter domains and exposes missing coverage as typed data.
