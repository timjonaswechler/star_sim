use crate::menu::{MenuSection, MenuTab};
use automation_control::{
    ArtifactRoot, AutomationControlPlugin, AutomationRequest, AutomationRequests, AutomationTarget,
    Command, RegistryLookupError, Response, RunState, ScreenshotSource, TargetRegistry,
    artifact_root_path, complete_request,
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured},
    ui_widgets::Activate,
    window::{PrimaryWindow, WindowResolution},
};
use serde_json::json;

const ARTIFACT_ROOT: &str = "artifacts/app-automation";
const WINDOW_TARGET: &str = "window.primary";
const CAMERA_TARGET: &str = "camera.main";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;

const CAPABILITIES: &[&str] = &[
    "inspect_ui",
    "inspect_scene",
    "inspect_selection",
    "inspect_camera",
    "click",
    "screenshot",
    "pause",
    "resume",
    "step_frames",
    "step_simulation",
    "wait_until",
    "inspect_run",
    "shutdown",
];

#[derive(Resource)]
struct AutomationArtifacts(ArtifactRoot);

pub(crate) struct AutomationPlugin;

impl Plugin for AutomationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutomationControlPlugin::stdio(CAPABILITIES.iter().copied()))
            .insert_resource(AutomationArtifacts(
                ArtifactRoot::new(artifact_root_path(ARTIFACT_ROOT))
                    .expect("automation artifact root should be available"),
            ))
            .add_systems(Startup, configure_window)
            .add_systems(PostStartup, register_targets)
            .add_systems(Update, (automation_adapter, sync_active_screen).chain());
    }
}

fn configure_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };
    window.resolution =
        WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT).with_scale_factor_override(1.0);
}

fn register_targets(
    mut commands: Commands,
    windows: Query<Entity, (With<PrimaryWindow>, Without<AutomationTarget>)>,
    cameras: Query<Entity, (With<Camera2d>, Without<AutomationTarget>)>,
    tabs: Query<(Entity, &MenuTab), Without<AutomationTarget>>,
) {
    for entity in &windows {
        commands.entity(entity).insert(AutomationTarget::new(
            WINDOW_TARGET,
            "window",
            "Primary window",
            ["screenshot"],
        ));
    }
    for entity in &cameras {
        commands.entity(entity).insert(AutomationTarget::new(
            CAMERA_TARGET,
            "camera",
            "Primary 2D camera",
            [] as [&str; 0],
        ));
    }
    for (entity, tab) in &tabs {
        commands.entity(entity).insert(AutomationTarget::new(
            tab.id,
            "button",
            tab.label,
            ["click"],
        ));
    }
}

fn automation_adapter(world: &mut World) {
    let requests: Vec<AutomationRequest> =
        world.resource_mut::<AutomationRequests>().drain().collect();
    for AutomationRequest(request) in requests {
        match request.command {
            Command::Click { target } => handle_click(world, request.id, target),
            Command::Screenshot {
                source,
                path,
                overwrite,
            } => handle_screenshot(world, request.id, source, path, overwrite),
            command => world
                .resource_mut::<AutomationRequests>()
                .defer(AutomationRequest(automation_control::Request {
                    version: request.version,
                    id: request.id,
                    command,
                })),
        }
    }
}

fn handle_click(world: &mut World, id: String, target: String) {
    let entity = match world.resource::<TargetRegistry>().entity(&target) {
        Ok(entity) => entity,
        Err(RegistryLookupError::Unknown(_)) => {
            complete_request(
                world,
                Response::error(
                    Some(id),
                    "unknown_target",
                    format!("unknown target: {target}"),
                ),
            );
            return;
        }
        Err(RegistryLookupError::Duplicate(_)) => {
            complete_request(
                world,
                Response::error(
                    Some(id),
                    "duplicate_target",
                    format!("duplicate target: {target}"),
                ),
            );
            return;
        }
    };
    let Some(tab) = world.get::<MenuTab>(entity).copied() else {
        complete_request(
            world,
            Response::error(
                Some(id),
                "unsupported_action",
                format!("{target} is not an application tab"),
            ),
        );
        return;
    };

    // Activate the real UI widget event. The application's TabsPlugin observer performs the
    // actual state transition used by human interaction; the adapter does not mutate TabsRoot.
    world.trigger(Activate { entity });
    let screen = menu_screen(tab.section);
    complete_request(
        world,
        Response::completed(id, json!({"target": target, "active_screen": screen})),
    );
}

fn handle_screenshot(
    world: &mut World,
    id: String,
    source: ScreenshotSource,
    path: Option<String>,
    overwrite: bool,
) {
    let screenshot = match source {
        ScreenshotSource::Window { target } if target == WINDOW_TARGET => {
            let Some(entity) = world
                .resource::<TargetRegistry>()
                .entity(WINDOW_TARGET)
                .ok()
            else {
                complete_request(world, Response::error(Some(id), "unknown_target", target));
                return;
            };
            Screenshot::window(entity)
        }
        ScreenshotSource::Window { target } => {
            complete_request(
                world,
                Response::error(
                    Some(id),
                    "unknown_target",
                    format!("unknown window target: {target}"),
                ),
            );
            return;
        }
        ScreenshotSource::Camera { target } => {
            complete_request(
                world,
                Response::error(
                    Some(id),
                    "unsupported_action",
                    format!("camera screenshots are not supported by the app: {target}"),
                ),
            );
            return;
        }
    };

    let destination_path = path.unwrap_or_else(|| format!("screenshots/{id}.png"));
    let destination = match world
        .resource::<AutomationArtifacts>()
        .0
        .reserve(destination_path, overwrite)
    {
        Ok(destination) => destination,
        Err(error) => {
            complete_request(
                world,
                Response::error(Some(id), "invalid_artifact_path", error.to_string()),
            );
            return;
        }
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

fn sync_active_screen(world: &mut World) {
    let screen = world
        .query::<&ui::components::tabs::TabsRoot<MenuSection>>()
        .iter(world)
        .next()
        .map(|root| menu_screen(root.active).to_owned());
    if let Some(screen) = screen {
        world.resource_mut::<RunState>().active_screen = Some(screen);
    }
}

fn menu_screen(section: MenuSection) -> &'static str {
    match section {
        MenuSection::Gym => "gym",
        MenuSection::Museum => "museum",
        MenuSection::Zoo => "zoo",
    }
}
