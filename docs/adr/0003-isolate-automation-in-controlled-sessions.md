---
status: accepted
---

# Isolate automation in controlled sessions

Star Sim uses separate build compositions for Player Runs and Controlled Sessions. A Player Run is built without `automation_control` and keeps the normal native Bevy input path. Only the Debug Host enables the optional automation feature and starts a Controlled Session, which accepts Virtual Input and rejects native window, keyboard, and pointer input. This keeps automation out of the normal game binary while allowing the host to run isolated sessions for humans, agents, scripts, and replay.

## Consequences

- `cargo run -p app` must neither compile nor link `automation_control` into the app binary.
- Enabling the automation feature selects the controlled app composition. An additional public `--automation` argument is unnecessary.
- `star_sim_debug` hides the controlled build command, child-process transport, and internal launch configuration.
- Recording belongs to the Controlled Session. The Debug Host records the same canonical actions and results regardless of which Controller produced them.
- Player Runs do not expose automation or Session Recording.
- Controlled Sessions never use global hooks or operating-system input injection. Each instance owns its Virtual Input state, artifacts, status, and recording.
- `logical` and `rendered` describe execution modes. They do not name fixed scenarios.
- Session commands do not require Controllers to repeat a protocol version or invent request IDs. The session negotiates compatibility at startup, and the host assigns ordered action sequences.
- Variant A (disable `bevy::input::InputPlugin` in rendered sessions) is the chosen implementation: native input resources are removed, and a rendered prototype demonstrates stable runtime behavior after explicitly registering lower-level Bevy message channels consumed by Winit/picking internals.
