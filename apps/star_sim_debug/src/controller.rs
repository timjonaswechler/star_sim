use automation_control::{
    Command as WireCommand, RunMode,
    driver::{
        DriverError, LaunchSpec, LaunchTargetKind, RecentLogs, Session as DriverSession,
        SessionOptions,
    },
    keyboard::{Command as KeyboardCommand, Key},
    observation::{Projection, Request as ObservationRequest, Selector},
    pointer::{Button as WireButton, Command as PointerCommand},
    text::Command as TextCommand,
    time::{Command as TimeCommand, MAX_FRAMES},
};
use serde_json::{Value, json};
use std::{fmt, path::PathBuf, time::Duration};

const APP_PACKAGE: &str = "app";
const APP_TARGET: &str = "app";
const APP_FEATURE: &str = "automation-control";
const SESSION_OBSERVATION_TYPE: &str = "app::menu::SessionObservation";
const SETTLE_STEP_NANOSECONDS: u64 = 16_666_667;
const CLICK_WAIT_FRAMES: u64 = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Mode {
    Logical,
    Rendered,
}

impl Mode {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Logical => "logical",
            Self::Rendered => "rendered",
        }
    }

    const fn wire(self) -> RunMode {
        match self {
            Self::Logical => RunMode::Logical,
            Self::Rendered => RunMode::Rendered,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SurfaceSize {
    width: f32,
    height: f32,
}

impl SurfaceSize {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
        }
    }

    fn position(self, x: f32, y: f32) -> Result<[f32; 2], ControllerError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(ControllerError::InvalidAction(
                "pointer coordinates must be finite".into(),
            ));
        }
        if !(0.0..1.0).contains(&x) || !(0.0..1.0).contains(&y) {
            return Err(ControllerError::InvalidAction(
                "pointer coordinates must be normalized values in [0, 1)".into(),
            ));
        }
        Ok([x * self.width, y * self.height])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PointerButton {
    Left,
    Right,
    Middle,
}

impl PointerButton {
    const fn wire(self) -> WireButton {
        match self {
            Self::Left => WireButton::Primary,
            Self::Right => WireButton::Secondary,
            Self::Middle => WireButton::Middle,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Middle => "middle",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Action {
    PointerMove { x: f32, y: f32 },
    PointerPress(PointerButton),
    PointerRelease(PointerButton),
    PointerClick(PointerButton),
    Scroll { x: f32, y: f32 },
    KeyPress(String),
    KeyRelease(String),
    Text(String),
}

impl Action {
    fn description(&self) -> String {
        match self {
            Self::PointerMove { x, y } => format!("pointer move {x} {y}"),
            Self::PointerPress(button) => format!("pointer press {}", button.as_str()),
            Self::PointerRelease(button) => format!("pointer release {}", button.as_str()),
            Self::PointerClick(button) => format!("pointer click {}", button.as_str()),
            Self::Scroll { x, y } => format!("scroll {x} {y}"),
            Self::KeyPress(key) => format!("key {key} press"),
            Self::KeyRelease(key) => format!("key {key} release"),
            Self::Text(text) => format!("text {text}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Observation {
    Targets,
    Ui,
    Pointers,
    VirtualInput,
    Clock,
    ActiveScreen,
}

impl Observation {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Targets => "targets",
            Self::Ui => "ui",
            Self::Pointers => "pointers",
            Self::VirtualInput => "input",
            Self::Clock => "clock",
            Self::ActiveScreen => "screen",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Status {
    pub(crate) instance: String,
    pub(crate) mode: Mode,
    pub(crate) active_screen: String,
    pub(crate) paused: bool,
    pub(crate) last_action: String,
}

#[derive(Debug)]
pub(crate) enum ControllerError {
    Launch(String),
    Communication(String),
    Child(String),
    Request { code: String, message: String },
    InvalidAction(String),
    InvalidObservation(String),
    PausedWait,
    WaitLimitReached { frames: u64 },
    Shutdown,
}

impl fmt::Display for ControllerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Launch(message) => {
                write!(formatter, "could not start Controlled Session: {message}")
            }
            Self::Communication(message) => write!(
                formatter,
                "Controlled Session communication failed: {message}"
            ),
            Self::Child(message) => write!(formatter, "Controlled Session ended: {message}"),
            Self::Request { code, message } => write!(formatter, "{code}: {message}"),
            Self::InvalidAction(message) | Self::InvalidObservation(message) => {
                formatter.write_str(message)
            }
            Self::PausedWait => formatter.write_str(
                "wait condition is not met and the session is paused; use step or resume",
            ),
            Self::WaitLimitReached { frames } => {
                write!(
                    formatter,
                    "wait condition was not met within {frames} controlled frames"
                )
            }
            Self::Shutdown => formatter.write_str("Controlled Session is already shut down"),
        }
    }
}

impl std::error::Error for ControllerError {}

impl ControllerError {
    pub(crate) const fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::Launch(_) | Self::Communication(_) | Self::Child(_) | Self::Shutdown
        )
    }
}

pub(crate) struct ControllerSession {
    driver: Option<DriverSession>,
    mode: Mode,
    surface: SurfaceSize,
    instance: String,
    paused: bool,
    last_action: String,
}

impl ControllerSession {
    pub(crate) fn start(
        mode: Mode,
        surface: SurfaceSize,
        artifact_dir: PathBuf,
        recent_logs: RecentLogs,
    ) -> Result<Self, ControllerError> {
        let launch = LaunchSpec {
            package: APP_PACKAGE.into(),
            kind: LaunchTargetKind::Binary,
            target: APP_TARGET.into(),
            features: vec![APP_FEATURE.into()],
            arguments: vec!["--controlled-mode".into(), mode.as_str().into()],
        };
        let options = SessionOptions::new(Duration::from_secs(180))
            .with_recent_logs(recent_logs)
            .with_artifact_dir(artifact_dir);
        let mut driver = DriverSession::spawn(&launch, options).map_err(map_driver_error)?;
        let ready = driver.ready().map_err(map_driver_error)?;
        if ready.mode != mode.wire() {
            return Err(ControllerError::Communication(
                "child reported a different execution mode".into(),
            ));
        }
        let mut session = Self {
            driver: Some(driver),
            mode,
            surface,
            instance: "alpha".into(),
            paused: false,
            last_action: "none".into(),
        };
        session.advance(1)?;
        Ok(session)
    }

    #[cfg(test)]
    fn from_driver(driver: DriverSession, mode: Mode) -> Self {
        Self {
            driver: Some(driver),
            mode,
            surface: SurfaceSize::new(640, 360),
            instance: "alpha".into(),
            paused: false,
            last_action: "none".into(),
        }
    }

    pub(crate) fn perform(&mut self, action: Action) -> Result<Value, ControllerError> {
        let description = action.description();
        let result = match action {
            Action::PointerMove { x, y } => {
                let position = self.surface.position(x, y)?;
                self.pointer_transition(PointerCommand::Move {
                    surface: None,
                    position,
                })?
            }
            Action::PointerPress(button) => self.pointer_transition(PointerCommand::Press {
                button: button.wire(),
            })?,
            Action::PointerRelease(button) => self.pointer_transition(PointerCommand::Release {
                button: button.wire(),
            })?,
            Action::PointerClick(button) => {
                self.pointer_transition(PointerCommand::Press {
                    button: button.wire(),
                })?;
                self.pointer_transition(PointerCommand::Release {
                    button: button.wire(),
                })?
            }
            Action::Scroll { x, y } => {
                if !x.is_finite() || !y.is_finite() {
                    return Err(ControllerError::InvalidAction(
                        "scroll deltas must be finite".into(),
                    ));
                }
                self.pointer_transition(PointerCommand::Scroll { delta: [x, y] })?
            }
            Action::KeyPress(name) => {
                let key = parse_key(&name)?;
                self.input_transition(WireCommand::Keyboard(KeyboardCommand::Press { key }))?
            }
            Action::KeyRelease(name) => {
                let key = parse_key(&name)?;
                self.input_transition(WireCommand::Keyboard(KeyboardCommand::Release { key }))?
            }
            Action::Text(text) => {
                self.input_transition(WireCommand::Text(TextCommand::new(text)))?
            }
        };
        self.last_action = description;
        Ok(result)
    }

    pub(crate) fn click_target(&mut self, target: &str) -> Result<(), ControllerError> {
        let expected_screen = match target {
            "menu.tab.gym" => "gym",
            "menu.tab.museum" => "museum",
            "menu.tab.zoo" => "zoo",
            _ => {
                return Err(ControllerError::InvalidAction(format!(
                    "unknown Star Sim click target {target:?}"
                )));
            }
        };
        let targets = self.observe_raw(Observation::Targets)?;
        let item = targets["items"]
            .as_array()
            .and_then(|items| items.iter().find(|item| item["name"] == target))
            .ok_or_else(|| {
                ControllerError::InvalidObservation(format!(
                    "Star Sim target {target:?} is not available"
                ))
            })?;
        let bounds = &item["bounds"];
        let x = number(bounds, "x")? + number(bounds, "width")? / 2.0;
        let y = number(bounds, "y")? + number(bounds, "height")? / 2.0;

        self.pointer_transition(PointerCommand::Move {
            surface: None,
            position: [x as f32, y as f32],
        })?;
        self.pointer_transition(PointerCommand::Press {
            button: WireButton::Primary,
        })?;
        self.pointer_transition(PointerCommand::Release {
            button: WireButton::Primary,
        })?;
        self.wait_for(Observation::ActiveScreen, CLICK_WAIT_FRAMES, |value| {
            value["active_screen"] == expected_screen
        })?;
        self.last_action = format!("click {target}");
        Ok(())
    }

    pub(crate) fn observe(&mut self, observation: Observation) -> Result<Value, ControllerError> {
        let value = self.observe_raw(observation)?;
        self.last_action = format!("observe {}", observation.as_str());
        Ok(value)
    }

    pub(crate) fn wait_for<F>(
        &mut self,
        observation: Observation,
        frame_limit: u64,
        mut predicate: F,
    ) -> Result<Value, ControllerError>
    where
        F: FnMut(&Value) -> bool,
    {
        if frame_limit > MAX_FRAMES {
            return Err(ControllerError::InvalidAction(format!(
                "wait frame limit must be at most {MAX_FRAMES}"
            )));
        }
        let mut value = self.observe_raw(observation)?;
        if predicate(&value) {
            return Ok(value);
        }
        if self.paused {
            return Err(ControllerError::PausedWait);
        }
        for _ in 0..frame_limit {
            self.advance(1)?;
            value = self.observe_raw(observation)?;
            if predicate(&value) {
                return Ok(value);
            }
        }
        Err(ControllerError::WaitLimitReached {
            frames: frame_limit,
        })
    }

    pub(crate) fn pause(&mut self) {
        self.paused = true;
        self.last_action = "pause".into();
    }

    pub(crate) fn resume(&mut self) {
        self.paused = false;
        self.last_action = "resume".into();
    }

    pub(crate) fn step(&mut self, frames: u64) -> Result<(), ControllerError> {
        self.advance(frames)?;
        self.last_action = format!("step {frames}");
        Ok(())
    }

    pub(crate) fn status(&mut self) -> Result<Status, ControllerError> {
        let active_screen = self.active_screen()?;
        Ok(Status {
            instance: self.instance.clone(),
            mode: self.mode,
            active_screen,
            paused: self.paused,
            last_action: self.last_action.clone(),
        })
    }

    pub(crate) fn ensure_running(&mut self) -> Result<(), ControllerError> {
        self.driver_mut()?
            .ensure_running()
            .map_err(map_driver_error)
    }

    pub(crate) fn shutdown(&mut self) -> Result<(), ControllerError> {
        let driver = self.driver.take().ok_or(ControllerError::Shutdown)?;
        driver.shutdown().map_err(map_driver_error)
    }

    fn pointer_transition(&mut self, command: PointerCommand) -> Result<Value, ControllerError> {
        self.input_transition(WireCommand::Pointer(command))
    }

    fn input_transition(&mut self, command: WireCommand) -> Result<Value, ControllerError> {
        let result = self.request(command)?;
        self.advance(1)?;
        Ok(result)
    }

    fn advance(&mut self, frames: u64) -> Result<Value, ControllerError> {
        if frames == 0 || frames > MAX_FRAMES {
            return Err(ControllerError::InvalidAction(format!(
                "step frames must be between 1 and {MAX_FRAMES}"
            )));
        }
        self.request(WireCommand::Time(TimeCommand::advance(
            frames,
            SETTLE_STEP_NANOSECONDS,
        )))
    }

    fn observe_raw(&mut self, observation: Observation) -> Result<Value, ControllerError> {
        if observation == Observation::ActiveScreen {
            return Ok(json!({"active_screen": self.active_screen()?}));
        }
        let selector = match observation {
            Observation::Targets => Selector::Targets,
            Observation::Ui => Selector::Ui,
            Observation::Pointers => Selector::Pointers,
            Observation::VirtualInput => Selector::VirtualInput,
            Observation::Clock => Selector::Clock,
            Observation::ActiveScreen => unreachable!(),
        };
        self.request(WireCommand::Observe(ObservationRequest::new(
            selector,
            Projection::Summary,
        )))
    }

    fn active_screen(&mut self) -> Result<String, ControllerError> {
        let targets = self.observe_raw(Observation::Targets)?;
        let handle = targets["items"]
            .as_array()
            .and_then(|items| items.iter().find(|item| item["name"] == "session.status"))
            .and_then(|item| serde_json::from_value(item["entity"].clone()).ok())
            .ok_or_else(|| {
                ControllerError::InvalidObservation(
                    "Star Sim session status target is not available".into(),
                )
            })?;
        let result = self.request(WireCommand::Observe(ObservationRequest::new(
            Selector::Entity(handle),
            Projection::Components {
                type_paths: vec![SESSION_OBSERVATION_TYPE.into()],
            },
        )))?;
        let component = &result["items"][0]["components"][SESSION_OBSERVATION_TYPE];
        if component["status"] != "available" {
            return Err(ControllerError::InvalidObservation(
                "Star Sim active screen observation is unavailable".into(),
            ));
        }
        component["value"]["active_screen"]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| {
                ControllerError::InvalidObservation(
                    "Star Sim active screen observation is invalid".into(),
                )
            })
    }

    fn request(&mut self, command: WireCommand) -> Result<Value, ControllerError> {
        let response = self
            .driver_mut()?
            .request(command)
            .map_err(map_driver_error)?;
        response
            .result
            .ok_or_else(|| ControllerError::Communication("child returned an empty result".into()))
    }

    fn driver_mut(&mut self) -> Result<&mut DriverSession, ControllerError> {
        self.driver.as_mut().ok_or(ControllerError::Shutdown)
    }
}

fn parse_key(name: &str) -> Result<Key, ControllerError> {
    let wire_name = snake_case(name);
    let key: Key = serde_json::from_value(json!(wire_name)).map_err(|_| {
        ControllerError::InvalidAction(format!("unsupported keyboard key {name:?}"))
    })?;
    KeyboardCommand::Press { key: key.clone() }
        .validate()
        .map_err(|error| ControllerError::InvalidAction(error.to_string()))?;
    Ok(key)
}

fn snake_case(name: &str) -> String {
    let mut output = String::with_capacity(name.len());
    for (index, character) in name.chars().enumerate() {
        if character == '-' || character == ' ' {
            output.push('_');
        } else if character.is_ascii_uppercase() {
            if index > 0 {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else {
            output.push(character.to_ascii_lowercase());
        }
    }
    output
}

fn number(object: &Value, field: &str) -> Result<f64, ControllerError> {
    object[field].as_f64().ok_or_else(|| {
        ControllerError::InvalidObservation(format!("Star Sim target has no numeric {field} bound"))
    })
}

fn map_driver_error(error: DriverError) -> ControllerError {
    match error {
        DriverError::Launch(message) => ControllerError::Launch(message),
        DriverError::RequestFailed(response) => {
            if let Some(error) = response.error {
                ControllerError::Request {
                    code: error.code,
                    message: error.message,
                }
            } else {
                ControllerError::Communication("child rejected a request".into())
            }
        }
        DriverError::Child(message) => ControllerError::Child(message),
        DriverError::Timeout(_) => {
            ControllerError::Communication("child did not respond before the timeout".into())
        }
        DriverError::Io(_) | DriverError::Protocol(_) => {
            ControllerError::Communication("the child transport was invalid".into())
        }
        DriverError::WaitLimitReached { frame_limit, .. } => ControllerError::WaitLimitReached {
            frames: frame_limit,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command as ProcessCommand;

    fn shell_session(script: &str) -> DriverSession {
        let mut command = ProcessCommand::new("sh");
        command.args(["-c", script]);
        let mut driver =
            DriverSession::spawn_command(command, SessionOptions::new(Duration::from_secs(2)))
                .unwrap();
        driver.ready().unwrap();
        driver
    }

    #[test]
    fn normalized_pointer_coordinates_exclude_the_surface_edge() {
        let surface = SurfaceSize::new(640, 360);
        assert_eq!(surface.position(0.5, 0.25).unwrap(), [320.0, 90.0]);
        assert!(surface.position(1.0, 0.5).is_err());
        assert!(surface.position(-0.1, 0.5).is_err());
    }

    #[test]
    fn keyboard_names_are_case_insensitive_and_accept_pascal_case() {
        assert_eq!(parse_key("Escape").unwrap(), Key::Escape);
        assert_eq!(parse_key("ArrowLeft").unwrap(), Key::ArrowLeft);
        assert!(parse_key("Hyperdrive").is_err());
    }

    #[test]
    fn request_errors_hide_protocol_envelopes_and_sequences() {
        let driver = shell_session(
            r#"printf '%s\n' '{"type":"ready","version":2,"mode":"logical","controls":["pointer","time"],"observation_scopes":[]}'; read line; printf '%s\n' '{"sequence":1,"status":"error","error":{"code":"pointer_failed","message":"no pointer location"}}'; sleep 1"#,
        );
        let mut session = ControllerSession::from_driver(driver, Mode::Logical);
        let message = session
            .perform(Action::PointerPress(PointerButton::Left))
            .unwrap_err()
            .to_string();
        assert_eq!(message, "pointer_failed: no pointer location");
        assert!(!message.contains("sequence"));
        assert!(!message.contains("Response"));
        assert!(!message.contains("version"));
    }

    #[test]
    fn paused_wait_observes_once_without_advancing() {
        let driver = shell_session(
            r#"printf '%s\n' '{"type":"ready","version":2,"mode":"logical","controls":["time"],"observation_scopes":["clock"]}'; read line; printf '%s\n' '{"sequence":1,"status":"completed","result":{"items":[{"ready":false}]}}'; sleep 1"#,
        );
        let mut session = ControllerSession::from_driver(driver, Mode::Logical);
        session.pause();
        let error = session
            .wait_for(Observation::Clock, 4, |value| {
                value["items"][0]["ready"] == true
            })
            .unwrap_err();
        assert!(matches!(error, ControllerError::PausedWait));
    }
}
