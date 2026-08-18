# `automation_control`

`automation_control` is the reusable Bevy integration and controller-side driver for deterministic automation.

It defines a versioned JSON Lines protocol and the Bevy plugin that consumes it. The crate does **not** know an application's business actions: an application registers semantic targets, translates application-owned requests, and completes them through the public request queue.

## Features

- default: Bevy plugin, protocol, target registry, observations, run state, and transport;
- `render`: screenshot artifact writing;
- `render-example`: rendering dependencies for the `automation_control_prototype` example;
- `driver`: optional process launcher, TOML configuration, JSONL session client, diagnostics, recording, and PNG validation.

The `driver` feature is intended for tools such as `star_sim_debug`. A normal Bevy application only needs the default crate features and, when necessary, `render`.

## Embed the plugin

A rendered application can opt into automation at runtime:

```rust
use automation_control::AutomationControlPlugin;

app.add_plugins(AutomationControlPlugin::default());
```

For a logical/headless application, configure the plugin explicitly:

```rust
use automation_control::{AutomationControlPlugin, RunMode};

app.add_plugins(
    AutomationControlPlugin::stdio(["click", "inspect_run", "shutdown"])
        .configured(RunMode::Logical, 42, 50),
);
```

The plugin emits a `ready` message, polls stdin without blocking the Bevy thread, and exposes decoded requests through `AutomationRequests`.

## Application adapter seam

The crate owns protocol-level commands such as `click`, camera operations, stepping, waits, and screenshots. The application owns what those commands mean for its state.

Register stable semantic targets instead of exposing Bevy entity IDs:

```rust
commands.spawn(AutomationTarget::new(
    "toolbar.generate",
    "button",
    "Generate",
    ["click"],
));
```

Application-owned requests are drained and completed through the public seam:

```rust
fn automation_adapter(world: &mut World) {
    let requests: Vec<_> = world
        .resource_mut::<AutomationRequests>()
        .drain()
        .collect();

    for AutomationRequest(request) in requests {
        // Apply the normal application action, then:
        // complete_request(world, Response::completed(request.id, result));
        // or defer the request back to AutomationRequests.
    }
}
```

`complete_request` guarantees that a request ID receives at most one final response. The target registry reports unknown and duplicate semantic IDs as protocol errors.

## Logical and rendered modes

Logical mode uses `MinimalPlugins` and deterministic manual stepping. It does not provide screenshots or a render device. Rendered mode uses the application's normal renderer and can provide camera/window screenshots when the application supplies the required adapters.

Capabilities advertised in the `ready` message must describe what the host actually supports. Camera and screenshot commands remain pending for the application adapter; the plugin rejects them when their capability is absent.

## Controller-side driver

Enable the optional feature:

```toml
automation_control = { path = "../../crates/automation_control", features = ["driver"] }
```

The driver exposes a deliberately small interface:

```rust
use automation_control::driver::{Session, SessionOptions};
use automation_control::Command;
use std::time::Duration;

let mut session = Session::spawn(&launch_spec, SessionOptions::new(Duration::from_secs(60)))?;
session.ready(&["inspect_run"])?;
session.request("state", Command::InspectRun)?;
session.shutdown()?;
```

`Session` hides child-process lifecycle, JSON serialization, response matching, timeouts, stderr streaming, and optional ordered JSONL recording. `SessionOptions::from_config` combines a reusable timeout policy with consumer-selected recording, logging, and artifact paths; `SessionOptions::with_artifact_dir` supplies the selected artifact root to the child through the fixed `AUTOMATION_CONTROL_ARTIFACT_DIR` environment variable. `RecentLogs` retains a bounded diagnostic window and extracts panic/Bevy error headlines. On failure, `RecentLogs::persist_failure_artifacts` writes the rolling log and a validated, versioned `FailureReport`, returning both artifact paths. `validate_png` checks screenshot signatures and dimensions without requiring Bevy rendering.

`driver::CommandLine::parse` provides typed `Run` and `Report` options for controller tools. It accepts a consumer-owned opaque scenario name, optional `--config`, `--artifact-dir`, and `--record` overrides, and the `report ARTIFACT_DIR [--create]` form without making any tool's scenario names part of this crate.

`driver::IssueDraft::from_artifacts` loads and validates `failure.json`, includes optional `recent.log` and the last twelve entries from the selected recording, and produces a reusable Markdown draft. `IssueDraft::write_to` writes `github-issue.md` beneath the artifact directory.

`driver::github::Report::prepare` applies the optional configured attribution and writes the draft. `Report::publish` uses the authenticated `gh` CLI, returns an exact-title match instead of creating a duplicate, and exposes a typed, serializable outcome. GitHub is the currently supported publisher; additional publishers can be added later without moving this generic behavior back into a consuming application.

## Configuration

`driver::Config::load` reads a versioned TOML file. It contains one application launch specification, minimal session policy, and optional report attribution. Launch targets can be Cargo binaries or examples:

```toml
version = 1

[application]
package = "app"
kind = "binary"
target = "app"
features = ["automation-control"]
arguments = ["--automation"]

[session]
timeout_seconds = 60

[report]
generated_by = "star_sim_debug report"
```

Relative paths in launch arguments retain their normal process semantics. Configuration validation rejects unsupported versions, missing application package or target names, invalid timeouts, and empty configured attribution. Scenario targets, capabilities, screenshot paths, and image dimensions belong to the runtime scenario or session rather than launch configuration.

## Examples

Run the logical demonstration without a window or GPU:

```bash
cargo run -p automation_control --example automation_control_headless -- --automation
```

Run the rendered prototype when a display and graphics backend are available. The standalone example uses a fallback directory; set the standard environment variable to select another root:

```bash
AUTOMATION_CONTROL_ARTIFACT_DIR=artifacts/demo \
cargo run -p automation_control --example automation_control_prototype \
  --features render-example -- --automation
```

The examples are demonstrations, not application-independent behavior. A consuming application should provide its own target registration and action adapter. Rendered applications can resolve the controller-selected root with `automation_control::artifact_root_path("artifacts/standalone")` and then construct `ArtifactRoot`; screenshot paths remain relative protocol values and stay confined beneath that root.

## Testing

```bash
cargo test -p automation_control
cargo test -p automation_control --features driver
```

The `driver_recording` integration test launches the existing headless example through the driver and verifies ordered JSONL recording. The `issue_report` integration test exercises typed failure metadata and Markdown draft generation without launching an application. The `github_report` integration test verifies configured attribution and the typed draft outcome without contacting GitHub. Logical tests should remain free of window and native display dependencies. Render smoke tests should validate artifact format and dimensions rather than assuming byte-identical pixels across platforms.
