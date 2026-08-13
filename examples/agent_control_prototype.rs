//! Development demonstration for issues #34/#35. Run with `tools/demo_agent_control_prototype.py`.

use agent_control::{
    AgentControlPlugin, AgentRequest, AgentRequests, AgentTarget, ArtifactRoot, CameraPose,
    Command, Coordinate, DeterministicAnimation, OperationMode, Response, RunMode, RunState,
    ScreenshotSource, complete_request, focus_pose, orbit_pose, viewport_pixels, zoom_pose,
};
use bevy::{
    asset::RenderAssetUsages,
    camera::RenderTarget,
    image::Image,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::screenshot::{Screenshot, ScreenshotCaptured},
    },
    ui::InteractionDisabled,
    window::PrimaryWindow,
};
use serde_json::json;
use std::path::PathBuf;

#[derive(Component)]
struct PrototypeButton;
#[derive(Component)]
struct ControlledCamera;
#[derive(Component)]
struct CameraCapture(Handle<Image>);
#[derive(Component)]
struct CaptureCamera;
#[derive(Component)]
struct FocusTarget {
    radius: f32,
}
#[derive(Resource, Default)]
struct ClickCount(u32);
#[derive(Resource)]
struct DemoArtifacts(ArtifactRoot);
#[derive(Resource)]
struct CameraState {
    target: Vec3,
}
#[derive(Resource)]
struct PendingCamera {
    id: String,
    start: CameraPose,
    end: CameraPose,
    animation: DeterministicAnimation,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args.iter().any(|argument| argument == "--agent") {
        eprintln!("This demonstration only runs with --agent");
        std::process::exit(2);
    }
    let artifact_dir = args
        .iter()
        .position(|value| value == "--artifact-dir")
        .and_then(|index| args.get(index + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/agent-prototype"));
    let artifacts = ArtifactRoot::new(artifact_dir).expect("create artifact root");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Agent Control #35".into(),
                resolution: (640, 360).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AgentControlPlugin::default().configured(RunMode::Rendered, 42, 50))
        .insert_resource(DemoArtifacts(artifacts))
        .insert_resource(CameraState { target: Vec3::ZERO })
        .init_resource::<ClickCount>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                human_button_interaction,
                agent_adapter,
                advance_camera,
                sync_capture_camera,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    window: Single<Entity, With<PrimaryWindow>>,
) {
    commands.entity(*window).insert(AgentTarget::new(
        "window.primary",
        "window",
        "Primary window",
        ["screenshot"],
    ));
    let mut image = Image::new_uninit(
        Extent3d {
            width: 320,
            height: 180,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    let capture = images.add(image);
    let transform = Transform::from_xyz(0.0, 2.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3d::default(),
        transform,
        ControlledCamera,
        CameraCapture(capture.clone()),
        AgentTarget::new(
            "camera.main",
            "camera",
            "Main camera",
            ["focus", "orbit", "pan", "zoom", "screenshot"],
        ),
    ));
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: -1,
            ..default()
        },
        RenderTarget::Image(capture.into()),
        transform,
        CaptureCamera,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.55, 0.1),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        FocusTarget { radius: 0.8 },
        AgentTarget::new("scene.prototype_star", "scene", "Prototype star", ["focus"]),
    ));
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 6.0, 4.0),
    ));
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: px(20),
                top: px(20),
                padding: UiRect::all(px(12)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.25, 0.45)),
            PrototypeButton,
            AgentTarget::new("toolbar.generate", "button", "Generate", ["click"]),
        ))
        .with_child((Text::new("Generate (0)"), TextColor(Color::WHITE)));
}

fn human_button_interaction(
    interactions: Query<&Interaction, (Changed<Interaction>, With<PrototypeButton>)>,
    mut count: ResMut<ClickCount>,
    button: Single<&Children, With<PrototypeButton>>,
    mut text: Query<&mut Text>,
) {
    if interactions
        .iter()
        .any(|value| *value == Interaction::Pressed)
    {
        activate_generate(&mut count, &button, &mut text);
    }
}

fn agent_adapter(world: &mut World) {
    if world.contains_resource::<PendingCamera>() {
        return;
    }
    let requests: Vec<AgentRequest> = world.resource_mut::<AgentRequests>().drain().collect();
    let mut deferred = Vec::new();
    for AgentRequest(request) in requests {
        match request.command.clone() {
            Command::Click { target } if target == "toolbar.generate" => {
                handle_click(world, request.id, target)
            }
            Command::CameraFocus {
                camera,
                target,
                duration_ms,
            } => {
                let result = begin_focus(world, &camera, &target, duration_ms, request.id.clone());
                if let Err((code, message)) = result {
                    complete_request(world, Response::error(Some(request.id), code, message));
                }
            }
            Command::CameraOrbit {
                camera,
                mode,
                yaw_deg,
                pitch_deg,
                duration_ms,
            } => {
                let result = begin_orbit(
                    world,
                    &camera,
                    mode,
                    yaw_deg,
                    pitch_deg,
                    duration_ms,
                    request.id.clone(),
                );
                if let Err((code, message)) = result {
                    complete_request(world, Response::error(Some(request.id), code, message));
                }
            }
            Command::CameraPan {
                camera,
                mode,
                offset,
                duration_ms,
            } => {
                let result = begin_pan(
                    world,
                    &camera,
                    mode,
                    offset,
                    duration_ms,
                    request.id.clone(),
                );
                if let Err((code, message)) = result {
                    complete_request(world, Response::error(Some(request.id), code, message));
                }
            }
            Command::CameraZoom {
                camera,
                mode,
                value,
                duration_ms,
            } => {
                let result =
                    begin_zoom(world, &camera, mode, value, duration_ms, request.id.clone());
                if let Err((code, message)) = result {
                    complete_request(world, Response::error(Some(request.id), code, message));
                }
            }
            Command::Screenshot {
                source,
                path,
                overwrite,
            } => handle_screenshot(world, request.id, source, path, overwrite),
            _ => deferred.push(AgentRequest(request)),
        }
        if world.contains_resource::<PendingCamera>() {
            deferred.extend(world.resource_mut::<AgentRequests>().drain());
            break;
        }
    }
    for request in deferred {
        world.resource_mut::<AgentRequests>().defer(request);
    }
}

type AdapterResult = Result<(), (&'static str, String)>;

fn unique_target(world: &World, id: &str, role: &str) -> Result<Entity, (&'static str, String)> {
    let entity = world
        .resource::<agent_control::TargetRegistry>()
        .entity(id)
        .map_err(|error| match error {
            agent_control::RegistryLookupError::Unknown(_) => {
                ("unknown_target", format!("unknown target: {id}"))
            }
            agent_control::RegistryLookupError::Duplicate(_) => {
                ("ambiguous_target", format!("ambiguous target: {id}"))
            }
        })?;
    let actual = world
        .get::<AgentTarget>(entity)
        .map(|target| target.role.as_str());
    (actual == Some(role))
        .then_some(entity)
        .ok_or(("wrong_target_role", format!("{id} is not a {role}")))
}

fn current_pose(world: &World, camera: Entity) -> CameraPose {
    let transform = world.get::<Transform>(camera).unwrap();
    CameraPose {
        position: transform.translation,
        target: world.resource::<CameraState>().target,
        up: Vec3::Y,
    }
}

fn begin(world: &mut World, id: String, start: CameraPose, end: CameraPose, duration_ms: u32) {
    world.resource_mut::<RunState>().camera_motion_pending = true;
    world.insert_resource(PendingCamera {
        id,
        start,
        end,
        animation: DeterministicAnimation::new(duration_ms),
    });
}

fn begin_focus(
    world: &mut World,
    camera_id: &str,
    target_id: &str,
    duration: u32,
    id: String,
) -> AdapterResult {
    let camera = unique_target(world, camera_id, "camera")?;
    let target = unique_target(world, target_id, "scene")?;
    let start = current_pose(world, camera);
    let transform = world.get::<GlobalTransform>(target).unwrap();
    let radius = world
        .get::<FocusTarget>(target)
        .ok_or(("unsupported_action", "target has no visual extent".into()))?
        .radius;
    let end = focus_pose(
        start,
        transform.translation(),
        radius,
        60_f32.to_radians(),
        1.2,
    )
    .map_err(|value| ("invalid_arguments", value.into()))?;
    begin(world, id, start, end, duration);
    Ok(())
}

fn begin_orbit(
    world: &mut World,
    camera_id: &str,
    mode: OperationMode,
    yaw: f32,
    pitch: f32,
    duration: u32,
    id: String,
) -> AdapterResult {
    let camera = unique_target(world, camera_id, "camera")?;
    let start = current_pose(world, camera);
    let end = orbit_pose(start, mode, yaw, pitch)
        .map_err(|message| ("invalid_arguments", message.into()))?;
    begin(world, id, start, end, duration);
    Ok(())
}

fn begin_pan(
    world: &mut World,
    camera_id: &str,
    mode: OperationMode,
    offset: Coordinate,
    duration: u32,
    id: String,
) -> AdapterResult {
    let camera = unique_target(world, camera_id, "camera")?;
    let start = current_pose(world, camera);
    let delta = match offset {
        Coordinate::World { x, y, z } => Vec3::new(x, y, z),
        value => {
            let pixel = viewport_pixels(value, Vec2::new(640.0, 360.0))
                .map_err(|message| ("invalid_arguments", message.into()))?;
            Vec3::new(pixel.x / 640.0 - 0.5, 0.5 - pixel.y / 360.0, 0.0)
        }
    };
    let target = if mode == OperationMode::Relative {
        start.target + delta
    } else {
        delta
    };
    let end = CameraPose {
        position: start.position + (target - start.target),
        target,
        ..start
    };
    begin(world, id, start, end, duration);
    Ok(())
}

fn begin_zoom(
    world: &mut World,
    camera_id: &str,
    mode: OperationMode,
    value: f32,
    duration: u32,
    id: String,
) -> AdapterResult {
    let camera = unique_target(world, camera_id, "camera")?;
    let start = current_pose(world, camera);
    let end =
        zoom_pose(start, mode, value).map_err(|message| ("invalid_arguments", message.into()))?;
    begin(world, id, start, end, duration);
    Ok(())
}

fn advance_camera(world: &mut World) {
    let Some(mut pending) = world.remove_resource::<PendingCamera>() else {
        return;
    };
    let fraction = pending.animation.advance(50);
    let pose = pending.start.interpolate(pending.end, fraction);
    let mut cameras = world.query_filtered::<&mut Transform, With<ControlledCamera>>();
    let Ok(mut transform) = cameras.single_mut(world) else {
        return;
    };
    *transform = Transform::from_translation(pose.position).looking_at(pose.target, pose.up);
    world.resource_mut::<CameraState>().target = pose.target;
    if pending.animation.complete() {
        world.resource_mut::<RunState>().camera_motion_pending = false;
        complete_request(
            world,
            Response::completed(
                pending.id,
                json!({"position": pose.position.to_array(), "target": pose.target.to_array()}),
            ),
        );
    } else {
        world.insert_resource(pending);
    }
}

fn sync_capture_camera(
    controlled: Single<&Transform, (With<ControlledCamera>, Without<CaptureCamera>)>,
    mut capture: Single<&mut Transform, (With<CaptureCamera>, Without<ControlledCamera>)>,
) {
    **capture = **controlled;
}

fn handle_click(world: &mut World, id: String, target: String) {
    let Ok(entity) = unique_target(world, &target, "button") else {
        complete_request(world, Response::error(Some(id), "unknown_target", target));
        return;
    };
    if world.get::<InteractionDisabled>(entity).is_some() {
        complete_request(
            world,
            Response::error(Some(id), "target_disabled", "target is disabled"),
        );
        return;
    }
    world.resource_scope(|world, mut count: Mut<ClickCount>| {
        let child = world
            .get::<Children>(entity)
            .and_then(|children| children.first())
            .copied();
        let value = generate(&mut count);
        if let Some(child) = child {
            world.get_mut::<Text>(child).unwrap().0 = format!("Generate ({value})");
        }
    });
    complete_request(
        world,
        Response::completed(
            id,
            json!({"target": target, "click_count": world.resource::<ClickCount>().0}),
        ),
    );
}

fn handle_screenshot(
    world: &mut World,
    id: String,
    source: ScreenshotSource,
    path: Option<String>,
    overwrite: bool,
) {
    let default_path = format!("screenshots/{id}.png");
    let destination = match world
        .resource::<DemoArtifacts>()
        .0
        .reserve(path.as_deref().unwrap_or(&default_path), overwrite)
    {
        Ok(value) => value,
        Err(error) => {
            complete_request(
                world,
                Response::error(Some(id), "invalid_artifact_path", error.to_string()),
            );
            return;
        }
    };
    let screenshot = match source {
        ScreenshotSource::Window { target } => match unique_target(world, &target, "window") {
            Ok(entity) => Screenshot::window(entity),
            Err((code, message)) => {
                complete_request(world, Response::error(Some(id), code, message));
                return;
            }
        },
        ScreenshotSource::Camera { target } => match unique_target(world, &target, "camera") {
            Ok(entity) => Screenshot::image(world.get::<CameraCapture>(entity).unwrap().0.clone()),
            Err((code, message)) => {
                complete_request(world, Response::error(Some(id), code, message));
                return;
            }
        },
    };
    world.resource_mut::<RunState>().screenshot_pending = true;
    let mut destination = Some(destination);
    world.spawn(screenshot).observe(
        move |event: On<ScreenshotCaptured>, mut commands: Commands| {
            let Some(destination) = destination.take() else {
                return;
            };
            let response = match destination.write_png(event.image.clone()) {
                Ok(path) => Response::completed(id.clone(), json!({"path": path})),
                Err(error) => {
                    Response::error(Some(id.clone()), "screenshot_failed", error.to_string())
                }
            };
            commands.queue(move |world: &mut World| {
                world.resource_mut::<RunState>().screenshot_pending = false;
                complete_request(world, response);
            });
        },
    );
}

fn activate_generate(count: &mut ClickCount, button: &Children, text: &mut Query<&mut Text>) {
    let value = generate(count);
    if let Some(child) = button.first()
        && let Ok(mut label) = text.get_mut(*child)
    {
        label.0 = format!("Generate ({value})");
    }
}
fn generate(count: &mut ClickCount) -> u32 {
    count.0 += 1;
    count.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_operation_does_not_require_agent_transport() {
        let mut count = ClickCount::default();
        assert_eq!(generate(&mut count), 1);
    }
}
