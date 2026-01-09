use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod nvm;
mod nvmrc;

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
    /// Setup shell integration (placeholder for Phase 2)
    Setup,
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
    // TODO: Implement shell setup (placeholder for Phase 2)
    println!("Setup functionality will be implemented in Phase 2");
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
