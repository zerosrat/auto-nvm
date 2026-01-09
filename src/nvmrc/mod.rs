use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Find .nvmrc file in the current working directory
pub fn find_nvmrc_current_dir() -> Result<Option<PathBuf>> {
    let current_dir = std::env::current_dir()?;
    let nvmrc_path = current_dir.join(".nvmrc");

    if nvmrc_path.exists() && nvmrc_path.is_file() {
        Ok(Some(nvmrc_path))
    } else {
        Ok(None)
    }
}

/// Parse version specification from .nvmrc file
pub fn parse_nvmrc(nvmrc_path: &Path) -> Result<String> {
    let content = fs::read_to_string(nvmrc_path)
        .map_err(|e| anyhow!("Failed to read .nvmrc file: {}", e))?;

    // Trim whitespace and remove any comments
    let version = content
        .lines()
        .next() // Take first line only
        .unwrap_or("")
        .trim()
        .to_string();

    if version.is_empty() {
        return Err(anyhow!(".nvmrc file is empty or contains only whitespace"));
    }

    Ok(version)
}

/// Validate version specification format
pub fn validate_version(version: &str) -> Result<()> {
    if version.is_empty() {
        return Err(anyhow!("Version cannot be empty"));
    }

    // Basic validation - allow common formats:
    // - Exact versions: 18.17.0, v18.17.0
    // - Major versions: 18, v18
    // - Special keywords: lts, node, stable
    let cleaned_version = version.trim_start_matches('v');

    // Check for special keywords
    if matches!(cleaned_version, "lts" | "node" | "stable" | "latest") {
        return Ok(());
    }

    // Check for semantic version format (major.minor.patch or just major)
    if cleaned_version.chars().all(|c| c.is_ascii_digit() || c == '.') {
        let parts: Vec<&str> = cleaned_version.split('.').collect();
        if parts.len() >= 1 && parts.len() <= 3 {
            // Validate each part is a number
            for part in parts {
                if part.is_empty() || !part.chars().all(|c| c.is_ascii_digit()) {
                    return Err(anyhow!("Invalid version format: {}", version));
                }
            }
            return Ok(());
        }
    }

    Err(anyhow!("Invalid version format: {}. Expected formats: 18.17.0, v18, lts, etc.", version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_validate_version() {
        // Valid versions
        assert!(validate_version("18.17.0").is_ok());
        assert!(validate_version("v18.17.0").is_ok());
        assert!(validate_version("18").is_ok());
        assert!(validate_version("v18").is_ok());
        assert!(validate_version("lts").is_ok());
        assert!(validate_version("node").is_ok());
        assert!(validate_version("stable").is_ok());
        assert!(validate_version("latest").is_ok());

        // Invalid versions
        assert!(validate_version("").is_err());
        assert!(validate_version("invalid").is_err());
        assert!(validate_version("18.17.0.1").is_err());
        assert!(validate_version("18.").is_err());
        assert!(validate_version(".17").is_err());
    }

    #[test]
    fn test_parse_nvmrc() -> Result<()> {
        let dir = tempdir()?;
        let nvmrc_path = dir.path().join(".nvmrc");

        // Test normal version
        let mut file = fs::File::create(&nvmrc_path)?;
        writeln!(file, "18.17.0")?;
        drop(file);

        let version = parse_nvmrc(&nvmrc_path)?;
        assert_eq!(version, "18.17.0");

        // Test version with whitespace
        let mut file = fs::File::create(&nvmrc_path)?;
        writeln!(file, "  v18  ")?;
        drop(file);

        let version = parse_nvmrc(&nvmrc_path)?;
        assert_eq!(version, "v18");

        // Test empty file
        fs::write(&nvmrc_path, "")?;
        assert!(parse_nvmrc(&nvmrc_path).is_err());

        Ok(())
    }
}