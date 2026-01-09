use anyhow::{anyhow, Result};
use std::process::Command;
use which::which;

/// Detect if nvm is available on the system
pub fn detect_nvm() -> Result<bool> {
    // First check if nvm command is available
    if which("nvm").is_ok() {
        return Ok(true);
    }

    // Check if NVM_DIR environment variable is set (common nvm installation indicator)
    if std::env::var("NVM_DIR").is_ok() {
        return Ok(true);
    }

    // Try to source nvm and check if it's available
    let output = Command::new("bash")
        .arg("-c")
        .arg("source ~/.nvm/nvm.sh && type nvm")
        .output();

    match output {
        Ok(result) => Ok(result.status.success()),
        Err(_) => Ok(false),
    }
}

/// Get the currently active Node.js version
pub fn get_current_version() -> Result<String> {
    // Try to get version using node directly first
    let output = Command::new("node")
        .arg("--version")
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let version = String::from_utf8_lossy(&result.stdout)
                    .trim()
                    .to_string();
                return Ok(version);
            }
        }
        Err(_) => {}
    }

    // If node is not available, try using nvm current
    let output = Command::new("bash")
        .arg("-c")
        .arg("source ~/.nvm/nvm.sh && nvm current")
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let version = String::from_utf8_lossy(&result.stdout)
                    .trim()
                    .to_string();
                Ok(version)
            } else {
                Err(anyhow!("No Node.js version currently active"))
            }
        }
        Err(e) => Err(anyhow!("Failed to get current Node.js version: {}", e)),
    }
}

/// Switch to the specified Node.js version using nvm
pub fn switch_version(version: &str) -> Result<()> {
    if !detect_nvm()? {
        return Err(anyhow!("nvm is not available on this system"));
    }

    // Use nvm to switch to the specified version
    let command = format!("source ~/.nvm/nvm.sh && nvm use {}", version);
    let output = Command::new("bash")
        .arg("-c")
        .arg(&command)
        .output();

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
                Err(anyhow!("Failed to switch to version {}: {}", version, error_msg.trim()))
            }
        }
        Err(e) => Err(anyhow!("Failed to execute nvm command: {}", e)),
    }
}

/// Check if a specific Node.js version is installed
pub fn is_version_installed(version: &str) -> Result<bool> {
    if !detect_nvm()? {
        return Ok(false);
    }

    let command = format!("source ~/.nvm/nvm.sh && nvm list | grep -q '{}'", version);
    let output = Command::new("bash")
        .arg("-c")
        .arg(&command)
        .output();

    match output {
        Ok(result) => Ok(result.status.success()),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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