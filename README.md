# Auto-NVM

[![CI](https://github.com/zerosrat/auto-nvm/workflows/CI/badge.svg)](https://github.com/zerosrat/auto-nvm/actions)
[![Release](https://github.com/zerosrat/auto-nvm/workflows/Release/badge.svg)](https://github.com/zerosrat/auto-nvm/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Unix Node.js version auto-switcher with multi-shell support**

Auto-NVM automatically switches Node.js versions when you `cd` into directories containing `.nvmrc` files. Built with Rust for speed and reliability, supporting Bash, Zsh, and Fish across Linux and macOS.

## ✨ Features

- 🚀 **Automatic version switching** - No manual intervention needed
- 🔧 **Multi-shell support** - Works with Bash, Zsh, and Fish
- 🌍 **Unix platforms** - Linux and macOS
- ⚡ **Fast performance** - Built in Rust, < 500ms switching delay
- 🎯 **Zero configuration** - Works out of the box after setup
- 🛡️ **Safe and reliable** - Comprehensive error handling and rollback
- 📦 **Easy installation** - One command installs and configures everything

## 🚀 Quick Start

### One-Command Installation

**Unix (Linux/macOS):**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

That's it! The install script will:
1. Download the appropriate binary for your platform
2. Install it to your PATH
3. Automatically configure shell integration
4. Set up automatic version switching

### Verify Installation

```bash
auto-nvm --version
```

Test automatic switching:
```bash
echo "18.17.0" > .nvmrc
cd .  # Triggers automatic switch
node --version  # Should show v18.17.0
```

## 📖 How It Works

1. **Create a `.nvmrc` file** in your project directory:
   ```bash
   echo "18.17.0" > .nvmrc
   ```

2. **Navigate to the directory** - auto-nvm automatically detects and switches:
   ```bash
   cd my-project/  # Automatically switches to Node.js 18.17.0
   ```

3. **Supported version formats**:
   - Specific versions: `18.17.0`, `16.20.1`
   - Semantic versions: `18`, `16.20`
   - Aliases: `lts`, `stable`, `latest`

## 🔧 Installation Methods

### 1. Universal Install Script (Recommended)

**Features:**
- ✅ Automatic platform detection
- ✅ Binary installation + shell configuration
- ✅ PATH setup
- ✅ Works on all supported platforms

**Unix:**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

### 2. Build from Source

```bash
git clone https://github.com/zerosrat/auto-nvm.git
cd auto-nvm
cargo build --release
cp target/release/auto-nvm ~/.local/bin/
auto-nvm setup
```

## 🐚 Shell Integration

Auto-NVM integrates with your shell by wrapping the `cd` command. After installation, it works automatically in:

### Bash
Integration added to `~/.bashrc` or `~/.bash_profile`

### Zsh
Integration added to `~/.zshrc`

### Fish
Integration added to `~/.config/fish/config.fish`


### Manual Shell Setup

If automatic setup fails, you can configure manually:

```bash
auto-nvm setup  # Configure for current shell
```

Or specify shell explicitly:
```bash
SHELL=/bin/zsh auto-nvm setup  # Configure for Zsh
```

## 📋 Commands

### `auto-nvm check`
Check current directory for `.nvmrc` and show version info:
```bash
auto-nvm check
# Output: Found .nvmrc with version: 18.17.0
#         Current Node.js version: v16.20.1
#         Switch needed: yes
```

### `auto-nvm switch`
Manually switch to `.nvmrc` version:
```bash
auto-nvm switch
# Output: Switched to Node.js v18.17.0
```

### `auto-nvm setup`
Configure shell integration:
```bash
auto-nvm setup
# Output: Shell integration configured for Zsh
#         Added auto-nvm configuration to ~/.zshrc
```

### `auto-nvm uninstall`
Remove shell integration (keeps binary):
```bash
auto-nvm uninstall
# Output: Removed auto-nvm integration from ~/.zshrc
```


## 🔍 Troubleshooting

### Auto-switching not working
1. **Verify installation**: `auto-nvm --version`
2. **Check shell integration**: Look for auto-nvm configuration in your shell RC file
3. **Re-run setup**: `auto-nvm setup`
4. **Restart shell**: Open new terminal or `source ~/.bashrc`

### Command not found
1. **Check PATH**: `echo $PATH` should include auto-nvm install directory
2. **Manual PATH setup**: Add `export PATH="$HOME/.local/bin:$PATH"` to shell RC file
3. **Reinstall**: Use the install script again

### Permission errors
1. **Install to user directory**: Use `AUTO_NVM_INSTALL_DIR=~/.local/bin`
2. **Check permissions**: Ensure install directory is writable
3. **Use sudo** (not recommended): Only for system-wide installation

### Version not switching
1. **Check .nvmrc format**: Should contain only version number
2. **Verify NVM installation**: `nvm --version`
3. **Check version exists**: `nvm ls-remote | grep <version>`

---

**Made with 🐟 and Rust**