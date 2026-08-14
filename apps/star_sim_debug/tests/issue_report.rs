use serde_json::{Value, json};
use std::{fs, path::PathBuf, process::Command};

#[test]
fn failure_artifacts_generate_a_github_issue_draft() {
    let artifact_dir = unique_temp_dir();
    fs::create_dir_all(&artifact_dir).unwrap();
    fs::write(
        artifact_dir.join("failure.json"),
        serde_json::to_vec_pretty(&json!({
            "version": 1,
            "kind": "panic",
            "message": "stellar catalog invariant violated",
            "cli_error": "child exited with status 101"
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        artifact_dir.join("recent.log"),
        "ERROR star_sim::automation: stellar catalog invariant violated\nstack frame\n",
    )
    .unwrap();
    let session = (0..15)
        .map(|sequence| {
            json!({
                "sequence": sequence,
                "direction": "to_app",
                "message": {"id": format!("request-{sequence}")}
            })
            .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(artifact_dir.join("session.jsonl"), format!("{session}\n")).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_star_sim_debug"))
        .arg("report")
        .arg(&artifact_dir)
        .output()
        .expect("report command should start");

    assert!(
        output.status.success(),
        "report failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "drafted");
    let draft = fs::read_to_string(artifact_dir.join("github-issue.md")).unwrap();
    assert!(draft.contains("# [automation failure] stellar catalog invariant violated"));
    assert!(draft.contains("ERROR star_sim::automation"));
    assert!(!draft.contains("request-2\""));
    assert!(draft.contains("request-3\""));
    assert!(draft.contains("request-14\""));

    fs::remove_dir_all(artifact_dir).ok();
}

fn unique_temp_dir() -> PathBuf {
    std::env::temp_dir().join(format!("star-sim-issue-report-test-{}", std::process::id()))
}
