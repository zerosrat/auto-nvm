use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Information about a PATH entry found in a shell configuration file
#[derive(Debug, Clone)]
pub struct PathEntry {
    pub file_path: PathBuf,
    pub line_number: usize,
    #[allow(dead_code)]
    pub line_content: String,
    #[allow(dead_code)]
    pub install_dir: String,
}

/// Remove PATH entries related to auto-nvm from shell configuration files
pub fn remove_path_entries() -> Result<()> {
    // Skip Windows entirely as per requirements
    if cfg!(windows) {
        return Ok(());
    }

    let path_entries = find_path_entries()?;

    if path_entries.is_empty() {
        return Ok(());
    }

    // Group entries by file for efficient processing
    let mut files_to_process: std::collections::HashMap<PathBuf, Vec<PathEntry>> =
        std::collections::HashMap::new();

    for entry in path_entries {
        files_to_process
            .entry(entry.file_path.clone())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    // Process each file
    for (file_path, entries) in files_to_process {
        remove_path_entries_from_file(&file_path, &entries)?;
    }

    Ok(())
}

/// Find PATH entries in shell configuration files that reference auto-nvm installation directories
pub fn find_path_entries() -> Result<Vec<PathEntry>> {
    let mut entries = Vec::new();

    // Get possible installation directories
    let install_dirs = get_possible_install_dirs()?;

    // Get shell configuration files to check
    let config_files = get_shell_config_files()?;

    for config_file in config_files {
        if !config_file.exists() {
            continue;
        }

        let file_entries = find_path_entries_in_file(&config_file, &install_dirs)?;
        entries.extend(file_entries);
    }

    Ok(entries)
}

/// Get possible auto-nvm installation directories
fn get_possible_install_dirs() -> Result<Vec<String>> {
    let mut dirs = Vec::new();

    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    // Default installation directory
    dirs.push(home_dir.join(".local/bin").to_string_lossy().to_string());

    // Check AUTO_NVM_INSTALL_DIR environment variable
    if let Ok(custom_dir) = env::var("AUTO_NVM_INSTALL_DIR") {
        dirs.push(custom_dir);
    }

    // Other common directories
    dirs.push(home_dir.join("bin").to_string_lossy().to_string());
    dirs.push(home_dir.join(".bin").to_string_lossy().to_string());

    Ok(dirs)
}

/// Get shell configuration files to check for PATH entries
fn get_shell_config_files() -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    // Bash files
    files.push(home_dir.join(".bashrc"));
    files.push(home_dir.join(".bash_profile"));
    files.push(home_dir.join(".profile"));

    // Zsh files
    files.push(home_dir.join(".zshrc"));
    files.push(home_dir.join(".zprofile"));

    // Fish files
    files.push(home_dir.join(".config/fish/config.fish"));

    Ok(files)
}

/// Find PATH entries in a specific file that reference the given installation directories
fn find_path_entries_in_file(file_path: &Path, install_dirs: &[String]) -> Result<Vec<PathEntry>> {
    let mut entries = Vec::new();

    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return Ok(entries), // File doesn't exist or can't be read
    };

    for (line_number, line) in content.lines().enumerate() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Look for PATH export statements
        if is_path_export_line(line) {
            // Check if this line references any of our installation directories
            for install_dir in install_dirs {
                if line.contains(install_dir) {
                    entries.push(PathEntry {
                        file_path: file_path.to_path_buf(),
                        line_number: line_number + 1, // 1-indexed
                        line_content: line.to_string(),
                        install_dir: install_dir.clone(),
                    });
                    break; // Only add each line once
                }
            }
        }
    }

    Ok(entries)
}

/// Check if a line is a PATH export statement
fn is_path_export_line(line: &str) -> bool {
    let line = line.trim();

    // Skip comments
    if line.starts_with('#') {
        return false;
    }

    // Common PATH export patterns
    let patterns = [
        "export PATH=",
        "PATH=",
        "set -gx PATH", // Fish shell
        "set PATH",     // Fish shell alternative
    ];

    for pattern in &patterns {
        if line.contains(pattern) {
            return true;
        }
    }

    false
}

/// Remove PATH entries from a specific file
fn remove_path_entries_from_file(file_path: &Path, entries: &[PathEntry]) -> Result<()> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file {}", file_path.display()))?;

    let mut lines: Vec<&str> = content.lines().collect();

    // Sort entries by line number in descending order to avoid index shifting
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|a, b| b.line_number.cmp(&a.line_number));

    // Remove lines (in reverse order to maintain correct indices)
    for entry in sorted_entries {
        if entry.line_number <= lines.len() {
            lines.remove(entry.line_number - 1); // Convert to 0-indexed
        }
    }

    // Write the modified content back
    let new_content = lines.join("\n");
    fs::write(file_path, new_content)
        .with_context(|| format!("Failed to write file {}", file_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_is_path_export_line() {
        assert!(is_path_export_line(
            "export PATH=\"$HOME/.local/bin:$PATH\""
        ));
        assert!(is_path_export_line("PATH=\"$HOME/.local/bin:$PATH\""));
        assert!(is_path_export_line("set -gx PATH $HOME/.local/bin $PATH"));
        assert!(is_path_export_line("set PATH $HOME/.local/bin $PATH"));

        assert!(!is_path_export_line(
            "# export PATH=\"$HOME/.local/bin:$PATH\""
        ));
        assert!(!is_path_export_line("echo $PATH"));
        assert!(!is_path_export_line(""));
    }

    #[test]
    fn test_find_path_entries_in_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"# My shell config
export PATH="$HOME/.local/bin:$PATH"
alias ll='ls -la'
PATH="$HOME/bin:$PATH"
# Another comment
export PATH="$HOME/.local/bin:$PATH"
"#;
        temp_file.write_all(content.as_bytes()).unwrap();

        let install_dirs = vec!["$HOME/.local/bin".to_string()];
        let entries = find_path_entries_in_file(temp_file.path(), &install_dirs).unwrap();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].line_number, 2);
        assert_eq!(entries[1].line_number, 6);
    }

    #[test]
    fn test_remove_path_entries_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"# My shell config
export PATH="$HOME/.local/bin:$PATH"
alias ll='ls -la'
PATH="$HOME/bin:$PATH"
# Another comment
"#;
        temp_file.write_all(content.as_bytes()).unwrap();

        let entries = vec![PathEntry {
            file_path: temp_file.path().to_path_buf(),
            line_number: 2,
            line_content: "export PATH=\"$HOME/.local/bin:$PATH\"".to_string(),
            install_dir: "$HOME/.local/bin".to_string(),
        }];

        remove_path_entries_from_file(temp_file.path(), &entries).unwrap();

        let new_content = fs::read_to_string(temp_file.path()).unwrap();
        assert!(!new_content.contains("export PATH=\"$HOME/.local/bin:$PATH\""));
        assert!(new_content.contains("alias ll='ls -la'"));
        assert!(new_content.contains("PATH=\"$HOME/bin:$PATH\""));
    }
}
