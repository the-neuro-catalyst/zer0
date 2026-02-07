use assert_cmd::Command;
use predicates::prelude::predicate;
use std::path::PathBuf;

#[test]
#[allow(deprecated)]
fn test_cli_inspect_json() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../test_data/complex_data.json");

    cmd.args(["inspect", "--data-path", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Metric Analysis:"))
        .stdout(predicate::str::contains("Entropy Score:"))
        .stdout(predicate::str::contains("Structural Depth:"));
}

#[test]
#[allow(deprecated)]
fn test_cli_inspect_detailed_json() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../test_data/complex_data_pii.json");

    cmd.args(["inspect", "--data-path", path.to_str().unwrap(), "--detailed"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initiating security baseline audit..."))
        .stdout(predicate::str::contains("Metric Analysis:"));
}

#[test]
#[allow(deprecated)]
fn test_cli_inspect_yaml() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../test_data/deploy.yaml");

    cmd.args(["inspect", "--data-path", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Metric Analysis:"))
        .stdout(predicate::str::contains("Structural Depth:"));
}

#[test]
#[allow(deprecated)]
fn test_cli_inspect_toml() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../test_data/settings.toml");

    cmd.args(["inspect", "--data-path", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Metric Analysis:"))
        .stdout(predicate::str::contains("Structural Depth:"));
}
