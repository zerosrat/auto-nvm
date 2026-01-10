/// Basic configuration for auto-nvm
#[derive(Debug, Clone)]
pub struct Config {
    /// Enable quiet mode (suppress non-error output)
    pub quiet: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            quiet: false,
        }
    }
}

impl Config {
    /// Create a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create config from command-line arguments
    pub fn from_cli(quiet: bool) -> Self {
        Self {
            quiet,
        }
    }

    /// Check if quiet mode is enabled
    pub fn is_quiet(&self) -> bool {
        self.quiet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.quiet);
        assert!(!config.is_quiet());
    }

    #[test]
    fn test_config_from_cli() {
        let config = Config::from_cli(true);
        assert!(config.quiet);
        assert!(config.is_quiet());

        let config = Config::from_cli(false);
        assert!(!config.quiet);
        assert!(!config.is_quiet());
    }

    #[test]
    fn test_new_config() {
        let config = Config::new();
        assert!(!config.quiet);
    }
}