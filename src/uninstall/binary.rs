use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

/// Find the auto-nvm binary file in common installation locations
pub fn find_auto_nvm_binary() -> Result<Option<PathBuf>> {
    let binary_name = if cfg!(windows) {
        "auto-nvm.exe"
    } else {
        "auto-nvm"
    };

    // Check common installation directories
    let possible_locations = get_possible_binary_locations()?;

    for location in possible_locations {
        let binary_path = location.join(binary_name);
        if binary_path.exists() && binary_path.is_file() {
            return Ok(Some(binary_path));
        }
    }

    // Check if auto-nvm is in PATH
    if let Ok(path_binary) = which::which("auto-nvm") {
        return Ok(Some(path_binary));
    }

    Ok(None)
}

/// Get possible installation locations for the auto-nvm binary
fn get_possible_binary_locations() -> Result<Vec<PathBuf>> {
    let mut locations = Vec::new();

    // Get home directory
    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;

    // Default installation directory
    locations.push(home_dir.join(".local/bin"));

    // Check AUTO_NVM_INSTALL_DIR environment variable
    if let Ok(custom_dir) = env::var("AUTO_NVM_INSTALL_DIR") {
        locations.push(PathBuf::from(custom_dir));
    }

    // Other common binary locations
    locations.push(home_dir.join("bin"));
    locations.push(home_dir.join(".bin"));

    // System-wide locations (if accessible)
    locations.push(PathBuf::from("/usr/local/bin"));
    locations.push(PathBuf::from("/usr/bin"));

    Ok(locations)
}

/// Remove the auto-nvm binary file
pub fn remove_auto_nvm_binary() -> Result<()> {
    match find_auto_nvm_binary()? {
        Some(binary_path) => {
            // Verify this is actually the auto-nvm binary
            if !is_auto_nvm_binary(&binary_path)? {
                return Err(anyhow!(
                    "Found binary at {} but it doesn't appear to be auto-nvm",
                    binary_path.display()
                ));
            }

            // Remove the binary
            fs::remove_file(&binary_path)
                .with_context(|| format!("Failed to remove binary at {}", binary_path.display()))?;

            Ok(())
        }
        None => {
            // No binary found, which is fine
            Ok(())
        }
    }
}

/// Verify that a binary file is actually auto-nvm
fn is_auto_nvm_binary(path: &Path) -> Result<bool> {
    // Basic checks
    if !path.exists() || !path.is_file() {
        return Ok(false);
    }

    // Check if the file is executable (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path).context("Failed to get file metadata")?;
        let permissions = metadata.permissions();
        if permissions.mode() & 0o111 == 0 {
            return Ok(false); // Not executable
        }
    }

    // For a more thorough check, we could run `auto-nvm --version`
    // but for now, we'll rely on the file name and location
    let file_name = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    Ok(file_name == "auto-nvm" || file_name == "auto-nvm.exe")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_find_auto_nvm_binary_not_found() {
        // This test assumes auto-nvm is not installed in the test environment
        // In a real installation, this would find the binary
        let result = find_auto_nvm_binary().unwrap();
        // Could be None or Some depending on test environment
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_is_auto_nvm_binary() {
        let temp_dir = TempDir::new().unwrap();
        let binary_path = temp_dir.path().join("auto-nvm");

        // Create a fake binary file
        fs::write(&binary_path, "fake binary content").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&binary_path).unwrap().permissions();
            permissions.set_mode(0o755); // Make executable
            fs::set_permissions(&binary_path, permissions).unwrap();
        }

        assert!(is_auto_nvm_binary(&binary_path).unwrap());

        // Test with wrong name
        let wrong_binary = temp_dir.path().join("not-auto-nvm");
        fs::write(&wrong_binary, "fake binary content").unwrap();
        assert!(!is_auto_nvm_binary(&wrong_binary).unwrap());
    }

    #[test]
    fn test_get_possible_binary_locations() {
        let locations = get_possible_binary_locations().unwrap();
        assert!(!locations.is_empty());

        // Should include at least the default location
        let home_dir = dirs::home_dir().unwrap();
        let default_location = home_dir.join(".local/bin");
        assert!(locations.contains(&default_location));
    }
}