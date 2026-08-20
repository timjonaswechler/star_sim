# Bevy test applications

This package holds rendered Bevy applications used to exercise `automation_control`. Each application is an explicit binary, so another application only needs a new `src/bin/*.rs` file and `[[bin]]` entry.

Without features, a binary is a Player Run with Bevy's native input plugins. With `--features automation`, the same binary is a Controlled Session: native input plugins are absent and `automation_control` supplies Virtual Input over protocol v2.

## Context menu

Run the application with native mouse and keyboard input:

```bash
cargo run -p bevy_test_apps --bin context_menu
```

The controller launches the automation composition and checks pointer buttons, scrolling, keyboard state, focused text entry, reflected state, and screenshots through the public `driver::Session` interface:

```bash
cargo run -p automation_control --example bevy_controller --features driver
```

## Application conventions

- Put application systems, components, and semantic test state in the binary that owns them. Shared code is limited to run composition.
- Give observable entities stable, descriptive `Name` values. Names must not depend on spawn order or an entity handle. Use lowercase kebab-case where a name has multiple words.
- Add `AutomationTarget` only to entities a Controller must find or operate. Keep the marker behind `cfg(feature = "automation")`.
- Store assertions that span several UI systems in a small application-owned component. Derive `Reflect`, add `#[reflect(Component)]`, and register the type with `App::register_type`.

## Source and license

`context_menu` adapts Bevy 0.19.1's [`examples/usage/context_menu.rs`](https://github.com/bevyengine/bevy/blob/v0.19.1/examples/usage/context_menu.rs). The text-input field, semantic session state, stable names, and Controlled Session integration are Star Sim changes. Bevy distributes the example under either the MIT License or Apache License 2.0, as recorded in Bevy's [repository license files](https://github.com/bevyengine/bevy/tree/v0.19.1#license).
