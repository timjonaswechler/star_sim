# Bevy 0.19 seams for an automation-controlled viewer

Research for [#32](https://github.com/timjonaswechler/star_sim/issues/32), supporting the Wayfinder map [#31](https://github.com/timjonaswechler/star_sim/issues/31).

## Conclusion

Bevy 0.19 provides the rendering, UI layout, accessibility, fixed-time, and screenshot primitives needed by the proposed viewer. It does not provide an automation protocol, semantic target registry, or complete user-facing UI observation tree. Those belong in an application-owned `AutomationControlPlugin`:

```text
JSONL ingress
  -> semantic command/message
  -> shared domain action
  -> layout/observation
  -> optional asynchronous capture
  -> JSONL response
```

Use separate runtime configurations:

- **Logical headless:** `MinimalPlugins`, manually controlled updates, no real screenshots.
- **Rendered automation mode:** renderer enabled, either a normal window or an externally driven windowless image target.

The first prototype in #33 should compile-check and exercise the exact APIs identified below.

## 1. Native UI observation

### What Bevy exposes

`ComputedNode` is populated by `ui_layout_system` and exposes calculated size, content size, scroll position, border, padding, scale, and related layout values. `UiSystems::Layout` is the public `PostUpdate` set after which layout has been updated; `UiSystems::PostLayout` follows it. An observation system can therefore run in `PostUpdate` after `UiSystems::Layout` or, if clipping is needed, after `UiSystems::PostLayout`.

Sources:

- [`ComputedNode` in Bevy 0.19 source](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_ui/src/ui_node.rs#L28-L203)
- [`UiSystems` and `UiPlugin` scheduling](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_ui/src/lib.rs#L90-L205)
- [`ui_layout_system`](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_ui/src/layout/mod.rs#L77-L254)

Authored UI text is available through Bevy UI's `Text` component and additional spans through `TextSpan`; computed glyph layout is represented by `TextLayoutInfo`. Aggregating a user-facing label across spans remains application code.

Sources:

- [`Text`](https://docs.rs/bevy/0.19.0/bevy/ui/widget/struct.Text.html)
- [`TextSpan`](https://docs.rs/bevy/0.19.0/bevy/text/struct.TextSpan.html)
- [`TextLayoutInfo`](https://docs.rs/bevy/0.19.0/bevy/text/struct.TextLayoutInfo.html)

`AccessibilityNode` wraps an AccessKit node and carries semantic role/name/action information. Bevy also exposes platform accessibility actions as the `ActionRequest` message. `InteractionDisabled` prevents user interaction, updates the accessibility node's disabled state, but intentionally does not prevent rendering, widget updates, or keyboard focus.

Sources:

- [`AccessibilityNode` and `ActionRequest`](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_a11y/src/lib.rs#L80-L250)
- [`InteractionDisabled`](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_ui/src/interaction_states.rs#L1-L42)

### What remains application-owned

Bevy does not automatically produce the complete public tree promised in #31. `AutomationControlPlugin` must define:

- stable `AutomationTarget` identity;
- user-facing label and semantic action names;
- policy for enabled and visible state;
- which authored/computed properties are public;
- hierarchy and deterministic serialization order.

Recommended first observation fields:

```text
id, role, label, visible, enabled, actions, bounds
```

Determine `visible` from the application's active-screen policy plus `Node::display`, propagated visibility, computed layout, clipping, and target window/viewport. Do not claim pixel-perfect occlusion detection in the first version.

### Prototype sequence

```rust
app.add_systems(
    PostUpdate,
    build_ui_observation.after(UiSystems::PostLayout),
);
```

Query only entities carrying `AutomationTarget`; join their UI, text, accessibility, disabled, layout, visibility, and hierarchy data into the public schema. Never serialize Bevy `Entity` values.

## 2. Shared semantic actions

Bevy 0.19 ECS messages (`#[derive(Message)]`, `MessageWriter`, `MessageReader`) are an appropriate one-way seam. Human pointer/keyboard translation and JSONL ingress should both emit the same domain-level action, consumed by one action system:

```text
human input ---+
               +--> ViewerAction --> apply once --> resulting state
JSON command --+
```

Source:

- [Bevy 0.19 ECS messages module](https://github.com/bevyengine/bevy/tree/v0.19.0/crates/bevy_ecs/src/message)

This avoids making simulated pointer coordinates the primary interface and prevents duplicated domain behavior. The transport reader must not block Bevy's main thread; use a reader thread/channel or non-blocking polling, then enqueue messages in a named application system set.

**Needs prototype validation:** the exact channel/backpressure and shutdown implementation for stdin. This is application behavior, not a Bevy-provided transport.

## 3. Full-window and individual-camera screenshots

### Full window

The official screenshot interface is:

```rust
commands
    .spawn(Screenshot::primary_window())
    .observe(save_to_disk(path));
```

A specific window is available through `Screenshot::window(window_entity)`. Capture and GPU readback are asynchronous. A protocol response must not be sent when the request entity is spawned; it must follow `ScreenshotCaptured` and successful application-owned file writing.

Sources:

- [Official Bevy 0.19 screenshot example](https://github.com/bevyengine/bevy/blob/v0.19.0/examples/window/screenshot.rs)
- [`Screenshot::{window, primary_window, image}` and `ScreenshotCaptured`](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_render/src/view/window/screenshot.rs#L65-L170)

`save_to_disk` logs write failures rather than exposing them as a protocol result. Therefore #33 should attach its own `ScreenshotCaptured` observer, perform validated PNG writing, and only then complete the request.

### Individual camera

Render the camera into an image target and capture that image:

1. Create an `Image` with fixed dimensions and render-attachment usage.
2. Set the camera target to `RenderTarget::Image`.
3. Spawn `Screenshot::image(image_handle)`.
4. Complete the request from the capture/save observer.

Sources:

- [Official Bevy 0.19 render-to-texture example](https://github.com/bevyengine/bevy/blob/v0.19.0/examples/3d/render_to_texture.rs)
- [Official externally driven headless renderer example](https://github.com/bevyengine/bevy/blob/v0.19.0/examples/app/externally_driven_headless_renderer.rs)

The externally driven example is especially relevant: it disables `WinitPlugin`, configures no primary window, renders a camera to an `Image`, manually pumps `SubApps`, waits for the render device, and captures with `Screenshot::image`.

### Limitations

- `MinimalPlugins` cannot produce a real screenshot because it has no renderer.
- A camera image is not automatically identical to a complete window capture containing all window-targeted UI and compositing.
- Fixed dimensions, camera settings, asset readiness, renderer backend, and sufficient update/readback frames must be controlled.
- Cross-platform pixel identity is not promised.

## 4. Logical and rendered headless operation

`MinimalPlugins` includes the core app/task/time/transform/hierarchy/diagnostic/input/window/schedule-runner plugins but no renderer or winit event loop. `ScheduleRunnerPlugin::run_once()` executes one update; `run_loop(Duration)` repeatedly updates with wall-clock pacing.

Sources:

- [`MinimalPlugins`](https://docs.rs/bevy/0.19.0/bevy/struct.MinimalPlugins.html)
- [`ScheduleRunnerPlugin`](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_app/src/schedule_runner.rs)

Recommended modes:

```text
logical headless: MinimalPlugins + app-owned state/automation plugins
rendered window:  DefaultPlugins + WinitPlugin + primary window
rendered offscreen: DefaultPlugins, no primary window, WinitPlugin disabled,
                    camera -> Image, externally pumped updates
```

The third option is supported by an official Bevy 0.19 example and is preferable to treating Xvfb as the only possible rendered path. It still requires a working GPU or software rendering adapter.

## 5. Fixed time and deterministic stepping

Configure the fixed clock explicitly, for example:

```rust
app.insert_resource(Time::<Fixed>::from_hz(60.0));
```

Both `Time::<Fixed>::from_hz` and `from_duration` exist in Bevy 0.19. Put deterministic simulation in `FixedUpdate`.

For externally driven tests and automation stepping, `TimeUpdateStrategy::FixedTimesteps(n)` guarantees that each `App::update()` advances by the fixed timestep multiplied by `n` and runs the fixed loop exactly `n` times. `ScheduleRunnerPlugin` pacing alone does not provide this guarantee.

Sources:

- [`TimeUpdateStrategy::FixedTimesteps`](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_time/src/lib.rs#L100-L190)
- [`Time<Fixed>` constructors](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_time/src/fixed.rs#L69-L120)

Also control:

- simulation seed and identity-addressed random draws;
- request/action order;
- explicit system ordering;
- asset readiness before capture;
- window/image dimensions, scale factor, and camera configuration.

Fixed time does not guarantee bit-identical floating-point rendering or complete-world state across different platforms and backends.

## 6. Scheduling JSON, actions, observation, rendering, and response

Recommended application-owned sets:

```text
First:       poll decoded requests from a channel
PreUpdate:   translate human interaction to ViewerAction
Update:      apply ViewerAction and update domain/UI/camera state
PostUpdate:  Bevy UI Prepare -> Propagate -> Content -> Layout -> PostLayout
PostUpdate:  build structured observation after UiSystems::PostLayout
render:      Bevy extracts and submits render work
capture:     ScreenshotCaptured observer validates/writes image
response:    complete logical requests after observation;
             complete capture requests only after successful write
```

Use named `SystemSet`s and `.before`/`.after`; never rely on otherwise parallel systems happening to execute in a desired order. Queue requests as pending state across frames. A request that mutates UI and immediately asks for its post-layout state may require completion after the next `PostUpdate`, not directly inside the action system.

Verified public layout labels: `UiSystems::Layout`, `UiSystems::PostLayout`, and `UiSystems::Stack` in Bevy 0.19.

Source:

- [`UiSystems` scheduling](https://github.com/bevyengine/bevy/blob/v0.19.0/crates/bevy_ui/src/lib.rs#L90-L205)

**Needs prototype validation:** exact frame latency from camera mutation through render extraction, screenshot readback, application write, and final JSON response.

## 7. Linux CI rendering

Bevy's official v0.19 example workflow installs Xvfb and Mesa/Vulkan-related dependencies and runs examples under `xvfb-run`. The Linux dependency guide lists the distribution-specific window/audio/Vulkan packages.

Sources:

- [Official Bevy v0.19 example-run workflow](https://github.com/bevyengine/bevy/blob/v0.19.0/.github/workflows/example-run.yml)
- [Official Bevy v0.19 Linux dependencies](https://github.com/bevyengine/bevy/blob/v0.19.0/docs/linux_dependencies.md)

Recommended first CI split:

1. Logical job using `MinimalPlugins`, no display server.
2. Render smoke job using a fixed target size and the official dependency/Xvfb pattern.
3. Store the PNG as an artifact and validate format/dimensions, not exact pixels.

The official externally driven headless-renderer example offers a later windowless alternative, but it still depends on wgpu finding an adapter.

**UNVERIFIED:** the exact software-adapter environment variables and behavior on the repository's chosen GitHub Actions Ubuntu image. Do not hard-code `WGPU_FORCE_FALLBACK_ADAPTER` or promise llvmpipe/lavapipe support until #33 or #36 runs that image successfully.

## 8. Stable identities already in this repository

The simulation core has public validated textual provenance identifiers:

- `ClaimId`
- `ObjectId`
- `PrescriptionId`
- `SourceId`
- `ModelRealizationId`
- `CorrelationGroupId`

Source:

- [`crates/simulation/src/core/provenance/identifiers.rs`](../../crates/simulation/src/core/provenance/identifiers.rs)

The catalog implementation also contains deterministic numeric helper functions such as `stable_system_id`, `stable_member_id`, `stable_planet_host_id`, `stable_orbit_draw_id`, and `stable_explicit_planet_id`.

Source:

- [`crates/simulation/src/core/mod.rs`](../../crates/simulation/src/core/mod.rs)

Important limitation: those numeric helper functions are currently private implementation details. They should not be coupled directly into a public viewer protocol merely because they are deterministic. Prefer an existing public `ObjectId`, or deliberately introduce a public stable domain identity in the simulation interface. Static UI targets remain explicit IDs such as `toolbar.generate`.

The current `apps/bevy_viewer` only starts `DefaultPlugins`; it does not yet materialize a catalog object from which to choose the first dynamic identity. The vertical prototype can therefore begin with static targets, then add a dynamic target only after selecting an actual public simulation object.

## Recommendation for #33

Build the prototype around these verified seams:

1. Add a small application-owned target registry and static `AutomationTarget` values.
2. Read stdin on a worker thread and pass decoded requests through a channel.
3. Translate both a real Bevy button and the automation command into one `ViewerAction` message.
4. Observe UI after `UiSystems::PostLayout`.
5. Use `TimeUpdateStrategy::FixedTimesteps` and explicit `Time<Fixed>` for deterministic stepping tests.
6. Capture the primary window with `Screenshot::primary_window` and an application-owned `ScreenshotCaptured` observer.
7. Separately prove a camera image using the official externally driven renderer pattern; do not make this a prerequisite for the first window screenshot.
8. Test duplicate/unknown target errors and ensure stdout contains only JSONL.

## Remaining uncertainties for the prototype

1. Exact stdin worker shutdown/backpressure behavior.
2. Exact number of updates needed between visual mutation and a valid capture.
3. The target CI runner's available wgpu software adapter.
4. Which public simulation identity the first dynamic viewer entity should expose.

These are implementation experiments for #33 rather than unresolved architecture decisions in #31.
