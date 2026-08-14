use serde_json::Value;
use std::{fs, path::PathBuf, process::Command};

#[test]
fn logical_session_can_be_recorded_as_ordered_json_lines() {
    let transcript = unique_temp_path("logical-session.jsonl");
    let output = Command::new(env!("CARGO_BIN_EXE_star_sim_debug"))
        .args(["logical", "--record"])
        .arg(&transcript)
        .output()
        .expect("star_sim_debug should start");

    assert!(
        output.status.success(),
        "CLI failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let entries: Vec<Value> = fs::read_to_string(&transcript)
        .expect("recording should exist")
        .lines()
        .map(|line| serde_json::from_str(line).expect("each recording line should be JSON"))
        .collect();

    assert_eq!(entries[0]["sequence"], 0);
    assert_eq!(entries[0]["direction"], "from_app");
    assert_eq!(entries[0]["message"]["type"], "ready");

    for (index, entry) in entries.iter().enumerate() {
        assert_eq!(entry["sequence"], index as u64);
    }

    let request = entries
        .iter()
        .find(|entry| entry["direction"] == "to_app" && entry["message"]["id"] == "pause")
        .expect("pause request should be recorded");
    assert_eq!(request["message"]["command"]["type"], "pause");

    let response = entries
        .iter()
        .find(|entry| entry["direction"] == "from_app" && entry["message"]["id"] == "pause")
        .expect("pause response should be recorded");
    assert_eq!(response["message"]["status"], "completed");

    let shutdown_request_index = entries
        .iter()
        .position(|entry| entry["direction"] == "to_app" && entry["message"]["id"] == "shutdown")
        .expect("shutdown request should be recorded");
    let shutdown_response_index = entries
        .iter()
        .position(|entry| entry["direction"] == "from_app" && entry["message"]["id"] == "shutdown")
        .expect("shutdown response should be recorded");
    assert!(shutdown_request_index < shutdown_response_index);

    fs::remove_file(transcript).ok();
}

fn unique_temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "star-sim-debug-{}-{}-{name}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ))
}
