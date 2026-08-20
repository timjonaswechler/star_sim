"""Deterministic article-mechanism scenarios.

Calculated motion is delegated to mechanics.py.  Every uncertain topology
change passes through ``_fire_heuristic`` and leaves a numerical provenance
record tied to research.md.
"""

from __future__ import annotations

import copy
import json
import math
import re
from dataclasses import dataclass, field
from typing import Any

import numpy as np

from geometry import (
    equatorial_arc,
    from_latlon,
    hinge_arc,
    meridian,
    polyline_from_latlon,
    rodrigues,
    unit,
)
from mechanics import advance_calculated_state, calculate_driver_torques, classify_boundaries
from model import (
    Boundary,
    ContinentalBlock,
    CrustMarker,
    Event,
    Feature,
    Milestone,
    Observation,
    Plate,
    WorldState,
)


@dataclass(frozen=True)
class Transition:
    rule: str
    progress_threshold: float
    citation: str
    effects: tuple[dict[str, Any], ...]
    observations: tuple[str, ...]
    decisive: bool = False
    minimum_activity_km_per_myr: float = 0.01


@dataclass(frozen=True)
class ScenarioDefinition:
    name: str
    summary: str
    transitions: tuple[Transition, ...]
    notes: tuple[str, ...] = ()


@dataclass
class SimulationResult:
    definition: ScenarioDefinition
    history: list[WorldState]
    milestone_indices: dict[str, int]

    @property
    def final(self) -> WorldState:
        return self.history[-1]

    def to_dict(self) -> dict[str, Any]:
        return {
            "scenario": self.definition.name,
            "summary": self.definition.summary,
            "milestone_indices": dict(self.milestone_indices),
            "steps": [state.to_dict() for state in self.history],
        }

    def canonical_json(self) -> str:
        return json.dumps(self.to_dict(), sort_keys=True, separators=(",", ":"), allow_nan=False)


def _transition(
    rule: str,
    at: float,
    citation: str,
    effects: list[dict[str, Any]],
    observations: list[str],
    decisive: bool = False,
) -> Transition:
    return Transition(rule, at, citation, tuple(effects), tuple(observations), decisive)


def _point_geometry(effect: dict[str, Any]) -> np.ndarray:
    if "coordinates" in effect:
        return polyline_from_latlon(effect["coordinates"])
    if "point" in effect:
        return np.asarray([from_latlon(*effect["point"])])
    if "meridian" in effect:
        return meridian(effect["meridian"], effect.get("count", 9), effect.get("latitude_limit", 58.0))
    if "equator" in effect:
        return equatorial_arc(effect["equator"][0], effect["equator"][1], effect.get("count", 9))
    raise ValueError(f"effect has no geometry: {effect}")


def _block_points(latitude: float, longitude: float, width: float = 11.0) -> np.ndarray:
    return polyline_from_latlon(
        [
            (latitude - width * 0.55, longitude - width),
            (latitude + width * 0.55, longitude - width),
            (latitude + width * 0.65, longitude + width),
            (latitude - width * 0.45, longitude + width),
            (latitude - width * 0.55, longitude - width),
        ]
    )


def _make_base_world(name: str, seed: int) -> WorldState:
    rng = np.random.default_rng(seed)
    world = WorldState(scenario=name, seed=seed)
    world.plates = {
        "continent_w": Plate(
            "continent_w", "continental", np.array([0.0, 0.0, -0.0022]), from_latlon(8, -38), inertia=2.2
        ),
        "ocean_w": Plate("ocean_w", "oceanic", np.array([0.0, 0.0, 0.0065]), from_latlon(-10, -15)),
        "ocean_e": Plate("ocean_e", "oceanic", np.array([0.0, 0.0, -0.0060]), from_latlon(12, 15)),
        "continent_e": Plate(
            "continent_e", "continental", np.array([0.0, 0.0, 0.0024]), from_latlon(-6, 40), inertia=2.0
        ),
        "arc_plate": Plate(
            "arc_plate", "island-arc continental", np.array([0.0010, -0.0007, 0.0012]), from_latlon(4, 65), inertia=1.3
        ),
    }
    world.boundaries = {
        "west_trench": Boundary(
            "west_trench", "exterior trench", "continent_w", "ocean_w", meridian(-68), "ocean_w"
        ),
        "main_ridge": Boundary(
            "main_ridge", "interior ridge", "ocean_w", "ocean_e", meridian(0), None
        ),
        "east_trench": Boundary(
            "east_trench", "exterior trench", "ocean_e", "continent_e", meridian(68), "ocean_e"
        ),
        "megashear": Boundary(
            "megashear", "internal transform", "continent_w", "continent_e", equatorial_arc(-32, 32), None
        ),
    }
    if name != "megashear":
        world.boundaries["megashear"].active = False
    world.continental_blocks = {
        "west_continent": ContinentalBlock(
            "west_continent", "continent_w", _block_points(8, -38), "west"
        ),
        "east_continent": ContinentalBlock(
            "east_continent", "continent_e", _block_points(-6, 40), "east"
        ),
    }
    marker_index = 0
    for plate_id, center in (("ocean_w", -24.0), ("ocean_e", 24.0)):
        for row in range(8):
            marker_index += 1
            latitude = -45.0 + row * 13.0 + float(rng.uniform(-1.0, 1.0))
            longitude = center + float(rng.uniform(-8.0, 8.0))
            world.crust_markers.append(
                CrustMarker(
                    f"crust_{marker_index:02d}",
                    plate_id,
                    from_latlon(latitude, longitude),
                    age_myr=18.0 + row * 14.0,
                )
            )
    world.metrics.update(
        {
            "interior_ocean_width_km": 2500.0,
            "exterior_ocean_width_km": 18000.0,
            "initial_interior_ocean_width_km": 2500.0,
            "initial_exterior_ocean_width_km": 18000.0,
        }
    )
    world.scenario_notes = list(SCENARIOS[name].notes)
    _scenario_initial_state(world)
    world.refresh_boundary_membership()
    classify_boundaries(world)
    world.driver_torques = calculate_driver_torques(world)
    return world


def _scenario_initial_state(world: WorldState) -> None:
    """Install known starting geometry.  No inferred transition occurs here."""
    name = world.scenario
    if name in {"extroversion", "introversion"}:
        world.boundaries["exterior_ridge"] = Boundary(
            "exterior_ridge", "exterior ridge", "ocean_w", "ocean_e", meridian(165), None
        )
        if name == "introversion":
            world.features["inherited_margin_fault"] = Feature(
                "inherited_margin_fault",
                "failed_rift",
                meridian(-28, 7, 32),
                "continent_w",
                properties={"scenario_input": True},
            )
    elif name == "flat_slab":
        for marker in world.crust_markers:
            if marker.plate_id == "ocean_w":
                marker.age_myr *= 0.18
        world.plates["continent_w"].omega = np.array([0.0, 0.0, -0.0052])
    elif name == "slab_rollback":
        for marker in world.crust_markers:
            if marker.plate_id == "ocean_w":
                marker.age_myr += 90.0
        world.plates["continent_w"].omega *= 0.35
    elif name == "arc_accretion":
        world.continental_blocks["offshore_arc"] = ContinentalBlock(
            "offshore_arc", "arc_plate", _block_points(5, 48, 4), "offshore_arc"
        )
        world.features["offshore_arc"] = Feature(
            "offshore_arc", "island_arc", meridian(48, 7, 30), "arc_plate"
        )
    elif name == "subduction_jump":
        world.continental_blocks["microcontinent"] = ContinentalBlock(
            "microcontinent", "ocean_w", _block_points(10, -52, 4), "micro"
        )
        world.continental_blocks["large_continent"] = ContinentalBlock(
            "large_continent", "continent_e", _block_points(-5, 58, 14), "large"
        )
    elif name == "polarity_reversal":
        world.features["initial_arc"] = Feature(
            "initial_arc", "island_arc", meridian(-55, 7, 34), "arc_plate"
        )
        world.boundaries["west_trench"].left_plate = "arc_plate"
        world.boundaries["west_trench"].right_plate = "ocean_w"
        world.boundaries["west_trench"].polarity = "ocean_w"
    elif name == "rotation_arc":
        world.plates["continent_w"].omega = np.array([0.0017, -0.0012, -0.0025])
        world.plates["continent_e"].omega = np.array([-0.0011, 0.0018, 0.0028])
    elif name == "subduction_invasion":
        world.boundaries["gateway_transform"] = Boundary(
            "gateway_transform",
            "ocean gateway transform",
            "ocean_w",
            "ocean_e",
            equatorial_arc(-12, 22),
            None,
        )
        world.boundaries["main_ridge"].concept = "young interior ridge"
        for marker in world.crust_markers:
            if marker.plate_id == "ocean_e":
                marker.age_myr += 80.0
    elif name == "triple_junction_plate":
        world.plates["arc_plate"].omega = np.array([0.0, 0.0050, 0.0])
        junction = {
            "ridge_nw": polyline_from_latlon([(0, 0), (18, -10), (35, -22)]),
            "ridge_ne": polyline_from_latlon([(0, 0), (18, 10), (35, 22)]),
            "ridge_s": polyline_from_latlon([(0, 0), (-18, 0), (-35, 0)]),
        }
        world.boundaries["ridge_nw"] = Boundary(
            "ridge_nw", "triple junction ridge", "ocean_w", "ocean_e", junction["ridge_nw"], None
        )
        world.boundaries["ridge_ne"] = Boundary(
            "ridge_ne", "triple junction ridge", "arc_plate", "ocean_e", junction["ridge_ne"], None
        )
        world.boundaries["ridge_s"] = Boundary(
            "ridge_s", "triple junction ridge", "arc_plate", "ocean_w", junction["ridge_s"], None
        )
    elif name == "megashear":
        for block in world.continental_blocks.values():
            block.assemblage_id = "joined_supercontinent"
        world.metrics["megashear_slip_km"] = 0.0
    elif name == "tethys_ocean":
        world.features["arcuate_continent"] = Feature(
            "arcuate_continent",
            "continental_margin",
            polyline_from_latlon([(30, -70), (48, -30), (52, 20), (38, 70)]),
            "continent_w",
        )
        world.boundaries["west_trench"].concept = "inside arcuate continent trench"
    elif name == "complex_collision":
        world.plates["continent_w"].omega = np.array([0.0, 0.0, 0.0032])
        world.plates["continent_e"].omega = np.array([0.0, 0.0, -0.0030])
        world.features["irregular_west_coast"] = Feature(
            "irregular_west_coast",
            "continental_margin",
            polyline_from_latlon([(-45, -15), (-18, -2), (2, -15), (24, 1), (48, -12)]),
            "continent_w",
        )
        world.features["irregular_east_coast"] = Feature(
            "irregular_east_coast",
            "continental_margin",
            polyline_from_latlon([(-44, 18), (-20, 4), (4, 20), (28, 7), (49, 17)]),
            "continent_e",
        )


def _effect(world: WorldState, effect: dict[str, Any]) -> str:
    operation = effect["op"]
    if operation == "feature":
        points = _point_geometry(effect)
        world.features[effect["id"]] = Feature(
            effect["id"],
            effect["kind"],
            points,
            effect.get("plate"),
            effect.get("active", True),
            dict(effect.get("properties", {})),
        )
        return f"feature {effect['id']} created as {effect['kind']} with {len(points)} spherical samples"
    if operation == "feature_from_boundary":
        boundary = world.boundaries[effect["boundary"]]
        points = rodrigues(
            boundary.points,
            np.array([0.0, 0.0, math.radians(effect.get("offset_degrees", 0.0))]),
            1.0,
        )
        world.features[effect["id"]] = Feature(
            effect["id"],
            effect["kind"],
            points,
            effect.get("plate"),
            True,
            dict(effect.get("properties", {})),
        )
        return f"feature {effect['id']} created from shifted boundary {boundary.id} with {len(points)} samples"
    if operation == "feature_off":
        world.features[effect["id"]].active = False
        return f"feature {effect['id']} set inactive"
    if operation == "feature_property":
        world.features[effect["id"]].properties[effect["key"]] = effect["value"]
        return f"feature {effect['id']}.{effect['key']} set to {effect['value']}"
    if operation == "boundary":
        points = _point_geometry(effect)
        world.boundaries[effect["id"]] = Boundary(
            effect["id"], effect["concept"], effect["left"], effect["right"], points, effect.get("polarity")
        )
        return f"boundary {effect['id']} created between {effect['left']} and {effect['right']} with {len(points)} samples"
    if operation == "boundary_off":
        world.boundaries[effect["id"]].active = False
        return f"boundary {effect['id']} set inactive"
    if operation == "boundary_property":
        boundary = world.boundaries[effect["id"]]
        boundary.properties[effect["key"]] = effect["value"]
        return f"boundary {effect['id']}.{effect['key']} set to {effect['value']}"
    if operation == "boundary_shift":
        boundary = world.boundaries[effect["id"]]
        boundary.points = rodrigues(boundary.points, np.array([0.0, 0.0, math.radians(effect["degrees"])]), 1.0)
        return f"boundary {effect['id']} retreated {effect['degrees']} degrees by spherical rotation"
    if operation == "plate":
        world.plates[effect["id"]] = Plate(
            effect["id"],
            effect["composition"],
            np.asarray(effect["omega"], dtype=float),
            from_latlon(*effect["reference"]),
            effect.get("inertia", 1.0),
        )
        return f"plate {effect['id']} created with Euler omega {effect['omega']}"
    if operation == "omega":
        before = world.plates[effect["id"]].omega.copy()
        world.plates[effect["id"]].omega = np.asarray(effect["value"], dtype=float)
        return f"plate {effect['id']} omega changed from {before.tolist()} to {effect['value']}"
    if operation == "plate_property":
        world.plates[effect["id"]].properties[effect["key"]] = effect["value"]
        return f"plate {effect['id']}.{effect['key']} set to {effect['value']}"
    if operation == "block":
        world.continental_blocks[effect["id"]] = ContinentalBlock(
            effect["id"],
            effect["plate"],
            _block_points(effect["center"][0], effect["center"][1], effect.get("width", 5.0)),
            effect.get("assemblage", effect["id"]),
            properties=dict(effect.get("properties", {})),
        )
        return f"continental block {effect['id']} created on plate {effect['plate']}"
    if operation == "collision":
        first = world.continental_blocks[effect["first"]]
        second = world.continental_blocks[effect["second"]]
        record = {"time_myr": world.elapsed_myr, "with": second.id, "outcome": effect["outcome"]}
        first.collision_history.append(record)
        second.collision_history.append({**record, "with": first.id})
        assemblage = effect.get("assemblage", f"{first.id}+{second.id}")
        first.assemblage_id = assemblage
        second.assemblage_id = assemblage
        if effect.get("join_motion", False):
            world.plates[first.plate_id].properties["rigid_assemblage"] = assemblage
            world.plates[second.plate_id].properties["rigid_assemblage"] = assemblage
            world.plates[second.plate_id].omega = world.plates[first.plate_id].omega.copy()
        return f"blocks {first.id} and {second.id} joined assemblage {assemblage}; outcome {effect['outcome']}"
    if operation == "block_plate":
        block = world.continental_blocks[effect["id"]]
        old = block.plate_id
        block.plate_id = effect["plate"]
        return f"block {block.id} transferred from plate {old} to {effect['plate']}"
    if operation == "markers":
        start = len(world.crust_markers)
        for index in range(effect.get("count", 5)):
            world.crust_markers.append(
                CrustMarker(
                    f"{effect['id']}_{index:02d}",
                    effect["plate"],
                    from_latlon(-30 + index * 12, effect.get("longitude", 0) + index * 0.8),
                    effect.get("age", 0.0),
                )
            )
        return f"{len(world.crust_markers) - start} crust markers added to plate {effect['plate']}"
    if operation == "ridge_crust":
        count = effect.get("count", 4)
        longitude = effect.get("longitude", 0.0)
        offset = effect.get("offset", 1.5)
        for side, plate_id, sign in (
            ("left", effect["left"], -1.0),
            ("right", effect["right"], 1.0),
        ):
            for index in range(count):
                world.crust_markers.append(
                    CrustMarker(
                        f"{effect['id']}_{side}_{index:02d}",
                        plate_id,
                        from_latlon(-24 + index * 48 / max(1, count - 1), longitude + sign * offset),
                        effect.get("age", 0.0),
                    )
                )
        return f"ridge created {count} crust markers on each of plates {effect['left']} and {effect['right']}"
    if operation == "basin_width":
        before = world.metrics.get(effect["metric"], 0.0)
        world.metrics[effect["metric"]] = float(effect["value"])
        return f"calculated basin metric {effect['metric']} changed from {before} to {effect['value']} km after topology closure"
    if operation == "consume_markers":
        candidates = sorted(
            [marker for marker in world.crust_markers if marker.plate_id == effect["plate"]],
            key=lambda marker: (-marker.age_myr, marker.id),
        )
        count = max(1, int(len(candidates) * effect.get("fraction", 0.5))) if candidates else 0
        removed = {marker.id for marker in candidates[:count]}
        world.crust_markers = [marker for marker in world.crust_markers if marker.id not in removed]
        return f"trench consumed {count} oldest crust markers from plate {effect['plate']}"
    if operation == "hinge_arc":
        base = meridian(effect.get("longitude", 0.0), effect.get("count", 13), effect.get("latitude_limit", 55.0))
        points = hinge_arc(
            base,
            world.plates[effect["left"]].omega,
            world.plates[effect["right"]].omega,
            effect.get("elapsed", 120.0),
        )
        world.boundaries[effect["id"]] = Boundary(
            effect["id"], "rift-propagated arcuate trench", effect["left"], effect["right"], points, effect["polarity"]
        )
        world.features[effect["arc_id"]] = Feature(
            effect["arc_id"], "island_arc", rodrigues(points, np.array([0.0, 0.0, 0.035]), 1.0), effect["right"]
        )
        return f"hinge advection created {len(points)} non-flat trench and island-arc samples"
    raise ValueError(f"unknown scenario effect: {operation}")


def _diagnostic_inputs(world: WorldState) -> dict[str, float]:
    active = [boundary for boundary in world.boundaries.values() if boundary.active and boundary.diagnostics]
    convergence = max((boundary.mean_rate("convergence_rate") for boundary in active), default=0.0)
    divergence = max((boundary.mean_rate("divergence_rate") for boundary in active), default=0.0)
    shear = max((boundary.mean_rate("shear_rate") for boundary in active), default=0.0)
    ages = [marker.age_myr for marker in world.crust_markers]
    inputs = {
        "progress_fraction": world.progress,
        "maximum_convergence_km_per_myr": convergence,
        "maximum_divergence_km_per_myr": divergence,
        "maximum_shear_km_per_myr": shear,
        "mean_ocean_crust_age_myr": float(np.mean(ages)) if ages else 0.0,
        "active_boundary_count": float(len(active)),
        "inherited_fault_present": float(
            "inherited_margin_fault" in world.features
            and world.features["inherited_margin_fault"].active
        ),
    }
    for boundary in active:
        prefix = boundary.id
        inputs[f"{prefix}_convergence_km_per_myr"] = boundary.mean_rate("convergence_rate")
        inputs[f"{prefix}_divergence_km_per_myr"] = boundary.mean_rate("divergence_rate")
        inputs[f"{prefix}_normal_speed_km_per_myr"] = (
            inputs[f"{prefix}_convergence_km_per_myr"]
            + inputs[f"{prefix}_divergence_km_per_myr"]
        )
        inputs[f"{prefix}_shear_km_per_myr"] = boundary.mean_rate("shear_rate")
    mean_ages: dict[str, float] = {}
    for plate_id in world.plates:
        inputs[f"plate_{plate_id}_present"] = 1.0
        plate_ages = [marker.age_myr for marker in world.crust_markers if marker.plate_id == plate_id]
        mean_ages[plate_id] = float(np.mean(plate_ages)) if plate_ages else 0.0
        inputs[f"{plate_id}_mean_crust_age_myr"] = mean_ages[plate_id]
        inputs[f"{plate_id}_youngest_crust_age_myr"] = min(plate_ages, default=0.0)
        speed = np.linalg.norm(
            np.cross(world.plates[plate_id].omega, world.radius_km * world.plates[plate_id].reference_position)
        )
        inputs[f"{plate_id}_speed_cm_per_year"] = float(speed / 10.0)
    inputs["ocean_plate_age_contrast_myr"] = abs(
        mean_ages.get("ocean_w", 0.0) - mean_ages.get("ocean_e", 0.0)
    )
    inputs["continent_rotation_difference_rad_per_myr"] = float(
        np.linalg.norm(world.plates["continent_w"].omega - world.plates["continent_e"].omega)
    )
    for feature_id, feature in world.features.items():
        inputs[f"feature_{feature_id}_active"] = float(feature.active)
    for block_id, block in world.continental_blocks.items():
        inputs[f"block_{block_id}_present"] = 1.0
        inputs[f"block_{block_id}_collision_count"] = float(len(block.collision_history))
    for key, value in world.metrics.items():
        inputs[f"metric_{key}"] = float(value)
    microcontinent = world.continental_blocks.get("microcontinent")
    if microcontinent is None:
        inputs["microcontinent_width_degrees"] = 0.0
    else:
        dots = microcontinent.sample_points @ microcontinent.sample_points.T
        inputs["microcontinent_width_degrees"] = math.degrees(
            math.acos(float(np.clip(np.min(dots), -1.0, 1.0)))
        )
    triple_ridges = [
        world.boundaries[boundary_id].mean_rate("shear_rate")
        for boundary_id in ("ridge_nw", "ridge_ne", "ridge_s")
        if boundary_id in world.boundaries and world.boundaries[boundary_id].active
    ]
    inputs["triple_ridge_rotational_shear_km_per_myr"] = max(triple_ridges, default=0.0)
    inputs["triple_ridge_divergence_km_per_myr"] = max(
        (
            world.boundaries[boundary_id].mean_rate("divergence_rate")
            for boundary_id in ("ridge_nw", "ridge_ne", "ridge_s")
            if boundary_id in world.boundaries and world.boundaries[boundary_id].active
        ),
        default=0.0,
    )

    def pair_closure(first_id: str, second_id: str) -> float:
        if first_id not in world.plates or second_id not in world.plates:
            return 0.0
        first = world.plates[first_id]
        second = world.plates[second_id]
        sine = np.linalg.norm(np.cross(first.reference_position, second.reference_position))
        if sine < 1.0e-12:
            return 0.0
        first_velocity = np.cross(first.omega, world.radius_km * first.reference_position)
        second_velocity = np.cross(second.omega, world.radius_km * second.reference_position)
        cosine_rate = (
            np.dot(first_velocity, second.reference_position)
            + np.dot(first.reference_position, second_velocity)
        ) / world.radius_km
        return float(world.radius_km * cosine_rate / sine)

    inputs["continent_pair_closure_km_per_myr"] = pair_closure("continent_w", "continent_e")
    inputs["continent_pair_normal_speed_km_per_myr"] = abs(inputs["continent_pair_closure_km_per_myr"])
    inputs["east_continent_arc_closure_km_per_myr"] = pair_closure("continent_e", "arc_plate")
    inputs["west_continent_ocean_closure_km_per_myr"] = pair_closure("continent_w", "ocean_w")
    return inputs


# These are authored scenario thresholds, not predictive geophysics. They
# make each uncertain initiation depend on its local calculated mechanism
# rather than on an unrelated active boundary. Sources are on each Transition.
RULE_CONDITIONS: dict[str, tuple[tuple[str, str, float], ...]] = {
    "plume-weakened triple-arm nucleation": (("west_trench_convergence_km_per_myr", ">=", 10.0),),
    "zigzag connection and failed arms": (("continent_rotation_difference_rad_per_myr", ">=", 0.001),),
    "rift-to-ridge transition": (("main_ridge_divergence_km_per_myr", ">=", 10.0),),
    "asymmetric moving-continent margins": (("main_ridge_divergence_km_per_myr", ">=", 10.0),),
    "imbalanced exterior ridge drift and capture": (("exterior_ridge_normal_speed_km_per_myr", ">=", 1.0),),
    "far-side continent collision": (("metric_exterior_ocean_width_km", "<=", 15000.0),),
    "one-sided young-ocean subduction initiation": (
        ("inherited_fault_present", ">=", 1.0),
        ("ocean_w_youngest_crust_age_myr", "<=", 90.0),
        ("west_trench_convergence_km_per_myr", ">=", 10.0),
    ),
    "interior ridge consumption and motion reversal": (("interior_trench_convergence_km_per_myr", ">=", 10.0),),
    "interior Wilson-cycle collision": (("continent_pair_closure_km_per_myr", ">=", 10.0),),
    "flat-slab dip reduction": (
        ("continent_w_speed_cm_per_year", ">=", 2.0),
        ("ocean_w_mean_crust_age_myr", "<=", 90.0),
        ("west_trench_convergence_km_per_myr", ">=", 10.0),
    ),
    "ridge-subduction slab window": (("west_trench_convergence_km_per_myr", ">=", 10.0),),
    "old-crust slab steepening and trench retreat": (
        ("ocean_w_mean_crust_age_myr", ">=", 200.0),
        ("west_trench_convergence_km_per_myr", ">=", 10.0),
    ),
    "back-arc extension": (("west_trench_convergence_km_per_myr", ">=", 10.0),),
    "offshore trench arc production": (("east_trench_convergence_km_per_myr", ">=", 10.0),),
    "intervening basin consumption": (
        ("east_trench_convergence_km_per_myr", ">=", 5.0),
        ("feature_offshore_arc_active", ">=", 1.0),
    ),
    "arc collision and margin advance": (("arc_basin_trench_convergence_km_per_myr", ">=", 5.0),),
    "thin-fragment trench jump": (
        ("microcontinent_width_degrees", "<=", 12.0),
        ("west_trench_convergence_km_per_myr", ">=", 10.0),
    ),
    "large-continent trench termination": (("jumped_trench_convergence_km_per_myr", ">=", 1.0),),
    "continent-attached plate approaches arc": (("west_trench_convergence_km_per_myr", ">=", 10.0),),
    "collision-induced polarity reversal": (("west_trench_normal_speed_km_per_myr", ">=", 1.0),),
    "uneven-torque hinge rotation": (("continent_rotation_difference_rad_per_myr", ">=", 0.003),),
    "rift-cut trench propagation": (("continent_rotation_difference_rad_per_myr", ">=", 0.003),),
    "transform-gateway collapse": (
        ("gateway_transform_shear_km_per_myr", ">=", 20.0),
        ("ocean_plate_age_contrast_myr", ">=", 50.0),
    ),
    "interior-ocean trench propagation": (("gateway_trench_convergence_km_per_myr", ">=", 10.0),),
    "rotational triple-junction gap": (("triple_ridge_rotational_shear_km_per_myr", ">=", 10.0),),
    "central plate ridge enclosure": (
        ("plate_central_plate_present", ">=", 1.0),
        ("triple_ridge_divergence_km_per_myr", ">=", 5.0),
    ),
    "peripheral consumption of old plates": (("central_ridge_w_convergence_km_per_myr", ">=", 5.0),),
    "continental megashear displacement": (("megashear_shear_km_per_myr", ">=", 10.0),),
    "arcuate-shore microcontinent tear": (("west_trench_convergence_km_per_myr", ">=", 10.0),),
    "small Tethys collision and trench jump": (
        ("west_trench_convergence_km_per_myr", ">=", 5.0),
        ("block_tethys_micro_1_present", ">=", 1.0),
    ),
    "repeated Tethys fragment accretion": (
        ("tethys_jumped_trench_convergence_km_per_myr", ">=", 5.0),
        ("block_tethys_micro_1_collision_count", ">=", 1.0),
    ),
    "final Tethys plateau collision": (
        ("continent_pair_closure_km_per_myr", ">=", 10.0),
        ("block_tethys_micro_2_collision_count", ">=", 1.0),
    ),
    "asynchronous coast contact": (("continent_pair_closure_km_per_myr", ">=", 10.0),),
    "enclosed-sea rollback and back-arc fracture": (
        ("continent_pair_closure_km_per_myr", ">=", 1.0),
        ("feature_trapped_ocean_pocket_active", ">=", 1.0),
    ),
    "Mediterranean-like invasion mosaic": (
        ("feature_enclosed_back_arc_active", ">=", 1.0),
        ("continent_pair_normal_speed_km_per_myr", ">=", 1.0),
    ),
}


def _condition_passes(value: float, operator: str, threshold: float) -> bool:
    if operator == ">=":
        return value + 1.0e-12 >= threshold
    if operator == "<=":
        return value - 1.0e-12 <= threshold
    raise ValueError(f"unsupported heuristic condition operator {operator!r}")


def _mutation_inventory(world: WorldState) -> dict[str, dict[str, Any]]:
    return {
        "plates": {plate_id: plate.to_dict() for plate_id, plate in sorted(world.plates.items())},
        "boundaries": {
            boundary_id: {
                "id": boundary.id,
                "concept": boundary.concept,
                "left_plate": boundary.left_plate,
                "right_plate": boundary.right_plate,
                "points": boundary.points.tolist(),
                "polarity": boundary.polarity,
                "active": boundary.active,
                "properties": copy.deepcopy(boundary.properties),
            }
            for boundary_id, boundary in sorted(world.boundaries.items())
        },
        "ocean_crust_markers": {
            marker.id: marker.to_dict() for marker in sorted(world.crust_markers, key=lambda item: item.id)
        },
        "continental_blocks": {
            block_id: block.to_dict() for block_id, block in sorted(world.continental_blocks.items())
        },
        "features": {feature_id: feature.to_dict() for feature_id, feature in sorted(world.features.items())},
        "metrics": {key: float(value) for key, value in sorted(world.metrics.items())},
    }


def _exact_state_diff(
    before: dict[str, dict[str, Any]], after: dict[str, dict[str, Any]]
) -> list[dict[str, Any]]:
    changes: list[dict[str, Any]] = []
    for section in sorted(set(before) | set(after)):
        old_items = before.get(section, {})
        new_items = after.get(section, {})
        for key in sorted(set(old_items) | set(new_items)):
            old = old_items.get(key)
            new = new_items.get(key)
            if old != new:
                changes.append({"path": f"/{section}/{key}", "before": old, "after": new})
    return changes


def _triple_arm_separations(world: WorldState) -> list[float]:
    center = from_latlon(0.0, 0.0)
    directions = []
    for feature_id in ("rift_arm_n", "rift_arm_sw", "rift_arm_se"):
        endpoint = world.features[feature_id].points[-1]
        directions.append(unit(endpoint - np.dot(endpoint, center) * center))
    return sorted(
        math.degrees(math.acos(float(np.clip(np.dot(first, second), -1.0, 1.0))))
        for index, first in enumerate(directions)
        for second in directions[index + 1 :]
    )


def _observation_key(rule: str, index: int) -> str:
    return f"{re.sub(r'[^a-z0-9]+', '_', rule.lower()).strip('_')}_{index + 1}"


def _fire_heuristic(world: WorldState, transition: Transition) -> bool:
    if transition.rule in world.fired_rules:
        return False
    inputs = _diagnostic_inputs(world)
    activity = max(
        inputs["maximum_convergence_km_per_myr"],
        inputs["maximum_divergence_km_per_myr"],
        inputs["maximum_shear_km_per_myr"],
    )
    if world.progress + 1.0e-12 < transition.progress_threshold:
        return False
    if activity < transition.minimum_activity_km_per_myr:
        return False
    try:
        conditions = RULE_CONDITIONS[transition.rule]
    except KeyError as error:
        raise RuntimeError(f"heuristic {transition.rule!r} has no mechanism-specific condition") from error
    if not all(_condition_passes(inputs.get(metric, 0.0), operator, value) for metric, operator, value in conditions):
        return False

    before = _mutation_inventory(world)
    descriptions = [_effect(world, effect) for effect in transition.effects]
    world.refresh_boundary_membership()
    classify_boundaries(world)
    changes = _exact_state_diff(before, _mutation_inventory(world))
    if not changes:
        raise RuntimeError(f"heuristic {transition.rule!r} produced no state mutation")

    condition_evidence = [
        {"metric": metric, "actual": inputs[metric], "operator": operator, "threshold": value}
        for metric, operator, value in conditions
    ]
    if transition.rule == "plume-weakened triple-arm nucleation":
        separations = _triple_arm_separations(world)
        if min(separations) < 100.0 or max(separations) > 140.0:
            raise RuntimeError(f"three-arm rift angles are not approximately 120 degrees: {separations}")
        condition_evidence.append(
            {"metric": "rift_arm_separations_degrees", "actual": separations, "operator": "within", "threshold": [100.0, 140.0]}
        )

    world.fired_rules.add(transition.rule)
    threshold = {
        "progress_fraction_at_least": transition.progress_threshold,
        "calculated_boundary_activity_km_per_myr_at_least": transition.minimum_activity_km_per_myr,
        "mechanism_conditions": [
            {"metric": metric, "operator": operator, "value": value}
            for metric, operator, value in conditions
        ],
    }
    world.events.append(
        Event(
            time_myr=world.elapsed_myr,
            step=world.step_index,
            rule=transition.rule,
            inputs=inputs,
            threshold=threshold,
            state_changes=changes,
            effect_descriptions=descriptions,
            citation=transition.citation,
        )
    )
    evidence = [{"verified_conditions": condition_evidence, "exact_mutation_paths": [item["path"] for item in changes]}]
    for index, description in enumerate(transition.observations):
        key = _observation_key(transition.rule, index)
        world.observations[key] = Observation(
            key=key,
            description=description,
            achieved=bool(changes) and all(
                _condition_passes(inputs[metric], operator, value) for metric, operator, value in conditions
            ),
            time_myr=world.elapsed_myr,
            evidence=copy.deepcopy(evidence),
        )
    return True


def run_scenario(name: str, steps: int = 40, seed: int = 7) -> SimulationResult:
    if name not in SCENARIOS:
        raise KeyError(f"unknown scenario {name!r}")
    if steps < 3:
        raise ValueError("steps must be at least 3 so initial, decisive, and final states differ")
    definition = SCENARIOS[name]
    world = _make_base_world(name, seed)
    world.milestones.append(Milestone("initial", 0, 0.0))
    history = [world.snapshot()]
    milestone_indices = {"initial": 0}
    dt = world.duration_myr / steps
    for step in range(1, steps + 1):
        advance_calculated_state(world, dt)
        world.step_index = step
        world.elapsed_myr = step * dt
        for transition in definition.transitions:
            fired = _fire_heuristic(world, transition)
            if fired and transition.decisive and "decisive_transition" not in milestone_indices:
                world.milestones.append(Milestone("decisive_transition", step, world.elapsed_myr))
                milestone_indices["decisive_transition"] = len(history)
        classify_boundaries(world)
        history.append(world.snapshot())
    if "decisive_transition" not in milestone_indices:
        raise RuntimeError(f"scenario {name} did not reach its decisive transition")
    world.milestones.append(Milestone("final", steps, world.elapsed_myr))
    history[-1] = world.snapshot()
    milestone_indices["final"] = len(history) - 1
    return SimulationResult(definition, history, milestone_indices)


# Each observation below mirrors one semicolon-delimited completion item in
# README.md.  Effects mutate geometry, plate motion, markers, or persistent
# feature/block state before the observation is recorded.
SCENARIOS: dict[str, ScenarioDefinition] = {
    "supercontinent_breakup": ScenarioDefinition(
        "supercontinent_breakup",
        "Plume-assisted three-arm rifting and staged fragment rotation.",
        (
            _transition(
                "plume-weakened triple-arm nucleation",
                0.12,
                "research.md#supercontinent-breakup-images-002-004",
                [
                    {"op": "feature", "id": "breakup_plume", "kind": "hotspot", "point": (0, 0), "properties": {"mantle_fixed": True}},
                    {"op": "feature", "id": "breakup_lip", "kind": "large_igneous_province", "point": (4, 2)},
                    {"op": "feature", "id": "rift_arm_n", "kind": "active_rift", "coordinates": [(0, 0), (40, 0)]},
                    {"op": "feature", "id": "rift_arm_sw", "kind": "active_rift", "coordinates": [(0, 0), (-19, -36)]},
                    {"op": "feature", "id": "rift_arm_se", "kind": "active_rift", "coordinates": [(0, 0), (-19, 36)]},
                ],
                ["Plume uplift weakens the interior and nucleates a rift.", "The rift starts as three arms about 120 degrees apart."],
            ),
            _transition(
                "zigzag connection and failed arms",
                0.36,
                "research.md#supercontinent-breakup-images-005-007",
                [
                    {"op": "feature", "id": "zigzag_rift", "kind": "active_rift", "coordinates": [(-50, -30), (-18, -5), (12, -22), (48, 4)]},
                    {"op": "feature", "id": "failed_rift_w", "kind": "failed_rift", "coordinates": [(0, -5), (-30, -48)]},
                    {"op": "feature", "id": "failed_rift_e", "kind": "failed_rift", "coordinates": [(10, -20), (18, 28)]},
                    {"op": "omega", "id": "continent_w", "value": [0.0015, -0.0010, -0.0030]},
                    {"op": "omega", "id": "continent_e", "value": [-0.0012, 0.0016, 0.0032]},
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, 0.014]},
                    {"op": "omega", "id": "ocean_e", "value": [0.0, 0.0, -0.014]},
                ],
                ["Neighboring rift arms join into a zigzag boundary.", "Unused third arms stop as persistent failed rifts.", "Continental fragments rotate apart around distinct Euler poles."],
                decisive=True,
            ),
            _transition(
                "rift-to-ridge transition",
                0.68,
                "research.md#supercontinent-breakup-images-008-010",
                [
                    {"op": "feature_off", "id": "zigzag_rift"},
                    {"op": "feature", "id": "new_ocean_ridge", "kind": "ridge", "meridian": 0},
                    {"op": "feature", "id": "west_passive_margin", "kind": "passive_margin", "meridian": -18},
                    {"op": "feature", "id": "east_passive_margin", "kind": "passive_margin", "meridian": 18},
                    {"op": "ridge_crust", "id": "new_crust", "left": "ocean_w", "right": "ocean_e", "count": 6, "longitude": 0, "age": 0.0},
                ],
                ["A mid-ocean ridge and matching passive margins replace the active rift."],
            ),
        ),
    ),
    "extroversion": ScenarioDefinition(
        "extroversion",
        "Interior spreading survives while the old exterior ocean closes.",
        (
            _transition(
                "asymmetric moving-continent margins",
                0.16,
                "research.md#extroversion-images-009-011",
                [
                    {"op": "feature", "id": "leading_active_w", "kind": "active_margin", "meridian": -68},
                    {"op": "feature", "id": "trailing_passive_w", "kind": "passive_margin", "meridian": -18},
                    {"op": "feature_property", "id": "leading_active_w", "key": "calculated_motion", "value": "leading"},
                ],
                ["Calculated spreading grows the interior ocean while trench convergence shrinks the exterior ocean.", "An active leading margin and passive trailing margin form on a moving continent."],
            ),
            _transition(
                "imbalanced exterior ridge drift and capture",
                0.48,
                "research.md#extroversion-images-012-013",
                [
                    {"op": "boundary_shift", "id": "exterior_ridge", "degrees": 24},
                    {"op": "boundary_off", "id": "exterior_ridge"},
                    {"op": "feature", "id": "ridge_trench_capture", "kind": "trench", "meridian": -68},
                    {"op": "consume_markers", "plate": "ocean_w", "fraction": 0.45},
                ],
                ["Velocity imbalance drifts the exterior ridge into a trench.", "Loss of the exterior ridge removes its crust source and lets the exterior ocean close."],
                decisive=True,
            ),
            _transition(
                "far-side continent collision",
                0.84,
                "research.md#extroversion-images-013-014",
                [
                    {"op": "collision", "first": "west_continent", "second": "east_continent", "outcome": "large-continent suture", "assemblage": "extroverted_supercontinent", "join_motion": True},
                    {"op": "feature", "id": "extroversion_suture", "kind": "suture", "meridian": 178},
                    {"op": "feature", "id": "extroversion_orogeny", "kind": "orogeny", "meridian": 174, "properties": {"style": "Ural"}},
                    {"op": "consume_markers", "plate": "ocean_e", "fraction": 1.0},
                    {"op": "basin_width", "metric": "exterior_ocean_width_km", "value": 0.0},
                    {"op": "boundary_off", "id": "west_trench"},
                    {"op": "boundary_off", "id": "east_trench"},
                ],
                ["Far-side continent collision creates a persistent suture and collision orogeny."],
            ),
        ),
    ),
    "introversion": ScenarioDefinition(
        "introversion",
        "One-sided interior subduction reverses separation and closes the young ocean.",
        (
            _transition(
                "one-sided young-ocean subduction initiation",
                0.20,
                "research.md#introversion-images-015-020",
                [
                    {"op": "boundary", "id": "interior_trench", "concept": "interior trench", "left": "continent_w", "right": "ocean_w", "polarity": "ocean_w", "meridian": -28},
                    {"op": "feature", "id": "interior_active_margin", "kind": "trench", "meridian": -28},
                    {"op": "omega", "id": "continent_w", "value": [0.0, 0.0, -0.002]},
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, 0.012]},
                ],
                ["The young interior ocean gains deterministic one-sided subduction along inherited weakness."],
            ),
            _transition(
                "interior ridge consumption and motion reversal",
                0.44,
                "research.md#introversion-images-020-021",
                [
                    {"op": "boundary_off", "id": "main_ridge"},
                    {"op": "consume_markers", "plate": "ocean_w", "fraction": 0.55},
                    {"op": "omega", "id": "continent_w", "value": [0.0, 0.0, 0.0038]},
                    {"op": "omega", "id": "continent_e", "value": [0.0, 0.0, -0.0035]},
                ],
                ["One-sided consumption captures the interior ridge.", "Slab pull reverses the fragments' calculated relative motion."],
                decisive=True,
            ),
            _transition(
                "interior Wilson-cycle collision",
                0.82,
                "research.md#introversion-images-021-022",
                [
                    {"op": "collision", "first": "west_continent", "second": "east_continent", "outcome": "interior-ocean closure", "assemblage": "introverted_supercontinent", "join_motion": True},
                    {"op": "feature", "id": "interior_suture", "kind": "suture", "meridian": 0},
                    {"op": "feature", "id": "surviving_exterior_ocean", "kind": "ocean_basin", "meridian": 175, "properties": {"survives": True}},
                    {"op": "consume_markers", "plate": "ocean_w", "fraction": 1.0},
                    {"op": "basin_width", "metric": "interior_ocean_width_km", "value": 0.0},
                    {"op": "boundary_off", "id": "interior_trench"},
                ],
                ["The interior ocean closes while the exterior ocean remains an active basin."],
            ),
        ),
    ),
    "flat_slab": ScenarioDefinition(
        "flat_slab",
        "Fast overriding motion and buoyant young crust flatten a slab.",
        (
            _transition(
                "flat-slab dip reduction",
                0.22,
                "research.md#flat-slab-subduction-images-025-026",
                [
                    {"op": "boundary_property", "id": "west_trench", "key": "slab_dip_degrees", "value": 18.0},
                    {"op": "boundary_property", "id": "west_trench", "key": "cause", "value": "fast overriding motion plus young buoyant crust"},
                    {"op": "feature", "id": "broad_inland_uplift", "kind": "orogeny", "coordinates": [(-52, -45), (0, -32), (52, -42)], "properties": {"style": "Laramide", "width_km": 1050}},
                    {"op": "feature", "id": "inland_arc", "kind": "island_arc", "meridian": -38, "properties": {"migration": "inland"}},
                ],
                ["Fast overriding motion and young buoyant crust lower the slab dip.", "Compression and uplift widen far inland.", "The volcanic arc migrates inland as the mantle wedge narrows."],
                decisive=True,
            ),
            _transition(
                "ridge-subduction slab window",
                0.60,
                "research.md#flat-slab-subduction-image-026",
                [
                    {"op": "boundary_off", "id": "main_ridge"},
                    {"op": "feature_property", "id": "inland_arc", "key": "volcanism", "value": "inactive over flat segment"},
                    {"op": "feature", "id": "slab_window", "kind": "slab_window", "point": (0, -22), "properties": {"ridge_subducted": True}},
                ],
                ["Ridge capture breaks the slab, opens a slab window, and switches off part of the arc."],
            ),
        ),
    ),
    "slab_rollback": ScenarioDefinition(
        "slab_rollback",
        "Old crust steepens the slab and retreats the trench into the ocean.",
        (
            _transition(
                "old-crust slab steepening and trench retreat",
                0.24,
                "research.md#slab-rollback-images-027-029",
                [
                    {"op": "boundary_property", "id": "west_trench", "key": "slab_dip_degrees", "value": 72.0},
                    {"op": "boundary_property", "id": "west_trench", "key": "supply", "value": "slow and progressively older"},
                    {"op": "boundary_shift", "id": "west_trench", "degrees": 12},
                    {"op": "feature_from_boundary", "id": "retreating_island_arc", "kind": "island_arc", "boundary": "west_trench", "plate": "continent_w", "offset_degrees": 2.0, "properties": {"migration": "seaward with retreated trench"}},
                    {"op": "omega", "id": "continent_w", "value": [0.0, 0.0, -0.001]},
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, 0.015]},
                ],
                ["Slow supply delivers old dense crust and steepens the slab.", "The trench retreats toward the ocean.", "Calculated slab suction moves the overriding edge and its island arc seaward."],
                decisive=True,
            ),
            _transition(
                "back-arc extension",
                0.52,
                "research.md#slab-rollback-images-028-029",
                [
                    {"op": "feature", "id": "back_arc_basin", "kind": "back_arc_basin", "meridian": -44, "properties": {"extension": True}},
                    {"op": "feature", "id": "back_arc_rift", "kind": "active_rift", "meridian": -43},
                    {"op": "markers", "id": "back_arc_crust", "plate": "continent_w", "count": 4, "longitude": -44, "age": 0.0},
                ],
                ["Extension behind the migrating arc opens a back-arc basin."],
            ),
        ),
    ),
    "arc_accretion": ScenarioDefinition(
        "arc_accretion",
        "An offshore volcanic arc collides and becomes continental crust.",
        (
            _transition(
                "offshore trench arc production",
                0.18,
                "research.md#island-arc-growth-images-030-032",
                [
                    {"op": "feature", "id": "offshore_trench", "kind": "trench", "meridian": 57},
                    {"op": "feature_property", "id": "offshore_arc", "key": "felsic_crust_growth", "value": True},
                ],
                ["An offshore trench produces a parallel island arc on its overriding side."],
            ),
            _transition(
                "intervening basin consumption",
                0.46,
                "research.md#island-arc-accretion-images-031-032",
                [
                    {"op": "omega", "id": "arc_plate", "value": [0.0, 0.0, 0.0060]},
                    {"op": "boundary", "id": "arc_basin_trench", "concept": "arc-continent basin trench", "left": "continent_e", "right": "arc_plate", "polarity": "arc_plate", "meridian": 46},
                    {"op": "consume_markers", "plate": "ocean_e", "fraction": 0.5},
                ],
                ["Later subduction consumes the basin between the arc and continent."],
                decisive=True,
            ),
            _transition(
                "arc collision and margin advance",
                0.76,
                "research.md#island-arc-accretion-image-032",
                [
                    {"op": "collision", "first": "east_continent", "second": "offshore_arc", "outcome": "arc accretion", "assemblage": "grown_east_continent", "join_motion": True},
                    {"op": "block_plate", "id": "offshore_arc", "plate": "continent_e"},
                    {"op": "feature", "id": "arc_accretion_orogeny", "kind": "orogeny", "meridian": 48, "properties": {"style": "Ural"}},
                    {"op": "boundary_off", "id": "arc_basin_trench"},
                    {"op": "boundary", "id": "seaward_active_margin", "concept": "advanced active margin", "left": "ocean_e", "right": "continent_e", "polarity": "ocean_e", "meridian": 62},
                ],
                ["Arc collision adds its felsic block to continental crust and raises an orogeny.", "The active margin shifts seaward of the accreted arc."],
            ),
        ),
    ),
    "subduction_jump": ScenarioDefinition(
        "subduction_jump",
        "Thin and thick buoyant arrivals produce different trench outcomes.",
        (
            _transition(
                "thin-fragment trench jump",
                0.32,
                "research.md#subduction-jumping-images-033-034",
                [
                    {"op": "collision", "first": "west_continent", "second": "microcontinent", "outcome": "thin-fragment collision", "assemblage": "west_plus_micro"},
                    {"op": "feature", "id": "micro_orogeny", "kind": "orogeny", "meridian": -54, "properties": {"style": "Ural"}},
                    {"op": "boundary_off", "id": "west_trench"},
                    {"op": "boundary", "id": "jumped_trench", "concept": "far-side jumped trench", "left": "continent_w", "right": "ocean_w", "polarity": "ocean_w", "meridian": -42},
                    {"op": "omega", "id": "continent_w", "value": [0.0, 0.0, -0.002]},
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, 0.012]},
                ],
                ["A microcontinent reaches the trench and collision raises an orogeny.", "A thin fragment lets stress fracture its far side and a replacement trench starts there."],
                decisive=True,
            ),
            _transition(
                "large-continent trench termination",
                0.76,
                "research.md#subduction-jumping-images-033-037",
                [
                    {"op": "collision", "first": "west_continent", "second": "large_continent", "outcome": "large-continent collision stops subduction", "assemblage": "major_collision", "join_motion": True},
                    {"op": "boundary_off", "id": "jumped_trench"},
                    {"op": "feature", "id": "major_collision_suture", "kind": "suture", "meridian": 58},
                ],
                ["A later large continent stops the trench instead of allowing another jump."],
            ),
        ),
    ),
    "polarity_reversal": ScenarioDefinition(
        "polarity_reversal",
        "Collision closes an arc trench and reverses plate roles offshore.",
        (
            _transition(
                "continent-attached plate approaches arc",
                0.18,
                "research.md#polarity-reversal-images-035-036",
                [
                    {"op": "feature_property", "id": "initial_arc", "key": "overrides", "value": "ocean_w"},
                    {"op": "plate_property", "id": "ocean_w", "key": "attached_continent", "value": "west_continent"},
                ],
                ["A continent-attached ocean plate initially subducts beneath an island arc."],
            ),
            _transition(
                "collision-induced polarity reversal",
                0.54,
                "research.md#polarity-reversal-images-035-037",
                [
                    {"op": "boundary_off", "id": "west_trench"},
                    {"op": "collision", "first": "west_continent", "second": "east_continent", "outcome": "arc-continent collision", "assemblage": "arc_continent_assemblage", "join_motion": True},
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, -0.0040]},
                    {"op": "omega", "id": "arc_plate", "value": [0.0, 0.0, 0.0040]},
                    {"op": "boundary", "id": "reversed_trench", "concept": "reversed-polarity trench", "left": "ocean_w", "right": "arc_plate", "polarity": "arc_plate", "meridian": -48},
                    {"op": "feature", "id": "reversal_orogeny", "kind": "orogeny", "meridian": -55},
                ],
                ["Collision closes the original trench.", "A far-side trench starts with the former subducting side overriding and the former arc side subducting."],
                decisive=True,
            ),
        ),
    ),
    "rotation_arc": ScenarioDefinition(
        "rotation_arc",
        "Uneven Euler torques open a hinge and bend a propagating trench.",
        (
            _transition(
                "uneven-torque hinge rotation",
                0.24,
                "research.md#rotation-and-arc-formation-images-038-039",
                [
                    {"op": "feature", "id": "hinge_rift", "kind": "active_rift", "equator": (-32, 32)},
                    {"op": "plate_property", "id": "continent_w", "key": "hinge_role", "value": "clockwise wing"},
                    {"op": "plate_property", "id": "continent_e", "key": "hinge_role", "value": "counterclockwise wing"},
                ],
                ["Uneven calculated torques produce hinge-like plate rotation about distinct Euler poles."],
            ),
            _transition(
                "rift-cut trench propagation",
                0.48,
                "research.md#rotation-and-arc-formation-images-038-039",
                [
                    {"op": "hinge_arc", "id": "propagated_arc_trench", "arc_id": "spherical_island_arc", "left": "continent_w", "right": "continent_e", "polarity": "continent_w", "longitude": 3, "count": 15, "elapsed": 150.0},
                    {"op": "feature_property", "id": "spherical_island_arc", "key": "construction", "value": "differential Rodrigues advection"},
                ],
                ["A trench cut by the rift propagates across the opening between separating landmasses.", "Sampled trench and island-arc unit vectors form an arc through differential spherical advection."],
                decisive=True,
            ),
        ),
    ),
    "subduction_invasion": ScenarioDefinition(
        "subduction_invasion",
        "A transform gateway collapses and a trench invades a young ocean.",
        (
            _transition(
                "transform-gateway collapse",
                0.28,
                "research.md#subduction-invasion-images-040-041",
                [
                    {"op": "boundary_off", "id": "gateway_transform"},
                    # Coordinates are (latitude, longitude): this is a
                    # north-south meridian, so opposing Z-axis rotations
                    # supply trench-normal convergence rather than shear.
                    {"op": "boundary", "id": "gateway_trench", "concept": "gateway invasion trench", "left": "ocean_e", "right": "ocean_w", "polarity": "ocean_e", "coordinates": [(-35, 5), (0, 5), (35, 5)]},
                    {"op": "feature", "id": "gateway_trench_feature", "kind": "trench", "coordinates": [(-35, 5), (0, 5), (35, 5)]},
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, 0.012]},
                    {"op": "omega", "id": "ocean_e", "value": [0.0, 0.0, -0.012]},
                ],
                ["Age contrast and calculated shear collapse a transform ocean gateway into a trench."],
                decisive=True,
            ),
            _transition(
                "interior-ocean trench propagation",
                0.58,
                "research.md#subduction-invasion-image-042",
                [
                    {"op": "feature", "id": "invading_trench", "kind": "trench", "coordinates": [(0, 5), (18, 28), (36, 44), (52, 62)], "properties": {"basin": "younger interior ocean"}},
                    {"op": "boundary", "id": "attached_margin_trench", "concept": "invading trench attached to continental margin", "left": "ocean_e", "right": "continent_e", "polarity": "ocean_e", "meridian": 62},
                    {"op": "feature", "id": "attached_margin", "kind": "active_margin", "meridian": 62, "plate": "continent_e"},
                    {"op": "consume_markers", "plate": "ocean_e", "fraction": 0.35},
                ],
                ["The trench propagates into the younger interior ocean.", "The invading trench attaches to a nearby continental margin."],
            ),
        ),
    ),
    "triple_junction_plate": ScenarioDefinition(
        "triple_junction_plate",
        "Rotational ridge shear opens a central plate that grows as old plates shrink.",
        (
            _transition(
                "rotational triple-junction gap",
                0.26,
                "research.md#triple-junction-plate-formation-images-043-044",
                [
                    {"op": "plate", "id": "central_plate", "composition": "new oceanic", "omega": [0.0002, -0.0001, 0.00015], "reference": (0, 0)},
                    {"op": "feature", "id": "unattached_gap", "kind": "large_igneous_province", "point": (0, 0), "properties": {"origin": "rotational ridge gap"}},
                    {"op": "markers", "id": "central_new_crust", "plate": "central_plate", "count": 5, "longitude": 0, "age": 0.0},
                ],
                ["Three ridges initially meet while calculated rotational shear opens an unattached gap.", "Mantle fill in the gap creates a distinct new ocean plate."],
                decisive=True,
            ),
            _transition(
                "central plate ridge enclosure",
                0.48,
                "research.md#triple-junction-plate-formation-images-044-045",
                [
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, 0.006]},
                    {"op": "omega", "id": "ocean_e", "value": [0.0, 0.0, -0.006]},
                    {"op": "omega", "id": "central_plate", "value": [0.0001, -0.0001, 0.0]},
                    {"op": "boundary", "id": "central_ridge_w", "concept": "central plate ridge", "left": "ocean_w", "right": "central_plate", "polarity": None, "meridian": -8, "latitude_limit": 28},
                    {"op": "boundary", "id": "central_ridge_e", "concept": "central plate ridge", "left": "central_plate", "right": "ocean_e", "polarity": None, "meridian": 8, "latitude_limit": 28},
                    {"op": "boundary", "id": "central_ridge_s", "concept": "central plate ridge", "left": "arc_plate", "right": "central_plate", "polarity": None, "equator": (-8, 8)},
                    {"op": "ridge_crust", "id": "central_w_crust", "left": "ocean_w", "right": "central_plate", "count": 3, "longitude": -8},
                    {"op": "ridge_crust", "id": "central_e_crust", "left": "central_plate", "right": "ocean_e", "count": 3, "longitude": 8},
                    {"op": "ridge_crust", "id": "central_s_crust", "left": "arc_plate", "right": "central_plate", "count": 3, "longitude": 0},
                    {"op": "plate_property", "id": "central_plate", "key": "growth", "value": "surrounded by migrating ridges"},
                ],
                ["New ridges surround and grow the comparatively slow central plate."],
            ),
            _transition(
                "peripheral consumption of old plates",
                0.80,
                "research.md#triple-junction-plate-formation-image-045",
                [
                    {"op": "boundary_off", "id": "ridge_nw"},
                    {"op": "boundary_off", "id": "ridge_ne"},
                    {"op": "boundary_off", "id": "ridge_s"},
                    {"op": "consume_markers", "plate": "ocean_w", "fraction": 0.85},
                    {"op": "consume_markers", "plate": "ocean_e", "fraction": 0.85},
                    {"op": "plate_property", "id": "ocean_w", "key": "peripherally_consumed", "value": True},
                    {"op": "plate_property", "id": "ocean_e", "key": "peripherally_consumed", "value": True},
                ],
                ["Old plates and their ridges move toward peripheral trenches and can disappear."],
            ),
        ),
    ),
    "megashear": ScenarioDefinition(
        "megashear",
        "A deliberately disputed long-transform plausibility demonstration.",
        (
            _transition(
                "continental megashear displacement",
                0.50,
                "research.md#continental-megashear-image-046",
                [
                    {"op": "feature", "id": "megashear_trace", "kind": "transform_fault", "equator": (-32, 32), "properties": {"evidence": "plausible but poorly evidenced", "length_km": 6000}},
                    {"op": "plate_property", "id": "continent_w", "key": "assemblage_constraint", "value": "joined_supercontinent"},
                    {"op": "plate_property", "id": "continent_e", "key": "assemblage_constraint", "value": "joined_supercontinent"},
                ],
                ["Opposed calculated tangential velocity drives long transform slip while both blocks remain one assemblage; this is plausible but poorly evidenced."],
                decisive=True,
            ),
        ),
        notes=("Pangea B megashear is a plausibility demonstration, not an expected cycle stage.",),
    ),
    "tethys_ocean": ScenarioDefinition(
        "tethys_ocean",
        "Repeated microcontinent transfer ends in a broad plateau collision.",
        (
            _transition(
                "arcuate-shore microcontinent tear",
                0.20,
                "research.md#tethys-type-oceans-images-047-048",
                [
                    {"op": "plate", "id": "tethys_micro_plate", "composition": "continental fragment", "omega": [0.0005, 0.0002, -0.0068], "reference": (8, 52)},
                    {"op": "block", "id": "tethys_micro_1", "plate": "tethys_micro_plate", "center": (8, 52), "width": 4},
                    {"op": "feature", "id": "tear_rift", "kind": "active_rift", "meridian": 48, "latitude_limit": 20},
                ],
                ["One-sided subduction inside an arcuate continent tears off a brittle microcontinent.", "The microcontinent crosses the Tethys basin on its own fast Euler plate."],
            ),
            _transition(
                "small Tethys collision and trench jump",
                0.44,
                "research.md#tethys-type-oceans-images-048-049",
                [
                    {"op": "collision", "first": "west_continent", "second": "tethys_micro_1", "outcome": "small Tethys accretion", "assemblage": "tethys_north"},
                    {"op": "boundary_off", "id": "west_trench"},
                    {"op": "boundary", "id": "tethys_jumped_trench", "concept": "Tethys jumped trench", "left": "continent_w", "right": "ocean_w", "polarity": "ocean_w", "meridian": -50},
                    {"op": "feature", "id": "tethys_weak_belt_1", "kind": "orogeny", "meridian": -54, "properties": {"reactivatable": True}},
                    {"op": "omega", "id": "continent_w", "value": [0.0, 0.0, -0.002]},
                    {"op": "omega", "id": "ocean_w", "value": [0.0, 0.0, 0.012]},
                ],
                ["A small collision accretes the fragment and lets subduction jump across it."],
                decisive=True,
            ),
            _transition(
                "repeated Tethys fragment accretion",
                0.64,
                "research.md#tethys-type-oceans-images-048-050",
                [
                    {"op": "block", "id": "tethys_micro_2", "plate": "ocean_e", "center": (-4, 34), "width": 3},
                    {"op": "collision", "first": "west_continent", "second": "tethys_micro_2", "outcome": "second fragment accretion", "assemblage": "tethys_north"},
                    {"op": "feature", "id": "tethys_weak_belt_2", "kind": "orogeny", "meridian": -38, "properties": {"reactivatable": True}},
                    {"op": "omega", "id": "continent_w", "value": [0.0, 0.0, 0.006]},
                    {"op": "omega", "id": "continent_e", "value": [0.0, 0.0, -0.006]},
                ],
                ["Repeated fragment collisions leave multiple weak accretion belts."],
            ),
            _transition(
                "final Tethys plateau collision",
                0.86,
                "research.md#tethys-type-oceans-image-050",
                [
                    {"op": "collision", "first": "west_continent", "second": "east_continent", "outcome": "large rapid collision", "assemblage": "tethys_final_continent", "join_motion": True},
                    {"op": "boundary_off", "id": "tethys_jumped_trench"},
                    {"op": "feature", "id": "himalayan_plateau", "kind": "orogeny", "coordinates": [(-30, -35), (0, -24), (30, -34)], "properties": {"style": "Himalayan", "width_km": 1200, "reactivated_belts": 2}},
                ],
                ["A final large collision stops subduction and reactivates prior belts into a broad Himalayan-type plateau."],
            ),
        ),
    ),
    "complex_collision": ScenarioDefinition(
        "complex_collision",
        "Asynchronous irregular-coast collision leaves an enclosed, active sea.",
        (
            _transition(
                "asynchronous coast contact",
                0.24,
                "research.md#complex-collisions-image-051",
                [
                    {"op": "feature", "id": "early_contact_suture", "kind": "suture", "coordinates": [(24, 1), (32, 5)]},
                    {"op": "feature", "id": "trapped_ocean_pocket", "kind": "ocean_basin", "coordinates": [(-28, 5), (-10, 18), (8, 20)], "properties": {"enclosed": True}},
                    {"op": "feature", "id": "local_pocket_trench", "kind": "trench", "coordinates": [(-25, 8), (-8, 18), (5, 19)]},
                ],
                ["Irregular coast protrusions contact at different times.", "Contacted samples become sutures while embayments retain ocean pockets and local trenches."],
            ),
            _transition(
                "enclosed-sea rollback and back-arc fracture",
                0.50,
                "research.md#complex-collisions-image-052",
                [
                    {"op": "feature", "id": "enclosed_back_arc", "kind": "back_arc_basin", "coordinates": [(-22, 9), (-5, 14), (12, 13)]},
                    {"op": "feature", "id": "rollback_arc", "kind": "island_arc", "coordinates": [(-28, 13), (-7, 23), (16, 22)]},
                    {"op": "feature", "id": "fracture_islands", "kind": "hotspot", "coordinates": [(-12, 12), (0, 15), (10, 16)]},
                    {"op": "boundary_shift", "id": "west_trench", "degrees": -10},
                ],
                ["Local rollback and back-arc spreading fracture crust inside the enclosed collision zone."],
                decisive=True,
            ),
            _transition(
                "Mediterranean-like invasion mosaic",
                0.72,
                "research.md#complex-collisions-images-051-052",
                [
                    {"op": "feature", "id": "pocket_invasion_trench", "kind": "trench", "coordinates": [(-20, 12), (2, 28), (24, 18)]},
                    {"op": "feature", "id": "local_orogeny", "kind": "orogeny", "coordinates": [(20, 2), (30, 8), (38, 4)]},
                    {"op": "feature_property", "id": "trapped_ocean_pocket", "key": "result", "value": "Mediterranean-like enclosed sea"},
                    {"op": "consume_markers", "plate": "ocean_w", "fraction": 0.30},
                ],
                ["Fracture and trench invasion leave a Mediterranean-like sea with small plates, islands, peninsulas, and local orogenies."],
            ),
        ),
    ),
}

SCENARIO_NAMES = tuple(SCENARIOS)
