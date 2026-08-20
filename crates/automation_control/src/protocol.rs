use crate::{
    keyboard::Command as KeyboardCommand, observation::Request as ObservationRequest,
    pointer::Command as PointerCommand, text::Command as TextCommand, time::Command as TimeCommand,
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError, ser::SerializeMap,
};
use serde_json::{Map, Value};
use std::fmt;

pub const PROTOCOL_VERSION: u32 = 2;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub sequence: u64,
    pub command: Command,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Observe(ObservationRequest),
    Pointer(PointerCommand),
    Keyboard(KeyboardCommand),
    Text(TextCommand),
    Time(TimeCommand),
    Shutdown,
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Observe(request) => {
                let content = serde_json::to_value(request).map_err(serde::ser::Error::custom)?;
                let Value::Object(content) = content else {
                    return Err(serde::ser::Error::custom(
                        "observe payload must be an object",
                    ));
                };
                map.serialize_entry("type", "observe")?;
                for (key, value) in content {
                    map.serialize_entry(&key, &value)?;
                }
            }
            Self::Pointer(command) => {
                map.serialize_entry("type", "pointer")?;
                map.serialize_entry("action", command)?;
            }
            Self::Keyboard(command) => {
                map.serialize_entry("type", "keyboard")?;
                map.serialize_entry("action", command)?;
            }
            Self::Text(command) => {
                map.serialize_entry("type", "text")?;
                map.serialize_entry("text", &command.text)?;
            }
            Self::Time(command) => {
                map.serialize_entry("type", "time")?;
                map.serialize_entry("action", command)?;
            }
            Self::Shutdown => map.serialize_entry("type", "shutdown")?,
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut object = Map::<String, Value>::deserialize(deserializer)?;
        let kind = object
            .remove("type")
            .and_then(|value| value.as_str().map(str::to_owned))
            .ok_or_else(|| D::Error::custom("command.type must be a string"))?;
        match kind.as_str() {
            "observe" => serde_json::from_value(Value::Object(object))
                .map(Self::Observe)
                .map_err(D::Error::custom),
            "pointer" => {
                let action = object
                    .remove("action")
                    .ok_or_else(|| D::Error::custom("pointer command requires action"))?;
                reject_extra_fields::<D::Error>(&object)?;
                serde_json::from_value(action)
                    .map(Self::Pointer)
                    .map_err(D::Error::custom)
            }
            "keyboard" => {
                let action = object
                    .remove("action")
                    .ok_or_else(|| D::Error::custom("keyboard command requires action"))?;
                reject_extra_fields::<D::Error>(&object)?;
                serde_json::from_value(action)
                    .map(Self::Keyboard)
                    .map_err(D::Error::custom)
            }
            "text" => serde_json::from_value(Value::Object(object))
                .map(Self::Text)
                .map_err(D::Error::custom),
            "time" => {
                let action = object
                    .remove("action")
                    .ok_or_else(|| D::Error::custom("time command requires action"))?;
                reject_extra_fields::<D::Error>(&object)?;
                serde_json::from_value(action)
                    .map(Self::Time)
                    .map_err(D::Error::custom)
            }
            "shutdown" => {
                reject_extra_fields::<D::Error>(&object)?;
                Ok(Self::Shutdown)
            }
            other => Err(D::Error::custom(format!(
                "unsupported command type {other:?}"
            ))),
        }
    }
}

fn reject_extra_fields<E: DeError>(object: &Map<String, Value>) -> Result<(), E> {
    if object.is_empty() {
        Ok(())
    } else {
        Err(E::custom(format!(
            "unexpected command fields: {}",
            object.keys().cloned().collect::<Vec<_>>().join(", ")
        )))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunMode {
    Logical,
    Rendered,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ready {
    #[serde(rename = "type")]
    pub kind: String,
    pub version: u32,
    pub mode: RunMode,
    pub controls: Vec<String>,
    pub observation_scopes: Vec<String>,
}

impl Ready {
    pub fn new(mode: RunMode) -> Self {
        Self {
            kind: "ready".into(),
            version: PROTOCOL_VERSION,
            mode,
            controls: vec![
                "pointer".into(),
                "keyboard".into(),
                "text".into(),
                "time".into(),
            ],
            observation_scopes: vec![
                "targets".into(),
                "ui".into(),
                "pointers".into(),
                "entity".into(),
                "virtual_input".into(),
                "clock".into(),
            ],
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Response {
    pub sequence: u64,
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProtocolError>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Completed,
    Error,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProtocolError {
    pub code: String,
    pub message: String,
}

impl Response {
    pub fn completed(sequence: u64, result: Value) -> Self {
        Self {
            sequence,
            status: ResponseStatus::Completed,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(sequence: u64, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            sequence,
            status: ResponseStatus::Error,
            result: None,
            error: Some(ProtocolError {
                code: code.into(),
                message: message.into(),
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodeError {
    Malformed(String),
    UnsupportedVersion(u32),
    InvalidSequence,
    InvalidArguments(String),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed(message) => write!(formatter, "malformed request: {message}"),
            Self::UnsupportedVersion(version) => {
                write!(formatter, "unsupported protocol version {version}")
            }
            Self::InvalidSequence => {
                formatter.write_str("request sequence must be greater than zero")
            }
            Self::InvalidArguments(message) => {
                write!(formatter, "invalid request arguments: {message}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

/// Decodes one protocol-v2 request. The caller supplies no request ID or protocol version.
pub fn decode_request(line: &str) -> Result<Request, Response> {
    let value = serde_json::from_str::<Value>(line)
        .map_err(|error| Response::error(0, "malformed_request", error.to_string()))?;
    let sequence = value.get("sequence").and_then(Value::as_u64).unwrap_or(0);
    let request = serde_json::from_value::<Request>(value)
        .map_err(|error| Response::error(sequence, "malformed_request", error.to_string()))?;
    if request.sequence == 0 {
        return Err(Response::error(
            0,
            "invalid_sequence",
            "request sequence must be greater than zero",
        ));
    }
    validate_command(&request.command)
        .map_err(|error| Response::error(request.sequence, error.code, error.message))?;
    Ok(request)
}

fn validate_command(command: &Command) -> Result<(), ProtocolError> {
    match command {
        Command::Observe(request) => request
            .validate()
            .map_err(|error| validation_error("invalid_arguments", error)),
        Command::Pointer(command) => command
            .validate()
            .map_err(|error| validation_error("invalid_arguments", error)),
        Command::Keyboard(command) => command
            .validate()
            .map_err(|error| validation_error(error.code(), error)),
        Command::Text(command) => command
            .validate()
            .map_err(|error| validation_error(error.code(), error)),
        Command::Time(command) => command
            .validate()
            .map_err(|error| validation_error(error.code(), error)),
        Command::Shutdown => Ok(()),
    }
}

fn validation_error(code: impl Into<String>, error: impl fmt::Display) -> ProtocolError {
    ProtocolError {
        code: code.into(),
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entity::Handle,
        keyboard::{Command as Keyboard, Key},
        observation::{Projection, Selector},
        pointer::{Button, Command as Pointer},
        text::Command as Text,
        time::{Command as Time, MAX_FRAMES, MAX_STEP_NANOSECONDS},
    };

    #[test]
    fn ready_is_negotiated_once_and_uses_v2_capability_names() {
        let ready = Ready::new(RunMode::Rendered);
        assert_eq!(ready.version, 2);
        assert_eq!(serde_json::to_value(&ready).unwrap()["type"], "ready");
        assert_eq!(
            serde_json::to_value(&ready).unwrap()["controls"],
            serde_json::json!(["pointer", "keyboard", "text", "time"])
        );
        assert!(ready.observation_scopes.contains(&"virtual_input".into()));
        assert!(ready.observation_scopes.contains(&"clock".into()));
    }

    #[test]
    fn command_wire_forms_are_grouped_and_version_free() {
        let observe = serde_json::to_value(Command::Observe(ObservationRequest::new(
            Selector::Entity(Handle::new(42, 3)),
            Projection::Summary,
        )))
        .unwrap();
        assert_eq!(observe["type"], "observe");
        assert_eq!(observe["selector"]["type"], "entity");
        assert!(observe.get("version").is_none());

        let pointer = serde_json::to_value(Command::Pointer(Pointer::Press {
            button: Button::Primary,
        }))
        .unwrap();
        assert_eq!(
            pointer,
            serde_json::json!({"type":"pointer", "action":{"type":"press", "button":"primary"}})
        );

        let keyboard =
            serde_json::to_value(Command::Keyboard(Keyboard::Press { key: Key::A })).unwrap();
        assert_eq!(
            keyboard,
            serde_json::json!({"type":"keyboard", "action":{"type":"press", "key":"a"}})
        );
        let text = serde_json::to_value(Command::Text(Text::new("hello"))).unwrap();
        assert_eq!(text, serde_json::json!({"type":"text", "text":"hello"}));

        let time = serde_json::to_value(Command::Time(Time::advance(60, 16_666_667))).unwrap();
        assert_eq!(
            time,
            serde_json::json!({
                "type":"time",
                "action":{"type":"advance", "frames":60, "step_nanoseconds":16_666_667}
            })
        );
    }

    #[test]
    fn decodes_and_validates_every_pointer_variant() {
        let inputs = [
            r#"{"sequence":1,"command":{"type":"pointer","action":{"type":"move","surface":null,"position":[1.0,2.0]}}}"#,
            r#"{"sequence":2,"command":{"type":"pointer","action":{"type":"press","button":"primary"}}}"#,
            r#"{"sequence":3,"command":{"type":"pointer","action":{"type":"release","button":"middle"}}}"#,
            r#"{"sequence":4,"command":{"type":"pointer","action":{"type":"press","button":"secondary"}}}"#,
            r#"{"sequence":5,"command":{"type":"pointer","action":{"type":"scroll","delta":[0.0,-1.0]}}}"#,
        ];
        for input in inputs {
            assert!(decode_request(input).is_ok(), "failed to decode {input}");
        }
        let invalid = r#"{"sequence":1,"command":{"type":"pointer","action":{"type":"move","surface":null,"position":[null,2.0]}}}"#;
        let response = decode_request(invalid).unwrap_err();
        assert_eq!(response.sequence, 1);
        assert_eq!(response.error.unwrap().code, "malformed_request");
    }

    #[test]
    fn keyboard_and_text_validation_return_typed_protocol_errors() {
        let invalid_key = r#"{"sequence":1,"command":{"type":"keyboard","action":{"type":"press","key":"hyperdrive"}}}"#;
        assert_eq!(
            decode_request(invalid_key).unwrap_err().error.unwrap().code,
            "invalid_key"
        );
        let empty_key =
            r#"{"sequence":1,"command":{"type":"keyboard","action":{"type":"press","key":""}}}"#;
        assert_eq!(
            decode_request(empty_key).unwrap_err().error.unwrap().code,
            "invalid_key"
        );
        let oversized = serde_json::json!({
            "sequence": 1,
            "command": {
                "type": "text",
                "text": "x".repeat(crate::text::MAX_BYTES + 1),
            }
        });
        assert_eq!(
            decode_request(&oversized.to_string())
                .unwrap_err()
                .error
                .unwrap()
                .code,
            "text_too_large"
        );
    }

    #[test]
    fn time_validation_returns_specific_errors_without_float_coercion() {
        let cases = [
            (
                r#"{"sequence":1,"command":{"type":"time","action":{"type":"advance","frames":0,"step_nanoseconds":1}}}"#,
                "invalid_time_frames",
            ),
            (
                &format!(
                    r#"{{"sequence":1,"command":{{"type":"time","action":{{"type":"advance","frames":{},"step_nanoseconds":1}}}}}}"#,
                    MAX_FRAMES + 1
                ),
                "time_frames_too_large",
            ),
            (
                r#"{"sequence":1,"command":{"type":"time","action":{"type":"advance","frames":1,"step_nanoseconds":0}}}"#,
                "invalid_time_step",
            ),
            (
                &format!(
                    r#"{{"sequence":1,"command":{{"type":"time","action":{{"type":"advance","frames":1,"step_nanoseconds":{}}}}}}}"#,
                    MAX_STEP_NANOSECONDS + 1
                ),
                "time_step_too_large",
            ),
        ];
        for (input, expected_code) in cases {
            assert_eq!(
                decode_request(input).unwrap_err().error.unwrap().code,
                expected_code
            );
        }

        let float = r#"{"sequence":1,"command":{"type":"time","action":{"type":"advance","frames":1,"step_nanoseconds":1.5}}}"#;
        assert_eq!(
            decode_request(float).unwrap_err().error.unwrap().code,
            "malformed_request"
        );
    }

    #[test]
    fn rejects_controller_ids_versions_and_extra_command_fields() {
        for input in [
            r#"{"sequence":1,"version":2,"command":{"type":"shutdown"}}"#,
            r#"{"sequence":1,"id":"controller","command":{"type":"shutdown"}}"#,
            r#"{"sequence":1,"command":{"type":"shutdown","extra":true}}"#,
            r#"{"sequence":1,"command":{"type":"keyboard","extra":true,"action":{"type":"press","key":"a"}}}"#,
            r#"{"sequence":1,"command":{"type":"keyboard","action":{"type":"press","key":"a","extra":true}}}"#,
        ] {
            let response = decode_request(input).unwrap_err();
            assert_eq!(response.sequence, 1, "wrong response sequence for {input}");
            assert_eq!(
                response.error.unwrap().code,
                "malformed_request",
                "accepted {input}"
            );
        }
    }

    #[test]
    fn response_has_sequence_without_a_controller_id() {
        let response = Response::completed(7, serde_json::json!({"ok": true}));
        let value = serde_json::to_value(response).unwrap();
        assert_eq!(value["sequence"], 7);
        assert!(value.get("id").is_none());
    }
}
