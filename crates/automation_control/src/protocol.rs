use crate::{observation::Request as ObservationRequest, pointer::Command as PointerCommand};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError, ser::SerializeMap,
};
use serde_json::{Map, Value};
use std::fmt;

pub const PROTOCOL_VERSION: u32 = 2;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Request {
    pub sequence: u64,
    pub command: Command,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Observe(ObservationRequest),
    Pointer(PointerCommand),
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
                serde_json::from_value(action)
                    .map(Self::Pointer)
                    .map_err(D::Error::custom)
            }
            "shutdown" => Ok(Self::Shutdown),
            other => Err(D::Error::custom(format!(
                "unsupported command type {other:?}"
            ))),
        }
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
            controls: vec!["pointer".into()],
            observation_scopes: vec![
                "targets".into(),
                "ui".into(),
                "pointers".into(),
                "entity".into(),
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
    let request = serde_json::from_str::<Request>(line)
        .map_err(|error| Response::error(0, "malformed_request", error.to_string()))?;
    if request.sequence == 0 {
        return Err(Response::error(
            0,
            "invalid_sequence",
            "request sequence must be greater than zero",
        ));
    }
    validate_command(&request.command)
        .map_err(|message| Response::error(request.sequence, "invalid_arguments", message))?;
    Ok(request)
}

fn validate_command(command: &Command) -> Result<(), String> {
    match command {
        Command::Observe(request) => request.validate().map_err(|error| error.to_string()),
        Command::Pointer(command) => command.validate().map_err(|error| error.to_string()),
        Command::Shutdown => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entity::Handle,
        observation::{Projection, Selector},
        pointer::{Button, Command as Pointer},
    };

    #[test]
    fn ready_is_negotiated_once_and_uses_v2_capability_names() {
        let ready = Ready::new(RunMode::Rendered);
        assert_eq!(ready.version, 2);
        assert_eq!(serde_json::to_value(&ready).unwrap()["type"], "ready");
        assert_eq!(
            serde_json::to_value(&ready).unwrap()["controls"],
            serde_json::json!(["pointer"])
        );
    }

    #[test]
    fn observe_and_pointer_wire_forms_are_flat_and_version_free() {
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
        assert_eq!(
            decode_request(invalid).unwrap_err().error.unwrap().code,
            "malformed_request"
        );
    }

    #[test]
    fn response_has_sequence_without_a_controller_id() {
        let response = Response::completed(7, serde_json::json!({"ok": true}));
        let value = serde_json::to_value(response).unwrap();
        assert_eq!(value["sequence"], 7);
        assert!(value.get("id").is_none());
    }
}
