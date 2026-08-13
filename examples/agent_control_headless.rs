//! Logical headless agent demonstration for issue #36. No window, renderer, GPU or display server.

use agent_control::{
    AgentControlPlugin, AgentRequest, AgentRequests, AgentTarget, Command, Response, RunMode,
    RunState, complete_request,
};
use bevy::prelude::*;
use serde_json::json;

#[derive(Resource, Default)]
struct ClickCount(u32);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args.iter().any(|value| value == "--agent") {
        eprintln!("logical demonstration requires --agent");
        std::process::exit(2);
    }
    let seed = argument(&args, "--seed")
        .and_then(|value| value.parse().ok())
        .unwrap_or(42);

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(
            AgentControlPlugin::stdio([
                "inspect_ui",
                "inspect_scene",
                "click",
                "pause",
                "resume",
                "step_frames",
                "step_simulation",
                "wait_until",
                "inspect_run",
                "shutdown",
            ])
            .configured(RunMode::Logical, seed, 50),
        )
        .init_resource::<ClickCount>()
        .add_systems(Startup, setup)
        .add_systems(Update, logical_adapter)
        .run();
}

fn argument<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|value| value == key)
        .and_then(|index| args.get(index + 1))
        .map(String::as_str)
}

fn setup(mut commands: Commands, mut state: ResMut<RunState>) {
    state.active_screen = Some("prototype".into());
    commands.spawn(AgentTarget::new(
        "toolbar.generate",
        "ui",
        "Generate",
        ["click"],
    ));
    commands.spawn(AgentTarget::new(
        "scene.prototype_star",
        "scene",
        "Prototype star",
        [] as [&str; 0],
    ));
}

fn logical_adapter(world: &mut World) {
    let requests: Vec<AgentRequest> = world.resource_mut::<AgentRequests>().drain().collect();
    for AgentRequest(request) in requests {
        match request.command {
            Command::Click { target } if target == "toolbar.generate" => {
                world.resource_mut::<ClickCount>().0 += 1;
                world.resource_mut::<RunState>().selection = Some("scene.prototype_star".into());
                let count = world.resource::<ClickCount>().0;
                complete_request(
                    world,
                    Response::completed(
                        request.id,
                        json!({"target": target, "click_count": count}),
                    ),
                );
            }
            Command::Screenshot { .. }
            | Command::CameraFocus { .. }
            | Command::CameraOrbit { .. }
            | Command::CameraPan { .. }
            | Command::CameraZoom { .. } => {
                complete_request(
                    world,
                    Response::error(
                        Some(request.id),
                        "unsupported_in_logical_mode",
                        "rendering capability is unavailable in logical mode",
                    ),
                );
            }
            _ => world
                .resource_mut::<AgentRequests>()
                .defer(AgentRequest(request)),
        }
    }
}
