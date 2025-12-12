//! CLI integration tests using assert_cmd.
//!
//! Following BAML's pattern of testing CLI behavior through subprocess execution.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn snps_cmd() -> Command {
    Command::cargo_bin("snps").unwrap()
}

#[test]
fn test_help_command() {
    snps_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("PMSynapse"))
        .stdout(predicate::str::contains("project management"));
}

#[test]
fn test_version() {
    snps_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("snps"));
}

#[test]
fn test_status_not_initialized() {
    let dir = tempdir().expect("Failed to create temp directory");

    snps_cmd()
        .current_dir(dir.path())
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Not initialized"));
}

#[test]
fn test_init_command() {
    let dir = tempdir().expect("Failed to create temp directory");

    // Initialize
    snps_cmd()
        .current_dir(dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing PMSynapse"))
        .stdout(predicate::str::contains("initialized successfully"));

    // Verify .pmsynapse directory was created
    assert!(dir.path().join(".pmsynapse").exists());
    assert!(dir.path().join(".pmsynapse/config.yaml").exists());
}

#[test]
fn test_init_already_initialized() {
    let dir = tempdir().expect("Failed to create temp directory");

    // First init
    snps_cmd()
        .current_dir(dir.path())
        .arg("init")
        .assert()
        .success();

    // Second init without force should warn
    snps_cmd()
        .current_dir(dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Already initialized"));
}

#[test]
fn test_init_force() {
    let dir = tempdir().expect("Failed to create temp directory");

    // First init
    snps_cmd()
        .current_dir(dir.path())
        .arg("init")
        .assert()
        .success();

    // Force reinit
    snps_cmd()
        .current_dir(dir.path())
        .args(["init", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Force mode enabled"));
}

#[test]
fn test_daemon_status() {
    snps_cmd()
        .args(["daemon", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Daemon Status"));
}

#[test]
fn test_thoughts_not_initialized() {
    let dir = tempdir().expect("Failed to create temp directory");

    snps_cmd()
        .current_dir(dir.path())
        .args(["thoughts", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("not initialized"));
}

#[test]
fn test_graph_empty() {
    snps_cmd()
        .arg("graph")
        .assert()
        .success()
        .stdout(predicate::str::contains("Knowledge Graph"))
        .stdout(predicate::str::contains("Nodes: 0"));
}

#[test]
fn test_templates_list() {
    snps_cmd()
        .args(["templates", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Templates"));
}

#[test]
fn test_team_list() {
    snps_cmd()
        .args(["team", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Teams"));
}

#[test]
fn test_proposals_list() {
    snps_cmd()
        .args(["proposals", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Proposals"));
}
