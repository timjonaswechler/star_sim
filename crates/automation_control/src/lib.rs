//! Optional development tooling for controlling a Bevy application over JSON Lines.
//!
//! This crate owns protocol, transport, target discovery, and observation. It deliberately does
//! not own application actions: the host polls [`AutomationRequest`] and calls its normal operation.

#[cfg(feature = "render")]
mod artifact;
mod control;
mod coordinates;
mod protocol;
mod target;
mod transport;

#[cfg(feature = "render")]
pub use artifact::{ArtifactDestination, ArtifactError, ArtifactRoot};
pub use control::{DEFAULT_FIXED_STEP_MS, RunState, WaitEvaluation, evaluate_wait};
pub use coordinates::{
    CameraPose, Coordinate, DeterministicAnimation, OperationMode, PublicRay3d, degrees,
    focus_pose, orbit_pose, project_ray_to_plane, validate_coordinate, viewport_pixels,
    viewport_to_world_ray, world_to_viewport, zoom_pose,
};
pub use protocol::{
    Command, DEFAULT_CAMERA_DURATION_MS, DEFAULT_WAIT_TIMEOUT_FRAMES, MAX_STEP_FRAMES,
    MAX_STEP_SIMULATION_MS, MAX_WAIT_TIMEOUT_FRAMES, PROTOCOL_VERSION, ProtocolError, Ready,
    Request, Response, ResponseStatus, RunMode, ScreenshotSource, WaitCondition, decode_request,
};
pub use target::{
    AutomationTarget, Bounds, Observations, RegistryLookupError, TargetObservation, TargetRegistry,
};
pub use transport::{Input, JsonLinesInput, Output, StdoutOutput};

use bevy::{app::AppExit, prelude::*, time::TimeUpdateStrategy, ui::UiSystems};
use serde_json::json;
use std::{collections::HashSet, sync::Arc, sync::mpsc::TryRecvError};

#[derive(Clone)]
pub struct AutomationControlPlugin {
    capabilities: Vec<String>,
    mode: RunMode,
    seed: u64,
    fixed_step_ms: u32,
    input_capacity: usize,
    output: Arc<dyn Output>,
    input_factory: Arc<dyn Fn() -> JsonLinesInput + Send + Sync>,
}

impl std::fmt::Debug for AutomationControlPlugin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AutomationControlPlugin")
            .field("capabilities", &self.capabilities)
            .field("mode", &self.mode)
            .field("seed", &self.seed)
            .field("fixed_step_ms", &self.fixed_step_ms)
            .field("input_capacity", &self.input_capacity)
            .finish_non_exhaustive()
    }
}

impl Default for AutomationControlPlugin {
    fn default() -> Self {
        Self::stdio([
            "inspect_ui",
            "inspect_scene",
            "inspect_selection",
            "inspect_camera",
            "click",
            "camera_focus",
            "camera_orbit",
            "camera_pan",
            "camera_zoom",
            "screenshot",
            "pause",
            "resume",
            "step_frames",
            "step_simulation",
            "wait_until",
            "inspect_run",
            "shutdown",
        ])
    }
}

impl AutomationControlPlugin {
    pub fn stdio(capabilities: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let input_capacity = 64;
        Self {
            capabilities: capabilities.into_iter().map(Into::into).collect(),
            mode: RunMode::Rendered,
            seed: 0,
            fixed_step_ms: DEFAULT_FIXED_STEP_MS,
            input_capacity,
            output: Arc::new(StdoutOutput),
            input_factory: Arc::new(move || JsonLinesInput::stdin(input_capacity)),
        }
    }

    pub fn with_io(
        capabilities: impl IntoIterator<Item = impl Into<String>>,
        input: impl Fn() -> JsonLinesInput + Send + Sync + 'static,
        output: Arc<dyn Output>,
    ) -> Self {
        Self {
            capabilities: capabilities.into_iter().map(Into::into).collect(),
            mode: RunMode::Logical,
            seed: 0,
            fixed_step_ms: DEFAULT_FIXED_STEP_MS,
            input_capacity: 0,
            output,
            input_factory: Arc::new(input),
        }
    }

    pub fn configured(mut self, mode: RunMode, seed: u64, fixed_step_ms: u32) -> Self {
        assert!(fixed_step_ms > 0, "fixed_step_ms must be positive");
        self.mode = mode;
        self.seed = seed;
        self.fixed_step_ms = fixed_step_ms;
        self
    }
}

impl Plugin for AutomationControlPlugin {
    fn build(&self, app: &mut App) {
        if self.mode == RunMode::Logical {
            app.init_schedule(FixedUpdate);
            app.insert_resource(TimeUpdateStrategy::ManualDuration(
                std::time::Duration::ZERO,
            ));
            app.insert_resource(Time::<Fixed>::from_duration(
                std::time::Duration::from_millis(u64::from(self.fixed_step_ms)),
            ));
        }
        app.insert_resource(Transport {
            input: (self.input_factory)(),
            output: Arc::clone(&self.output),
        })
        .insert_resource(Capabilities(self.capabilities.clone()))
        .insert_resource(AutomationConfiguration {
            mode: self.mode,
            seed: self.seed,
            fixed_step_ms: self.fixed_step_ms,
        })
        .insert_resource(RunState::new(self.seed, self.fixed_step_ms))
        .init_resource::<TargetRegistry>()
        .init_resource::<Observations>()
        .init_resource::<AutomationRequests>()
        .init_resource::<CompletedRequests>()
        .init_resource::<PendingWaits>()
        .add_systems(Startup, emit_ready)
        .add_systems(First, receive_requests)
        .add_systems(PreUpdate, target::sync_registry)
        .add_systems(
            PostUpdate,
            target::observe_targets.after(UiSystems::PostLayout),
        )
        .add_systems(
            Last,
            (complete_builtin_requests, evaluate_pending_waits).chain(),
        );
    }
}

#[derive(Resource)]
struct Transport {
    input: JsonLinesInput,
    output: Arc<dyn Output>,
}

#[derive(Resource)]
struct Capabilities(Vec<String>);

#[derive(Clone, Copy, Resource)]
struct AutomationConfiguration {
    mode: RunMode,
    seed: u64,
    fixed_step_ms: u32,
}

struct PendingWait {
    id: String,
    condition: WaitCondition,
    remaining_frames: u32,
    baseline_frames: u64,
}

#[derive(Default, Resource)]
struct PendingWaits(Vec<PendingWait>);

#[derive(Clone, Debug)]
pub struct AutomationRequest(pub Request);

#[derive(Default, Resource)]
pub struct AutomationRequests(Vec<AutomationRequest>);

impl AutomationRequests {
    pub fn drain(&mut self) -> impl Iterator<Item = AutomationRequest> + '_ {
        self.0.drain(..)
    }

    /// Returns a request to the crate for built-in handling later in the frame.
    pub fn defer(&mut self, request: AutomationRequest) {
        self.0.push(request);
    }
}

#[derive(Default, Resource)]
struct CompletedRequests(HashSet<String>);

#[derive(Resource, Default)]
pub struct ShutdownRequested(pub bool);

/// Completes an app-owned request exactly once.
///
/// Returns `false` if this request ID already received a final response.
pub fn complete_request(world: &mut World, response: Response) -> bool {
    let Some(id) = response.id.clone() else {
        return write_response(world, response);
    };
    if !world.resource_mut::<CompletedRequests>().0.insert(id) {
        return false;
    }
    write_response(world, response)
}

fn write_response(world: &World, response: Response) -> bool {
    if let Err(error) = world.resource::<Transport>().output.response(&response) {
        eprintln!("automation-control response failed: {error}");
        return false;
    }
    true
}

fn emit_ready(
    transport: Res<Transport>,
    capabilities: Res<Capabilities>,
    configuration: Res<AutomationConfiguration>,
) {
    if let Err(error) = transport.output.ready(&Ready::new(
        capabilities.0.clone(),
        configuration.mode,
        configuration.seed,
        configuration.fixed_step_ms,
    )) {
        eprintln!("automation-control ready failed: {error}");
    }
}

fn receive_requests(world: &mut World) {
    loop {
        let input = {
            let transport = world.resource::<Transport>();
            transport.input.try_recv()
        };
        match input {
            Ok(Input::Line(line)) => match decode_request(&line) {
                Ok(request) => world
                    .resource_mut::<AutomationRequests>()
                    .0
                    .push(AutomationRequest(request)),
                Err(response) => {
                    complete_request(world, response);
                }
            },
            Ok(Input::Eof) => {
                world.write_message(AppExit::Success);
                break;
            }
            Ok(Input::Error(message)) => {
                let response = Response::error(None::<String>, "input_error", message);
                write_response(world, response);
                world.write_message(AppExit::error());
                break;
            }
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
        }
    }
}

fn complete_builtin_requests(world: &mut World) {
    let requests: Vec<_> = world.resource_mut::<AutomationRequests>().drain().collect();
    for AutomationRequest(request) in requests {
        match request.command {
            Command::InspectUi
            | Command::InspectScene
            | Command::InspectSelection
            | Command::InspectCamera => {
                let observations = world.resource::<Observations>();
                let elements = match request.command {
                    Command::InspectUi => observations.ui.clone(),
                    Command::InspectScene => observations.scene.clone(),
                    Command::InspectSelection => observations.selection.clone(),
                    Command::InspectCamera => observations.camera.clone(),
                    _ => unreachable!(),
                };
                complete_request(
                    world,
                    Response::completed(request.id, json!({ "elements": elements })),
                );
            }
            Command::InspectRun => {
                let result = world.resource::<RunState>().observation();
                complete_request(world, Response::completed(request.id, result));
            }
            Command::Pause => {
                world.resource_mut::<RunState>().paused = true;
                let result = world.resource::<RunState>().observation();
                complete_request(world, Response::completed(request.id, result));
            }
            Command::Resume => {
                world.resource_mut::<RunState>().paused = false;
                let result = world.resource::<RunState>().observation();
                complete_request(world, Response::completed(request.id, result));
            }
            Command::StepFrames { count } => {
                world.resource_mut::<RunState>().step_frames(count);
                let result = world.resource::<RunState>().observation();
                complete_request(world, Response::completed(request.id, result));
            }
            Command::StepSimulation { duration_ms } => {
                let advanced_ms = world
                    .resource_mut::<RunState>()
                    .step_simulation(duration_ms);
                let fixed_step_ms = u64::from(world.resource::<RunState>().fixed_step_ms);
                let fixed_steps = advanced_ms / fixed_step_ms;
                for _ in 0..fixed_steps {
                    world.run_schedule(FixedUpdate);
                }
                let mut result = world.resource::<RunState>().observation();
                result["fixed_steps"] = json!(fixed_steps);
                result["advanced_ms"] = json!(advanced_ms);
                complete_request(world, Response::completed(request.id, result));
            }
            Command::WaitUntil {
                condition,
                timeout_frames,
            } => {
                let baseline_frames = world.resource::<RunState>().rendered_frames;
                let evaluation = evaluate_wait(
                    &condition,
                    world.resource::<RunState>(),
                    world.resource::<TargetRegistry>(),
                    world.resource::<Observations>(),
                    baseline_frames,
                );
                if evaluation.satisfied {
                    complete_request(
                        world,
                        Response::completed(request.id, evaluation.observation),
                    );
                } else {
                    world.resource_mut::<PendingWaits>().0.push(PendingWait {
                        id: request.id,
                        condition,
                        remaining_frames: timeout_frames,
                        baseline_frames,
                    });
                }
            }
            Command::Shutdown => {
                complete_request(world, Response::completed(request.id, json!({})));
                world.write_message(AppExit::Success);
            }
            Command::Click { target } => {
                let response = match world.resource::<TargetRegistry>().entity(&target) {
                    Err(RegistryLookupError::Unknown(_)) => Response::error(
                        Some(request.id),
                        "unknown_target",
                        format!("unknown target: {target}"),
                    ),
                    Err(RegistryLookupError::Duplicate(_)) => Response::error(
                        Some(request.id),
                        "duplicate_target",
                        format!("duplicate target: {target}"),
                    ),
                    Ok(entity) => {
                        let supports = world
                            .get::<AutomationTarget>(entity)
                            .is_some_and(|metadata| metadata.supports("click"));
                        if supports {
                            world
                                .resource_mut::<AutomationRequests>()
                                .0
                                .push(AutomationRequest(Request {
                                    version: request.version,
                                    id: request.id,
                                    command: Command::Click { target },
                                }));
                            continue;
                        }
                        Response::error(
                            Some(request.id),
                            "unsupported_action",
                            format!("{target} cannot be clicked"),
                        )
                    }
                };
                complete_request(world, response);
            }
            Command::CameraFocus { .. }
            | Command::CameraOrbit { .. }
            | Command::CameraPan { .. }
            | Command::CameraZoom { .. }
            | Command::Screenshot { .. } => {
                let capability = match request.command {
                    Command::CameraFocus { .. } => "camera_focus",
                    Command::CameraOrbit { .. } => "camera_orbit",
                    Command::CameraPan { .. } => "camera_pan",
                    Command::CameraZoom { .. } => "camera_zoom",
                    Command::Screenshot { .. } => "screenshot",
                    _ => unreachable!(),
                };
                if !world
                    .resource::<Capabilities>()
                    .0
                    .iter()
                    .any(|value| value == capability)
                {
                    complete_request(
                        world,
                        Response::error(
                            Some(request.id),
                            "unsupported_capability",
                            format!("capability {capability} is unavailable in this mode"),
                        ),
                    );
                } else {
                    world
                        .resource_mut::<AutomationRequests>()
                        .0
                        .push(AutomationRequest(request));
                }
            }
        }
    }
}

fn evaluate_pending_waits(world: &mut World) {
    let mut pending = std::mem::take(&mut world.resource_mut::<PendingWaits>().0);
    let mut retained = Vec::new();
    for mut wait in pending.drain(..) {
        let evaluation = evaluate_wait(
            &wait.condition,
            world.resource::<RunState>(),
            world.resource::<TargetRegistry>(),
            world.resource::<Observations>(),
            wait.baseline_frames,
        );
        if evaluation.satisfied {
            complete_request(world, Response::completed(wait.id, evaluation.observation));
            continue;
        }
        wait.remaining_frames -= 1;
        if wait.remaining_frames == 0 {
            complete_request(
                world,
                Response::error(
                    Some(wait.id),
                    "wait_timeout",
                    serde_json::to_string(&json!({
                        "condition": wait.condition,
                        "last_observation": evaluation.observation,
                    }))
                    .expect("timeout receipt serializes"),
                ),
            );
        } else {
            retained.push(wait);
        }
    }
    world.resource_mut::<PendingWaits>().0 = retained;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, mpsc};

    #[derive(Default)]
    struct MemoryOutput {
        ready: Mutex<Vec<Ready>>,
        responses: Mutex<Vec<Response>>,
    }

    impl Output for MemoryOutput {
        fn ready(&self, value: &Ready) -> std::io::Result<()> {
            self.ready.lock().unwrap().push(value.clone());
            Ok(())
        }

        fn response(&self, value: &Response) -> std::io::Result<()> {
            self.responses.lock().unwrap().push(value.clone());
            Ok(())
        }
    }

    fn test_app() -> (App, mpsc::SyncSender<Input>, Arc<MemoryOutput>) {
        let (sender, receiver) = mpsc::sync_channel(16);
        let receiver = Mutex::new(Some(receiver));
        let output = Arc::new(MemoryOutput::default());
        let plugin_output: Arc<dyn Output> = output.clone();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AutomationControlPlugin::with_io(
            ["inspect_ui", "click", "shutdown"],
            move || JsonLinesInput::from_receiver(receiver.lock().unwrap().take().unwrap()),
            plugin_output,
        ));
        (app, sender, output)
    }

    #[test]
    fn protocol_rejects_malformed_and_unsupported_input() {
        assert_eq!(
            decode_request("nope").unwrap_err().error.unwrap().code,
            "malformed_request"
        );
        let unsupported = r#"{"version":9,"id":"x","command":{"type":"inspect_ui"}}"#;
        assert_eq!(
            decode_request(unsupported).unwrap_err().error.unwrap().code,
            "unsupported_version"
        );
    }

    #[test]
    fn registry_tracks_spawn_change_despawn_and_duplicates() {
        let (mut app, _sender, _output) = test_app();
        let one = app
            .world_mut()
            .spawn(AutomationTarget::new(
                "one",
                "scene",
                "One",
                [] as [&str; 0],
            ))
            .id();
        app.update();
        assert!(app.world().resource::<TargetRegistry>().contains("one"));
        app.world_mut()
            .entity_mut(one)
            .insert(AutomationTarget::new(
                "two",
                "scene",
                "Two",
                [] as [&str; 0],
            ));
        app.update();
        assert!(!app.world().resource::<TargetRegistry>().contains("one"));
        assert!(app.world().resource::<TargetRegistry>().contains("two"));
        let duplicate = app
            .world_mut()
            .spawn(AutomationTarget::new(
                "two",
                "scene",
                "Duplicate",
                [] as [&str; 0],
            ))
            .id();
        app.update();
        assert_eq!(
            app.world().resource::<TargetRegistry>().entity("two"),
            Err(RegistryLookupError::Duplicate("two".into()))
        );
        app.world_mut().despawn(duplicate);
        app.update();
        assert!(app.world().resource::<TargetRegistry>().contains("two"));
        app.world_mut().despawn(one);
        app.update();
        assert!(app.world().resource::<TargetRegistry>().is_empty());
    }

    #[test]
    fn observations_never_serialize_entity_ids() {
        let observation = TargetObservation {
            id: "toolbar.generate".into(),
            role: "button".into(),
            label: "Generate".into(),
            visible: true,
            enabled: true,
            actions: vec!["click".into()],
            bounds: None,
        };
        let json = serde_json::to_string(&observation).unwrap();
        assert!(!json.contains("entity"));
    }

    #[test]
    fn logical_mode_has_no_render_device_and_simulation_steps_fixed_schedule() {
        #[derive(Resource, Default)]
        struct FixedTicks(u32);
        let (mut app, sender, _output) = test_app();
        app.init_resource::<FixedTicks>();
        app.add_systems(FixedUpdate, |mut ticks: ResMut<FixedTicks>| ticks.0 += 1);
        app.update();
        assert!(
            !app.world()
                .contains_resource::<bevy::render::renderer::RenderDevice>()
        );
        sender.send(Input::Line(
            r#"{"version":1,"id":"step","command":{"type":"step_simulation","duration_ms":120}}"#.into(),
        )).unwrap();
        app.update();
        assert_eq!(app.world().resource::<FixedTicks>().0, 3);
    }

    #[test]
    fn wait_timeout_returns_last_relevant_observation() {
        let (mut app, sender, output) = test_app();
        app.update();
        sender.send(Input::Line(
            r#"{"version":1,"id":"wait","command":{"type":"wait_until","condition":{"type":"target_visible","target":"missing"},"timeout_frames":2}}"#.into(),
        )).unwrap();
        app.update();
        app.update();
        let responses = output.responses.lock().unwrap();
        let timeout = responses
            .iter()
            .find(|response| response.id.as_deref() == Some("wait"))
            .unwrap();
        assert_eq!(timeout.error.as_ref().unwrap().code, "wait_timeout");
        assert!(
            timeout
                .error
                .as_ref()
                .unwrap()
                .message
                .contains("last_observation")
        );
    }

    #[test]
    fn requests_receive_only_one_final_response_and_shutdown_and_eof_exit() {
        let (mut app, sender, output) = test_app();
        app.update();
        sender
            .send(Input::Line(
                r#"{"version":1,"id":"stop","command":{"type":"shutdown"}}"#.into(),
            ))
            .unwrap();
        app.update();
        assert_eq!(
            output
                .responses
                .lock()
                .unwrap()
                .iter()
                .filter(|response| response.id.as_deref() == Some("stop"))
                .count(),
            1
        );
        let duplicate = Response::completed("stop", json!({}));
        assert!(!complete_request(app.world_mut(), duplicate));

        let (mut eof_app, eof_sender, _output) = test_app();
        eof_app.update();
        eof_sender.send(Input::Eof).unwrap();
        eof_app.update();
        assert!(eof_app.should_exit().is_some());
    }
}
