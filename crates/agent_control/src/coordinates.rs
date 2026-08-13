//! Coordinate and camera-math primitives shared by optional host adapters.
//!
//! Viewport coordinates use a top-left origin. +X points right and +Y points down.

use bevy::{camera::ViewportConversionError, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "space", rename_all = "snake_case")]
pub enum Coordinate {
    World { x: f32, y: f32, z: f32 },
    ViewportPixels { x: f32, y: f32 },
    ViewportNormalized { x: f32, y: f32 },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OperationMode {
    #[default]
    Relative,
    Absolute,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PublicRay3d {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub fn validate_coordinate(value: Coordinate) -> Result<(), &'static str> {
    let valid = match value {
        Coordinate::World { x, y, z } => x.is_finite() && y.is_finite() && z.is_finite(),
        Coordinate::ViewportPixels { x, y } => {
            x.is_finite() && y.is_finite() && x >= 0.0 && y >= 0.0
        }
        Coordinate::ViewportNormalized { x, y } => {
            x.is_finite() && y.is_finite() && (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y)
        }
    };
    valid
        .then_some(())
        .ok_or("coordinate is non-finite or outside its coordinate space")
}

pub fn viewport_pixels(value: Coordinate, viewport_size: Vec2) -> Result<Vec2, &'static str> {
    validate_coordinate(value)?;
    if !viewport_size.is_finite() || viewport_size.x <= 0.0 || viewport_size.y <= 0.0 {
        return Err("viewport size must be finite and positive");
    }
    match value {
        Coordinate::ViewportPixels { x, y } if x <= viewport_size.x && y <= viewport_size.y => {
            Ok(Vec2::new(x, y))
        }
        Coordinate::ViewportNormalized { x, y } => Ok(Vec2::new(x, y) * viewport_size),
        Coordinate::ViewportPixels { .. } => Err("pixel coordinate lies outside the viewport"),
        Coordinate::World { .. } => {
            Err("world coordinates cannot be interpreted as viewport pixels")
        }
    }
}

/// Converts an explicit viewport coordinate to a world ray starting on the near plane.
pub fn viewport_to_world_ray(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    coordinate: Coordinate,
) -> Result<PublicRay3d, ViewportConversionError> {
    let size = camera
        .logical_viewport_size()
        .ok_or(ViewportConversionError::NoViewportSize)?;
    let pixels =
        viewport_pixels(coordinate, size).map_err(|_| ViewportConversionError::InvalidData)?;
    let ray = camera.viewport_to_world(camera_transform, pixels)?;
    Ok(PublicRay3d {
        origin: ray.origin,
        direction: *ray.direction,
    })
}

/// Projects a world point to top-left-origin logical viewport pixels.
pub fn world_to_viewport(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world: Vec3,
) -> Result<Vec2, ViewportConversionError> {
    camera.world_to_viewport(camera_transform, world)
}

/// Intersects a world ray with a plane. A viewport coordinate never silently becomes a point;
/// callers must select an explicit projection such as this plane intersection.
pub fn project_ray_to_plane(
    ray: PublicRay3d,
    plane_point: Vec3,
    plane_normal: Vec3,
) -> Option<Vec3> {
    if !ray.origin.is_finite()
        || !ray.direction.is_finite()
        || !plane_point.is_finite()
        || !plane_normal.is_finite()
    {
        return None;
    }
    let normal = plane_normal.try_normalize()?;
    let denominator = ray.direction.dot(normal);
    if denominator.abs() <= f32::EPSILON {
        return None;
    }
    let distance = (plane_point - ray.origin).dot(normal) / denominator;
    (distance >= 0.0).then(|| ray.origin + ray.direction * distance)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraPose {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

impl CameraPose {
    pub fn interpolate(self, end: Self, fraction: f32) -> Self {
        let t = fraction.clamp(0.0, 1.0);
        Self {
            position: self.position.lerp(end.position, t),
            target: self.target.lerp(end.target, t),
            up: self.up.lerp(end.up, t).normalize_or_zero(),
        }
    }
}

/// Computes a perspective-camera pose that frames a sphere with a configurable margin.
pub fn degrees(value: f32) -> Result<f32, &'static str> {
    value
        .is_finite()
        .then(|| value.to_radians())
        .ok_or("angle must be finite")
}

pub fn orbit_pose(
    current: CameraPose,
    mode: OperationMode,
    yaw_degrees: f32,
    pitch_degrees: f32,
) -> Result<CameraPose, &'static str> {
    let yaw = degrees(yaw_degrees)?;
    let pitch = degrees(pitch_degrees)?;
    let radius = current.position.distance(current.target);
    if !radius.is_finite() || radius <= f32::EPSILON {
        return Err("camera must be separated from its target");
    }
    let offset = match mode {
        OperationMode::Relative => {
            Quat::from_rotation_y(yaw)
                * Quat::from_rotation_x(pitch)
                * (current.position - current.target)
        }
        OperationMode::Absolute => {
            Vec3::new(
                yaw.sin() * pitch.cos(),
                pitch.sin(),
                yaw.cos() * pitch.cos(),
            ) * radius
        }
    };
    Ok(CameraPose {
        position: current.target + offset,
        ..current
    })
}

pub fn zoom_pose(
    current: CameraPose,
    mode: OperationMode,
    value: f32,
) -> Result<CameraPose, &'static str> {
    if !value.is_finite()
        || (mode == OperationMode::Relative && value == 0.0)
        || (mode == OperationMode::Absolute && value <= 0.0)
    {
        return Err("relative zoom must be nonzero and absolute zoom must be positive");
    }
    let direction = (current.position - current.target)
        .try_normalize()
        .ok_or("camera must be separated from its target")?;
    let current_distance = current.position.distance(current.target);
    let distance = match mode {
        OperationMode::Relative => current_distance * value,
        OperationMode::Absolute => value,
    };
    if !distance.is_finite() || distance <= 0.0 {
        return Err("zoom produces an invalid camera distance");
    }
    Ok(CameraPose {
        position: current.target + direction * distance,
        ..current
    })
}

pub fn focus_pose(
    current: CameraPose,
    target: Vec3,
    visual_radius: f32,
    vertical_fov_radians: f32,
    margin: f32,
) -> Result<CameraPose, &'static str> {
    if !target.is_finite()
        || !visual_radius.is_finite()
        || visual_radius <= 0.0
        || !vertical_fov_radians.is_finite()
        || !(0.0..std::f32::consts::PI).contains(&vertical_fov_radians)
        || !margin.is_finite()
        || margin < 1.0
    {
        return Err("invalid focus framing arguments");
    }
    let direction = (current.position - current.target)
        .try_normalize()
        .unwrap_or(Vec3::Z);
    let distance = visual_radius * margin / (vertical_fov_radians * 0.5).tan();
    Ok(CameraPose {
        position: target + direction * distance,
        target,
        up: current.up,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeterministicAnimation {
    duration_ms: u32,
    elapsed_ms: u32,
}

impl DeterministicAnimation {
    pub fn new(duration_ms: u32) -> Self {
        Self {
            duration_ms,
            elapsed_ms: 0,
        }
    }

    pub fn advance(&mut self, delta_ms: u32) -> f32 {
        self.elapsed_ms = self
            .elapsed_ms
            .saturating_add(delta_ms)
            .min(self.duration_ms);
        self.fraction()
    }

    pub fn fraction(&self) -> f32 {
        if self.duration_ms == 0 {
            1.0
        } else {
            self.elapsed_ms as f32 / self.duration_ms as f32
        }
    }

    pub fn complete(&self) -> bool {
        self.elapsed_ms >= self.duration_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_and_pixel_coordinates_use_top_left_convention() {
        assert_eq!(
            viewport_pixels(
                Coordinate::ViewportNormalized { x: 0.25, y: 0.75 },
                Vec2::new(800.0, 600.0)
            ),
            Ok(Vec2::new(200.0, 450.0))
        );
        assert_eq!(
            viewport_pixels(
                Coordinate::ViewportPixels { x: 20.0, y: 30.0 },
                Vec2::new(800.0, 600.0)
            ),
            Ok(Vec2::new(20.0, 30.0))
        );
        assert!(
            viewport_pixels(Coordinate::ViewportNormalized { x: 1.1, y: 0.0 }, Vec2::ONE).is_err()
        );
    }

    #[test]
    fn ray_projection_is_explicit() {
        let ray = PublicRay3d {
            origin: Vec3::new(0.0, 0.0, 5.0),
            direction: -Vec3::Z,
        };
        assert_eq!(
            project_ray_to_plane(ray, Vec3::ZERO, Vec3::Z),
            Some(Vec3::ZERO)
        );
        assert_eq!(project_ray_to_plane(ray, Vec3::ZERO, Vec3::X), None);
    }

    #[test]
    fn relative_and_absolute_orbit_and_zoom_have_distinct_semantics() {
        let pose = CameraPose {
            position: Vec3::new(0.0, 0.0, 10.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
        };
        let relative = orbit_pose(pose, OperationMode::Relative, 90.0, 0.0).unwrap();
        let absolute = orbit_pose(pose, OperationMode::Absolute, 0.0, 0.0).unwrap();
        assert!(relative.position.abs_diff_eq(Vec3::X * 10.0, 1e-5));
        assert!(absolute.position.abs_diff_eq(Vec3::Z * 10.0, 1e-5));
        assert!(
            zoom_pose(pose, OperationMode::Relative, 0.5)
                .unwrap()
                .position
                .abs_diff_eq(Vec3::Z * 5.0, 1e-5)
        );
        assert!(
            zoom_pose(pose, OperationMode::Absolute, 2.0)
                .unwrap()
                .position
                .abs_diff_eq(Vec3::Z * 2.0, 1e-5)
        );
        assert!((degrees(180.0).unwrap() - std::f32::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn focus_uses_visual_extent() {
        let current = CameraPose {
            position: Vec3::new(0.0, 0.0, 10.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
        };
        let small = focus_pose(current, Vec3::ZERO, 1.0, 60_f32.to_radians(), 1.2).unwrap();
        let large = focus_pose(current, Vec3::ZERO, 2.0, 60_f32.to_radians(), 1.2).unwrap();
        assert!((large.position.length() - small.position.length() * 2.0).abs() < 1e-5);
    }

    #[test]
    fn animation_completes_only_at_duration_and_zero_is_immediate() {
        let mut animation = DeterministicAnimation::new(250);
        assert_eq!(animation.advance(100), 0.4);
        assert!(!animation.complete());
        assert_eq!(animation.advance(150), 1.0);
        assert!(animation.complete());
        assert!(DeterministicAnimation::new(0).complete());
    }
}
