use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod config;
mod nvm;
mod nvmrc;
mod shell;

#[derive(Parser)]
#[command(name = "auto-nvm")]
#[command(about = "A cross-platform Node.js version auto-switcher")]
#[command(version)]
struct Cli {
    /// Enable quiet mode (suppress non-error output)
    #[arg(short, long)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check current directory for .nvmrc and show version info
    Check,
    /// Setup shell integration
    Setup,
    /// Uninstall/remove shell integration
    Uninstall,
    /// Execute version switching based on .nvmrc
    Switch {
        /// Print nvm command instead of executing (for use with eval)
        #[arg(short, long, default_value_t = true)]
        print: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::Config::from_cli(cli.quiet);

    match cli.command {
        Commands::Check => {
            handle_check(&config)?;
        }
        Commands::Setup => {
            handle_setup(&config)?;
        }
        Commands::Uninstall => {
            handle_uninstall(&config)?;
        }
        Commands::Switch { print } => {
            handle_switch(&config, print)?;
        }
    }

    Ok(())
}

fn handle_check(config: &config::Config) -> Result<()> {
    if !config.is_quiet() {
        println!("Checking for .nvmrc in current directory...");
    }

    match nvmrc::find_nvmrc_current_dir()? {
        Some(nvmrc_path) => {
            let required_version = nvmrc::parse_nvmrc(&nvmrc_path)?;
            nvmrc::validate_version(&required_version)?;

            if !config.is_quiet() {
                println!("Found .nvmrc with version: {}", required_version);
                println!("Path: {}", nvmrc_path.display());

                // Try to get current Node.js version
                match nvm::get_current_version() {
                    Ok(current_version) => {
                        println!("Current Node.js version: {}", current_version);

                        // Simple version comparison (just string comparison for now)
                        let current_clean = current_version.trim_start_matches('v');
                        let required_clean = required_version.trim_start_matches('v');

                        if current_clean == required_clean ||
                           (required_version == "lts" || required_version == "stable" || required_version == "latest") {
                            println!("✓ Version matches requirement");
                        } else {
                            println!("✗ Version mismatch! Required: {}, Current: {}", required_version, current_version);
                        }
                    }
                    Err(_) => {
                        println!("⚠ No Node.js version currently active");
                    }
                }
            }
        }
        None => {
            if !config.is_quiet() {
                println!("No .nvmrc file found in current directory");

                // Still show current version if available
                if let Ok(current_version) = nvm::get_current_version() {
                    println!("Current Node.js version: {}", current_version);
                }
            }
        }
    }

    Ok(())
}

fn handle_setup(config: &config::Config) -> Result<()> {
    if !config.is_quiet() {
        println!("Setting up shell integration...");
    }

    // Detect user's current shell
    let shell = nvm::detect_shell();
    let shell_name = match shell {
        nvm::ShellType::Bash => "Bash",
        nvm::ShellType::Zsh => "Zsh",
        nvm::ShellType::Fish => "Fish",
        nvm::ShellType::PowerShell => "PowerShell",
    };

    if !config.is_quiet() {
        println!("Detected shell: {}", shell_name);
    }

    // Get the config file path for this shell
    let config_path = shell::get_config_file_path(shell)
        .context("Could not determine shell config file path")?;

    if !config.is_quiet() {
        println!("Config file: {}", config_path.display());
    }

    // Check if already configured
    if shell::check_already_configured(&config_path) {
        if !config.is_quiet() {
            println!("Auto-NVM is already configured in {}", config_path.display());
            println!("To reinstall, first run: auto-nvm uninstall");
        }
        return Ok(());
    }

    // Generate the integration script
    let integration_script = shell::generate_integration_script(shell);

    // Create backup of the config file
    let backup_path = shell::backup_config_file(&config_path)
        .context("Failed to create backup of config file")?;

    if !config.is_quiet() {
        println!("Backup created: {}", backup_path.display());
    }

    // Append the integration script to the config file
    shell::append_to_config_file(&config_path, &integration_script)
        .context("Failed to append integration script to config file")?;

    if !config.is_quiet() {
        println!();
        println!("Shell integration configured successfully!");
        println!();
        println!("To activate the changes:");
        println!("  - Restart your shell, or");
        println!("  - Run: source {}", config_path.display());
        println!();
        println!("To remove auto-nvm integration later, run:");
        println!("  auto-nvm uninstall");
    }

    Ok(())
}

fn handle_uninstall(config: &config::Config) -> Result<()> {
    if !config.is_quiet() {
        println!("Removing shell integration...");
    }

    // Detect user's current shell
    let shell = nvm::detect_shell();
    let shell_name = match shell {
        nvm::ShellType::Bash => "Bash",
        nvm::ShellType::Zsh => "Zsh",
        nvm::ShellType::Fish => "Fish",
        nvm::ShellType::PowerShell => "PowerShell",
    };

    if !config.is_quiet() {
        println!("Detected shell: {}", shell_name);
    }

    // Get the config file path for this shell
    let config_path = shell::get_config_file_path(shell)
        .context("Could not determine shell config file path")?;

    if !config.is_quiet() {
        println!("Config file: {}", config_path.display());
    }

    // Check if auto-nvm is configured
    if !shell::check_already_configured(&config_path) {
        if !config.is_quiet() {
            println!("Auto-NVM is not configured in {}", config_path.display());
        }
        return Ok(());
    }

    // Remove the integration
    let removed = shell::remove_integration_from_config(&config_path)
        .context("Failed to remove auto-nvm integration")?;

    if !config.is_quiet() {
        if removed {
            println!();
            println!("Auto-NVM integration removed successfully!");
            println!();
            println!("To apply the changes:");
            println!("  - Restart your shell, or");
            println!("  - Run: source {}", config_path.display());
        } else {
            println!("No auto-nvm configuration found to remove.");
        }
    }

    Ok(())
}

fn handle_switch(_config: &config::Config, _print: bool) -> Result<()> {
    // Find .nvmrc in current directory
    match nvmrc::find_nvmrc_current_dir()? {
        Some(nvmrc_path) => {
            let required_version = nvmrc::parse_nvmrc(&nvmrc_path)?;
            nvmrc::validate_version(&required_version)?;

            // Output nvm command for eval to execute in current shell
            // This is the only way to affect the parent shell's environment
            println!("nvm use {}", required_version);
        }
        None => {
            // Output to stderr so it doesn't interfere with eval
            eprintln!("No .nvmrc file found in current directory");
            std::process::exit(1);
        }
    }

    Ok(())
}
