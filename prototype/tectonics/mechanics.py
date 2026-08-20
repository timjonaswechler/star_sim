"""Calculated spherical kinematics and abstract plate-driving torques.

The force coefficients have no claimed physical units.  They only keep the
prototype in the speed classes quoted in research.md.  Topology changes live
in scenarios.py and are always logged as heuristics.
"""

from __future__ import annotations

import numpy as np

from geometry import boundary_frames, rodrigues, surface_velocity, unit
from model import BoundarySample, WorldState

DRIVERS = ("slab_pull", "slab_suction", "ridge_push", "mantle_drag")
DEFAULT_COEFFICIENTS = {
    "slab_pull": 0.0032,
    "slab_suction": 0.0010,
    "ridge_push": 0.0007,
    "mantle_drag": 0.22,
}


def classify_boundaries(world: WorldState) -> None:
    """Derive every boundary sample type from relative velocity."""
    for boundary in world.boundaries.values():
        boundary.diagnostics.clear()
        if not boundary.active:
            continue
        left = world.plates[boundary.left_plate]
        right = world.plates[boundary.right_plate]
        tangents, normals = boundary_frames(boundary.points)
        for point, tangent, normal in zip(boundary.points, tangents, normals, strict=True):
            relative = surface_velocity(right.omega, point, world.radius_km) - surface_velocity(
                left.omega, point, world.radius_km
            )
            signed_normal = float(np.dot(relative, normal))
            signed_shear = float(np.dot(relative, tangent))
            reconstructed = signed_normal * normal + signed_shear * tangent
            residual = float(np.linalg.norm(relative - reconstructed))
            normal_magnitude = abs(signed_normal)
            shear = abs(signed_shear)
            floor = 1.0e-7
            if normal_magnitude < floor and shear < floor:
                classification = "stationary"
            elif normal_magnitude < max(floor, 0.35 * shear):
                classification = "transform"
            elif shear > 0.65 * normal_magnitude:
                classification = "mixed-convergent" if signed_normal < 0.0 else "mixed-divergent"
            else:
                classification = "convergent" if signed_normal < 0.0 else "divergent"
            boundary.diagnostics.append(
                BoundarySample(
                    point=point.copy(),
                    tangent=tangent,
                    normal=normal,
                    relative_velocity=relative,
                    reconstructed_velocity=reconstructed,
                    normal_rate=signed_normal,
                    signed_shear_rate=signed_shear,
                    convergence_rate=max(0.0, -signed_normal),
                    divergence_rate=max(0.0, signed_normal),
                    shear_rate=shear,
                    classification=classification,
                    reconstruction_residual=residual,
                )
            )


def _line_force_torque(world: WorldState, point: np.ndarray, force: np.ndarray) -> np.ndarray:
    """Compute ``(R * p) x f``, normalized by R for prototype coefficients."""
    return np.cross(world.radius_km * point, force) / world.radius_km


def _mean_ocean_age(world: WorldState, plate_id: str) -> float:
    ages = [marker.age_myr for marker in world.crust_markers if marker.plate_id == plate_id]
    return float(np.mean(ages)) if ages else 60.0


def _boundary_length_km(world: WorldState, points: np.ndarray) -> float:
    dots = np.sum(points[:-1] * points[1:], axis=1)
    return world.radius_km * float(np.sum(np.arccos(np.clip(dots, -1.0, 1.0))))


def _empty_driver_table(world: WorldState) -> dict[str, dict[str, np.ndarray]]:
    return {
        plate_id: {driver: np.zeros(3, dtype=float) for driver in DRIVERS}
        for plate_id in world.plates
    }


def calculate_driver_torques(
    world: WorldState, coefficients: dict[str, float] | None = None
) -> dict[str, dict[str, np.ndarray]]:
    """Calculate slab pull, suction, ridge push, and distributed drag."""
    values = DEFAULT_COEFFICIENTS | (coefficients or {})
    torques = _empty_driver_table(world)

    for boundary in world.boundaries.values():
        if not boundary.active or not boundary.diagnostics:
            continue
        sample_count = len(boundary.diagnostics)
        if boundary.polarity and boundary.polarity in world.plates:
            subducting = boundary.polarity
            overriding = boundary.right_plate if subducting == boundary.left_plate else boundary.left_plate
            age_factor = 0.6 + _mean_ocean_age(world, subducting) / 100.0
            # The sampled line integral scales with trench length. Dividing
            # by sample count alone would make a 12,000 km slab pull no more
            # strongly than a 2,000 km slab.
            length_factor = max(0.1, _boundary_length_km(world, boundary.points) / 5000.0)
            for sample in boundary.diagnostics:
                if sample.convergence_rate <= 0.0:
                    continue
                rate_factor = 0.35 + min(2.0, sample.convergence_rate / 60.0)
                # p x t points to the geometric left of the directed
                # boundary. A plate on that side moves toward the trench
                # along -n; a plate on the right moves toward it along +n.
                toward_trench = -sample.normal if subducting == boundary.left_plate else sample.normal
                slab_force = (
                    toward_trench
                    * values["slab_pull"]
                    * age_factor
                    * rate_factor
                    * length_factor
                    / sample_count
                )
                suction_force = (
                    -toward_trench
                    * values["slab_suction"]
                    * rate_factor
                    * length_factor
                    / sample_count
                )
                torques[subducting]["slab_pull"] += _line_force_torque(world, sample.point, slab_force)
                torques[overriding]["slab_suction"] += _line_force_torque(world, sample.point, suction_force)

        for sample in boundary.diagnostics:
            if sample.divergence_rate <= 0.0:
                continue
            rate_factor = 0.25 + min(1.5, sample.divergence_rate / 60.0)
            # Research images 002 to 010 give plume-uplifted breakup the one
            # explicit ridge-push boost. Slab forces still dominate.
            plume_boost = 1.8 if world.scenario == "supercontinent_breakup" and "breakup_plume" in world.features else 1.0
            force = sample.normal * values["ridge_push"] * plume_boost * rate_factor / sample_count
            torques[boundary.left_plate]["ridge_push"] += _line_force_torque(world, sample.point, force)
            torques[boundary.right_plate]["ridge_push"] += _line_force_torque(world, sample.point, -force)

    for plate_id, plate in world.plates.items():
        root_factor = 1.8 if "continental" in plate.composition else 1.0
        reference = unit(plate.reference_position)
        support_points = np.asarray(
            [
                reference,
                unit(reference + np.array([0.27, -0.19, 0.13])),
                unit(reference + np.array([-0.16, 0.25, 0.21])),
            ]
        )
        for point in support_points:
            velocity = surface_velocity(plate.omega, point, world.radius_km)
            force = -values["mantle_drag"] * root_factor * velocity / world.radius_km / len(support_points)
            torques[plate_id]["mantle_drag"] += _line_force_torque(world, point, force)

    return torques


def apply_driver_torques(
    world: WorldState,
    dt_fraction: float,
    coefficients: dict[str, float] | None = None,
) -> None:
    """Integrate abstract angular acceleration over a fraction of the run."""
    torques = calculate_driver_torques(world, coefficients)
    world.driver_torques = torques
    for plate_id, plate in world.plates.items():
        total = sum(torques[plate_id].values(), start=np.zeros(3))
        plate.torque = total
        plate.accumulated_torque += total * dt_fraction
        plate.omega += total / plate.inertia * dt_fraction
        speed = float(np.linalg.norm(plate.omega))
        if speed > 0.03:
            plate.omega *= 0.03 / speed

    # Sutured blocks form one rigid assemblage. Integrate their separate
    # driver torques, then impose one inertia-weighted Euler vector. The
    # megashear scenario is the explicit internal-slip exception.
    if world.scenario != "megashear":
        assemblages: dict[str, set[str]] = {}
        for plate_id, plate in world.plates.items():
            assemblage_id = plate.properties.get("rigid_assemblage")
            if assemblage_id:
                assemblages.setdefault(str(assemblage_id), set()).add(plate_id)
        for plate_ids in assemblages.values():
            if len(plate_ids) < 2:
                continue
            total_inertia = sum(world.plates[plate_id].inertia for plate_id in plate_ids)
            common_omega = sum(
                (
                    world.plates[plate_id].omega * world.plates[plate_id].inertia
                    for plate_id in plate_ids
                ),
                start=np.zeros(3),
            ) / total_inertia
            for plate_id in plate_ids:
                world.plates[plate_id].omega = common_omega.copy()


def _advect_boundary(world: WorldState, boundary_id: str, dt_myr: float) -> None:
    boundary = world.boundaries[boundary_id]
    if not boundary.active:
        return
    mean_omega = 0.5 * (
        world.plates[boundary.left_plate].omega + world.plates[boundary.right_plate].omega
    )
    boundary.points = rodrigues(boundary.points, mean_omega, dt_myr)


def advect_world(world: WorldState, dt_myr: float) -> None:
    """Advect attached samples by their plate Euler rotations."""
    for marker in world.crust_markers:
        marker.position = rodrigues(marker.position, world.plates[marker.plate_id].omega, dt_myr)
        marker.age_myr += dt_myr
    for block in world.continental_blocks.values():
        block.sample_points = rodrigues(block.sample_points, world.plates[block.plate_id].omega, dt_myr)
    for feature in world.features.values():
        if feature.active and feature.plate_id in world.plates and not feature.properties.get("mantle_fixed", False):
            feature.points = rodrigues(feature.points, world.plates[feature.plate_id].omega, dt_myr)
    for boundary_id in sorted(world.boundaries):
        _advect_boundary(world, boundary_id, dt_myr)
    for plate in world.plates.values():
        plate.reference_position = rodrigues(plate.reference_position, plate.omega, dt_myr)


def update_calculated_metrics(world: WorldState, dt_myr: float) -> None:
    """Integrate basin widths and expose speeds from boundary diagnostics."""
    for boundary in world.boundaries.values():
        if not boundary.active or not boundary.diagnostics:
            continue
        convergence = boundary.mean_rate("convergence_rate")
        divergence = boundary.mean_rate("divergence_rate")
        shear = boundary.mean_rate("shear_rate")
        world.metrics[f"{boundary.id}.convergence_km_per_myr"] = convergence
        world.metrics[f"{boundary.id}.divergence_km_per_myr"] = divergence
        world.metrics[f"{boundary.id}.shear_km_per_myr"] = shear
        if "interior" in boundary.concept:
            world.metrics["interior_ocean_width_km"] = max(
                0.0,
                world.metrics.get("interior_ocean_width_km", 2500.0)
                + (divergence - convergence) * dt_myr,
            )
        if "exterior" in boundary.concept:
            world.metrics["exterior_ocean_width_km"] = max(
                0.0,
                world.metrics.get("exterior_ocean_width_km", 18000.0)
                + (divergence - convergence) * dt_myr,
            )
    if "gateway_transform" in world.boundaries:
        world.metrics["gateway_shear_km_per_myr"] = world.boundaries["gateway_transform"].mean_rate(
            "shear_rate"
        )
    if "megashear" in world.boundaries:
        shear = world.boundaries["megashear"].mean_rate("shear_rate")
        world.metrics["megashear_slip_km"] = world.metrics.get("megashear_slip_km", 0.0) + shear * dt_myr


def advance_calculated_state(
    world: WorldState,
    dt_myr: float,
    coefficients: dict[str, float] | None = None,
) -> None:
    """Run one calculated step.  No topology changes occur here."""
    classify_boundaries(world)
    update_calculated_metrics(world, dt_myr)
    apply_driver_torques(world, dt_myr / world.duration_myr, coefficients)
    advect_world(world, dt_myr)
    classify_boundaries(world)
