//! CLI integration tests.

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("aa").unwrap()
}

#[test]
fn test_help() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI for querying AI model benchmarks",
        ));
}

#[test]
fn test_version() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_profile_list_empty() {
    // Use a temp directory to avoid affecting real config
    let temp = tempfile::tempdir().unwrap();
    cmd()
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join("config"))
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No profiles configured"));
}

#[test]
fn test_cache_status() {
    let temp = tempfile::tempdir().unwrap();
    cmd()
        .env("HOME", temp.path())
        .env("XDG_CACHE_HOME", temp.path().join("cache"))
        .arg("cache")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cache Status"));
}

#[test]
fn test_llms_requires_api_key() {
    let temp = tempfile::tempdir().unwrap();
    cmd()
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join("config"))
        .env_remove("AA_API_KEY")
        .arg("llms")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No API key configured"));
}

#[test]
fn test_quota_no_data() {
    let temp = tempfile::tempdir().unwrap();
    // First create a profile
    cmd()
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join("config"))
        .env("XDG_CACHE_HOME", temp.path().join("cache"))
        .env("AA_API_KEY", "test_key")
        .arg("quota")
        .assert()
        .success()
        .stdout(predicate::str::contains("No quota data available"));
}

#[test]
fn test_output_format_flags() {
    // Test that mutually exclusive flags are enforced
    let temp = tempfile::tempdir().unwrap();
    cmd()
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join("config"))
        .arg("--json")
        .arg("--csv")
        .arg("llms")
        .assert()
        .failure();
}
