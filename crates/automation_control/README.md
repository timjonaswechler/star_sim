# `automation_control`

`automation_control` provides the protocol and Bevy integration used by isolated Controlled
Sessions. A Player Run does not depend on this crate. The crate does not know Star-Sim menu
semantics and never injects operating-system input.

## Protocol v2

The session emits one `ready` message. The host assigns request sequences beginning at `1`.
Controllers send only commands and do not provide protocol versions or request IDs.

```json
{"type":"ready","version":2,"mode":"rendered","controls":["pointer","keyboard","text"],"observation_scopes":["targets","ui","pointers","entity","virtual_input"]}
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
```

The public command groups are `Observe`, `Pointer`, `Keyboard`, `Text`, and `Shutdown`. Pointer
actions are `Move`, `Press`, `Release`, and `Scroll`; there is no wire-level click. Keyboard press
and release are separate requests. `keyboard::Key` defines the stable wire names for letters,
digits, punctuation, modifiers, navigation keys, and F1 through F12. Text commands accept at most
16 KiB of UTF-8 and commit the whole string to the focused Bevy `EditableText`. A missing,
stale, or non-editable focus returns `text_focus_unavailable`.

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
update and responds after application observers and text-edit systems have run. `stdout` contains
JSONL only. Diagnostics use `stderr`.

## Observation

`observation::observe_world` computes each answer from the current World. Selectors cover target
markers, UI entities, pointer entities, and one entity handle. Projections include summaries,
component names, selected reflected component values, and bounded hierarchies. Results are sorted
by session-local `{index,generation}` handles and paged with a hard limit and stateless cursor.
Reflection is read-only. Components that are absent, unregistered, not reflectable, not serializable,
or too large receive an explicit status.

The `virtual_input` selector with the `summary` projection reports the session's pointer position,
pressed pointer buttons, last scroll delta, held keyboard keys, current Bevy input focus, and last
text commit. These resources belong to one Controlled Session and are never shared with another
session.

```json
{"sequence":6,"command":{"type":"observe","selector":{"type":"virtual_input"},"projection":{"type":"summary"},"limit":1}}
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

The `bevy_example` package is a context-menu application. Without features it is a normal native
Bevy app. With `automation`, it disables `InputPlugin` and the native gamepad producer, enables the
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

Recording/replay, screenshots/camera operations, REPL orchestration, multiple process instances,
persistent semantic IDs, and model adapters are follow-up work outside this slice.
