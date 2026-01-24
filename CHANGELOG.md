# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Phase 4 distribution infrastructure
- Universal install scripts for Unix platforms
- Comprehensive CI/CD pipeline with GitHub Actions
- Cross-platform binary releases (Linux/macOS)
- Complete documentation suite
- cargo-binstall support metadata

### Changed
- Updated Cargo.toml with complete package metadata
- Version bumped to 0.1.1 for distribution release

## [0.1.0] - 2026-01-10

### Added
- Initial release with core functionality (Phase 1 & 2 complete)
- Cross-platform Node.js version auto-switcher
- Multi-shell support (Bash, Zsh, Fish)
- Automatic .nvmrc detection and version switching
- Shell integration with `cd` command wrapping
- Manual commands: `check`, `switch`, `setup`, `uninstall`
- Comprehensive test suite (37+ tests)
- Quiet mode for performance optimization
- Error handling and user-friendly messages

### Core Modules
- `src/main.rs` - CLI entry point with clap integration
- `src/config/mod.rs` - Configuration management
- `src/nvmrc/mod.rs` - .nvmrc file detection and parsing
- `src/nvm/mod.rs` - NVM command abstraction layer
- `src/shell/mod.rs` - Shell integration and configuration

### Shell Integration
- `shell-integration/bash/auto-nvm.bash` - Bash support
- `shell-integration/zsh/auto-nvm.zsh` - Zsh support
- `shell-integration/fish/auto-nvm.fish` - Fish support

### Testing
- Unit tests for all core modules
- Integration tests for CLI commands
- Shell detection and configuration tests
- Cross-platform compatibility verification

### Performance
- < 500ms switching delay target achieved
- Embedded shell scripts for zero external dependencies
- Optimized file system operations
- Minimal memory footprint

## [0.0.1] - 2026-01-08

### Added
- Project initialization
- Basic Cargo project structure
- Initial dependency configuration
- Development environment setup

---

## Release Notes

### Version 0.1.0 - Initial Release

This is the first functional release of auto-nvm, providing a complete Node.js version auto-switching solution built with Rust.

**Key Features:**
- ✅ **Automatic Switching**: Detects `.nvmrc` files and switches Node.js versions automatically
- ✅ **Multi-Shell Support**: Works with Bash, Zsh, and Fish
- ✅ **Unix Platforms**: Supports Linux and macOS
- ✅ **Fast Performance**: Built in Rust for speed and reliability
- ✅ **Easy Setup**: One command configures shell integration
- ✅ **Safe Operations**: Comprehensive error handling and rollback

**Installation:**
```bash
# Manual installation required for v0.1.0
cargo install --git https://github.com/zerosrat/auto-nvm.git
auto-nvm setup
```

**Usage:**
```bash
# Check current directory
auto-nvm check

# Manual switch
auto-nvm switch

# Configure shell integration
auto-nvm setup

# Remove shell integration
auto-nvm uninstall
```

### Version 0.1.1 - Distribution Release (Upcoming)

**New Features:**
- 🚀 **One-Command Installation**: Universal install scripts for all platforms
- 📦 **Pre-Compiled Binaries**: GitHub Releases with Unix binaries
- 🔄 **Automated CI/CD**: Complete testing and release pipeline
- 📚 **Complete Documentation**: Comprehensive guides and troubleshooting
- 🛠️ **cargo-binstall Support**: Fast binary installation for Rust users

**Installation (Coming Soon):**
```bash
# Unix (Linux/macOS)
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

---

## Upgrade Guide

### From 0.1.0 to 0.1.1

No breaking changes. The new version adds distribution infrastructure without changing core functionality.

**Upgrade Steps:**
1. Download new binary or use install script
2. Existing shell integration will continue to work
3. No configuration changes needed

### Future Upgrades

This project follows semantic versioning:
- **Patch releases (0.1.x)**: Bug fixes, no breaking changes
- **Minor releases (0.x.0)**: New features, backward compatible
- **Major releases (x.0.0)**: Breaking changes, migration required

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.