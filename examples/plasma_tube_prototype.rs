//! PROTOTYPE material for animated, emissive plasma flux tubes.

use bevy::{
    asset::{Asset, Handle, load_internal_asset, uuid_handle},
    pbr::{Material, MaterialPlugin},
    prelude::{AlphaMode, App, Plugin, Vec4},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::{Shader, ShaderRef},
};

const PLASMA_TUBE_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("7cb29069-b902-402e-a806-ed46db95e9e2");

pub struct PlasmaTubeMaterialPlugin;

impl Plugin for PlasmaTubeMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PLASMA_TUBE_SHADER_HANDLE,
            "../assets/shaders/plasma_tube.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<PlasmaTubeMaterial>::default());
    }
}

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct PlasmaTubeUniform {
    /// Pre-exposed linear HDR color used by this visual prototype.
    pub color: Vec4,
    /// x: brightness, y: fill, z: flow speed, w: per-tube phase offset.
    pub dynamics: Vec4,
    /// x: longitudinal detail scale, y: turbulence, z/w: reserved.
    pub detail: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlasmaTubeMaterial {
    #[uniform(0)]
    pub parameters: PlasmaTubeUniform,
}

impl Material for PlasmaTubeMaterial {
    fn fragment_shader() -> ShaderRef {
        PLASMA_TUBE_SHADER_HANDLE.clone().into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}
