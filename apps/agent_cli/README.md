# `star_sim_agent`

`star_sim_agent` is a development and debugging driver for the repository's agent-controlled Bevy examples. It runs deterministic checks against a fixed example; it is not a general-purpose Bevy debugger or an interactive application controller.

## When to use it

Use the CLI when a task needs one of these checks:

- verify logical Bevy behavior without a window or GPU;
- reproduce deterministic stepping, selection, and run-state behavior;
- validate rendered output through screenshots.

Choose `logical` for state and behavior checks. Choose `visual` when the rendered image or camera output matters.

## Logical mode

Logical mode runs the headless `agent_control` example. It does not require a display, window, renderer, or GPU.

```bash
cargo run -q -p star_sim_agent -- logical
```

The command pauses the run, advances frames and simulation time, clicks the generate target, waits for the expected selection, inspects the run state, and shuts the child down. A successful run prints one JSON object with `status: "passed"` and `mode: "logical"`.

If the first run times out while Cargo is compiling the child example, build it once before running the CLI:

```bash
cargo build -p star_sim_agent
cargo build -p agent_control --example agent_control_headless
cargo run -q -p star_sim_agent -- logical
```

## Visual mode

Visual mode runs the rendered `bevy_viewer` prototype, focuses the main camera on the prototype star, captures a window screenshot and a camera screenshot, and validates both PNG files.

```bash
cargo run -q -p star_sim_agent -- visual \
  --artifact-dir artifacts/agent-debug
```

The default artifact directory is `artifacts/agent-ci`. Pass `--artifact-dir` to keep debugging output separate. The current prototype produces:

- `window.png` at `640x360`;
- `camera.png` at `320x180`.

Visual mode requires a usable display and graphics backend. CI runs it with Xvfb and software Vulkan rendering. If the first run times out while Cargo is compiling the example, build it once first:

```bash
cargo build -p star_sim_agent
cargo build -p bevy_viewer --example agent_control_prototype --features agent-control
cargo run -q -p star_sim_agent -- visual \
  --artifact-dir artifacts/agent-debug
```

Use a fresh artifact directory when repeating a run because screenshots are not overwritten by default.

## Tests

```bash
cargo test -p agent_control -p star_sim_agent
```

The CLI scenarios live in [`src/main.rs`](src/main.rs). The protocol, capabilities, targets, and Bevy adapters live in [`crates/agent_control`](../../crates/agent_control) and the corresponding examples. Extend those pieces when the fixed scenarios need new behavior.
