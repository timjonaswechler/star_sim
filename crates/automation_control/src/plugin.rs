use crate::{
    observation::{self, Request as ObservationRequest},
    pointer::{self, Command as PointerCommand, State as PointerState},
    protocol::{self, Command, Response, RunMode},
    transport::{Input, JsonLinesInput, Output},
};
use bevy::{
    app::AppExit,
    input::{
        ButtonInput,
        keyboard::{Key, KeyCode, KeyboardFocusLost, KeyboardInput},
        mouse::{
            AccumulatedMouseMotion, AccumulatedMouseScroll, MouseButton, MouseButtonInput,
            MouseMotion, MouseWheel,
        },
        touch::{TouchInput, Touches},
    },
    picking::{PickingPlugin, input::PointerInputSettings},
    prelude::*,
};
use serde_json::json;
use std::sync::{Arc, Mutex};

const INPUT_CAPACITY: usize = 64;

#[derive(Clone)]
pub struct AutomationControlPlugin {
    mode: RunMode,
    output: Arc<dyn Output>,
    input_factory: Arc<dyn Fn() -> JsonLinesInput + Send + Sync>,
}

impl std::fmt::Debug for AutomationControlPlugin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AutomationControlPlugin")
            .field("mode", &self.mode)
            .finish_non_exhaustive()
    }
}

impl Default for AutomationControlPlugin {
    fn default() -> Self {
        Self::stdio()
    }
}

pub trait InputFactory: Send + Sync + 'static {
    fn factory(self) -> Arc<dyn Fn() -> JsonLinesInput + Send + Sync>;
}

impl InputFactory for JsonLinesInput {
    fn factory(self) -> Arc<dyn Fn() -> JsonLinesInput + Send + Sync> {
        let input = Mutex::new(Some(self));
        Arc::new(move || {
            input
                .lock()
                .unwrap()
                .take()
                .expect("controlled input factory called once")
        })
    }
}

impl<F> InputFactory for F
where
    F: Fn() -> JsonLinesInput + Send + Sync + 'static,
{
    fn factory(self) -> Arc<dyn Fn() -> JsonLinesInput + Send + Sync> {
        Arc::new(self)
    }
}

impl AutomationControlPlugin {
    /// Creates a rendered controlled composition using stdin/stdout JSONL.
    pub fn stdio() -> Self {
        Self {
            mode: RunMode::Rendered,
            output: Arc::new(crate::transport::StdoutOutput),
            input_factory: Arc::new(|| JsonLinesInput::stdin(INPUT_CAPACITY)),
        }
    }

    /// Creates a controlled composition with test or embedding transport adapters.
    pub fn with_io(input: impl InputFactory, output: Arc<dyn Output>) -> Self {
        Self {
            mode: RunMode::Logical,
            output,
            input_factory: input.factory(),
        }
    }

    pub fn configured(mut self, mode: RunMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Plugin for AutomationControlPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<PickingPlugin>() {
            app.add_plugins(PickingPlugin);
        }
        // The picking core remains active, but all native mouse and touch producers are disabled.
        // The controlled composition supplies PointerInput messages itself.
        app.insert_resource(PointerInputSettings {
            is_mouse_enabled: false,
            is_touch_enabled: false,
        });
        register_native_input_channels(app);
        app.insert_resource(Transport {
            input: (self.input_factory)(),
            output: Arc::clone(&self.output),
        });
        app.insert_resource(Configuration { mode: self.mode });
        app.init_resource::<PointerState>()
            .init_resource::<PendingRequests>()
            .init_resource::<ExpectedSequence>()
            .add_systems(Startup, emit_ready)
            .add_systems(PostStartup, pointer::ensure_mouse_pointer)
            .add_systems(First, (receive_request, dispatch_request).chain())
            .add_systems(Last, complete_request);
    }
}

#[derive(Resource)]
struct Transport {
    input: JsonLinesInput,
    output: Arc<dyn Output>,
}

#[derive(Clone, Copy, Resource)]
struct Configuration {
    mode: RunMode,
}

#[derive(Default, Resource)]
struct PendingRequests(Vec<PendingRequest>);

enum PendingRequest {
    Observe {
        sequence: u64,
        request: ObservationRequest,
    },
    Pointer {
        sequence: u64,
        command: PointerCommand,
    },
    Response(Response),
    Shutdown {
        sequence: u64,
    },
}

#[derive(Resource)]
struct ExpectedSequence(u64);

impl Default for ExpectedSequence {
    fn default() -> Self {
        Self(1)
    }
}

fn register_native_input_channels(app: &mut App) {
    // Winit, focus, UI widgets, and the picking backend still declare these low-level channels.
    // Registering empty channels lets the controlled composition omit InputPlugin without
    // admitting any native events into the session.
    app.add_message::<KeyboardInput>()
        .add_message::<KeyboardFocusLost>()
        .add_message::<MouseButtonInput>()
        .add_message::<MouseMotion>()
        .add_message::<MouseWheel>()
        .add_message::<TouchInput>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<Key>>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<AccumulatedMouseMotion>()
        .init_resource::<AccumulatedMouseScroll>()
        .init_resource::<Touches>();
}

fn emit_ready(transport: Res<Transport>, configuration: Res<Configuration>) {
    if let Err(error) = transport
        .output
        .ready(&protocol::Ready::new(configuration.mode))
    {
        eprintln!("automation-control ready failed: {error}");
    }
}

/// Reads at most one line per update. This keeps request processing ordered and prevents a burst
/// of pointer transitions from becoming one synthetic click in one Bevy update.
fn receive_request(world: &mut World) {
    if !world.resource::<PendingRequests>().0.is_empty() {
        return;
    }
    let input = world.resource::<Transport>().input.try_recv();
    match input {
        Ok(Input::Line(line)) => match protocol::decode_request(&line) {
            Ok(request) => {
                let sequence = request.sequence;
                if let Err(response) = accept_sequence(world, sequence) {
                    write_response(world, response);
                    return;
                }
                let pending = match request.command {
                    Command::Observe(request) => PendingRequest::Observe { sequence, request },
                    Command::Pointer(command) => PendingRequest::Pointer { sequence, command },
                    Command::Shutdown => PendingRequest::Shutdown { sequence },
                };
                world.resource_mut::<PendingRequests>().0.push(pending);
            }
            Err(response) => {
                if response.sequence > 0
                    && let Err(response) = accept_sequence(world, response.sequence)
                {
                    write_response(world, response);
                    return;
                }
                write_response(world, response);
            }
        },
        Ok(Input::Eof) => {
            world.write_message(AppExit::Success);
        }
        Ok(Input::Error(message)) => {
            write_response(world, Response::error(0, "input_error", message));
            world.write_message(AppExit::error());
        }
        Err(std::sync::mpsc::TryRecvError::Empty | std::sync::mpsc::TryRecvError::Disconnected) => {
        }
    }
}

fn accept_sequence(world: &mut World, sequence: u64) -> Result<(), Response> {
    let expected = world.resource::<ExpectedSequence>().0;
    if sequence != expected {
        return Err(Response::error(
            sequence,
            "unexpected_sequence",
            format!("expected sequence {expected}, got {sequence}"),
        ));
    }
    world.resource_mut::<ExpectedSequence>().0 = expected.saturating_add(1);
    Ok(())
}

fn dispatch_request(world: &mut World) {
    let Some(pending) = world.resource_mut::<PendingRequests>().0.pop() else {
        return;
    };
    match pending {
        PendingRequest::Observe { sequence, request } => {
            world
                .resource_mut::<PendingRequests>()
                .0
                .push(PendingRequest::Observe { sequence, request });
        }
        PendingRequest::Pointer { sequence, command } => {
            let result = {
                let mut state = world.remove_resource::<PointerState>().unwrap_or_default();
                let result = pointer::pointer_event(&mut state, world, &command);
                world.insert_resource(state);
                result
            };
            match result {
                Ok(event) => {
                    world.write_message(event);
                    world
                        .resource_mut::<PendingRequests>()
                        .0
                        .push(PendingRequest::Pointer { sequence, command });
                }
                Err(error) => {
                    world
                        .resource_mut::<PendingRequests>()
                        .0
                        .push(PendingRequest::Response(Response::error(
                            sequence,
                            "pointer_failed",
                            error.to_string(),
                        )))
                }
            }
        }
        PendingRequest::Response(response) => {
            world
                .resource_mut::<PendingRequests>()
                .0
                .push(PendingRequest::Response(response));
        }
        PendingRequest::Shutdown { sequence } => {
            world
                .resource_mut::<PendingRequests>()
                .0
                .push(PendingRequest::Shutdown { sequence });
        }
    }
}

fn complete_request(world: &mut World) {
    let Some(pending) = world.resource_mut::<PendingRequests>().0.pop() else {
        return;
    };
    match pending {
        PendingRequest::Observe { sequence, request } => {
            let response = match observation::observe_world(world, &request) {
                Ok(result) => Response::completed(sequence, result),
                Err(error) => Response::error(sequence, "observation_failed", error.to_string()),
            };
            write_response(world, response);
        }
        PendingRequest::Pointer { sequence, .. } => {
            write_response(
                world,
                Response::completed(
                    sequence,
                    json!({"pointer": world.resource::<PointerState>().observation()}),
                ),
            );
        }
        PendingRequest::Response(response) => write_response(world, response),
        PendingRequest::Shutdown { sequence } => {
            write_response(world, Response::completed(sequence, json!({})));
            world.write_message(AppExit::Success);
        }
    }
}

fn write_response(world: &World, response: Response) {
    if let Err(error) = world.resource::<Transport>().output.response(&response) {
        eprintln!("automation-control response failed: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::Input;
    use bevy::{
        input::ButtonState,
        picking::{
            DefaultPickingPlugins, Pickable, PickingSystems,
            backend::{HitData, PointerHits},
            pointer::{PointerId, PointerLocation},
        },
        window::{CursorMoved, PrimaryWindow, Window, WindowEvent},
    };
    use std::sync::{Mutex, mpsc};

    #[derive(Default)]
    struct MemoryOutput {
        ready: Mutex<Vec<protocol::Ready>>,
        responses: Mutex<Vec<Response>>,
    }

    impl Output for MemoryOutput {
        fn ready(&self, value: &protocol::Ready) -> std::io::Result<()> {
            self.ready.lock().unwrap().push(value.clone());
            Ok(())
        }

        fn response(&self, value: &Response) -> std::io::Result<()> {
            self.responses.lock().unwrap().push(value.clone());
            Ok(())
        }
    }

    fn controlled_app() -> (App, mpsc::SyncSender<Input>, Arc<MemoryOutput>, Entity) {
        let (sender, receiver) = mpsc::sync_channel(8);
        let receiver = Mutex::new(Some(receiver));
        let output = Arc::new(MemoryOutput::default());
        let output_trait: Arc<dyn Output> = output.clone();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_message::<WindowEvent>()
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(AutomationControlPlugin::with_io(
                move || JsonLinesInput::from_receiver(receiver.lock().unwrap().take().unwrap()),
                output_trait,
            ));
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();
        (app, sender, output, window)
    }

    #[test]
    fn plugin_uses_no_caller_capability_list_and_emits_v2_ready() {
        let (mut app, sender, output, _window) = controlled_app();
        app.update();
        assert_eq!(output.ready.lock().unwrap()[0].version, 2);
        assert_eq!(output.ready.lock().unwrap()[0].controls, ["pointer"]);
        sender
            .send(Input::Line(
                r#"{"sequence":1,"command":{"type":"shutdown"}}"#.into(),
            ))
            .unwrap();
        app.update();
        assert_eq!(output.responses.lock().unwrap()[0].sequence, 1);
    }

    #[test]
    fn out_of_order_sequences_are_rejected_without_hanging_the_session() {
        let (mut app, sender, output, _window) = controlled_app();
        app.update();
        sender
            .send(Input::Line(
                r#"{"sequence":2,"command":{"type":"shutdown"}}"#.into(),
            ))
            .unwrap();
        app.update();
        let responses = output.responses.lock().unwrap();
        assert_eq!(responses[0].sequence, 2);
        assert_eq!(
            responses[0].error.as_ref().map(|error| error.code.as_str()),
            Some("unexpected_sequence")
        );
        assert!(app.should_exit().is_none());
    }

    #[test]
    fn a_typed_command_error_consumes_its_sequence() {
        let (mut app, sender, output, _window) = controlled_app();
        app.update();
        sender
            .send(Input::Line(
                r#"{"sequence":1,"command":{"type":"observe","selector":{"type":"targets"},"projection":{"type":"summary"},"limit":0}}"#.into(),
            ))
            .unwrap();
        app.update();
        sender
            .send(Input::Line(
                r#"{"sequence":2,"command":{"type":"shutdown"}}"#.into(),
            ))
            .unwrap();
        app.update();
        let responses = output.responses.lock().unwrap();
        assert_eq!(responses[0].sequence, 1);
        assert_eq!(
            responses[0].error.as_ref().map(|error| error.code.as_str()),
            Some("invalid_arguments")
        );
        assert_eq!(responses[1].sequence, 2);
        assert_eq!(responses[1].status, protocol::ResponseStatus::Completed);
    }

    #[test]
    fn native_window_pointer_input_is_disabled_but_virtual_move_updates_the_pointer() {
        let (mut app, sender, output, window) = controlled_app();
        app.init_resource::<PressCount>();
        let camera = app.world_mut().spawn_empty().id();
        let target = app
            .world_mut()
            .spawn(Pickable::default())
            .observe(|_: On<Pointer<Press>>, mut count: ResMut<PressCount>| {
                count.0 += 1;
            })
            .id();
        app.insert_resource(FakeHit { camera, target });
        app.add_systems(PreUpdate, write_fake_hits.in_set(PickingSystems::Backend));
        app.update();
        app.world_mut()
            .write_message(WindowEvent::CursorMoved(CursorMoved {
                window,
                position: Vec2::new(400.0, 300.0),
                delta: None,
            }));
        app.world_mut()
            .write_message(WindowEvent::MouseButtonInput(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                window,
            }));
        app.update();
        let native_location = app
            .world_mut()
            .query::<&PointerLocation>()
            .single(app.world())
            .unwrap();
        assert!(native_location.location().is_none());
        assert_eq!(app.world().resource::<PressCount>().0, 0);

        sender
            .send(Input::Line(
                r#"{"sequence":1,"command":{"type":"pointer","action":{"type":"move","surface":null,"position":[20.0,30.0]}}}"#.into(),
            ))
            .unwrap();
        app.update();
        let virtual_location = app
            .world_mut()
            .query::<&PointerLocation>()
            .single(app.world())
            .unwrap();
        assert_eq!(
            virtual_location
                .location()
                .map(|location| location.position),
            Some(Vec2::new(20.0, 30.0))
        );
        assert_eq!(output.responses.lock().unwrap()[0].sequence, 1);
    }

    #[derive(Resource)]
    struct FakeHit {
        camera: Entity,
        target: Entity,
    }

    #[derive(Resource, Default)]
    struct PressCount(u32);

    fn write_fake_hits(
        hit: Res<FakeHit>,
        pointers: Query<(&PointerId, &PointerLocation)>,
        mut hits: MessageWriter<PointerHits>,
    ) {
        for (pointer, location) in &pointers {
            if location.location().is_some() {
                hits.write(PointerHits::new(
                    *pointer,
                    vec![(hit.target, HitData::new(hit.camera, 0.0, None, None))],
                    0.0,
                ));
            }
        }
    }

    #[test]
    fn virtual_move_press_and_release_traverse_picking_and_trigger_an_observer() {
        let (mut app, sender, output, _window) = controlled_app();
        app.init_resource::<PressCount>();
        let camera = app.world_mut().spawn_empty().id();
        let target = app
            .world_mut()
            .spawn(Pickable::default())
            .observe(|_: On<Pointer<Press>>, mut count: ResMut<PressCount>| {
                count.0 += 1;
            })
            .id();
        app.insert_resource(FakeHit { camera, target });
        app.add_systems(PreUpdate, write_fake_hits.in_set(PickingSystems::Backend));
        app.update();

        for (sequence, action) in [
            (
                1,
                r#"{"type":"move","surface":null,"position":[20.0,30.0]}"#,
            ),
            (2, r#"{"type":"press","button":"primary"}"#),
            (3, r#"{"type":"release","button":"primary"}"#),
        ] {
            sender
                .send(Input::Line(format!(
                    r#"{{"sequence":{sequence},"command":{{"type":"pointer","action":{action}}}}}"#
                )))
                .unwrap();
            app.update();
        }

        assert_eq!(app.world().resource::<PressCount>().0, 1);
        assert_eq!(
            output
                .responses
                .lock()
                .unwrap()
                .iter()
                .map(|response| response.sequence)
                .collect::<Vec<_>>(),
            [1, 2, 3]
        );
    }
}
