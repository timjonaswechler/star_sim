use automation_control::{
    Command,
    driver::{Session, SessionOptions},
};
use serde_json::Value;
use std::{fs, path::PathBuf, process::Command as ProcessCommand, time::Duration};

#[test]
fn child_protocol_uses_driver_sequences_starting_at_one() {
    let transcript = unique_temp_path("session.jsonl");
    let mut command = ProcessCommand::new("sh");
    command.args([
        "-c",
        "printf '%s\\n' '{\"type\":\"ready\",\"version\":2,\"mode\":\"logical\",\"controls\":[\"pointer\"],\"observation_scopes\":[\"targets\"]}'; read line; printf '%s\\n' '{\"sequence\":1,\"status\":\"completed\",\"result\":{}}'",
    ]);
    let mut session = Session::spawn_command(
        command,
        SessionOptions::new(Duration::from_secs(2)).with_record(Some(transcript.clone())),
    )
    .expect("child should start");
    assert_eq!(session.ready().unwrap().version, 2);
    session.request(Command::Shutdown).unwrap();

    let entries: Vec<Value> = fs::read_to_string(&transcript)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let request = entries
        .iter()
        .find(|entry| entry["direction"] == "to_app")
        .unwrap();
    assert_eq!(request["message"]["sequence"], 1);
    assert!(request["message"].get("id").is_none());
    fs::remove_file(transcript).ok();
}

fn unique_temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "automation-control-driver-test-{}-{}-{name}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}
