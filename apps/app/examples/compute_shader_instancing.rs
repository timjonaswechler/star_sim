//! Places many mesh instances entirely on the GPU with a compute shader.
//!
//! The CPU uploads only a small time/radius uniform. The compute shader writes
//! one position/scale value per instance, and the render pipeline consumes that
//! same buffer as an instanced vertex buffer.
//!
//! Run with:
//! `cargo run -p app --example compute_shader_instancing`

use std::borrow::Cow;

use bevy::core_pipeline::{
    core_3d::{Transparent3d, TransparentSortingInfo3d},
    schedule::camera_driver,
};
use bevy::pbr::{
    self, MeshInputUniform, MeshPipelineSystems, MeshUniform, SetMeshViewBindingArrayBindGroup,
    ViewKeyCache,
};
use bevy::{
    camera::visibility::NoFrustumCulling,
    ecs::{
        query::QueryItem,
        system::{SystemParamItem, lifetimeless::*},
    },
    mesh::{MeshVertexBufferLayoutRef, VertexBufferLayout},
    pbr::{
        MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup, RenderSystems,
        batching::gpu_preprocessing::BatchedInstanceBuffers,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        mesh::{RenderMesh, RenderMeshBufferInfo, allocator::MeshAllocator},
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
            RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
        },
        render_resource::{
            binding_types::{storage_buffer, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice, RenderQueue},
        sync_component::SyncComponent,
        sync_world::MainEntity,
        view::ExtractedView,
    },
};

const COMPUTE_SHADER_PATH: &str = "shaders/compute_instances.wgsl";
const RENDER_SHADER_PATH: &str = "shaders/render_compute_instances.wgsl";
const INSTANCE_COUNT: u32 = 10_000;
const WORKGROUP_SIZE: u32 = 64;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.005, 0.008, 0.02)))
        .insert_resource(PlacementSettings {
            time: 0.0,
            radius: 10.0,
        })
        .add_plugins((
            DefaultPlugins.set(bevy::asset::AssetPlugin {
                file_path: "../../assets".into(),
                ..default()
            }),
            ComputeInstancingPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_time)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.12, 0.12, 0.12))),
        GpuInstances {
            count: INSTANCE_COUNT,
        },
        // The source mesh remains at the origin, so its normal AABB does not
        // cover the GPU-generated positions.
        // NoFrustumCulling,
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update_time(time: Res<Time>, mut settings: ResMut<PlacementSettings>) {
    settings.time = time.elapsed_secs();
}

#[derive(Resource, Clone, ExtractResource)]
struct PlacementSettings {
    time: f32,
    radius: f32,
}

#[derive(Component, Clone)]
struct GpuInstances {
    count: u32,
}

impl SyncComponent for GpuInstances {
    type Target = Self;
}

impl ExtractComponent for GpuInstances {
    type QueryData = &'static GpuInstances;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, '_, Self::QueryData>) -> Option<Self> {
        Some(item.clone())
    }
}

struct ComputeInstancingPlugin;

impl Plugin for ComputeInstancingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<GpuInstances>::default(),
            ExtractResourcePlugin::<PlacementSettings>::default(),
        ));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_render_command::<Transparent3d, DrawGpuInstances>()
            .init_resource::<SpecializedMeshPipelines<InstanceRenderPipeline>>()
            .add_systems(RenderStartup, init_pipelines.after(MeshPipelineSystems))
            .add_systems(
                Render,
                (
                    prepare_instance_buffers.in_set(RenderSystems::PrepareResources),
                    queue_instances.in_set(RenderSystems::QueueMeshes),
                ),
            )
            // This guarantees that compute writes finish before camera rendering
            // reads the same buffers as vertex buffers.
            .add_systems(RenderGraph, place_instances.before(camera_driver));
    }
}

/// Must match `Instance` in both WGSL shaders.
#[derive(ShaderType)]
#[repr(C)]
struct InstanceData {
    position_scale: Vec4,
}

#[derive(Component)]
struct InstanceBuffer {
    buffer: Buffer,
    length: u32,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    instances: Query<(Entity, &GpuInstances), Without<InstanceBuffer>>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instances) in &instances {
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("compute instance buffer"),
            size: u64::from(instances.count) * size_of::<InstanceData>() as u64,
            // Compute writes this buffer; the render pass reads it as instance
            // vertex data without sending it back through the CPU.
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: instances.count,
        });
    }
}

#[derive(ShaderType)]
struct ComputeParameters {
    instance_count: u32,
    time: f32,
    radius: f32,
    padding: f32,
}

#[derive(Resource)]
struct InstanceComputePipeline {
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
}

#[derive(Resource)]
struct InstanceRenderPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

fn init_pipelines(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
    mesh_pipeline: Res<MeshPipeline>,
) {
    let compute_layout = BindGroupLayoutDescriptor::new(
        "compute instance layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                uniform_buffer::<ComputeParameters>(false),
                storage_buffer::<Vec<InstanceData>>(false),
            ),
        ),
    );
    let compute_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("place mesh instances".into()),
        layout: vec![compute_layout.clone()],
        shader: asset_server.load(COMPUTE_SHADER_PATH),
        entry_point: Some(Cow::Borrowed("place_instances")),
        ..default()
    });

    commands.insert_resource(InstanceComputePipeline {
        layout: compute_layout,
        pipeline: compute_pipeline,
    });
    commands.insert_resource(InstanceRenderPipeline {
        shader: asset_server.load(RENDER_SHADER_PATH),
        mesh_pipeline: mesh_pipeline.clone(),
    });
}

fn place_instances(
    mut render_context: RenderContext,
    instance_buffers: Query<&InstanceBuffer>,
    settings: Res<PlacementSettings>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<InstanceComputePipeline>,
    render_queue: Res<RenderQueue>,
) {
    let Some(compute_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) else {
        return;
    };

    for instance_buffer in &instance_buffers {
        let mut parameters = UniformBuffer::from(ComputeParameters {
            instance_count: instance_buffer.length,
            time: settings.time,
            radius: settings.radius,
            padding: 0.0,
        });
        parameters.write_buffer(render_context.render_device(), &render_queue);

        let bind_group = render_context.render_device().create_bind_group(
            Some("compute instance bind group"),
            &pipeline_cache.get_bind_group_layout(&pipeline.layout),
            &BindGroupEntries::sequential((
                &parameters,
                instance_buffer.buffer.as_entire_buffer_binding(),
            )),
        );

        let mut pass =
            render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: Some("place mesh instances"),
                    ..default()
                });
        pass.set_pipeline(compute_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(instance_buffer.length.div_ceil(WORKGROUP_SIZE), 1, 1);
    }
}

impl SpecializedMeshPipeline for InstanceRenderPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 0,
                shader_location: 3,
            }],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}

fn queue_instances(
    draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<InstanceRenderPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<InstanceRenderPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    maybe_batched_instance_buffers: Option<
        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    >,
    instance_meshes: Query<(Entity, &MainEntity), With<GpuInstances>>,
    mut transparent_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
    views: Query<&ExtractedView>,
    view_key_cache: Res<ViewKeyCache>,
) {
    let draw_function = draw_functions.read().id::<DrawGpuInstances>();

    for view in &views {
        let Some(phase) = transparent_phases.get_mut(&view.retained_view_entity) else {
            continue;
        };
        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
            continue;
        };

        for (entity, main_entity) in &instance_meshes {
            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*main_entity)
            else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id()) else {
                continue;
            };
            let key = view_key
                | MeshPipelineKey::from_primitive_topology_and_strip_index(
                    mesh.primitive_topology(),
                    mesh.index_format(),
                );
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                .unwrap();

            phase.add_retained(Transparent3d {
                sorting_info: TransparentSortingInfo3d::Sorted {
                    mesh_center: pbr::get_mesh_instance_world_from_local(
                        *main_entity,
                        mesh_instance.current_uniform_index,
                        &render_mesh_instances,
                        maybe_batched_instance_buffers.as_deref(),
                    )
                    .transform_point3(mesh.aabb_center),
                    depth_bias: 0.0,
                },
                entity: (entity, *main_entity),
                pipeline,
                draw_function,
                distance: 0.0,
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
                indexed: true,
            });
        }
    }
}

type DrawGpuInstances = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshViewBindingArrayBindGroup<1>,
    SetMeshBindGroup<2>,
    DrawMeshInstances,
);

struct DrawMeshInstances;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstances {
    type Param = (
        SRes<RenderAssets<RenderMesh>>,
        SRes<RenderMeshInstances>,
        SRes<MeshAllocator>,
    );
    type ViewQuery = ();
    type ItemQuery = Read<InstanceBuffer>;

    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: Option<&'w InstanceBuffer>,
        (meshes, render_mesh_instances, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_allocator = mesh_allocator.into_inner();
        let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(item.main_entity())
        else {
            return RenderCommandResult::Skip;
        };
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id()) else {
            return RenderCommandResult::Skip;
        };
        let Some(instance_buffer) = instance_buffer else {
            return RenderCommandResult::Skip;
        };
        let Some(vertex_buffer_slice) =
            mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id())
        else {
            return RenderCommandResult::Skip;
        };

        pass.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            RenderMeshBufferInfo::Indexed {
                index_format,
                count,
            } => {
                let Some(index_buffer_slice) =
                    mesh_allocator.mesh_index_slice(&mesh_instance.mesh_asset_id())
                else {
                    return RenderCommandResult::Skip;
                };
                pass.set_index_buffer(index_buffer_slice.buffer.slice(..), *index_format);
                pass.draw_indexed(
                    index_buffer_slice.range.start..index_buffer_slice.range.start + count,
                    vertex_buffer_slice.range.start as i32,
                    0..instance_buffer.length,
                );
            }
            RenderMeshBufferInfo::NonIndexed => {
                pass.draw(vertex_buffer_slice.range, 0..instance_buffer.length);
            }
        }

        RenderCommandResult::Success
    }
}
