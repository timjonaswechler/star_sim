# Plate tectonics mechanics prototype

This directory contains a throwaway, deterministic scenario simulator for the plate-motion patterns in Worldbuilding Pasta's "An Apple Pie From Scratch, Part Va." It answers one narrow question: can those patterns be represented as inspectable vector rules on a sphere and demonstrated one mechanism at a time?

The answer here is yes for the kinematic sequences. This is not an autonomous planet generator or a geodynamic prediction model.

## Run it

Python 3.11 or newer, NumPy, Matplotlib, and Matplotlib's Pillow support are required.

```bash
python prototype/tectonics/main.py --list
python prototype/tectonics/main.py --scenario slab_rollback --steps 80
python prototype/tectonics/main.py --scenario slab_rollback --steps 80 --save /tmp/slab-rollback.png
python prototype/tectonics/main.py --scenario triple_junction_plate --steps 80 --save /tmp/triple-junction.gif
python prototype/tectonics/main.py --scenario extroversion --steps 80 --report /tmp/extroversion.json
python prototype/tectonics/main.py --self-check
```

A PNG shows the initial state, decisive transition, and final state. A GIF shows every calculated step. `--report` writes the complete state after every step. `--json` prints the same report. The default run prints a short completion summary.

The fixed run duration is 300 million years. `--steps` changes the sampling interval while preserving transition order and final topology. A threshold crossing is recorded on the next sampled step, so event times can differ with step count. `--seed` controls the small initial crust-marker offsets. Identical arguments produce identical serialized state and renders.

## What is in the state

Each step records the planet radius and elapsed time, plates and Euler angular velocities, accumulated torque, sampled boundaries, continental blocks, ocean-crust markers and ages, features, observations, milestones, and the event log.

Every active boundary sample contains:

- its 3D unit position, local tangent, and oriented surface normal;
- the calculated relative velocity and its reconstructed value;
- signed normal and shear rates, plus nonnegative convergence, divergence, and shear magnitudes;
- a type derived from those rates and the boundary's subduction polarity.

The renderer labels plate identity and composition, draws calculated velocity arrows, colors boundary samples by calculated type, marks polarity, shades crust markers by age, and draws the generated features.

## Calculated motion and heuristic topology

`geometry.py` and `mechanics.py` calculate motion. Surface positions are 3D unit vectors. A plate stores an Euler angular-velocity vector `omega`; local velocity is `omega x (radius * position)`. Rodrigues rotation advects attached samples and renormalizes them. Boundary classification uses the relative velocities of adjacent plates. It never reads the boundary's scenario name or conceptual label.

Slab pull, slab suction, ridge push, and distributed mantle drag create line forces and spherical torques. Slab pull scales with sampled trench length and crust age. The prototype integrates the torques into `omega`; sutured plates then share one inertia-weighted Euler vector. Their coefficients target the article's speed classes and do not represent measured force units. Ridge events attach new crust markers to both adjacent plates.

`scenarios.py` owns topology changes because the source says their underlying geophysics is uncertain. Step zero is authored scenario input. Every later topology change passes through `_fire_heuristic`. The helper combines a fixed progress threshold with mechanism-specific calculated conditions such as local convergence, crust age, overriding speed, or rotational shear. It logs all numerical inputs, thresholds, exact before-and-after state diffs, and a `research.md` section and image citation. An observation passes only after its conditions and mutations have state evidence.

This separation is present in every JSON step as `initial_state_provenance`, `motion_provenance`, and `topology_provenance`.

## Article-to-code map

| Article mechanism | Implementation |
| --- | --- |
| Spherical rigid motion and boundary components | `geometry.rodrigues`, `geometry.surface_velocity`, `geometry.boundary_frames`, `mechanics.classify_boundaries` |
| Slab pull by age and length, slab suction, ridge push, and drag | `mechanics.calculate_driver_torques`, `mechanics.apply_driver_torques` |
| Shared motion after suturing | `mechanics.apply_driver_torques` rigid-assemblage update |
| Crust aging and spherical sample advection | `mechanics.advect_world`, `mechanics.update_calculated_metrics` |
| Supercontinent breakup, images 002 to 010 | `SCENARIOS["supercontinent_breakup"]` |
| Extroversion, images 009 to 014 | `SCENARIOS["extroversion"]` |
| Introversion, images 015 to 022 | `SCENARIOS["introversion"]` |
| Flat slabs and slab windows, images 025 to 026 | `SCENARIOS["flat_slab"]` |
| Slab rollback and back-arc spreading, images 027 to 029 | `SCENARIOS["slab_rollback"]` |
| Island-arc growth and accretion, images 030 to 032 | `SCENARIOS["arc_accretion"]` |
| Trench jumping, images 033 to 034 | `SCENARIOS["subduction_jump"]` |
| Polarity reversal, images 035 to 037 | `SCENARIOS["polarity_reversal"]` |
| Hinge rotation and arc formation, images 038 to 039 | `SCENARIOS["rotation_arc"]`, `geometry.hinge_arc` |
| Subduction invasion, images 040 to 042 | `SCENARIOS["subduction_invasion"]` |
| Triple-junction plate birth, images 043 to 045 | `SCENARIOS["triple_junction_plate"]` |
| Continental megashear, image 046 | `SCENARIOS["megashear"]` |
| Tethys-type oceans, images 047 to 050 | `SCENARIOS["tethys_ocean"]` |
| Complex collision, images 051 to 052 | `SCENARIOS["complex_collision"]` |
| Plan-view inspection and milestones | `render.draw`, `render.png`, `render.gif` |
| Mechanism thresholds and exact heuristic diffs | `scenarios.RULE_CONDITIONS`, `scenarios._fire_heuristic` |
| Geometry, driver direction, provenance, scenario, milestone, and determinism checks | `main.run_self_check` |

The event definitions contain the more precise `research.md` citation for each transition.

## Limits

The simulator demonstrates an authored event sequence. It does not discover plate topology, solve mantle flow, model finite-element stress or rheology, remesh spherical polygons, or predict which real trench will jump. Boundaries and crust use sampled polylines and markers, so collisions do not conserve polygon area. The torque coefficients are abstract. Feature widths and slab dips are scenario annotations based on the article, not computed cross sections.

There is no climate, erosion, sediment transport, sea-level solver, or self-organizing billion-year history. The megashear case is marked as plausible but poorly evidenced, matching the source. Flat-slab initiation, trench jumps, polarity reversal, subduction invasion, and triple-junction plate birth remain deterministic teaching rules rather than geodynamic claims.
