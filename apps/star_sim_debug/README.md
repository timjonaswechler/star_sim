# `star_sim_debug`

`star_sim_debug` is a development and debugging driver for the repository's automation-controlled Bevy examples. It runs deterministic checks against a fixed example; it is not a general-purpose Bevy debugger or an interactive application controller.

## When to use it

Use the CLI when a task needs one of these checks:

- verify logical Bevy behavior without a window or GPU;
- reproduce deterministic stepping, selection, and run-state behavior;
- validate rendered output through screenshots.

Choose `logical` for state and behavior checks. Choose `visual` when the rendered image or camera output matters.

## Logical mode

Logical mode runs the headless `automation_control` example. It does not require a display, window, renderer, or GPU.

```bash
cargo run -q -p star_sim_debug -- logical
```

The command pauses the run, advances frames and simulation time, clicks the generate target, waits for the expected selection, inspects the run state, and shuts the child down. A successful run prints one JSON object with `status: "passed"` and `mode: "logical"`.

Add `--record` to save the complete protocol session as ordered JSON Lines:

```bash
cargo run -q -p star_sim_debug -- logical \
  --record artifacts/debug-run/logical-session.jsonl
```

If the first run times out while Cargo is compiling the child example, build it once before running the CLI:

```bash
cargo build -p star_sim_debug
cargo build -p automation_control --example automation_control_headless
cargo run -q -p star_sim_debug -- logical
```

## Visual mode

Visual mode runs the rendered `bevy_viewer` prototype, focuses the main camera on the prototype star, captures a window screenshot and a camera screenshot, and validates both PNG files.

```bash
cargo run -q -p star_sim_debug -- visual \
  --artifact-dir artifacts/debug-run
```

The default artifact directory is `artifacts/debug-ci`. Pass `--artifact-dir` to keep debugging output separate. The current prototype produces:

- `window.png` at `640x360`;
- `camera.png` at `320x180`.

Visual mode requires a usable display and graphics backend. CI runs it with Xvfb and software Vulkan rendering. If the first run times out while Cargo is compiling the example, build it once first:

```bash
cargo build -p star_sim_debug
cargo build -p app --example automation_control_prototype --features render-example
cargo run -q -p star_sim_debug -- visual \
  --artifact-dir artifacts/debug-run
```

Use a fresh artifact directory when repeating a run because screenshots are not overwritten by default.

Visual sessions can be recorded alongside their screenshots:

```bash
cargo run -q -p star_sim_debug -- visual \
  --artifact-dir artifacts/debug-run \
  --record artifacts/debug-run/visual-session.jsonl
```

## Session recording

Each recording line contains an increasing `sequence`, a `direction` (`to_app` or `from_app`), and the exact JSON protocol `message`. The first entry is the application's `ready` message, followed by every request and response in transmission order. Recordings deliberately use sequence numbers rather than wall-clock timestamps so deterministic runs remain directly comparable.

Parent directories are created automatically. An existing recording at the selected path is replaced when a new session starts.

## Recent failure log

The CLI continues to print child-process stderr, including Bevy logs and panic messages, directly to the terminal. At the same time it retains a rolling window of the latest 50 stderr lines. If a logical or visual scenario fails, those lines are written to `recent.log` in the artifact directory:

```bash
cargo run -q -p star_sim_debug -- logical \
  --artifact-dir artifacts/debug-run
```

A successful run does not create `recent.log`. Runtime context such as the seed and protocol activity remains in the optional session recording instead of being duplicated in application error messages.

Panic headlines and Bevy `ERROR` messages are retained separately from the 50-line window, so a long backtrace cannot displace the message that identifies the failure. A detected failure also creates `failure.json` with the application headline and, when present, the CLI error.

## GitHub issue reports

Create a reviewable Markdown draft from `failure.json`, `recent.log`, and the last 12 session entries:

```bash
cargo run -q -p star_sim_debug -- report artifacts/debug-run
```

This writes `artifacts/debug-run/github-issue.md` without contacting GitHub. To publish explicitly:

```bash
cargo run -q -p star_sim_debug -- report artifacts/debug-run --create
```

Publishing uses the authenticated `gh` CLI. It first compares the generated title against existing open and closed issues; an exact match is returned instead of creating a duplicate.

## Tests

```bash
cargo test -p automation_control -p star_sim_debug
```

The CLI scenarios live in [`src/main.rs`](src/main.rs). The protocol, capabilities, targets, and Bevy adapters live in [`crates/automation_control`](../../crates/automation_control) and the corresponding examples. Extend those pieces when the fixed scenarios need new behavior.
