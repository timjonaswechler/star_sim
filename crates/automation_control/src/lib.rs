//! Transport and Bevy integration for isolated Controlled Sessions.
//!
//! The crate exposes one small machine interface: protocol-v2 commands, read-only World
//! observations, and Virtual Input. A Player Run does not depend on this crate.

#[cfg(feature = "driver")]
pub mod driver;
pub mod entity;
pub mod keyboard;
pub mod observation;
mod plugin;
pub mod pointer;
pub mod protocol;
pub mod screenshot;
pub mod target;
pub mod text;
pub mod time;
pub mod transport;

pub use entity::{Handle, HandleError};
pub use plugin::{AutomationControlPlugin, InputFactory};
pub use protocol::{
    Command, PROTOCOL_VERSION, ProtocolError, Ready, Request, Response, ResponseStatus, RunMode,
    decode_request,
};
pub use target::AutomationTarget;
pub use time::Clock as ControlledClock;
pub use transport::{Input, JsonLinesInput, Output, StdoutOutput};

/// Environment variable through which a controller supplies a child session's artifact root.
pub const AUTOMATION_CONTROL_ARTIFACT_DIR: &str = "AUTOMATION_CONTROL_ARTIFACT_DIR";

/// Resolves the controller-provided artifact root, falling back to `default` for standalone runs.
pub fn artifact_root_path(default: impl Into<std::path::PathBuf>) -> std::path::PathBuf {
    std::env::var_os(AUTOMATION_CONTROL_ARTIFACT_DIR)
        .filter(|value| !value.is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| default.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_uses_the_grouped_protocol_v2_wire_form() {
        let command = Command::Shutdown;
        assert_eq!(
            serde_json::to_value(command).unwrap(),
            serde_json::json!({"type": "shutdown"})
        );
        assert_eq!(PROTOCOL_VERSION, 2);
    }
}
