//! Bevy rendering adapter for physically calculated stellar surface emission.

use bevy::{
    app::{App, Plugin},
    asset::Asset,
    color::{ColorToComponents, LinearRgba},
    pbr::{Material, MaterialPlugin, StandardMaterial},
    prelude::{Color, Vec4},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

use crate::color_temperature::BlackBodyEmission;
const STAR_SURFACE_SHADER_PATH: &str = "shaders/star_surface.wgsl";

/// Registers the hot-reloadable surface shader material pipeline.
pub struct StarSurfaceMaterialPlugin;

impl Plugin for StarSurfaceMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<StarSurfaceMaterial>::default());
    }
}

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct StarSurfaceUniform {
    pub emissive: Vec4,
    pub limb_darkening: f32,
    pub granulation_strength: f32,
    pub granulation_scale: f32,
    pub animation_speed: f32,
    /// 0: physical stellar surface, 1: signed noisy dipole diagnostic.
    pub display_mode: u32,
    /// World-space direction of the magnetic poles; independent of geographic Y.
    pub magnetic_axis: Vec4,
}

/// HDR material with view-dependent limb darkening and animated granulation.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarSurfaceMaterial {
    #[uniform(0)]
    pub parameters: StarSurfaceUniform,
}

impl Material for StarSurfaceMaterial {
    fn fragment_shader() -> ShaderRef {
        STAR_SURFACE_SHADER_PATH.into()
    }
}

/// Creates an HDR Bevy material for the resolved surface of a star.
///
/// The emissive channels are scaled so their Rec. 709 luminance equals the
/// calculated photopic surface luminance in cd/m² (nits). Camera [`Exposure`],
/// tone mapping, and bloom remain rendering concerns. This material emits light
/// toward the camera, but does not illuminate other scene objects by itself.
///
/// [`Exposure`]: bevy::camera::Exposure
pub fn star_surface_material(emission: BlackBodyEmission) -> StandardMaterial {
    let emissive = physical_emissive_color(emission);

    StandardMaterial {
        base_color: Color::BLACK,
        emissive,
        // Physical camera exposure should affect stellar luminance.
        emissive_exposure_weight: 1.0,
        fog_enabled: false,
        ..Default::default()
    }
}

/// Creates the procedural HDR material used for a resolved stellar sphere.
pub fn procedural_star_surface_material(emission: BlackBodyEmission) -> StarSurfaceMaterial {
    StarSurfaceMaterial {
        parameters: StarSurfaceUniform {
            emissive: physical_emissive_color(emission).to_vec4(),
            limb_darkening: 0.6,
            granulation_strength: 0.16,
            granulation_scale: 18.0,
            animation_speed: 0.025,
            display_mode: 0,
            magnetic_axis: Vec4::new(0.31, 0.88, 0.36, 0.0).normalize(),
        },
    }
}

fn physical_emissive_color(emission: BlackBodyEmission) -> LinearRgba {
    let chromaticity: LinearRgba = emission.chromaticity.into();
    let chromaticity_luminance =
        0.2126 * chromaticity.red + 0.7152 * chromaticity.green + 0.0722 * chromaticity.blue;
    let scale = if chromaticity_luminance > 0.0 {
        emission.photopic_luminance_candelas_per_square_meter as f32 / chromaticity_luminance
    } else {
        0.0
    };

    LinearRgba::rgb(
        chromaticity.red * scale,
        chromaticity.green * scale,
        chromaticity.blue * scale,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_temperature::black_body_emission;

    #[test]
    fn material_preserves_the_calculated_photopic_luminance() {
        let emission = black_body_emission(5_772.0).unwrap();
        let material = star_surface_material(emission);
        let luminance = 0.2126 * material.emissive.red
            + 0.7152 * material.emissive.green
            + 0.0722 * material.emissive.blue;

        assert!(
            (f64::from(luminance) / emission.photopic_luminance_candelas_per_square_meter - 1.0)
                .abs()
                < 1.0e-5
        );
        assert_eq!(material.emissive_exposure_weight, 1.0);
    }
}
