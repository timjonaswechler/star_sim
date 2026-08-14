use crate::coordinates::{Coordinate, OperationMode, validate_coordinate};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PROTOCOL_VERSION: u32 = 1;
pub const DEFAULT_CAMERA_DURATION_MS: u32 = 250;
pub const MAX_STEP_FRAMES: u32 = 10_000;
pub const MAX_STEP_SIMULATION_MS: u64 = 86_400_000;
pub const DEFAULT_WAIT_TIMEOUT_FRAMES: u32 = 300;
pub const MAX_WAIT_TIMEOUT_FRAMES: u32 = 60_000;

fn default_duration_ms() -> u32 {
    DEFAULT_CAMERA_DURATION_MS
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Request {
    pub version: u32,
    pub id: String,
    pub command: Command,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    InspectUi,
    InspectScene,
    InspectSelection,
    InspectCamera,
    Click {
        target: String,
    },
    CameraFocus {
        camera: String,
        target: String,
        #[serde(default = "default_duration_ms")]
        duration_ms: u32,
    },
    CameraOrbit {
        camera: String,
        #[serde(default)]
        mode: OperationMode,
        yaw_deg: f32,
        pitch_deg: f32,
        #[serde(default = "default_duration_ms")]
        duration_ms: u32,
    },
    CameraPan {
        camera: String,
        #[serde(default)]
        mode: OperationMode,
        offset: Coordinate,
        #[serde(default = "default_duration_ms")]
        duration_ms: u32,
    },
    CameraZoom {
        camera: String,
        #[serde(default)]
        mode: OperationMode,
        value: f32,
        #[serde(default = "default_duration_ms")]
        duration_ms: u32,
    },
    Screenshot {
        source: ScreenshotSource,
        #[serde(default)]
        path: Option<String>,
        #[serde(default)]
        overwrite: bool,
    },
    Pause,
    Resume,
    StepFrames {
        count: u32,
    },
    StepSimulation {
        duration_ms: u64,
    },
    WaitUntil {
        condition: WaitCondition,
        #[serde(default = "default_wait_timeout_frames")]
        timeout_frames: u32,
    },
    InspectRun,
    Shutdown,
}

fn default_wait_timeout_frames() -> u32 {
    DEFAULT_WAIT_TIMEOUT_FRAMES
}

/// Closed, versioned set of conditions. It intentionally cannot express ECS queries.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WaitCondition {
    TargetExists { target: String },
    TargetVisible { target: String },
    TargetEnabled { target: String },
    TargetAbsent { target: String },
    ActiveScreen { screen: String },
    SelectionIs { target: String },
    CameraMotionComplete,
    ScreenshotComplete,
    SimulationPaused,
    FramesElapsed { count: u64 },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScreenshotSource {
    Window { target: String },
    Camera { target: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Ready {
    pub version: u32,
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub capabilities: Vec<String>,
    pub mode: RunMode,
    pub seed: u64,
    pub fixed_step_ms: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunMode {
    Logical,
    Rendered,
}

impl Ready {
    pub fn new(
        capabilities: impl IntoIterator<Item = impl Into<String>>,
        mode: RunMode,
        seed: u64,
        fixed_step_ms: u32,
    ) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            kind: "ready",
            capabilities: capabilities.into_iter().map(Into::into).collect(),
            mode,
            seed,
            fixed_step_ms,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProtocolError>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Completed,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolError {
    pub code: String,
    pub message: String,
}

impl Response {
    pub fn completed(id: impl Into<String>, result: Value) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            id: Some(id.into()),
            status: ResponseStatus::Completed,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(
        id: Option<impl Into<String>>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            id: id.map(Into::into),
            status: ResponseStatus::Error,
            result: None,
            error: Some(ProtocolError {
                code: code.into(),
                message: message.into(),
            }),
        }
    }
}

pub fn decode_request(line: &str) -> Result<Request, Response> {
    let request = serde_json::from_str::<Request>(line)
        .map_err(|error| Response::error(None::<String>, "malformed_request", error.to_string()))?;
    if request.version != PROTOCOL_VERSION {
        return Err(Response::error(
            Some(request.id.clone()),
            "unsupported_version",
            format!("expected version {PROTOCOL_VERSION}"),
        ));
    }
    if request.id.trim().is_empty() {
        return Err(Response::error(
            Some(request.id),
            "invalid_request_id",
            "request id must not be empty",
        ));
    }
    validate_command(&request.command).map_err(|message| {
        Response::error(Some(request.id.clone()), "invalid_arguments", message)
    })?;
    Ok(request)
}

fn validate_command(command: &Command) -> Result<(), String> {
    let finite = |value: f32, name: &str| {
        value
            .is_finite()
            .then_some(())
            .ok_or_else(|| format!("{name} must be finite"))
    };
    match command {
        Command::CameraOrbit {
            yaw_deg, pitch_deg, ..
        } => {
            finite(*yaw_deg, "yaw_deg")?;
            finite(*pitch_deg, "pitch_deg")
        }
        Command::CameraPan { offset, .. } => validate_coordinate(*offset).map_err(str::to_owned),
        Command::CameraZoom { value, mode, .. } => {
            finite(*value, "zoom value")?;
            if (*mode == OperationMode::Relative && *value == 0.0)
                || (*mode == OperationMode::Absolute && *value <= 0.0)
            {
                return Err(
                    "relative zoom must be nonzero and absolute zoom must be positive".into(),
                );
            }
            Ok(())
        }
        Command::Screenshot {
            path: Some(path), ..
        } if path.trim().is_empty() => Err("screenshot path must not be empty".into()),
        Command::StepFrames { count } if *count == 0 || *count > MAX_STEP_FRAMES => Err(format!(
            "frame count must be between 1 and {MAX_STEP_FRAMES}"
        )),
        Command::StepSimulation { duration_ms }
            if *duration_ms == 0 || *duration_ms > MAX_STEP_SIMULATION_MS =>
        {
            Err(format!(
                "simulation duration_ms must be between 1 and {MAX_STEP_SIMULATION_MS}"
            ))
        }
        Command::WaitUntil { timeout_frames, .. }
            if *timeout_frames == 0 || *timeout_frames > MAX_WAIT_TIMEOUT_FRAMES =>
        {
            Err(format!(
                "timeout_frames must be between 1 and {MAX_WAIT_TIMEOUT_FRAMES}"
            ))
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_protocol_defaults_to_relative_and_250ms() {
        let request = decode_request(r#"{"version":1,"id":"orbit","command":{"type":"camera_orbit","camera":"camera.main","yaw_deg":90,"pitch_deg":-10}}"#).unwrap();
        assert_eq!(
            request.command,
            Command::CameraOrbit {
                camera: "camera.main".into(),
                mode: OperationMode::Relative,
                yaw_deg: 90.0,
                pitch_deg: -10.0,
                duration_ms: 250
            }
        );
    }

    #[test]
    fn accepts_absolute_zero_duration_and_tagged_coordinates() {
        let request = decode_request(r#"{"version":1,"id":"pan","command":{"type":"camera_pan","camera":"camera.main","mode":"absolute","offset":{"space":"world","x":1,"y":2,"z":3},"duration_ms":0}}"#).unwrap();
        assert!(matches!(
            request.command,
            Command::CameraPan {
                mode: OperationMode::Absolute,
                duration_ms: 0,
                ..
            }
        ));
    }

    #[test]
    fn rejects_nonfinite_and_invalid_normalized_values() {
        let nan = r#"{"version":1,"id":"zoom","command":{"type":"camera_zoom","camera":"camera.main","value":1e400}}"#;
        assert_eq!(
            decode_request(nan).unwrap_err().error.unwrap().code,
            "malformed_request"
        );
        let invalid = r#"{"version":1,"id":"pan","command":{"type":"camera_pan","camera":"camera.main","offset":{"space":"viewport_normalized","x":2,"y":0}}}"#;
        assert_eq!(
            decode_request(invalid).unwrap_err().error.unwrap().code,
            "invalid_arguments"
        );
    }

    #[test]
    fn screenshot_sources_are_explicit() {
        let request = decode_request(r#"{"version":1,"id":"shot","command":{"type":"screenshot","source":{"type":"camera","target":"camera.main"}}}"#).unwrap();
        assert!(matches!(
            request.command,
            Command::Screenshot {
                source: ScreenshotSource::Camera { .. },
                path: None,
                overwrite: false
            }
        ));
    }

    #[test]
    fn step_and_wait_limits_are_validated() {
        for line in [
            r#"{"version":1,"id":"frames","command":{"type":"step_frames","count":0}}"#,
            r#"{"version":1,"id":"simulation","command":{"type":"step_simulation","duration_ms":0}}"#,
            r#"{"version":1,"id":"wait","command":{"type":"wait_until","condition":{"type":"simulation_paused"},"timeout_frames":0}}"#,
        ] {
            assert_eq!(
                decode_request(line).unwrap_err().error.unwrap().code,
                "invalid_arguments"
            );
        }
        let wait = decode_request(r#"{"version":1,"id":"wait","command":{"type":"wait_until","condition":{"type":"target_visible","target":"button"}}}"#).unwrap();
        assert!(matches!(
            wait.command,
            Command::WaitUntil {
                timeout_frames: 300,
                ..
            }
        ));
    }
}
