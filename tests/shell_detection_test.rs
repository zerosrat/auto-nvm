use std::env;

// Import the nvm module to test shell detection
use auto_nvm::nvm::{detect_shell, ShellType};

#[test]
fn test_shell_detection_zsh() {
    // Set SHELL environment variable to zsh
    env::set_var("SHELL", "/bin/zsh");
    let detected = detect_shell();
    assert_eq!(detected, ShellType::Zsh);
}

#[test]
fn test_shell_detection_bash() {
    // Set SHELL environment variable to bash
    env::set_var("SHELL", "/bin/bash");
    let detected = detect_shell();
    assert_eq!(detected, ShellType::Bash);
}

#[test]
fn test_shell_detection_fish() {
    // Set SHELL environment variable to fish
    env::set_var("SHELL", "/usr/local/bin/fish");
    let detected = detect_shell();
    assert_eq!(detected, ShellType::Fish);
}

#[test]
fn test_shell_detection_fallback() {
    // Unset SHELL environment variable to test fallback
    env::remove_var("SHELL");
    let detected = detect_shell();
    assert_eq!(detected, ShellType::Bash);
}
