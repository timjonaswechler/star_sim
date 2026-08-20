"""Matplotlib inspection views for scenario states."""

from __future__ import annotations

from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.animation as animation
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.axes import Axes
from matplotlib.colors import Normalize
from matplotlib.lines import Line2D

from geometry import surface_velocity, to_latlon
from model import WorldState
from scenarios import SimulationResult

BOUNDARY_COLORS = {
    "convergent": "#2367c9",
    "mixed-convergent": "#517fc4",
    "divergent": "#db3a34",
    "mixed-divergent": "#d96c67",
    "transform": "#258f55",
    "stationary": "#777777",
}
FEATURE_STYLES = {
    "ridge": ("#d73027", "-"),
    "trench": ("#225ea8", "-"),
    "transform_fault": ("#238b45", "-"),
    "active_rift": ("#ef3b2c", "--"),
    "failed_rift": ("#f39c34", ":"),
    "island_arc": ("#8e44ad", "-"),
    "back_arc_basin": ("#2ca9bc", "--"),
    "suture": ("#54278f", "-"),
    "orogeny": ("#8c510a", "-"),
    "hotspot": ("#cc0077", "None"),
    "large_igneous_province": ("#ff7f00", "None"),
    "passive_margin": ("#8c8c8c", "--"),
    "active_margin": ("#1f5daa", "-"),
    "slab_window": ("#e31a1c", "None"),
    "ocean_basin": ("#49a9d6", "--"),
    "continental_margin": ("#6b8e23", "-"),
}
PLATE_COLORS = ["#d9ef8b", "#fee08b", "#fdae61", "#abd9e9", "#c2a5cf", "#a6dba0"]


def _lonlat(points: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    values = [to_latlon(point) for point in points]
    return np.asarray([item[1] for item in values]), np.asarray([item[0] for item in values])


def _blocks(ax: Axes, state: WorldState) -> None:
    plate_colors = {plate_id: PLATE_COLORS[index % len(PLATE_COLORS)] for index, plate_id in enumerate(sorted(state.plates))}
    for block in state.continental_blocks.values():
        longitude, latitude = _lonlat(block.sample_points)
        ax.fill(longitude, latitude, color=plate_colors.get(block.plate_id, "#ddddaa"), alpha=0.55, zorder=1)
        ax.plot(longitude, latitude, color="#4d4d32", linewidth=0.8, zorder=2)
        ax.text(float(np.mean(longitude)), float(np.mean(latitude)), block.id, fontsize=6, ha="center", zorder=8)


def _boundaries(ax: Axes, state: WorldState) -> None:
    for boundary in state.boundaries.values():
        longitude, latitude = _lonlat(boundary.points)
        if not boundary.active:
            ax.plot(longitude, latitude, color="#aaaaaa", linewidth=0.7, linestyle=":", alpha=0.45, zorder=2)
            continue
        for index in range(len(boundary.points) - 1):
            classification = boundary.diagnostics[min(index, len(boundary.diagnostics) - 1)].classification
            ax.plot(
                longitude[index : index + 2],
                latitude[index : index + 2],
                color=BOUNDARY_COLORS[classification],
                linewidth=2.0,
                zorder=4,
            )
        midpoint = len(boundary.points) // 2
        ax.text(longitude[midpoint], latitude[midpoint] + 2.2, boundary.id, fontsize=5.5, ha="center", zorder=9)
        if boundary.polarity:
            ax.scatter(
                [longitude[midpoint]],
                [latitude[midpoint]],
                marker="^",
                s=28,
                color="#123c75",
                edgecolor="white",
                linewidth=0.4,
                zorder=10,
            )
            ax.text(
                longitude[midpoint],
                latitude[midpoint] - 4.2,
                f"sub: {boundary.polarity}",
                fontsize=4.8,
                ha="center",
                color="#123c75",
                zorder=10,
            )


def _features(ax: Axes, state: WorldState) -> None:
    for feature in state.features.values():
        longitude, latitude = _lonlat(feature.points)
        color, linestyle = FEATURE_STYLES.get(feature.kind, ("#333333", "-"))
        alpha = 1.0 if feature.active else 0.25
        if linestyle == "None" or len(feature.points) == 1:
            ax.scatter(longitude, latitude, s=23, color=color, alpha=alpha, zorder=6)
        else:
            ax.plot(longitude, latitude, color=color, linestyle=linestyle, linewidth=1.8, alpha=alpha, zorder=6)
        middle = len(feature.points) // 2
        ax.text(longitude[middle], latitude[middle] + 1.4, feature.id, fontsize=5, color=color, ha="center", alpha=alpha, zorder=9)


def _markers(ax: Axes, state: WorldState) -> None:
    if not state.crust_markers:
        return
    coordinates = [_lonlat(np.asarray([marker.position])) for marker in state.crust_markers]
    longitude = [item[0][0] for item in coordinates]
    latitude = [item[1][0] for item in coordinates]
    ages = [marker.age_myr for marker in state.crust_markers]
    ax.scatter(
        longitude,
        latitude,
        c=ages,
        cmap="Blues",
        norm=Normalize(0, max(200, max(ages))),
        s=11,
        edgecolor="#335577",
        linewidth=0.2,
        zorder=3,
    )
    ax.text(
        0.99,
        0.02,
        f"crust age {min(ages):.0f}-{max(ages):.0f} Myr; darker = older",
        transform=ax.transAxes,
        fontsize=5.5,
        ha="right",
        va="bottom",
        color="#335577",
        zorder=12,
    )


def _plate_motion(ax: Axes, state: WorldState) -> None:
    for plate in state.plates.values():
        latitude, longitude = to_latlon(plate.reference_position)
        velocity = surface_velocity(plate.omega, plate.reference_position, state.radius_km)
        lat_rad = np.radians(latitude)
        lon_rad = np.radians(longitude)
        east = np.array([-np.sin(lon_rad), np.cos(lon_rad), 0.0])
        north = np.array(
            [-np.sin(lat_rad) * np.cos(lon_rad), -np.sin(lat_rad) * np.sin(lon_rad), np.cos(lat_rad)]
        )
        east_rate = float(np.dot(velocity, east))
        north_rate = float(np.dot(velocity, north))
        scale = 0.16
        ax.arrow(
            longitude,
            latitude,
            east_rate * scale,
            north_rate * scale,
            width=0.18,
            head_width=2.2,
            head_length=2.4,
            length_includes_head=True,
            color="#111111",
            zorder=11,
        )
        ax.text(
            longitude,
            latitude - 5.0,
            f"{plate.id}\n{plate.composition}",
            fontsize=5.3,
            ha="center",
            va="top",
            color="#111111",
            zorder=11,
        )


def draw(ax: Axes, state: WorldState, title: str, include_legend: bool = False) -> None:
    ax.set_facecolor("#e8f5fb")
    ax.set_xlim(-180, 180)
    ax.set_ylim(-90, 90)
    ax.set_aspect("equal", adjustable="box")
    ax.set_xticks([-180, -120, -60, 0, 60, 120, 180])
    ax.set_yticks([-60, -30, 0, 30, 60])
    ax.grid(color="white", linewidth=0.5, alpha=0.8)
    _blocks(ax, state)
    _markers(ax, state)
    _boundaries(ax, state)
    _features(ax, state)
    _plate_motion(ax, state)
    ax.set_title(f"{title}\n{state.elapsed_myr:.1f} Myr, step {state.step_index}", fontsize=9)
    ax.set_xlabel("longitude")
    if include_legend:
        handles = [
            Line2D([0], [0], color=BOUNDARY_COLORS[name], linewidth=2, label=name)
            for name in ("convergent", "divergent", "transform")
        ]
        handles.append(Line2D([0], [0], marker="o", color="none", markerfacecolor="#8ebfe5", label="ocean crust age", markersize=5))
        ax.legend(handles=handles, loc="lower left", fontsize=6, framealpha=0.9)


def png(result: SimulationResult, path: str | Path) -> None:
    states = [result.history[result.milestone_indices[label]] for label in ("initial", "decisive_transition", "final")]
    figure, axes = plt.subplots(3, 1, figsize=(13, 12), constrained_layout=True)
    for axis, state, label in zip(axes, states, ("initial", "decisive transition", "final"), strict=True):
        draw(axis, state, label, include_legend=label == "initial")
    figure.suptitle(f"{result.definition.name}: {result.definition.summary}", fontsize=13)
    figure.savefig(path, dpi=120, metadata={"Software": "star_sim tectonics prototype"})
    plt.close(figure)


def gif(result: SimulationResult, path: str | Path, fps: int = 8) -> None:
    figure, axis = plt.subplots(figsize=(12, 6), constrained_layout=True)

    def update(frame: int) -> None:
        axis.clear()
        draw(axis, result.history[frame], result.definition.name, include_legend=frame == 0)

    movie = animation.FuncAnimation(figure, update, frames=len(result.history), interval=1000 / fps, repeat=False)
    writer = animation.PillowWriter(fps=fps, metadata={"Software": "star_sim tectonics prototype"})
    movie.save(path, writer=writer)
    plt.close(figure)


def save(result: SimulationResult, path: str | Path) -> None:
    destination = Path(path)
    destination.parent.mkdir(parents=True, exist_ok=True)
    suffix = destination.suffix.lower()
    if suffix == ".png":
        png(result, destination)
    elif suffix == ".gif":
        gif(result, destination)
    else:
        raise ValueError("--save path must end in .png or .gif")
