use anyhow::{anyhow, Context, Result};
use std::io::{self, Write};
use crate::config::Config;

pub mod binary;
pub mod path;

/// Complete uninstall manager that coordinates removal of all auto-nvm components
pub struct UninstallManager {
    config: Config,
}

impl UninstallManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Execute complete uninstall with user confirmation
    pub fn execute(&self) -> Result<()> {
        if !self.config.is_quiet() {
            println!("Auto-NVM Complete Uninstall");
            println!("===========================");
            println!();
            println!("This will remove:");
            println!("  • Binary file (auto-nvm executable)");
            println!("  • PATH entries from shell configuration files");
            println!("  • Shell integration code");
            println!();
            println!("Note: Backup files (.backup) will be preserved for safety.");
            println!();
        }

        // Ask for confirmation
        if !self.confirm_uninstall()? {
            if !self.config.is_quiet() {
                println!("Uninstall cancelled.");
            }
            return Ok(());
        }

        if !self.config.is_quiet() {
            println!("Proceeding with complete uninstall...");
            println!();
        }

        let mut success_count = 0;
        let mut total_count = 0;

        // Remove shell integration
        total_count += 1;
        if let Err(e) = self.remove_shell_integration() {
            if !self.config.is_quiet() {
                println!("⚠ Failed to remove shell integration: {}", e);
            }
        } else {
            success_count += 1;
            if !self.config.is_quiet() {
                println!("✓ Removed shell integration");
            }
        }

        // Remove binary
        total_count += 1;
        if let Err(e) = self.remove_binary() {
            if !self.config.is_quiet() {
                println!("⚠ Failed to remove binary: {}", e);
            }
        } else {
            success_count += 1;
            if !self.config.is_quiet() {
                println!("✓ Removed binary file");
            }
        }

        // Remove PATH entries
        total_count += 1;
        if let Err(e) = self.remove_path_entries() {
            if !self.config.is_quiet() {
                println!("⚠ Failed to remove PATH entries: {}", e);
            }
        } else {
            success_count += 1;
            if !self.config.is_quiet() {
                println!("✓ Removed PATH entries");
            }
        }

        if !self.config.is_quiet() {
            println!();
            if success_count == total_count {
                println!("🎉 Complete uninstall successful!");
                println!();
                println!("Auto-NVM has been completely removed from your system.");
                println!("You may need to restart your shell or source your shell configuration file.");
            } else {
                println!("⚠ Partial uninstall completed ({}/{} operations successful)", success_count, total_count);
                println!();
                println!("Some components could not be removed. You may need to remove them manually.");
            }
        }

        Ok(())
    }

    /// Ask user for confirmation
    fn confirm_uninstall(&self) -> Result<bool> {
        if self.config.is_quiet() {
            return Ok(true);
        }

        print!("Do you want to proceed with the complete uninstall? [y/N]: ");
        io::stdout().flush().context("Failed to flush stdout")?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).context("Failed to read user input")?;

        let input = input.trim().to_lowercase();
        Ok(input == "y" || input == "yes")
    }

    /// Remove shell integration using existing logic
    fn remove_shell_integration(&self) -> Result<()> {
        // Detect user's current shell
        let shell = crate::nvm::detect_shell();

        // Get the config file path for this shell
        let config_path = crate::shell::get_config_file_path(shell)
            .context("Could not determine shell config file path")?;

        // Check if auto-nvm is configured
        if !crate::shell::check_already_configured(&config_path) {
            return Ok(()); // Nothing to remove
        }

        // Remove the integration
        let removed = crate::shell::remove_integration_from_config(&config_path)
            .context("Failed to remove auto-nvm integration")?;

        if !removed {
            return Err(anyhow!("No auto-nvm configuration found to remove"));
        }

        Ok(())
    }

    /// Remove binary file
    fn remove_binary(&self) -> Result<()> {
        binary::remove_auto_nvm_binary()
    }

    /// Remove PATH entries from shell configuration files
    fn remove_path_entries(&self) -> Result<()> {
        path::remove_path_entries()
    }
}