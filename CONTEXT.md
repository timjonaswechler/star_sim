# Stellar Population Simulation

This context describes the generated stellar population and the spatial region in which it exists.

## Galactic environment

**Galactic Position**:
A position relative to the centre and reference plane of the modelled galaxy.
_Avoid_: World position, Bevy position

**Stellar Population**:
A group of stars sharing a statistical formation history, spatial distribution, age distribution, and chemical composition.
_Avoid_: Star type, region type

**Stellar Age**:
The elapsed time since a star formed; it describes how long the star has already existed, not how long it can survive.
_Avoid_: Lifetime, evolutionary state

**Stellar Lifetime**:
The duration for which a star of a given initial mass and composition remains in a specified evolutionary phase.
_Avoid_: Age

**Main-Sequence Lifetime**:
The duration from sustained core hydrogen burning until central hydrogen exhaustion for a star of a given initial mass and initial composition.
_Avoid_: Total lifetime, stellar age

**Evolutionary State**:
A star's current physical phase, such as main sequence, red giant, white dwarf, or neutron star.
_Avoid_: Age, spectral type

**Subgiant and Red-Giant Branch**:
The luminous post-main-sequence interval after central hydrogen exhaustion and before stable core-helium burning. The current track phase does not separate subgiants from red-giant-branch stars.
_Avoid_: Main sequence, asymptotic giant branch

**Core-Helium-Burning State**:
The phase in which helium fusion supplies the stellar core's principal nuclear energy source.
_Avoid_: Red-giant branch, helium abundance

**Asymptotic Giant Branch (AGB)**:
A luminous late phase with an inert carbon-oxygen core and nuclear burning in surrounding shells. Early and thermally pulsing AGB are distinct states.
_Avoid_: Red-giant branch, post-AGB

**Post-AGB State**:
The short transition after the asymptotic giant branch and before entry onto a white-dwarf cooling sequence.
_Avoid_: White dwarf, thermally pulsing AGB

**White-Dwarf Handoff**:
The model boundary at which a post-AGB stellar core becomes a cooling white dwarf. Cooling age is measured from this boundary, not from stellar formation.
_Avoid_: TAMS, track end

**White-Dwarf Cooling Age**:
The elapsed time since the white-dwarf handoff. It is distinct from the object's total age since formation.
_Avoid_: Stellar age, main-sequence lifetime

**Evolution-Track Branch**:
The physically distinct late-evolution route followed by a source track, such as a white-dwarf-progenitor branch or a massive-burning branch.
_Avoid_: Stellar population, arbitrary mass bin

**Track Termination**:
The last physical state actually supplied by an evolution track. It does not by itself identify a compact remnant.
_Avoid_: Stellar death, remnant type

**Current Stellar Mass**:
The mass retained by a stellar object at the evaluated age after winds, mass transfer, eruptions, or remnant formation.
_Avoid_: Initial stellar mass

**Stellar Remnant**:
A compact object left after nuclear-burning stellar evolution, such as a white dwarf, neutron star, or black hole.
_Avoid_: Main-sequence star, dead star

**Evolution Track**:
A sequence of physically corresponding stellar states for one initial mass and composition, indexed here by MIST equivalent evolutionary points.
_Avoid_: Orbit, population history

**Model Coverage**:
The explicit mass, composition, age, and phase domain supported by the bundled scientific data. Inputs outside it produce a typed unsupported result rather than a clamped estimate.
_Avoid_: Probability, confidence

**Iron Abundance ([Fe/H])**:
The logarithmic ratio of iron to hydrogen relative to the Sun, used here as one coordinate of stellar chemical composition.
_Avoid_: Total metallicity, [M/H]

**Alpha Enhancement ([alpha/Fe])**:
The logarithmic abundance of alpha-capture elements relative to iron and to the corresponding solar ratio. It distinguishes enrichment histories that can share the same iron abundance.
_Avoid_: Total metallicity, iron abundance

**Global Metallicity ([M/H])**:
The logarithmic abundance of all elements heavier than helium relative to hydrogen and to the Sun. It is derived from iron abundance and alpha enhancement in the current model.
_Avoid_: Iron abundance, metal mass fraction

**Metal Mass Fraction (Z)**:
The fraction of a star's initial mass contributed by elements heavier than helium.
_Avoid_: [Fe/H], [M/H]

**Helium Mass Fraction (Y)**:
The fraction of a star's initial mass contributed by helium.
_Avoid_: Alpha enhancement, metal mass fraction

**Stellar Chemistry**:
A coherent initial chemical composition containing iron abundance, alpha enhancement, global metallicity, metal mass fraction, and helium mass fraction.
_Avoid_: Metallicity when referring to the complete composition

**Region**:
A bounded volume around a galactic position for which stellar systems are requested or materialised.
_Avoid_: Galaxy sector, scene

**Stellar Catalog**:
The coherent generated state of all stellar systems and stellar members inside the local 10-parsec region, including their shared population history and each member's present evolutionary outcome.
_Avoid_: Birth-mass sample, parallel result lists

## Stellar systems

**Initial Mass Function (IMF)**:
A population-level probability distribution for stellar masses at formation. It describes an ensemble of births, not the present-day masses of surviving stars.
_Avoid_: Present-day mass function, lifetime distribution

**Initial Stellar Mass**:
The mass of a stellar object when it forms, expressed relative to the Sun. It remains an immutable input to stellar evolution even if the object's current mass later changes.
_Avoid_: Current mass, system mass

**Primary Star**:
The initially most massive stellar member used to condition a system's multiplicity and companion masses.
_Avoid_: Central star, barycentre

**Companion Mass Ratio**:
The ratio of a companion's initial mass to the primary's initial mass, constrained to be no greater than one.
_Avoid_: Binary fraction, orbital mass ratio

**Stellar System**:
A gravitationally bound system containing at least one star and possibly additional stars, planets, or smaller bodies.
_Avoid_: Solar system, planet system

**Planetary System**:
The planets and smaller bodies gravitationally bound within a stellar system.
_Avoid_: Stellar system

**Planet Occurrence**:
An empirically calibrated expectation or probability within a stated host-star, planet-size, and orbital-period domain.
_Avoid_: Universal planet probability, complete planetary system

**Planet Population Summary**:
A deterministic draw of occurrence counts or host fractions in explicitly calibrated observational domains; it does not yet assign physical planets or orbits.
_Avoid_: Planetary system, orbital architecture

**Orbital Node**:
A member of a hierarchical orbital arrangement that is either a physical body or a barycentre.
_Avoid_: Satellite, child entity

**Barycentre**:
The shared centre of mass about which two or more members of an orbital arrangement move.
_Avoid_: Central body, root object
