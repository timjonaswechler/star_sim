use automation_control::driver::{
    LaunchSpec, LaunchTargetKind, RecentLogs, Session, SessionOptions,
};
use automation_control::{Command, RunMode, WaitCondition};
use serde_json::Value;
use std::{fs, path::PathBuf, time::Duration};

#[test]
fn logical_headless_session_can_be_recorded_as_ordered_json_lines() {
    let transcript = unique_temp_path("logical-session.jsonl");
    let launch = LaunchSpec {
        package: "automation_control".into(),
        kind: LaunchTargetKind::Example,
        target: "automation_control_headless".into(),
        features: Vec::new(),
        arguments: vec![
            "--automation".into(),
            "--seed".into(),
            "42".into(),
            "--fixed-step-ms".into(),
            "50".into(),
        ],
    };
    let options = SessionOptions::new(Duration::from_secs(120))
        .with_record(Some(transcript.clone()))
        .with_recent_logs(RecentLogs::default());
    let mut session = Session::spawn(&launch, options).expect("headless example should start");

    let ready = session
        .ready(&[
            "click",
            "pause",
            "step_frames",
            "step_simulation",
            "wait_until",
            "inspect_run",
        ])
        .expect("headless example should announce its capabilities");
    assert_eq!(ready.mode, RunMode::Logical);
    session
        .request("pause", Command::Pause)
        .expect("pause request should complete");
    session
        .request("frames", Command::StepFrames { count: 3 })
        .expect("frame stepping request should complete");
    session
        .request("simulation", Command::StepSimulation { duration_ms: 120 })
        .expect("simulation stepping request should complete");
    session
        .request(
            "click",
            Command::Click {
                target: "toolbar.generate".into(),
            },
        )
        .expect("click request should complete");
    session
        .request(
            "wait-selection",
            Command::WaitUntil {
                condition: WaitCondition::SelectionIs {
                    target: "scene.prototype_star".into(),
                },
                timeout_frames: 5,
            },
        )
        .expect("selection wait should complete");
    session
        .request("state", Command::InspectRun)
        .expect("state inspection request should complete");
    session
        .shutdown()
        .expect("headless example should shut down");

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
        "automation-control-driver-test-{}-{}-{name}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos()
    ))
}
