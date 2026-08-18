# `star_sim_debug`

`star_sim_debug` is the repository-specific command-line driver for deterministic logical and visual checks. The reusable automation protocol, Bevy plugin, process session, diagnostics, and artifact primitives live in [`crates/automation_control`](../../crates/automation_control/README.md).

This app owns only scenario orchestration, its default configuration location, and executable dispatch. Reusable failure-artifact loading, Markdown issue-draft generation, and authenticated GitHub CLI publishing live in [`automation_control::driver`](../../crates/automation_control/README.md).

## Configuration

The default configuration is [`debug.toml`](config/automation/debug.toml). It is loaded when the CLI starts, not at compile time. It describes the application launch specification, session policy, and report attribution:

- the Cargo package, target kind, target name, features, and arguments to launch;
- the controller session timeout;
- the optional name included in generated issue-report footers.

Scenario targets, protocol capabilities, screenshot paths, and expected dimensions are selected by the scenario at runtime rather than by the launch configuration.

Select another file with `--config`:

```bash
cargo run -q -p star_sim_debug -- \
  --config path/to/debug.toml logical
```

`--artifact-dir` and `--record` are command-line overrides. Relative artifact and recording paths are interpreted by the current working directory. The driver uses `--artifact-dir` as its artifact root and supplies the selected root to the child through `AUTOMATION_CONTROL_ARTIFACT_DIR`; it does not append an application-specific artifact argument.

The checked-in configuration targets the `apps/app` binary with its `automation-control` feature. The app registers the stable targets `window.primary`, `camera.main`, and `menu.tab.gym`, `menu.tab.museum`, and `menu.tab.zoo` when launched with `--automation`. The test configuration in [`config/automation/test-rendered.toml`](config/automation/test-rendered.toml) is an equivalent explicit app configuration; reusable headless coverage remains in `automation_control` tests.

## Logical mode

The logical scenario validates the state-oriented capabilities it uses against the checked-in `apps/app` target. It requires a usable display because the real app uses Bevy's rendered window pipeline:

```bash
cargo run -q -p star_sim_debug -- \
  --config apps/star_sim_debug/config/automation/debug.toml logical
```

The app scenario inspects the real menu, activates the Museum tab through the same UI activation seam used by a human click, waits for `active_screen: "museum"`, inspects run state, and shuts down the child. A successful run prints one JSON object with `status: "passed"` and `mode: "logical"`. Reusable headless driver coverage lives in `automation_control` integration tests and does not depend on this app-specific scenario.

Record the ordered protocol session when diagnosing the app run:

```bash
cargo run -q -p star_sim_debug -- \
  --config apps/star_sim_debug/config/automation/debug.toml logical \
  --record artifacts/debug-run/logical-session.jsonl
```

## Visual mode

Visual mode launches the selected rendered target, captures the primary window beneath the selected artifact root, and validates its PNG dimensions:

```bash
cargo run -q -p star_sim_debug -- \
  --config apps/star_sim_debug/config/automation/debug.toml visual \
  --artifact-dir artifacts/debug-run
```

It requires a usable display and graphics backend. CI runs it with Xvfb and software Vulkan rendering. Build the target independently when the first protocol timeout is caused by Cargo compilation:

```bash
cargo build -p star_sim_debug
cargo build -p app --features automation-control
```

The checked-in app scenario expects `window.png` at `640x360`. A fresh artifact directory is recommended because screenshot reservation does not overwrite existing files. Camera operations are not advertised because the current app has no meaningful camera-control adapter.

## Failure artifacts

On a failed logical or visual run, the CLI continues to mirror child stderr and writes:

- `recent.log`: the configured rolling stderr window;
- `failure.json`: the detected panic/Bevy error or CLI failure, including the recording path when one was selected.

Create a reviewable Markdown issue draft without contacting GitHub:

```bash
cargo run -q -p star_sim_debug -- report artifacts/debug-run
```

Publish explicitly through the authenticated `gh` CLI:

```bash
cargo run -q -p star_sim_debug -- report artifacts/debug-run --create
```

The report command uses the reusable GitHub report module, reads its attribution from `[report].generated_by`, and avoids creating an exact-title duplicate issue.

## Tests

```bash
cargo test -p automation_control --features driver
cargo test -p star_sim_debug
# Requires a display and runs the checked-in apps/app target:
xvfb-run -a cargo test -p star_sim_debug --test app_integration -- --ignored
```

The reusable crate integration tests cover ordered session recordings against the headless example and typed issue drafts. The ignored `app_integration` tests exercise the checked-in `apps/app` target through the `star_sim_debug` CLI and public protocol seam. The crate tests also cover the driver, protocol, target registry, deterministic stepping, waits, and artifact path safety.
