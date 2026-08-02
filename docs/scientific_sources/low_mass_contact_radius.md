# Low-mass radius proxy for geometric contact rejection

## Scope

This note defines a deliberately narrow fallback for generated companions with
`0.08 <= M/M_sun < 0.10` that fall below the bundled MIST grid. Its only purpose
is rejecting stellar-orbit candidates whose periastron would put two finite
bodies in contact. It must not create a stellar-evolution snapshot, luminosity,
effective temperature, surface gravity, spectral type, or planet-host state.

## Primary model anchor

The solar-composition BHAC15 tracks include `0.08`, `0.09`, and `0.10 M_sun`
objects and tabulate radius explicitly. Values read from the authors' original
`BHAC15_tracks+structure` table are:

| age | `0.08 M_sun` | `0.09 M_sun` | `0.10 M_sun` |
|---:|---:|---:|---:|
| 1 Myr | `0.880 R_sun` | `0.961 R_sun` | `1.004 R_sun` |
| 10 Myr | `0.383 R_sun` | `0.405 R_sun` | `0.416 R_sun` |
| 50 Myr | `0.209 R_sun` | `0.221 R_sun` | `0.232 R_sun` |
| 100 Myr | `0.167 R_sun` | `0.176 R_sun` | `0.185 R_sun` |
| 1 Gyr | `0.102 R_sun` | `0.113 R_sun` | `0.124 R_sun` |
| 5 Gyr | `0.099 R_sun` | `0.113 R_sun` | `0.124 R_sun` |
| 10 Gyr | `0.099 R_sun` | `0.113 R_sun` | `0.124 R_sun` |

BHAC15 is a physical model, not a measured uncertainty distribution. This
release uses updated solar abundances but does not provide a metallicity axis,
so it cannot calibrate a general `R(M, age, [Fe/H])` relation. The strong early
contraction also shows why a Gyr-age radius cannot be applied when age is
unknown or the object is young ([Baraffe et al. 2015](https://doi.org/10.1051/0004-6361/201425481); [authors' model table](https://perso.ens-lyon.fr/isabelle.baraffe/BHAC15dir/BHAC15_tracks+structure); [authors' release notes](https://perso.ens-lyon.fr/isabelle.baraffe/BHAC15dir/READ_INFO)).

## Empirical checks near the hydrogen-burning limit

Direct eclipsing-binary measurements show both the scale and the remaining
scatter near this boundary:

- NGTS J0930-18 B: `0.0818 (+0.0040/-0.0015) M_sun` and
  `0.1059 (+0.0023/-0.0021) R_sun` ([Acton et al. 2020](https://doi.org/10.1093/mnras/staa2513)).
- EBLM J0555-57Ab: about `0.081 M_sun` and
  `0.084 (+0.014/-0.004) R_sun`; the inferred system age is
  `1.9 +/- 1.2 Gyr` and `[Fe/H] = -0.24 +/- 0.16`
  ([von Boetticher et al. 2017](https://doi.org/10.1051/0004-6361/201731107)).
- OGLE-TR-122b: `0.092 +/- 0.009 M_sun` and
  `0.120 (+0.024/-0.013) R_sun`
  ([Pont et al. 2005](https://doi.org/10.1051/0004-6361:200500025)).
- EBLM J2114-39 B: `0.0993 +/- 0.0033 M_sun` and
  `0.1250 +/- 0.0016 R_sun` at approximately solar metallicity; the authors
  find no significant inflation relative to evolutionary models
  ([Swayne et al. 2024](https://doi.org/10.1093/mnras/stae673)).

These measurements do not define a complete mass-age-metallicity distribution.
They do support treating `0.15 R_sun` as a conservative old-field upper envelope
for this limited engineering decision: it exceeds the old BHAC15 values and the
upper quoted radius of OGLE-TR-122b. The envelope is project policy inferred from
the cited measurements, not a published fit.

## Recommended fallback

For contact rejection only:

```text
if 0.08 <= mass_msun < 0.10 and age_gyr >= 1.0:
    collision_radius_rsun = 0.15
else if 0.08 <= mass_msun < 0.10 and 0.10 <= age_gyr < 1.0:
    collision_radius_rsun = 0.20
else:
    no fallback radius
```

The `0.20 R_sun` young-field envelope is the rounded-up maximum of the three
BHAC15 100-Myr values (`0.185 R_sun`) with about eight percent headroom. It must
not be used below 100 Myr: the same tracks reach roughly `0.4 R_sun` at 10 Myr
and `1 R_sun` at 1 Myr. A later implementation may interpolate a bundled BHAC15
radius table instead, but it must remain a separately named backend rather than
silently extending MIST.

Attach provenance/quality flags equivalent to:

- `LowMassContactRadiusProxy` for either envelope;
- `SolarCompositionRadiusProxy` whenever the system chemistry is not solar;
- `HydrogenBurningBoundaryAmbiguous` at the `0.08 M_sun` lower boundary;
- `BirthMassUsedAsDynamicalMass`, because no MIST current-mass snapshot exists.

The proxy should be represented as a collision radius, not as
`StellarEvolutionSnapshot.radius_rsun`. If the age is missing or below 100 Myr,
preserve `StellarRadiusUnavailable` rather than inventing a main-sequence radius.

## Architectural consequence for quadruples

A static hierarchy does not intrinsically require an evolution snapshot for
every member. Kepler conversion and point-mass hierarchy stability consume
dynamical masses and orbital elements; finite radii are needed only by the
separate periastron-contact test. The orbital seam should therefore consume a
small value object such as:

```text
OrbitalBodyGeometry {
    member_id,
    dynamical_mass_msun,
    collision_radius_rsun,
    provenance_flags,
}
```

Normal MIST-backed stars may populate it from their present-day snapshots. The
strictly bounded low-mass case may populate it from birth mass plus the envelope
above without claiming that stellar evolution was evaluated. Evolved stars and
remnants must still require their appropriate present-day mass/radius treatment;
this exception must not become a generic fallback.

Finally, supplying four radii resolves only contact rejection. Quadruple
stability remains a separate scientific problem. Dedicated four-body work finds
that reducing `2+2` and especially `3+1` quadruples to nested triple criteria can
lose important interactions; a future full classifier should preserve the
topology explicitly ([Vynatheya, Mardling & Hamers 2023](https://doi.org/10.1093/mnras/stad2410)). The existing nested screen may remain only as a named approximation with a quality flag.
