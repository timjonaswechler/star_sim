"""In-memory state for the plate tectonics scenario prototype."""

from __future__ import annotations

import copy
from dataclasses import dataclass, field
from typing import Any

import numpy as np


def _vector(value: np.ndarray) -> list[float]:
    return [float(component) for component in np.asarray(value, dtype=float)]


def _points(value: np.ndarray) -> list[list[float]]:
    return [_vector(point) for point in np.asarray(value, dtype=float)]


@dataclass
class Plate:
    id: str
    composition: str
    omega: np.ndarray
    reference_position: np.ndarray
    inertia: float = 1.0
    torque: np.ndarray = field(default_factory=lambda: np.zeros(3))
    accumulated_torque: np.ndarray = field(default_factory=lambda: np.zeros(3))
    boundary_ids: list[str] = field(default_factory=list)
    properties: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "composition": self.composition,
            "omega_rad_per_myr": _vector(self.omega),
            "reference_position": _vector(self.reference_position),
            "inertia": self.inertia,
            "torque": _vector(self.torque),
            "accumulated_torque": _vector(self.accumulated_torque),
            "sampled_boundaries": sorted(self.boundary_ids),
            "properties": copy.deepcopy(self.properties),
        }


@dataclass
class BoundarySample:
    point: np.ndarray
    tangent: np.ndarray
    normal: np.ndarray
    relative_velocity: np.ndarray
    reconstructed_velocity: np.ndarray
    normal_rate: float
    signed_shear_rate: float
    convergence_rate: float
    divergence_rate: float
    shear_rate: float
    classification: str
    reconstruction_residual: float

    def to_dict(self) -> dict[str, Any]:
        return {
            "position": _vector(self.point),
            "tangent": _vector(self.tangent),
            "normal": _vector(self.normal),
            "relative_velocity_km_per_myr": _vector(self.relative_velocity),
            "reconstructed_velocity_km_per_myr": _vector(self.reconstructed_velocity),
            "normal_rate_km_per_myr": self.normal_rate,
            "signed_shear_rate_km_per_myr": self.signed_shear_rate,
            "convergence_rate_km_per_myr": self.convergence_rate,
            "divergence_rate_km_per_myr": self.divergence_rate,
            "shear_rate_km_per_myr": self.shear_rate,
            "type": self.classification,
            "reconstruction_residual": self.reconstruction_residual,
        }


@dataclass
class Boundary:
    id: str
    concept: str
    left_plate: str
    right_plate: str
    points: np.ndarray
    polarity: str | None = None
    active: bool = True
    diagnostics: list[BoundarySample] = field(default_factory=list)
    properties: dict[str, Any] = field(default_factory=dict)

    def mean_rate(self, name: str) -> float:
        if not self.diagnostics:
            return 0.0
        return float(np.mean([getattr(sample, name) for sample in self.diagnostics]))

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "concept": self.concept,
            "adjacent_plates": {"left": self.left_plate, "right": self.right_plate},
            "sampled_polyline": _points(self.points),
            "endpoints": _points(np.asarray([self.points[0], self.points[-1]])),
            "subduction_polarity": self.polarity,
            "active": self.active,
            "samples": [sample.to_dict() for sample in self.diagnostics],
            "properties": copy.deepcopy(self.properties),
        }


@dataclass
class CrustMarker:
    id: str
    plate_id: str
    position: np.ndarray
    age_myr: float
    composition: str = "oceanic"

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "plate_id": self.plate_id,
            "position": _vector(self.position),
            "age_myr": self.age_myr,
            "composition": self.composition,
        }


@dataclass
class ContinentalBlock:
    id: str
    plate_id: str
    sample_points: np.ndarray
    assemblage_id: str
    collision_history: list[dict[str, Any]] = field(default_factory=list)
    properties: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "plate_id": self.plate_id,
            "sample_points": _points(self.sample_points),
            "assemblage_id": self.assemblage_id,
            "collision_history": copy.deepcopy(self.collision_history),
            "properties": copy.deepcopy(self.properties),
        }


@dataclass
class Feature:
    id: str
    kind: str
    points: np.ndarray
    plate_id: str | None = None
    active: bool = True
    properties: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "type": self.kind,
            "sampled_geometry": _points(self.points),
            "plate_id": self.plate_id,
            "active": self.active,
            "properties": copy.deepcopy(self.properties),
        }


@dataclass
class Event:
    time_myr: float
    step: int
    rule: str
    inputs: dict[str, Any]
    threshold: dict[str, Any]
    state_changes: list[dict[str, Any]]
    effect_descriptions: list[str]
    citation: str
    provenance: str = "deterministic heuristic"

    def to_dict(self) -> dict[str, Any]:
        return {
            "time_myr": self.time_myr,
            "step": self.step,
            "rule": self.rule,
            "inputs": copy.deepcopy(self.inputs),
            "threshold": copy.deepcopy(self.threshold),
            "state_changes": copy.deepcopy(self.state_changes),
            "effect_descriptions": list(self.effect_descriptions),
            "citation": self.citation,
            "provenance": self.provenance,
        }


@dataclass
class Observation:
    key: str
    description: str
    achieved: bool = False
    time_myr: float | None = None
    evidence: list[dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "key": self.key,
            "description": self.description,
            "achieved": self.achieved,
            "time_myr": self.time_myr,
            "state_evidence": copy.deepcopy(self.evidence),
        }


@dataclass
class Milestone:
    label: str
    step: int
    time_myr: float

    def to_dict(self) -> dict[str, Any]:
        return {"label": self.label, "step": self.step, "time_myr": self.time_myr}


@dataclass
class WorldState:
    scenario: str
    seed: int
    radius_km: float = 6371.0
    duration_myr: float = 300.0
    elapsed_myr: float = 0.0
    step_index: int = 0
    plates: dict[str, Plate] = field(default_factory=dict)
    boundaries: dict[str, Boundary] = field(default_factory=dict)
    crust_markers: list[CrustMarker] = field(default_factory=list)
    continental_blocks: dict[str, ContinentalBlock] = field(default_factory=dict)
    features: dict[str, Feature] = field(default_factory=dict)
    events: list[Event] = field(default_factory=list)
    observations: dict[str, Observation] = field(default_factory=dict)
    milestones: list[Milestone] = field(default_factory=list)
    metrics: dict[str, float] = field(default_factory=dict)
    driver_torques: dict[str, dict[str, np.ndarray]] = field(default_factory=dict)
    fired_rules: set[str] = field(default_factory=set)
    scenario_notes: list[str] = field(default_factory=list)

    @property
    def progress(self) -> float:
        return min(1.0, self.elapsed_myr / self.duration_myr)

    def refresh_boundary_membership(self) -> None:
        for plate in self.plates.values():
            plate.boundary_ids.clear()
        for boundary in self.boundaries.values():
            for plate_id in (boundary.left_plate, boundary.right_plate):
                if plate_id in self.plates:
                    self.plates[plate_id].boundary_ids.append(boundary.id)

    def snapshot(self) -> WorldState:
        return copy.deepcopy(self)

    def to_dict(self) -> dict[str, Any]:
        self.refresh_boundary_membership()
        return {
            "prototype": "throwaway scenario simulator",
            "motion_provenance": "calculated 3D Euler rotation and relative-velocity decomposition",
            "initial_state_provenance": "authored scenario input at step zero",
            "topology_provenance": "post-initial mutations are exact event-log diffs marked deterministic heuristic",
            "scenario": self.scenario,
            "seed": self.seed,
            "planet": {"radius_km": self.radius_km},
            "elapsed_myr": self.elapsed_myr,
            "duration_myr": self.duration_myr,
            "progress": self.progress,
            "step": self.step_index,
            "plates": [self.plates[key].to_dict() for key in sorted(self.plates)],
            "boundaries": [self.boundaries[key].to_dict() for key in sorted(self.boundaries)],
            "ocean_crust_markers": [marker.to_dict() for marker in sorted(self.crust_markers, key=lambda item: item.id)],
            "continental_blocks": [self.continental_blocks[key].to_dict() for key in sorted(self.continental_blocks)],
            "features": [self.features[key].to_dict() for key in sorted(self.features)],
            "driver_torques": {
                plate_id: {driver: _vector(vector) for driver, vector in sorted(drivers.items())}
                for plate_id, drivers in sorted(self.driver_torques.items())
            },
            "metrics": {key: float(value) for key, value in sorted(self.metrics.items())},
            "events": [event.to_dict() for event in self.events],
            "observations": [self.observations[key].to_dict() for key in sorted(self.observations)],
            "milestones": [milestone.to_dict() for milestone in self.milestones],
            "scenario_notes": list(self.scenario_notes),
        }
