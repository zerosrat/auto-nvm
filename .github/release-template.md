# Auto-NVM v{version}

**Unix Node.js version auto-switcher with multi-shell support**

## 🚀 One-Command Installation

### Unix (Linux/macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

The install script automatically:
- Downloads the appropriate binary for your platform
- Installs to your PATH
- Configures shell integration
- Sets up automatic version switching

## 📦 Manual Installation

Download the appropriate binary for your platform below, extract it, and follow the [installation guide](docs/installation.md).

## ✨ What's New in v{version}

<!-- This section will be populated based on CHANGELOG.md -->

## 🔧 Features

- 🚀 **Automatic version switching** when you `cd` into directories with `.nvmrc` files
- 🔧 **Multi-shell support** - Works with Bash, Zsh, and Fish
- 🌍 **Unix platforms** - Linux and macOS
- ⚡ **Fast performance** - Built in Rust, < 500ms switching delay
- 🎯 **Zero configuration** - Works out of the box after setup
- 🛡️ **Safe and reliable** - Comprehensive error handling

## 📋 Quick Usage

```bash
# The install script sets everything up automatically, but here are the manual commands:

# Check current directory for .nvmrc
auto-nvm check

# Switch to .nvmrc version manually
auto-nvm switch

# Configure shell integration (done automatically by install script)
auto-nvm setup

# Remove shell integration
auto-nvm uninstall
```

## 🔍 Verification

After installation, test that auto-nvm is working:

```bash
# Create a test .nvmrc file
echo "18.17.0" > .nvmrc

# Navigate to trigger auto-switching
cd .

# Verify Node.js version
node --version  # Should show v18.17.0
```

## 📚 Documentation

- [Installation Guide](docs/installation.md) - Detailed installation instructions
- [Usage Guide](docs/usage.md) - Complete usage documentation
- [Troubleshooting](docs/installation.md#troubleshooting) - Common issues and solutions
- [Contributing](CONTRIBUTING.md) - Development and contribution guide

## 🔗 Platform Support

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64 | ✅ Supported |
| Linux | aarch64 (ARM64) | ✅ Supported |
| macOS | x86_64 (Intel) | ✅ Supported |
| macOS | aarch64 (Apple Silicon) | ✅ Supported |

## 🐚 Shell Support

| Shell | Linux | macOS | Status |
|-------|-------|-------|--------|
| Bash | ✅ | ✅ | Supported |
| Zsh | ✅ | ✅ | Supported |
| Fish | ✅ | ✅ | Supported |

## 📁 Asset Checksums

All release assets include SHA256 checksums for verification. Download `checksums.txt` to verify the integrity of downloaded files:

```bash
# Verify download (Linux/macOS)
sha256sum -c checksums.txt
```

## 🐛 Issues and Support

- **Bug Reports**: [GitHub Issues](https://github.com/zerosrat/auto-nvm/issues)
- **Feature Requests**: [GitHub Discussions](https://github.com/zerosrat/auto-nvm/discussions)
- **Documentation**: [Installation Guide](docs/installation.md) and [Usage Guide](docs/usage.md)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Made with 🐟 and Rust**

For more information, visit the [project repository](https://github.com/zerosrat/auto-nvm).