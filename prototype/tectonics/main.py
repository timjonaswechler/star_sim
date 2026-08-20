#!/usr/bin/env python3
"""Command-line entry point for the throwaway tectonics prototype."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Callable

import numpy as np

from geometry import boundary_frames, from_latlon, meridian, rodrigues, surface_velocity, to_latlon, unit
from mechanics import DRIVERS, apply_driver_torques, classify_boundaries
from render import save
from scenarios import RULE_CONDITIONS, SCENARIO_NAMES, SCENARIOS, _make_base_world, run_scenario


class CheckFailure(RuntimeError):
    pass


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise CheckFailure(message)


def _all_positions(state) -> list[np.ndarray]:
    values: list[np.ndarray] = []
    values.extend(plate.reference_position for plate in state.plates.values())
    values.extend(point for boundary in state.boundaries.values() for point in boundary.points)
    values.extend(marker.position for marker in state.crust_markers)
    values.extend(point for block in state.continental_blocks.values() for point in block.sample_points)
    values.extend(point for feature in state.features.values() for point in feature.points)
    return values


def _geometry() -> str:
    points = np.asarray([from_latlon(-72, -170), from_latlon(0, 0), from_latlon(83, 141)])
    rotated = rodrigues(points, np.array([0.013, -0.021, 0.008]), 713.0)
    _require(np.allclose(np.linalg.norm(rotated, axis=1), 1.0, atol=1.0e-12), "Rodrigues rotation left unit sphere")
    original_distances = points @ points.T
    rotated_distances = rotated @ rotated.T
    _require(np.allclose(original_distances, rotated_distances, atol=1.0e-12), "rigid rotation changed angular distances")
    return "Rodrigues advection preserves unit length and pairwise spherical distance"


def _decomposition() -> str:
    points = np.asarray([from_latlon(latitude, 17) for latitude in np.linspace(-55, 55, 9)])
    tangents, normals = boundary_frames(points)
    omega_left = np.array([0.002, -0.004, 0.006])
    omega_right = np.array([-0.003, 0.001, -0.005])
    residuals = []
    for point, tangent, normal in zip(points, tangents, normals, strict=True):
        relative = surface_velocity(omega_right, point, 6371.0) - surface_velocity(omega_left, point, 6371.0)
        reconstructed = np.dot(relative, normal) * normal + np.dot(relative, tangent) * tangent
        residuals.append(np.linalg.norm(relative - reconstructed))
    _require(max(residuals) < 1.0e-9, f"normal/tangent reconstruction residual {max(residuals)}")
    return f"relative velocity reconstructs from local normal and tangent components, max residual {max(residuals):.3g}"


def _drivers() -> str:
    magnitudes: dict[str, float] = {}
    for driver in DRIVERS:
        world = _make_base_world("extroversion", 31)
        classify_boundaries(world)
        before = {plate_id: plate.omega.copy() for plate_id, plate in world.plates.items()}
        coefficients = {name: 0.0 for name in DRIVERS}
        coefficients[driver] = {"slab_pull": 0.0032, "slab_suction": 0.001, "ridge_push": 0.0007, "mantle_drag": 0.22}[driver]
        apply_driver_torques(world, 1.0, coefficients)
        driver_torque = sum(
            (np.linalg.norm(table[driver]) for table in world.driver_torques.values()), start=0.0
        )
        omega_change = max(
            np.linalg.norm(world.plates[plate_id].omega - value) for plate_id, value in before.items()
        )
        _require(driver_torque > 1.0e-12, f"{driver} produced no spherical torque")
        _require(omega_change > 1.0e-12, f"{driver} did not change omega")
        magnitudes[driver] = driver_torque

    direction_world = _make_base_world("extroversion", 31)
    for boundary_id, boundary in direction_world.boundaries.items():
        if boundary_id != "west_trench":
            boundary.active = False
    classify_boundaries(direction_world)
    sample = direction_world.boundaries["west_trench"].diagnostics[4]
    point = sample.point
    normal = sample.normal
    before_subducting = surface_velocity(direction_world.plates["ocean_w"].omega, point, direction_world.radius_km)
    before_overriding = surface_velocity(direction_world.plates["continent_w"].omega, point, direction_world.radius_km)
    apply_driver_torques(
        direction_world,
        1.0,
        {"slab_pull": 0.0032, "slab_suction": 0.001, "ridge_push": 0.0, "mantle_drag": 0.0},
    )
    subducting_delta = surface_velocity(direction_world.plates["ocean_w"].omega, point, direction_world.radius_km) - before_subducting
    overriding_delta = surface_velocity(direction_world.plates["continent_w"].omega, point, direction_world.radius_km) - before_overriding
    _require(np.dot(subducting_delta, normal) > 0.0, "right-side slab pull points away from its trench")
    _require(np.dot(overriding_delta, normal) < 0.0, "left-side slab suction points away from its trench")

    length_torques = []
    for latitude_limit in (15.0, 58.0):
        length_world = _make_base_world("extroversion", 31)
        for boundary_id, boundary in length_world.boundaries.items():
            boundary.active = boundary_id == "west_trench"
        length_world.boundaries["west_trench"].points = meridian(-68, 9, latitude_limit)
        classify_boundaries(length_world)
        apply_driver_torques(
            length_world,
            1.0,
            {"slab_pull": 0.0032, "slab_suction": 0.0, "ridge_push": 0.0, "mantle_drag": 0.0},
        )
        length_torques.append(
            sum(np.linalg.norm(table["slab_pull"]) for table in length_world.driver_torques.values())
        )
    _require(length_torques[1] > length_torques[0] * 2.0, "slab pull does not increase with trench length")
    return "all four spherical driver torques are nonzero, change omega, pull both trench sides inward, and scale slab pull with length: " + ", ".join(
        f"{name}={value:.3g}" for name, value in magnitudes.items()
    )


def _boundary_classes(state, boundary_id: str) -> set[str]:
    return {sample.classification for sample in state.boundaries[boundary_id].diagnostics}


def _required_scenario_state(result) -> None:
    """Tie each completion-table observation to mutated or calculated state."""
    name = result.definition.name
    initial = result.history[result.milestone_indices["initial"]]
    decisive = result.history[result.milestone_indices["decisive_transition"]]
    final = result.final
    if name == "supercontinent_breakup":
        _require({"breakup_plume", "rift_arm_n", "failed_rift_w", "zigzag_rift", "new_ocean_ridge"} <= set(final.features), "breakup features missing")
        _require(np.linalg.norm(np.cross(final.plates["continent_w"].omega, final.plates["continent_e"].omega)) > 1.0e-7, "breakup fragments do not use distinct Euler poles")
        new_crust_plates = {
            marker.plate_id for marker in final.crust_markers if marker.id.startswith("new_crust_")
        }
        _require(new_crust_plates == {"ocean_w", "ocean_e"}, "breakup ridge did not create crust on both sides")
    elif name == "extroversion":
        _require(decisive.metrics["interior_ocean_width_km"] > initial.metrics["interior_ocean_width_km"], "extroversion interior did not grow")
        _require(decisive.metrics["exterior_ocean_width_km"] < initial.metrics["exterior_ocean_width_km"], "extroversion exterior did not shrink")
        _require(final.metrics["exterior_ocean_width_km"] == 0.0 and "extroversion_suture" in final.features, "extroversion did not close and suture")
    elif name == "introversion":
        _require(final.metrics["interior_ocean_width_km"] == 0.0, "introversion interior stayed open")
        _require(final.metrics["exterior_ocean_width_km"] > 0.0 and final.features["surviving_exterior_ocean"].active, "introversion exterior did not survive")
        _require(not final.boundaries["main_ridge"].active, "introversion ridge was not consumed")
    elif name == "flat_slab":
        _require(final.boundaries["west_trench"].properties["slab_dip_degrees"] < 30.0, "flat slab dip did not fall")
        _require(final.features["broad_inland_uplift"].properties["width_km"] >= 750, "flat-slab uplift is not broad")
        _require(final.features["inland_arc"].properties["volcanism"].startswith("inactive") and "slab_window" in final.features, "flat-slab arc or window missing")
        flat_event = next(event for event in final.events if event.rule == "flat-slab dip reduction")
        _require(flat_event.inputs["continent_w_speed_cm_per_year"] >= 2.0, "flat slab lacks fast overriding motion")
        _require(flat_event.inputs["ocean_w_mean_crust_age_myr"] <= 90.0, "flat slab lacks young buoyant crust")
    elif name == "slab_rollback":
        _require(final.boundaries["west_trench"].properties["slab_dip_degrees"] > 60.0, "rollback slab did not steepen")
        _require({"retreating_island_arc", "back_arc_basin", "back_arc_rift"} <= set(final.features), "rollback arc or basin missing")
        _require(any(np.linalg.norm(drivers["slab_suction"]) > 0.0 for drivers in decisive.driver_torques.values()), "rollback has no calculated slab suction")
        rollback_event = next(event for event in final.events if event.rule == "old-crust slab steepening and trench retreat")
        _require(rollback_event.inputs["ocean_w_mean_crust_age_myr"] >= 200.0, "rollback lacks old dense crust")
        trench_change = next(change for change in rollback_event.state_changes if change["path"] == "/boundaries/west_trench")
        before_longitude = to_latlon(np.asarray(trench_change["before"]["points"][4]))[1]
        after_longitude = to_latlon(np.asarray(trench_change["after"]["points"][4]))[1]
        _require(after_longitude > before_longitude, "rollback trench did not retreat eastward toward ocean_w")
        _require(decisive.features["retreating_island_arc"].plate_id == "continent_w", "rollback arc is not attached to the overriding edge")
    elif name == "arc_accretion":
        _require(any("convergent" in item for item in _boundary_classes(decisive, "arc_basin_trench")), "arc basin was not consumed by convergence")
        _require(final.continental_blocks["offshore_arc"].plate_id == "continent_e" and final.boundaries["seaward_active_margin"].active, "arc did not accrete and advance the margin")
    elif name == "subduction_jump":
        _require(any("convergent" in item for item in _boundary_classes(decisive, "jumped_trench")), "jumped trench is not convergent")
        _require(not final.boundaries["jumped_trench"].active and len(final.continental_blocks["west_continent"].collision_history) == 2, "large collision did not stop the jumped trench")
    elif name == "polarity_reversal":
        _require(final.boundaries["reversed_trench"].polarity == "arc_plate", "polarity did not reverse")
        _require(any("convergent" in item for item in _boundary_classes(final, "reversed_trench")), "reversed trench is not convergent")
    elif name == "rotation_arc":
        arc_points = decisive.boundaries["propagated_arc_trench"].points
        _require(np.linalg.svd(arc_points, compute_uv=False)[-1] > 1.0e-4, "rotation arc collapsed to one great-circle plane")
        _require(decisive.features["spherical_island_arc"].properties["construction"] == "differential Rodrigues advection", "rotation arc provenance missing")
    elif name == "subduction_invasion":
        _require(_boundary_classes(initial, "gateway_transform") == {"transform"}, "gateway does not begin as transform")
        _require(any("convergent" in item for item in _boundary_classes(decisive, "gateway_trench")), "invading gateway trench is not convergent")
        _require("attached_margin" in final.features, "invading trench did not attach to a margin")
        attached = final.boundaries["attached_margin_trench"]
        _require(attached.active and "continent_e" in {attached.left_plate, attached.right_plate}, "invading trench lacks a topological continental attachment")
        invasion_event = next(event for event in final.events if event.rule == "transform-gateway collapse")
        _require(invasion_event.inputs["gateway_transform_shear_km_per_myr"] >= 20.0, "gateway collapse lacks local shear")
        _require(invasion_event.inputs["ocean_plate_age_contrast_myr"] >= 50.0, "gateway collapse lacks crust-age contrast")
    elif name == "triple_junction_plate":
        for boundary_id in ("ridge_nw", "ridge_ne", "ridge_s"):
            _require(all("divergent" in item for item in _boundary_classes(initial, boundary_id)), f"{boundary_id} is not initially divergent")
        enclosure = next(
            state
            for state in result.history
            if any(event.rule == "central plate ridge enclosure" for event in state.events)
            and "central_ridge_w" in state.boundaries
        )
        for boundary_id in ("central_ridge_w", "central_ridge_e", "central_ridge_s"):
            _require(all("divergent" in item for item in _boundary_classes(enclosure, boundary_id)), f"{boundary_id} does not grow the central plate")
        _require("central_plate" in final.plates and final.plates["ocean_w"].properties["peripherally_consumed"], "central birth or old-plate loss missing")
        central_crust_plates = {
            marker.plate_id for marker in final.crust_markers if marker.id.startswith("central_")
        }
        _require({"central_plate", "ocean_w", "ocean_e", "arc_plate"} <= central_crust_plates, "central ridges did not create two-sided crust")
        birth_event = next(event for event in final.events if event.rule == "rotational triple-junction gap")
        _require(birth_event.inputs["triple_ridge_rotational_shear_km_per_myr"] >= 10.0, "central plate birth lacks rotational ridge shear")
    elif name == "megashear":
        _require(_boundary_classes(initial, "megashear") == {"transform"}, "megashear is not shear-dominated")
        _require(final.metrics["megashear_slip_km"] > 6000.0, "megashear did not accumulate long displacement")
        _require(len({block.assemblage_id for block in final.continental_blocks.values()}) == 1, "megashear split the assemblage")
    elif name == "tethys_ocean":
        plateau = final.features["himalayan_plateau"]
        _require(plateau.properties["style"] == "Himalayan" and plateau.properties["reactivated_belts"] >= 2, "Tethys plateau lacks repeated accretion")
        _require(not final.boundaries["tethys_jumped_trench"].active, "large Tethys collision did not stop subduction")
    elif name == "complex_collision":
        required = {"early_contact_suture", "trapped_ocean_pocket", "local_pocket_trench", "enclosed_back_arc", "rollback_arc", "fracture_islands", "pocket_invasion_trench", "local_orogeny"}
        _require(required <= set(final.features), "complex collision mosaic is incomplete")
        _require(final.features["trapped_ocean_pocket"].properties["result"] == "Mediterranean-like enclosed sea", "enclosed sea result missing")


def _scenarios() -> tuple[str, dict[str, str]]:
    transition_rules = {
        transition.rule for definition in SCENARIOS.values() for transition in definition.transitions
    }
    _require(set(RULE_CONDITIONS) == transition_rules, "a heuristic lacks a mechanism condition or an unused condition remains")
    _require(all(RULE_CONDITIONS[rule] for rule in transition_rules), "a heuristic has an empty mechanism condition")
    canonical: dict[str, str] = {}
    summaries: list[str] = []
    for name in SCENARIO_NAMES:
        result = run_scenario(name, steps=24, seed=19)
        final = result.final
        _required_scenario_state(result)
        expected_observations = sum(len(transition.observations) for transition in SCENARIOS[name].transitions)
        _require(len(final.observations) == expected_observations, f"{name}: missing structured observations")
        _require(all(item.achieved and item.evidence for item in final.observations.values()), f"{name}: unvalidated observation")
        _require(len(final.events) == len(SCENARIOS[name].transitions), f"{name}: a heuristic did not fire")
        _require(set(result.milestone_indices) == {"initial", "decisive_transition", "final"}, f"{name}: milestones missing")
        _require(len(set(result.milestone_indices.values())) == 3, f"{name}: milestone states are not distinct")
        for event in final.events:
            _require(event.provenance == "deterministic heuristic", f"{name}: event provenance missing")
            _require(event.citation.startswith("research.md#"), f"{name}: event citation missing")
            _require(event.inputs and all(isinstance(value, (int, float)) for value in event.inputs.values()), f"{name}: event inputs are not numerical")
            _require(event.threshold and event.state_changes, f"{name}: threshold or exact state changes missing")
            _require(event.effect_descriptions, f"{name}: effect descriptions missing")
            _require(
                all(set(change) == {"path", "before", "after"} and change["before"] != change["after"] for change in event.state_changes),
                f"{name}: event does not contain exact before/after state diffs",
            )
        for state in result.history:
            _require(all(abs(np.linalg.norm(point) - 1.0) < 1.0e-10 for point in _all_positions(state)), f"{name}: non-unit state position")
            for boundary in state.boundaries.values():
                if boundary.active:
                    _require(boundary.diagnostics, f"{name}: active boundary lacks calculated samples")
                    _require(max(sample.reconstruction_residual for sample in boundary.diagnostics) < 1.0e-8, f"{name}: boundary decomposition failed")
        rigid_groups: dict[str, list[np.ndarray]] = {}
        for plate in final.plates.values():
            rigid_id = plate.properties.get("rigid_assemblage")
            if rigid_id:
                rigid_groups.setdefault(str(rigid_id), []).append(plate.omega)
        for rigid_id, omegas in rigid_groups.items():
            _require(
                all(np.allclose(omegas[0], omega, atol=1.0e-12) for omega in omegas[1:]),
                f"{name}: sutured assemblage {rigid_id} does not share one Euler vector",
            )
        report = final.to_dict()
        _require(report["motion_provenance"].startswith("calculated"), f"{name}: calculated provenance absent")
        _require(report["initial_state_provenance"].startswith("authored"), f"{name}: initial-input provenance absent")
        _require(report["topology_provenance"].startswith("post-initial"), f"{name}: topology provenance absent")
        canonical[name] = result.canonical_json()
        summaries.append(f"{name}({expected_observations} observations)")
    return "all scenarios passed: " + ", ".join(summaries), canonical


def _determinism(first_reports: dict[str, str]) -> str:
    for name in SCENARIO_NAMES:
        second = run_scenario(name, steps=24, seed=19).canonical_json()
        _require(second == first_reports[name], f"{name}: same seed and arguments changed serialized state")
    varied = run_scenario("supercontinent_breakup", steps=24, seed=20).canonical_json()
    _require(varied != first_reports["supercontinent_breakup"], "seed does not affect sampled initial state")
    return "all fourteen canonical per-step reports match on a repeated seed and arguments"


def _step_stability() -> str:
    for name in SCENARIO_NAMES:
        coarse = run_scenario(name, steps=17, seed=5).final
        fine = run_scenario(name, steps=53, seed=5).final
        _require(
            [event.rule for event in coarse.events] == [event.rule for event in fine.events],
            f"{name}: transition order depends on step count",
        )
        coarse_topology = (
            set(coarse.plates),
            {(key, value.active) for key, value in coarse.boundaries.items()},
            {(key, value.active) for key, value in coarse.features.items()},
            {(key, value.plate_id, value.assemblage_id) for key, value in coarse.continental_blocks.items()},
        )
        fine_topology = (
            set(fine.plates),
            {(key, value.active) for key, value in fine.boundaries.items()},
            {(key, value.active) for key, value in fine.features.items()},
            {(key, value.plate_id, value.assemblage_id) for key, value in fine.continental_blocks.items()},
        )
        _require(coarse_topology == fine_topology, f"{name}: topology outcome changed with step count")
    return "all fourteen scenarios retain transition order and topology across coarse and fine sampling"


def run_self_check() -> int:
    checks: list[tuple[str, Callable[[], str]]] = [
        ("geometry", _geometry),
        ("vector decomposition", _decomposition),
        ("plate drivers", _drivers),
    ]
    outputs: list[tuple[str, str]] = []
    try:
        for label, check in checks:
            outputs.append((label, check()))
        scenario_summary, reports = _scenarios()
        outputs.append(("scenarios, observations, milestones, provenance", scenario_summary))
        outputs.append(("determinism", _determinism(reports)))
        outputs.append(("step stability", _step_stability()))
    except (AssertionError, CheckFailure, KeyError, RuntimeError, ValueError) as error:
        print(f"SELF-CHECK FAILED: {error}", file=sys.stderr)
        return 1
    for label, output in outputs:
        print(f"PASS {label}: {output}")
    print(f"SELF-CHECK PASSED ({len(outputs)} groups, {len(SCENARIO_NAMES)} scenarios)")
    return 0


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run the throwaway spherical plate-tectonics scenario prototype.")
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--list", action="store_true", help="list the fourteen scenarios")
    mode.add_argument("--self-check", action="store_true", help="run built-in mechanics and scenario checks")
    parser.add_argument("--scenario", choices=SCENARIO_NAMES, help="scenario to run")
    parser.add_argument("--steps", type=int, default=40, help="calculated steps across the fixed 300 Myr duration")
    parser.add_argument("--seed", type=int, default=7, help="deterministic initial sampling seed")
    parser.add_argument("--save", help="render milestone PNG or per-step GIF")
    parser.add_argument("--report", help="write complete per-step JSON state to this path")
    parser.add_argument("--json", action="store_true", help="print complete per-step JSON state")
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = _parser()
    arguments = parser.parse_args(argv)
    if arguments.list:
        for name in SCENARIO_NAMES:
            print(f"{name:24} {SCENARIOS[name].summary}")
        return 0
    if arguments.self_check:
        return run_self_check()
    if not arguments.scenario:
        parser.error("choose --scenario, --list, or --self-check")
    try:
        result = run_scenario(arguments.scenario, arguments.steps, arguments.seed)
        report = result.to_dict()
        if arguments.save:
            save(result, arguments.save)
        if arguments.report:
            destination = Path(arguments.report)
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(json.dumps(report, indent=2, sort_keys=True, allow_nan=False) + "\n", encoding="utf-8")
        if arguments.json:
            print(json.dumps(report, indent=2, sort_keys=True, allow_nan=False))
        else:
            final = result.final
            print(
                f"{arguments.scenario}: {arguments.steps} steps, {final.elapsed_myr:.1f} Myr, "
                f"{len(final.events)} heuristic transitions, "
                f"{sum(item.achieved for item in final.observations.values())}/{len(final.observations)} observations"
            )
            print("milestones: " + ", ".join(f"{name}=step {result.history[index].step_index}" for name, index in result.milestone_indices.items()))
            if arguments.save:
                print(f"rendered {arguments.save}")
            if arguments.report:
                print(f"wrote complete per-step state to {arguments.report}")
    except (KeyError, RuntimeError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
