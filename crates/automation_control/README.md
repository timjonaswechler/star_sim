# `automation_control`

`automation_control` provides the protocol and Bevy integration used by isolated Controlled
Sessions. A Player Run does not depend on this crate. The crate does not know Star-Sim menu
semantics and never injects operating-system input.

## Protocol v2

The session emits one `ready` message. The host assigns request sequences beginning at `1`.
Controllers send only commands and do not provide protocol versions or request IDs.

```json
{"type":"ready","version":2,"mode":"rendered","controls":["pointer","keyboard","text","time","screenshot"],"observation_scopes":["targets","ui","pointers","entity","virtual_input","clock"]}
```

```json
{"sequence":1,"command":{"type":"observe","selector":{"type":"ui"},"projection":{"type":"summary"},"limit":64}}
```

```json
{"sequence":2,"command":{"type":"pointer","action":{"type":"move","surface":null,"position":[320.0,180.0]}}}
```

```json
{"sequence":3,"command":{"type":"keyboard","action":{"type":"press","key":"a"}}}
{"sequence":4,"command":{"type":"keyboard","action":{"type":"release","key":"a"}}}
{"sequence":5,"command":{"type":"text","text":"controlled text"}}
{"sequence":6,"command":{"type":"time","action":{"type":"advance","frames":1,"step_nanoseconds":16666667}}}
{"sequence":7,"command":{"type":"screenshot","path":"captures/after-step.png"}}
```

The public command groups are `Observe`, `Pointer`, `Keyboard`, `Text`, `Time`, `Screenshot`, and `Shutdown`. Pointer
actions are `Move`, `Press`, `Release`, and `Scroll`; there is no wire-level click. Keyboard press
and release are separate requests. `keyboard::Key` defines the stable wire names for letters,
digits, punctuation, modifiers, navigation keys, and F1 through F12. Text commands accept at most
16 KiB of UTF-8 and commit the whole string to the focused Bevy `EditableText`. A missing,
stale, or non-editable focus returns `text_focus_unavailable`.

`time.advance` runs between 1 and 10,000 complete application frames. `step_nanoseconds` must be an
integer between 1 and 1,000,000,000. Invalid frame counts and steps return
`invalid_time_frames`, `time_frames_too_large`, `invalid_time_step`, or `time_step_too_large`
without changing the clock. Controlled Sessions do not run Bevy simulation schedules while no
advance command is pending. Input commands update session-local Virtual Input immediately, while
Bevy application systems consume the queued transitions on the next controlled frame.

A rendered composition opts into screenshots with `automation_control::screenshot::Plugin`. The
plugin does not install a renderer, and `ready.controls` includes `screenshot` only when the
composition already has Bevy's renderer and screenshot capture. Logical Mode and renderer-free
compositions return `screenshot_capability_unavailable`.

Screenshot paths must be normalized, relative `.png` paths beneath the host-provided session
artifact root. Absolute paths, `.` or `..` components, and symbolic links are rejected. A completed
response contains the artifact type, session-relative path, MIME type, width, and height. The
response is written only after GPU readback, PNG encoding, and a successful read-back check. Fixed
scene dimensions and semantic image content are testable. Identical PNG bytes across operating
systems and GPUs are not guaranteed.

## Embed the plugin

```rust
use automation_control::AutomationControlPlugin;

app.add_plugins(AutomationControlPlugin::stdio());
```

Tests and embedding applications can provide transport adapters without capability lists:

```rust
app.add_plugins(AutomationControlPlugin::with_io(input, output));
```

The plugin disables native mouse and touch producers, writes `PointerInput` into Bevy's picking
pipeline, clears the OS cursor stored on Bevy windows, and clears native aggregate window,
keyboard, focus, mouse, touch, scroll, gamepad, and IME messages before focused-input dispatch.
Virtual keyboard commands
write `KeyboardInput`, update Bevy's `ButtonInput<KeyCode>` and `ButtonInput<Key>` resources, and
use Bevy's focused-input dispatch. Virtual text commands write `Ime::Commit`, which
`EditableTextInputPlugin` routes to the focused `EditableText`. The plugin processes one request per
event-loop update. Input responses acknowledge the queued session-local transition. An advance response is
written only after every requested frame, including application observers and text-edit systems, has
completed. `stdout` contains JSONL only. Diagnostics use `stderr`.

## Observation

`observation::observe_world` computes each answer from the current World. Selectors cover target
markers, UI entities, pointer entities, and one entity handle. Projections include summaries,
component names, selected reflected component values, and bounded hierarchies. Results are sorted
by session-local `{index,generation}` handles and paged with a hard limit and stateless cursor.
Reflection is read-only. Components that are absent, unregistered, not reflectable, not serializable,
or too large receive an explicit status.

The `clock` selector with the `summary` projection reports the completed frame index, elapsed
controlled nanoseconds, and the last step in nanoseconds. Repeated observations do not change it.

```json
{"sequence":7,"command":{"type":"observe","selector":{"type":"clock"},"projection":{"type":"summary"},"limit":1}}
```

The `virtual_input` selector with the `summary` projection reports the session's pointer position,
pressed pointer buttons, last scroll delta, held keyboard keys, current Bevy input focus, and last
text commit. These resources belong to one Controlled Session and are never shared with another
session.

```json
{"sequence":8,"command":{"type":"observe","selector":{"type":"virtual_input"},"projection":{"type":"summary"},"limit":1}}
```

`AutomationTarget` is an empty marker. It has no persistent semantic ID, role, label, or action
list. Handles are valid only for the current Bevy World.

## Driver

Enable the optional process driver for a host application:

```toml
automation_control = { path = "../../crates/automation_control", features = ["driver"] }
```

```rust
let mut session = Session::spawn(&launch, SessionOptions::new(Duration::from_secs(60)))?;
let ready = session.ready()?;
session.request(Command::Shutdown)?;
session.shutdown()?;
```

`driver::Session` owns sequences, child-process lifecycle, JSON serialization, response matching,
timeouts, stderr streaming, and optional ordered recording. `LaunchSpec` can start Cargo binaries
or examples. Diagnostics and report helpers remain available to host tools.

## Dummy app smoke test

The `bevy_context_menu` package is a context-menu application. Without features it runs as a Player
Run. With `automation`, it disables `InputPlugin` and the native gamepad producer, enables the
Controlled Session plugin, marks the background, buttons, menu items, and editable text target, and
exposes reflected pointer, keyboard, and text state.

```bash
cargo run -p automation_control --example bevy_controller --features driver
```

The default smoke exits as soon as its assertions pass. To keep the Controlled Session open while
moving the virtual pointer in one circle per second, provide a duration:

```bash
cargo run -p automation_control --example bevy_controller --features driver -- --circle-seconds 60
```

A display and render adapter are required for that smoke test. Display-free protocol, observation,
and driver tests do not need them.

## Testing

```bash
cargo test -p automation_control
cargo test -p automation_control --features driver
```

Recording/replay, camera operations, screenshot comparison, video capture, REPL orchestration,
persistent semantic IDs, and model adapters are follow-up work outside this slice.
