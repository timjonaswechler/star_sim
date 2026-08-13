//! THROWAWAY PROTOTYPE: a bounded raymarched stellar-corona volume.

use bevy::{
    asset::Asset,
    mesh::MeshVertexBufferLayoutRef,
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};

const CORONA_VOLUME_SHADER_PATH: &str = "shaders/corona_volume.wgsl";
const CORONA_OUTER_RADIUS: f32 = 1.8;

pub struct CoronaVolumePlugin;

impl Plugin for CoronaVolumePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CoronaVolumeMaterial>::default());
    }
}

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct CoronaVolumeUniform {
    /// HDR luminance anchor with an independent pale coronal chromaticity.
    pub emissive: Vec4,
    /// x: strength, y: density falloff, z: noise scale, w: animation speed.
    pub appearance: Vec4,
    /// x: stellar radius, y: outer radius, z/w: reserved.
    pub geometry: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CoronaVolumeMaterial {
    #[uniform(0)]
    pub parameters: CoronaVolumeUniform,
}

impl Material for CoronaVolumeMaterial {
    fn fragment_shader() -> ShaderRef {
        CORONA_VOLUME_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // The camera may enter the bounding sphere, so its inner faces must
        // remain rasterized as a surface on which to run the volume shader.
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoronaMode {
    Off,
    Natural,
    Enhanced,
}

impl CoronaMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Natural => "Natural",
            Self::Enhanced => "Enhanced",
        }
    }

    const fn strength(self) -> f32 {
        match self {
            Self::Off => 0.0,
            Self::Natural => 0.038,
            Self::Enhanced => 0.075,
        }
    }
}

#[derive(Component)]
pub struct CoronaVolume;

pub fn spawn_corona_volume(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CoronaVolumeMaterial>,
    surface_emissive: Vec4,
    mode: CoronaMode,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(CORONA_OUTER_RADIUS).mesh().ico(6).unwrap())),
        MeshMaterial3d(materials.add(corona_material(surface_emissive, mode))),
        CoronaVolume,
    ));
}

pub fn update_corona_volume(
    mode: CoronaMode,
    surface_emissive: Vec4,
    material_handle: &MeshMaterial3d<CoronaVolumeMaterial>,
    materials: &mut Assets<CoronaVolumeMaterial>,
) {
    if let Some(mut material) = materials.get_mut(&material_handle.0) {
        *material = corona_material(surface_emissive, mode);
    }
}

fn corona_material(surface_emissive: Vec4, mode: CoronaMode) -> CoronaVolumeMaterial {
    // These names are the public tuning surface for the packed shader uniform.
    // Fine-grained streamer/noise tuning lives in corona_volume.wgsl under
    // "Streamer tuning", next to the implementation it controls.
    let volume_intensity = mode.strength();
    let density_falloff = 5.2;
    let angular_noise_scale = 3.1;
    let animation_speed = 0.018;
    let stellar_radius = 0.99;

    CoronaVolumeMaterial {
        parameters: CoronaVolumeUniform {
            emissive: coronal_emissive(surface_emissive),
            appearance: Vec4::new(
                volume_intensity,
                density_falloff,
                angular_noise_scale,
                animation_speed,
            ),
            geometry: Vec4::new(stellar_radius, CORONA_OUTER_RADIUS, 0.0, 0.0),
        },
    }
}

fn coronal_emissive(surface_emissive: Vec4) -> Vec4 {
    const SURFACE_TINT: f32 = 0.3;
    let luminance_weights = Vec3::new(0.2126, 0.7152, 0.0722);
    let surface_rgb = surface_emissive.truncate();
    let surface_luminance = surface_rgb.dot(luminance_weights);
    let surface_chromaticity = surface_rgb / surface_luminance.max(f32::EPSILON);

    let pale_corona = Vec3::new(0.62, 0.78, 1.0);
    let pale_chromaticity = pale_corona / pale_corona.dot(luminance_weights);
    let tinted_chromaticity = pale_chromaticity.lerp(surface_chromaticity, SURFACE_TINT);
    let tinted_luminance = tinted_chromaticity.dot(luminance_weights);

    (tinted_chromaticity * (surface_luminance / tinted_luminance)).extend(1.0)
}
