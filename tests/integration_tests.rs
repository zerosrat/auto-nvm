use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to run auto-nvm command in a specific directory
fn run_auto_nvm_in_dir(dir: &Path, args: &[&str]) -> std::process::Output {
    let binary_path = env!("CARGO_BIN_EXE_auto-nvm");

    Command::new(binary_path)
        .args(args)
        .current_dir(dir)
        .output()
        .expect("Failed to execute auto-nvm command")
}

/// Helper function to create a temporary directory with .nvmrc
fn create_temp_dir_with_nvmrc(content: &str) -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let nvmrc_path = temp_dir.path().join(".nvmrc");
    fs::write(&nvmrc_path, content).expect("Failed to write .nvmrc");
    temp_dir
}

#[test]
fn test_check_command_with_valid_semantic_version() {
    let temp_dir = create_temp_dir_with_nvmrc("18.17.0");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["check"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Found .nvmrc with version: 18.17.0"));
    assert!(stdout.contains("Current Node.js version:"));
}

#[test]
fn test_check_command_with_lts_version() {
    let temp_dir = create_temp_dir_with_nvmrc("lts");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["check"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Found .nvmrc with version: lts"));
}

#[test]
fn test_check_command_with_invalid_version() {
    let temp_dir = create_temp_dir_with_nvmrc("not-a-valid-version");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["check"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid version format"));
}

#[test]
fn test_check_command_no_nvmrc() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["check"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No .nvmrc file found"));
}

#[test]
fn test_switch_command_with_valid_version() {
    let temp_dir = create_temp_dir_with_nvmrc("18.17.0");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["switch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nvm use 18.17.0"));
}

#[test]
fn test_switch_command_with_lts() {
    let temp_dir = create_temp_dir_with_nvmrc("lts");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["switch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nvm use lts"));
}

#[test]
fn test_switch_command_no_nvmrc() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["switch"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No .nvmrc file found"));
}

#[test]
fn test_quiet_flag() {
    let temp_dir = create_temp_dir_with_nvmrc("18.17.0");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["--quiet", "check"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // In quiet mode, output should be minimal
    assert!(!stdout.contains("Checking for .nvmrc"));
}

#[test]
fn test_version_parsing_edge_cases() {
    // Test with whitespace
    let temp_dir = create_temp_dir_with_nvmrc("  18.17.0  \n");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["check"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Found .nvmrc with version: 18.17.0"));
}

#[test]
fn test_v_prefix_version() {
    let temp_dir = create_temp_dir_with_nvmrc("v16.14.0");
    let output = run_auto_nvm_in_dir(temp_dir.path(), &["switch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nvm use v16.14.0"));
}
