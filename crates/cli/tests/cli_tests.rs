use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
#[allow(deprecated)]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("zero-copy data inspection engine"));
}

#[test]
#[allow(deprecated)]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
#[allow(deprecated)]
fn test_cli_config_show() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    cmd.args(["config", "show"]).assert().success();
}

#[test]
#[allow(deprecated)]
fn test_cli_invalid_command() {
    let mut cmd = Command::cargo_bin("zero-cli").unwrap();
    cmd.arg("invalid-command-xyz").assert().failure();
}
