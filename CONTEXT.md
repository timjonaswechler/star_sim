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
The planets, dwarf planets, natural satellites, rings, and small-body populations gravitationally bound within a stellar system. A planetary system may be circumstellar or circumbinary and is not necessarily a complete inventory of every individual body.
_Avoid_: Stellar system

**Planetary Architecture**:
The present-day orbital organization of all planetary bodies, collective orbital structures, and small-body populations within a Stellar System. Circumstellar and circumbinary host scopes may coexist in one architecture.
_Avoid_: Planet occurrence, formation history, object owner

**System Ownership**:
The relationship placing every generated object within exactly one Stellar System for identity and lifecycle purposes. Ownership is independent of orbital parentage, population membership, and claim derivation.
_Avoid_: Orbital parent, host scope, barycentre ownership

**Orbital Parent**:
The physical body or barycentre that is the present-day centre of an Orbital Node's orbit. It records neither formation history nor ownership.
_Avoid_: Owner, formation host, resonance partner

**Host Scope**:
The stellar member or barycentre whose stable orbital domain contains a planetary body or collective structure. A host scope organizes a Planetary Architecture but does not own its members.
_Avoid_: System ownership, orbital parent when referring to a collective structure

**Population Membership**:
The relationship from a materialised small body to the statistical population from which it was drawn. The body still orbits its physical or barycentric Orbital Parent rather than the population.
_Avoid_: Orbital parent, ownership

**Planet**:
A non-stellar body represented as an individual member of a planetary architecture whose Orbital Parent is a stellar member or barycentre.
_Avoid_: Planet candidate, planetesimal

**Natural Satellite**:
A non-stellar body whose Orbital Parent is a planet or dwarf planet. A notable ring moonlet is a Natural Satellite rather than an individually represented ring particle.
_Avoid_: Stellar companion, ring particle

**Ring System**:
A circumplanetary population of solid particles represented as a collective orbital structure associated with a planet or dwarf planet, not as an Orbital Node or individually materialised particles.
_Avoid_: Debris disk, asteroid belt, orbital parent

**Dwarf Planet**:
A near-round non-stellar body that does not dynamically dominate its orbital neighbourhood. It may be materialised as a notable member of a Planetesimal Reservoir while retaining a stellar member or barycentre as its Orbital Parent.
_Avoid_: Small planet, arbitrary large asteroid, reservoir satellite

**Planetesimal Reservoir**:
A statistical population of smaller solid bodies sharing a formation and stellar host scope; it describes an orbital domain but is neither an Orbital Node nor the Orbital Parent of its materialised members.
_Avoid_: Planet list, debris disk, orbital parent

**Asteroid Belt**:
A circumstellar planetesimal reservoir dominated by relatively rocky bodies in a bounded orbital band.
_Avoid_: Individual asteroid, ring system

**Outer Planetesimal Belt**:
A cold circumstellar reservoir analogous in role, but not necessarily identical, to the Solar System's Kuiper belt.
_Avoid_: Universal Kuiper belt, comet cloud

**Comet Reservoir**:
A cold planetesimal population capable of supplying bodies onto comet-like orbits; its members need not be materialised individually.
_Avoid_: Debris disk, asteroid belt

**Trojan Population**:
A co-orbital small-body population sharing a planet's stellar Orbital Parent in a 1:1 resonance. The planet is its resonance partner, not its Orbital Parent.
_Avoid_: Independent belt, moon system, planetary satellite

**Debris Disk**:
An observable dust population replenished by collisions in one or more compatible Planetesimal Reservoirs. It shares their Host Scope but neither owns nor is interchangeable with them.
_Avoid_: Protoplanetary disk, asteroid inventory, planetesimal reservoir

**Planet Occurrence**:
An empirically calibrated expectation or probability within a stated host-star, planet-size, and orbital-period domain.
_Avoid_: Universal planet probability, complete planetary system

**Planet Population Summary**:
A deterministic draw of occurrence counts or host fractions in explicitly calibrated observational domains; it does not yet assign physical planets or orbits.
_Avoid_: Planetary system, orbital architecture

**Explicit Planet Candidate**:
A deterministic realization of one occurrence channel with sampled observable properties and an orbit scale, before dynamical acceptance.
_Avoid_: Confirmed planet, complete planetary system

**Accepted Explicit Planet**:
An explicit planet candidate that lies inside every stability constraint evaluated by the current model.
_Avoid_: Permanently stable planet, observed planet

**Rejected Planet Candidate**:
An explicit planet candidate retained as provenance after failing a modeled coverage or stability condition; it is not a member of the accepted planetary system.
_Avoid_: Destroyed planet, resampling request

**Unresolved Planet Population**:
A positive occurrence result whose source domain does not determine enough properties to materialise individual planets without inventing a distribution.
_Avoid_: Empty planetary system, unsupported host

**Scientific Claim**:
A scientifically meaningful generated value together with the provenance that explains how that value was obtained. Its provenance remains inseparable from the value, allowing one object to contain claims supported by different kinds of evidence.
_Avoid_: Unannotated generated property, object-level evidence label

**Evidence Level**:
The kind of support behind a scientific claim: `Empirical` when a named source directly supports the claim inside its calibrated domain, `PhysicalProxy` when a physical model or explicitly transferred analogue produces it, and `Decorative` when bounded variation is added only for presentation. Extrapolation is a qualifier on a physical-proxy claim, not a separate evidence level.
_Avoid_: Confidence, quality score, speculative evidence level

**Generating Prescription**:
An immutable, versioned identity for the method that produced a scientific claim. It refers to every scientific source used by that method and preserves any precise source locator needed to audit the claim.
_Avoid_: Unversioned model name, citation

**Scientific Source**:
A uniquely identified publication, dataset, or other scientific record supporting a generating prescription. Its complete citation may be stored once in a shared catalog while each scientific claim retains stable references to it.
_Avoid_: Generating prescription, free-text citation

**Aleatory Variation**:
The variation among possible stellar systems or properties represented by a generating prescription. It is sampled seed-deterministically for each applicable claim.
_Avoid_: Epistemic uncertainty, model error

**Epistemic Uncertainty**:
Uncertainty in scientific knowledge, source parameters, or model assumptions rather than variation among generated systems. Shared epistemic parameters are selected consistently for a whole model realization rather than independently for each object.
_Avoid_: Aleatory variation, random decorative variation

**Model Realization**:
One coherent selection of shared epistemic parameters used throughout a generated stellar catalog. It allows sensitivity or ensemble runs without turning common scientific uncertainty into independent object-to-object variation.
_Avoid_: Stellar system seed, individual property draw

**Claim Uncertainty**:
The source-faithful quantitative or explicitly unquantified uncertainty attached to a scientific claim. It preserves whether the source supplied an interval, bound, parametric distribution, posterior artifact, or no quantified uncertainty, including confidence or credible level and correlations shared with other claims.
_Avoid_: Generic confidence score, automatically Gaussian error, aleatory variation

**Claim Extrapolation**:
A structured applicability record for a physical-proxy claim evaluated beyond a scientific source's calibrated domain. It identifies every exceeded input axis, the source domain, the evaluated input, and the direction, extent, and method of extrapolation. An empirical prescription never silently extrapolates or clamps its inputs.
_Avoid_: Empirical claim, generic warning flag, clamped estimate

**Claim Derivation**:
The auditable relationship from a derived scientific claim to its immediate input claims. Stable claim identities and transitive derivation links preserve the complete origin without duplicating entire provenance chains on every value. A derived claim's evidence level is the least-supported level among its generating prescription and every essential input; decorative input may not silently support a physical claim.
_Avoid_: Copied provenance text, source list without input relationships

**Object Evidence Summary**:
A derived overview of an object's claim composition: counts by evidence level, the least-supported physical evidence level, and separate indications of decorative claims, extrapolation, unquantified uncertainty, rejection, and unsupported coverage. Individual scientific claims remain the source of truth; the summary is never a single object-wide evidence label.
_Avoid_: Object-level evidence label, replacement for claim provenance

**Claim Outcome**:
The result of attempting to produce a scientific claim: `Accepted` retains a plausibility-checked claim, `NotSelected` records a normal evidence-based draw that produced no candidate, `Rejected` retains the candidate and violated constraints, and `Unsupported` records why no value may be generated. Only accepted and rejected outcomes contain a generated value.
_Avoid_: Optional value without reason, replacing rejected candidates, clamped unsupported value

**Random Draw Address**:
The stable identity of a stochastic decision within a simulation seed, formed from the generating-prescription namespace, stable object identity, claim key, and bounded-attempt index under a named random-number algorithm version. It allows one claim or non-selection to be reproduced without depending on unrelated draw order.
_Avoid_: Mutable global draw position, unexplained derived seed

**Validation Receipt**:
The versioned record of the plausibility policy applied to a generated candidate, its input claims, and every relevant passed, failed, or unevaluated constraint, including evaluated margins and thresholds. It explains acceptance or rejection but does not guarantee unmodelled long-term stability.
_Avoid_: Boolean validity flag, first failure only, stability guarantee

**Whole-System Plausibility**:
The state in which all accepted members jointly satisfy every applicable required constraint evaluated by one versioned policy. Unevaluated advisory constraints remain visible limitations rather than guarantees.
_Avoid_: Individual candidate validity, permanent stability, complete model coverage

**Reconciliation Policy**:
The immutable, versioned rules that order candidate evaluation, resolve conflict groups, permit bounded placement attempts, and determine final outcomes for one Model Realization.
_Avoid_: Evidence ranking, mutable retry logic, validator

**Conflict Group**:
A recorded set of individually generated candidates that cannot coexist under the Reconciliation Policy. It preserves every participant and the deterministic reason for each winner, rejection, or revoked provisional acceptance.
_Avoid_: Duplicate candidate, first failure

**Provisional Acceptance**:
A reversible candidate state that has passed the currently available checks but has not reached the final whole-system fixpoint. It is not an accepted published outcome.
_Avoid_: Accepted outcome, guaranteed member

**Placement Attempt**:
One immutable, deterministically addressed candidate produced within a Generating Prescription's versioned attempt bound. Changing a candidate's placement creates another attempt rather than mutating the original.
_Avoid_: Silent retry, in-place correction

**Whole-System Fixpoint**:
The publishable state with no unresolved conflict groups, current dependent Validation Receipts, satisfied required constraints, and internally valid references.
_Avoid_: Intermediate generation state, partially validated system

**Provenance Integrity**:
The invariant that every scientific claim and claim outcome is complete and internally consistent with its evidence level, sources, applicability, derivation, uncertainty, stochastic origin, and validation state. Invalid combinations fail during construction or deserialization rather than continuing as warning-bearing data.
_Avoid_: Best-effort provenance, partially trusted claim

**Orbital Node**:
A member of a hierarchical orbital arrangement that is either an individually materialised physical body or a barycentre. Collective structures and unmaterialised statistical populations are not Orbital Nodes.
_Avoid_: Collective orbital structure, child entity

**Barycentre**:
A structural Orbital Node representing the shared centre of mass about which two or more children move. It has no independent physical-body properties or ownership; its mass and position are derived from its descendants.
_Avoid_: Central body, object owner, physical body

**Stellar Orbital Hierarchy**:
A nested arrangement of stellar members and barycentres connected by relative stellar orbits.
_Avoid_: Flat companion list, instantaneous scene positions

**Relative Stellar Orbit**:
The orbit of two child bodies or child barycentres relative to one another; its semimajor axis is not either child's barycentric radius.
_Avoid_: Instantaneous separation, projected separation

**Nearest Companion Scale**:
The smallest relative-orbit semimajor axis encountered from a stellar member through its orbital hierarchy.
_Avoid_: Current distance, projected angular separation

**Orbital Contact Radius**:
A finite-body radius used only to reject relative-orbit candidates whose periastron would make two stellar members overlap. It may come from a narrower proxy than the full stellar-evolution model and must carry that provenance.
_Avoid_: Evolution snapshot, observed stellar radius

**Circumstellar S-Type Stability Zone**:
The range of planet orbits centred on one stellar member that remains dynamically viable against its limiting stellar companion under a stated stability model. Its outer boundary is a critical semimajor axis, not a planet's generated orbit.
_Avoid_: Habitable zone, Hill sphere, guaranteed long-term stability

**Limiting Stellar Companion**:
The physical star or sibling barycentre whose relative orbit sets the smallest modeled outer boundary of a member's circumstellar stability zone.
_Avoid_: Nearest current position, planetary host
