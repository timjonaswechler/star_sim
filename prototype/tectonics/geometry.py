"""Spherical geometry for the throwaway tectonics prototype.

All public position values are three-dimensional unit vectors.  Map
coordinates only enter and leave at the rendering edge.
"""

from __future__ import annotations

import math
from typing import Iterable

import numpy as np

EPSILON = 1.0e-12


def unit(value: np.ndarray | Iterable[float]) -> np.ndarray:
    """Return *value* normalized along its last axis."""
    array = np.asarray(value, dtype=float)
    length = np.linalg.norm(array, axis=-1, keepdims=True)
    if np.any(length < EPSILON):
        raise ValueError("cannot normalize a zero vector")
    return array / length


def from_latlon(latitude_deg: float, longitude_deg: float) -> np.ndarray:
    latitude = math.radians(latitude_deg)
    longitude = math.radians(longitude_deg)
    return np.array(
        [
            math.cos(latitude) * math.cos(longitude),
            math.cos(latitude) * math.sin(longitude),
            math.sin(latitude),
        ],
        dtype=float,
    )


def to_latlon(point: np.ndarray) -> tuple[float, float]:
    x, y, z = unit(point)
    return math.degrees(math.asin(float(z))), math.degrees(math.atan2(float(y), float(x)))


def rodrigues(points: np.ndarray, omega: np.ndarray, dt: float) -> np.ndarray:
    """Advect unit vectors through the Euler rotation ``omega * dt``."""
    values = np.asarray(points, dtype=float)
    angular_speed = float(np.linalg.norm(omega))
    if angular_speed < EPSILON or dt == 0.0:
        return unit(values.copy())
    axis = np.asarray(omega, dtype=float) / angular_speed
    angle = angular_speed * dt
    cosine = math.cos(angle)
    sine = math.sin(angle)
    cross = np.cross(axis, values)
    dot = np.sum(values * axis, axis=-1, keepdims=True)
    rotated = values * cosine + cross * sine + axis * dot * (1.0 - cosine)
    return unit(rotated)


def surface_velocity(omega: np.ndarray, point: np.ndarray, radius: float) -> np.ndarray:
    """Rigid velocity at a unit surface point, ``omega x (radius * p)``."""
    return np.cross(np.asarray(omega, dtype=float), radius * np.asarray(point, dtype=float))


def great_circle(start: np.ndarray, end: np.ndarray, count: int) -> np.ndarray:
    """Sample the shorter great-circle path, including both endpoints."""
    if count < 2:
        raise ValueError("a polyline needs at least two samples")
    first = unit(start)
    last = unit(end)
    cosine = float(np.clip(np.dot(first, last), -1.0, 1.0))
    angle = math.acos(cosine)
    if angle < EPSILON:
        return np.repeat(first[None, :], count, axis=0)
    fractions = np.linspace(0.0, 1.0, count)[:, None]
    denominator = math.sin(angle)
    return unit(
        np.sin((1.0 - fractions) * angle) / denominator * first
        + np.sin(fractions * angle) / denominator * last
    )


def polyline_from_latlon(coordinates: Iterable[tuple[float, float]]) -> np.ndarray:
    return np.asarray([from_latlon(latitude, longitude) for latitude, longitude in coordinates])


def meridian(longitude_deg: float, count: int = 9, latitude_limit: float = 58.0) -> np.ndarray:
    return np.asarray(
        [from_latlon(latitude, longitude_deg) for latitude in np.linspace(-latitude_limit, latitude_limit, count)]
    )


def equatorial_arc(longitude_start: float, longitude_end: float, count: int = 9) -> np.ndarray:
    return np.asarray(
        [from_latlon(0.0, longitude) for longitude in np.linspace(longitude_start, longitude_end, count)]
    )


def boundary_frames(points: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """Return local polyline tangents and oriented in-surface normals.

    The normal follows the research convention ``n = normalize(p x t)``.
    """
    values = unit(np.asarray(points, dtype=float))
    differences = np.empty_like(values)
    differences[0] = values[1] - values[0]
    differences[-1] = values[-1] - values[-2]
    differences[1:-1] = values[2:] - values[:-2]
    projected = differences - np.sum(differences * values, axis=1, keepdims=True) * values
    tangents = unit(projected)
    normals = unit(np.cross(values, tangents))
    return tangents, normals


def decompose_tangent(
    relative_velocity: np.ndarray, tangent: np.ndarray, normal: np.ndarray
) -> tuple[float, float, np.ndarray, float]:
    """Split a relative velocity into boundary tangent and normal parts."""
    signed_normal = float(np.dot(relative_velocity, normal))
    signed_shear = float(np.dot(relative_velocity, tangent))
    reconstructed = signed_normal * normal + signed_shear * tangent
    residual = float(np.linalg.norm(relative_velocity - reconstructed))
    return signed_normal, signed_shear, reconstructed, residual


def angular_distance(first: np.ndarray, second: np.ndarray) -> float:
    return math.acos(float(np.clip(np.dot(unit(first), unit(second)), -1.0, 1.0)))


def offset_on_surface(point: np.ndarray, direction: np.ndarray, angular_distance_rad: float) -> np.ndarray:
    """Move a point along a local tangent direction by an angular distance."""
    start = unit(point)
    tangent = np.asarray(direction, dtype=float)
    tangent = unit(tangent - np.dot(tangent, start) * start)
    return unit(start * math.cos(angular_distance_rad) + tangent * math.sin(angular_distance_rad))


def hinge_arc(
    base_points: np.ndarray, omega_start: np.ndarray, omega_end: np.ndarray, elapsed: float
) -> np.ndarray:
    """Bend samples by interpolated Euler rotations, not a flat-map arc."""
    result: list[np.ndarray] = []
    for fraction, point in zip(np.linspace(0.0, 1.0, len(base_points)), base_points, strict=True):
        omega = (1.0 - fraction) * omega_start + fraction * omega_end
        result.append(rodrigues(point, omega, elapsed))
    return unit(np.asarray(result))
