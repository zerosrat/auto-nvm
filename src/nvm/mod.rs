use anyhow::{anyhow, Result};
use std::process::Command;
use which::which;

/// Supported shell types for nvm invocation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

/// Detect the current shell type based on environment
pub fn detect_shell() -> ShellType {
    // On Windows, default to PowerShell
    if cfg!(windows) {
        return ShellType::PowerShell;
    }

    // On Unix, detect via SHELL environment variable
    std::env::var("SHELL")
        .map(|s| {
            if s.contains("fish") {
                ShellType::Fish
            } else if s.contains("zsh") {
                ShellType::Zsh
            } else {
                ShellType::Bash
            }
        })
        .unwrap_or(ShellType::Bash)
}

/// Build shell command for nvm operations
fn build_nvm_command(shell: ShellType, nvm_args: &str) -> Command {
    match shell {
        ShellType::Fish => {
            // Fish shell: nvm is a function, no source needed
            let mut cmd = Command::new("fish");
            cmd.arg("-c").arg(format!("nvm {}", nvm_args));
            cmd
        }
        ShellType::PowerShell => {
            // PowerShell: nvm-windows uses direct command
            let mut cmd = Command::new("powershell");
            cmd.arg("-Command").arg(format!("nvm {}", nvm_args));
            cmd
        }
        ShellType::Zsh => {
            // Zsh: need to source nvm.sh first
            let mut cmd = Command::new("zsh");
            cmd.arg("-c")
                .arg(format!("source ~/.nvm/nvm.sh && nvm {}", nvm_args));
            cmd
        }
        ShellType::Bash => {
            // Bash: need to source nvm.sh first
            let mut cmd = Command::new("bash");
            cmd.arg("-c")
                .arg(format!("source ~/.nvm/nvm.sh && nvm {}", nvm_args));
            cmd
        }
    }
}

/// Detect if nvm is available on the system
pub fn detect_nvm() -> Result<bool> {
    // First check if nvm command is available in PATH (mainly for nvm-windows)
    if which("nvm").is_ok() {
        return Ok(true);
    }

    // Check if NVM_DIR environment variable is set (common nvm installation indicator)
    if std::env::var("NVM_DIR").is_ok() {
        return Ok(true);
    }

    // Try to run nvm and check if it's available using detected shell
    let shell = detect_shell();
    let output = build_nvm_command(shell, "--version").output();

    match output {
        Ok(result) => Ok(result.status.success()),
        Err(_) => Ok(false),
    }
}

/// Get the currently active Node.js version
pub fn get_current_version() -> Result<String> {
    // Try to get version using node directly first
    let output = Command::new("node").arg("--version").output();

    if let Ok(result) = output {
        if result.status.success() {
            let version = String::from_utf8_lossy(&result.stdout).trim().to_string();
            return Ok(version);
        }
    }

    // If node is not available, try using nvm current
    let shell = detect_shell();
    let output = build_nvm_command(shell, "current").output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let version = String::from_utf8_lossy(&result.stdout).trim().to_string();
                Ok(version)
            } else {
                Err(anyhow!("No Node.js version currently active"))
            }
        }
        Err(e) => Err(anyhow!("Failed to get current Node.js version: {}", e)),
    }
}

/// Switch to the specified Node.js version using nvm
#[allow(dead_code)]
pub fn switch_version(version: &str) -> Result<()> {
    if !detect_nvm()? {
        return Err(anyhow!("nvm is not available on this system"));
    }

    let shell = detect_shell();
    let output = build_nvm_command(shell, &format!("use {}", version)).output();

    match output {
        Ok(result) => {
            if result.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                let stdout = String::from_utf8_lossy(&result.stdout);
                let error_msg = if !stderr.is_empty() {
                    stderr.to_string()
                } else {
                    stdout.to_string()
                };
                Err(anyhow!(
                    "Failed to switch to version {}: {}",
                    version,
                    error_msg.trim()
                ))
            }
        }
        Err(e) => Err(anyhow!("Failed to execute nvm command: {}", e)),
    }
}

/// Check if a specific Node.js version is installed
#[allow(dead_code)]
pub fn is_version_installed(version: &str) -> Result<bool> {
    if !detect_nvm()? {
        return Ok(false);
    }

    let shell = detect_shell();
    let output = build_nvm_command(shell, "list").output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let output_str = String::from_utf8_lossy(&result.stdout);
                Ok(output_str.contains(version))
            } else {
                Ok(false)
            }
        }
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_shell() {
        // Just ensure it doesn't panic and returns a valid shell type
        let shell = detect_shell();
        assert!(matches!(
            shell,
            ShellType::Bash | ShellType::Zsh | ShellType::Fish | ShellType::PowerShell
        ));
    }

    #[test]
    fn test_detect_nvm() {
        // This test will vary based on system configuration
        // Just ensure it doesn't panic
        let _result = detect_nvm();
    }

    #[test]
    fn test_get_current_version_format() {
        // Test that if we can get a version, it has the expected format
        if let Ok(version) = get_current_version() {
            // Should start with 'v' followed by a number
            assert!(version.starts_with('v') || version.chars().next().unwrap().is_ascii_digit());
        }
    }
}
