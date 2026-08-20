# `star_sim_debug`

`star_sim_debug` is the Star Sim Debug Host. It starts one isolated Controlled Session and opens a line-oriented REPL. The host owns the child process, protocol transport, request ordering, and shutdown. Controllers never supply Cargo features, child arguments, protocol versions, IDs, or sequences.

The normal `app` binary remains a Player Run with Native Input and no `automation_control` dependency.

## Start a session

Rendered Mode is the default:

```bash
cargo run -p star_sim_debug
cargo run -p star_sim_debug -- --mode rendered
```

Logical Mode has no OS window, renderer, display-server requirement, or screenshot support:

```bash
cargo run -p star_sim_debug -- --mode logical
```

Both modes use a fixed 640 by 360 session surface. The host keeps the session alive until `quit`, EOF, Ctrl-C, a child failure, or another terminal error. Child logs go to stderr. Human status and observations go to the host's stdout; raw JSONL protocol messages do not.

Start Session Recording with a path relative to the Session artifact root:

```bash
cargo run -p star_sim_debug -- --mode logical --record recordings/logical.jsonl
```

Absolute paths, traversal, symbolic-link escapes, and existing files are rejected.

## REPL commands

```text
click menu.tab.museum
pointer move 0.5 0.3
pointer press left
pointer release left
pointer click left
scroll 0 -1
key Escape press
key Escape release
text hello museum visitors
observe targets
observe ui
observe pointers
observe input
observe clock
pause
resume
step 3
record start recordings/manual.jsonl
record stop
record start
status
help
quit
```

`pointer move X Y` uses normalized coordinates in the half-open range `[0, 1)`. Pointer buttons are `left`, `right`, and `middle`. Key names are case-insensitive and use names such as `A`, `Escape`, and `ArrowLeft`. `text` sends the rest of the line as one text commit.

`click menu.tab.museum` is a Star Sim host macro. The host observes that known menu target, moves to its current bounds, presses and releases the primary pointer button, and waits for the reflected active screen. It does not add a Click command to the child protocol or a generic target registry.

Every Virtual Input transition includes one controlled settle frame so Bevy consumes it. There is no background clock. `pause` prevents bounded wait loops from advancing extra frames; input settle frames and explicit `step FRAMES` still run. `resume` lets bounded waits advance again. A paused unmet wait returns an error instead of changing time.

`record start [PATH]` starts a new JSONL segment while the Controlled Session remains open. Without a
path, the host creates a collision-free file under `recordings/` in the artifact root. `record stop`
flushes the segment. Recording can be started and stopped repeatedly; host sequences continue while
recording is off, and every new segment begins with the current session context. Quit and EOF write a
completed end event. Fatal errors and child aborts write an aborted end event when recording is active.

After startup and each successful operation, the REPL prints:

```text
instance=alpha mode=logical screen=gym paused=false
last action: none
```

## Failure reports

Session diagnostics default to `artifacts/star-sim-debug`. Select another root with `--artifact-dir PATH`. Draft an existing failure report without contacting GitHub:

```bash
cargo run -p star_sim_debug -- report artifacts/star-sim-debug
```

Publishing remains explicit:

```bash
cargo run -p star_sim_debug -- report artifacts/star-sim-debug --create
```

## Tests

```bash
cargo test -p app
cargo test -p app --features automation-control --bin app
cargo test -p automation_control --features driver
cargo test -p star_sim_debug
```

The `logical_repl` integration test launches the real app with display variables removed, clicks the Museum tab, and checks both `quit` and EOF shutdown. Rendered Mode needs a display and render adapter.
