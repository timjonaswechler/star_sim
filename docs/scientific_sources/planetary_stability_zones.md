# Scientific basis for circumstellar planetary stability zones

This note defines a deliberately narrow **S-type stability-zone v1**. It answers one question: for a planet orbiting one stellar member, what is the empirical outer semimajor-axis boundary imposed by that member's nearest stellar companion? It does not generate planets, prove Gyr stability, or replace an N-body calculation.

## Binary calibration

Holman & Wiegert numerically integrated the elliptic restricted three-body problem: two stars remained on a fixed eccentric binary orbit and planets were massless, non-interacting test particles. The test particles began on circular, prograde orbits in the binary plane. Eight starting longitudes were tested at each planetary semimajor axis, and the reported S-type critical semimajor axis was the largest radius at which all eight survived `10^4` binary periods ([Holman & Wiegert 1999](https://doi.org/10.1086/300695); [author-hosted paper](https://physics.uwo.ca/~pwiegert/papers/1999AJ.117.621.pdf)).

For a planet orbiting a host star of mass `M_host` while a stellar companion of mass `M_perturber` follows a relative orbit with semimajor axis `a_binary` and eccentricity `e_binary`, define

```text
mu = M_perturber / (M_host + M_perturber)

a_critical / a_binary =
      ( 0.464 +/- 0.006)
    + (-0.380 +/- 0.010) * mu
    + (-0.631 +/- 0.034) * e_binary
    + ( 0.586 +/- 0.061) * mu * e_binary
    + ( 0.150 +/- 0.041) * e_binary^2
    + (-0.198 +/- 0.074) * mu * e_binary^2
```

The uncertainties shown are the formal coefficient uncertainties reported by the paper. The implementation should use the central coefficients; adding or subtracting all coefficient uncertainties independently would ignore their covariance and is not a justified confidence bound.

`a_critical` is an empirical outer boundary for the tested S-type configuration. It is not the stellar Hill radius and not the instantaneous distance to the companion. `a_binary` must be the semimajor axis of the relative stellar orbit, matching the existing hierarchy model's `RelativeOrbit` semantics.

The fit is calibrated only for

```text
0.1 <= mu <= 0.9
0.0 <= e_binary <= 0.8
```

and was typically within `4%`, with a worst quoted discrepancy of `11%`, relative to the numerical grid. The source explicitly notes that `10^4` binary periods are short compared with stellar ages and that longer-term erosion can occur. Version 1 must therefore return a typed coverage error outside either parameter interval; it must not clamp `mu` or `e`, extrapolate the polynomial, or label the result “long-term stable” ([Holman & Wiegert 1999](https://doi.org/10.1086/300695)).

## Recommended output semantics

Keep the empirical boundary distinct from any later planet-generation policy:

```text
CircumstellarStabilityZone {
    model_id: "holman_wiegert_1999_s_type_v1",
    host_member_id,
    constraining_orbit_node_id,
    nominal_outer_critical_semimajor_axis_au,
    fit_residual_lower_semimajor_axis_au,
    assumptions,
    quality_flags,
}

fit_residual_lower_semimajor_axis_au = 0.89 * nominal_outer_critical_semimajor_axis_au
```

The `0.89` value only applies the paper's worst reported polynomial-fit discrepancy. It is useful as a conservative **fit-residual margin**, but it does not cover longer integrations, inclined or eccentric planets, additional stars, planet-planet interactions, or resonances. It must not be named a guaranteed-safe radius. A separately configurable planet-generation margin may be added later, but no universal value is supported by this source.

For a single star, return an unbounded-by-stellar-companion result such as `NoStellarCompanionLimit`; do not invent a finite outer boundary. Disc truncation, Galactic tides, passing stars, and the system's Jacobi radius are different models.

## Hierarchical triples and quadruples

Verrier & Evans modelled a hierarchical triple as an inner binary plus an outer pseudo-binary in which the inner pair is approximated by a point mass. In their explicit circumbinary-planet experiment, decoupled Holman-Wiegert boundaries reproduced most triple-system boundaries, but deviations appeared for close, massive, or highly eccentric stellar configurations and from combined resonances. Their simulations were coplanar over most of the tested grid; their limited inclination experiment did not cover the high-inclination Kozai-Lidov regime. The paper argues that binary criteria should extend approximately to deeper hierarchies, but it did not numerically calibrate a universal S-type formula for arbitrary triples or quadruples ([Verrier & Evans 2007](https://doi.org/10.1111/j.1365-2966.2007.12493.x); [open manuscript](https://arxiv.org/abs/0710.1167)).

The defensible v1 approximation for a stellar leaf is therefore:

1. Use only the leaf's **direct parent orbit**, the existing nearest hierarchy edge.
2. If the sibling is a leaf, use its stellar mass as `M_perturber`; this is the actual binary case.
3. If the sibling is a subtree, use the sum of its current stellar masses as a point-mass perturber and set `SiblingSubtreePointMassApproximation`.
4. Require the stellar hierarchy itself to have passed its existing nested stability screen.
5. Apply the Holman-Wiegert domain checks unchanged and attach `HierarchicalMultipleNearestEdgeOnly` for every triple or quadruple.

This produces a local truncation indicator under the explicit modelling assumption that the nearest hierarchy level is the dominant stellar constraint. It does **not** demonstrate that farther ancestors are dynamically irrelevant. Applying the formula to ancestor edges with the host-containing subtree's total mass would change the central object from “host star” to “subsystem barycentre” and would no longer describe an S-type orbit around the leaf. Version 1 should not do that.

Consequently, a triple/quadruple result should expose both the numeric nearest-edge boundary and a status such as `ApproximateAdditionalPerturbersNotIntegrated`; it must not be promoted to `VerifiedStableZone`. Systems with non-hierarchical stellar topology, a failed/unknown stellar hierarchy, or an unresolved direct-parent orbit remain explicitly unsupported.

## What remains unsupported

The following cases are outside the scientific claim of v1:

- `mu < 0.1`, `mu > 0.9`, or stellar `e > 0.8`;
- eccentric, inclined, polar, retrograde, resonant, or co-orbital planetary initial conditions;
- stability of a finite-mass planet, multiple mutually interacting planets, moons, or debris discs;
- circumbinary (P-type), circumtriple, or other subsystem-centred planetary orbits;
- complete triple/quadruple secular dynamics, Kozai-Lidov cycles, and combined resonances;
- evolved stellar systems whose orbital evolution, mass loss, tides, mass transfer, kicks, or mergers are not modelled;
- survival for a stellar age or a claim of habitability.

The multiple-planet exclusion is important for the current project. Holman & Wiegert's one Solar-System experiment found that the test-particle formula overestimated the viable outer scale by about a factor of two when planet-planet interactions were included. That single experiment establishes that the single-particle boundary is insufficient; it does not calibrate a universal factor-of-two correction. Concrete multi-planet architectures therefore need an additional mutual-Hill/resonance screen and ultimately an N-body validation step ([Holman & Wiegert 1999](https://doi.org/10.1086/300695)).

## Recommended v1 role in the pipeline

```text
stellar orbital hierarchy
    -> nearest direct-parent stellar edge
    -> nominal Holman-Wiegert S-type boundary
    -> explicit fit-residual lower estimate
    -> candidate period/semimajor-axis sampling inside the chosen policy limit
    -> planet-planet architecture screen
    -> optional N-body validation
```

The first implementation should stop after calculating and reporting the first two boundaries. That gives later planet generation a typed, reproducible input without claiming more dynamical knowledge than the calibration contains.
