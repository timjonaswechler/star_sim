# `star_sim_debug`

`star_sim_debug` is the Star Sim Debug Host. During issue #50 it provides a small compatibility
probe for protocol v2 and retains the existing failure-report command. The public REPL, real
Star-Sim composition, scripts, replay, recording orchestration, and model adapters belong to the
follow-up work tracked under #49.

## Controlled-session probe

The checked-in configuration launches the `context_menu` binary from `bevy_test_apps` with its
`automation` feature. The feature selects the Controlled Session composition internally; there is no
public `--automation` argument.

```bash
cargo run -q -p star_sim_debug -- rendered
```

The probe waits for `ready`, requests a target summary through the driver-owned sequence, prints one
JSON result, and shuts the child down. It requires a display and render adapter. The dedicated
controller smoke test exercises the complete pointer path:

```bash
cargo run -p automation_control --example bevy_controller --features driver
```

Logical Mode and the long-lived REPL are not implemented by issue #50.

## Failure reports

A failed probe mirrors child diagnostics to stderr and writes the existing failure artifacts beneath
`--artifact-dir`. A report can be prepared without contacting GitHub:

```bash
cargo run -q -p star_sim_debug -- report artifacts/debug-ci
```

Publishing remains explicit:

```bash
cargo run -q -p star_sim_debug -- report artifacts/debug-ci --create
```

## Tests

```bash
cargo test -p automation_control
cargo test -p automation_control --features driver
cargo test -p star_sim_debug
```

The normal `app` package is a Player Run. It has Native Input and no dependency on
`automation_control`.
