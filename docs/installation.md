# Installation Guide

This guide provides detailed installation instructions for auto-nvm on all supported platforms.

## Table of Contents

- [Quick Installation](#quick-installation)
- [Installation Methods](#installation-methods)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Shell Configuration](#shell-configuration)
- [Verification](#verification)
- [Uninstallation](#uninstallation)
- [Troubleshooting](#troubleshooting)

## Quick Installation

### Unix (Linux/macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.ps1 | iex
```

The install script will automatically:
1. Detect your platform and architecture
2. Download the appropriate binary
3. Install to your PATH
4. Configure shell integration
5. Verify the installation

## Installation Methods

### 1. Universal Install Script (Recommended)

The install script is the easiest way to get auto-nvm up and running. It handles all the complexity of platform detection, binary installation, and shell configuration.

#### Features
- ✅ Automatic platform detection
- ✅ Binary download and installation
- ✅ PATH configuration
- ✅ Shell integration setup
- ✅ Installation verification
- ✅ Error handling and rollback

#### Unix Installation Options

**Standard installation:**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

**Custom install directory:**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | AUTO_NVM_INSTALL_DIR=/usr/local/bin bash
```

**Skip automatic shell setup:**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | AUTO_NVM_AUTO_SETUP=false bash
```

**Quiet installation:**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | AUTO_NVM_QUIET=true bash
```

**Force reinstallation:**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | AUTO_NVM_FORCE=true bash
```

#### Windows Installation Options

**Standard installation:**
```powershell
iwr -useb https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.ps1 | iex
```

**Custom install directory:**
```powershell
$env:AUTO_NVM_INSTALL_DIR="C:\tools\bin"
iwr -useb https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.ps1 | iex
```

**Skip automatic shell setup:**
```powershell
$env:AUTO_NVM_AUTO_SETUP="false"
iwr -useb https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.ps1 | iex
```

**PowerShell parameters:**
```powershell
# Download and run with parameters
iwr -useb https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.ps1 -OutFile install.ps1
.\install.ps1 -Force -InstallDir "C:\tools\bin"
```

### 2. Manual Binary Installation

If you prefer manual installation or the install script doesn't work for your environment:

#### Step 1: Download Binary

1. Go to [GitHub Releases](https://github.com/zerosrat/auto-nvm/releases)
2. Download the appropriate binary for your platform:
   - **Linux x64**: `auto-nvm-v{version}-x86_64-unknown-linux-gnu.tar.gz`
   - **Linux ARM64**: `auto-nvm-v{version}-aarch64-unknown-linux-gnu.tar.gz`
   - **macOS Intel**: `auto-nvm-v{version}-x86_64-apple-darwin.tar.gz`
   - **macOS Apple Silicon**: `auto-nvm-v{version}-aarch64-apple-darwin.tar.gz`
   - **Windows**: `auto-nvm-v{version}-x86_64-pc-windows-msvc.zip`

#### Step 2: Extract and Install

**Unix (Linux/macOS):**
```bash
# Extract archive
tar -xzf auto-nvm-v{version}-{platform}.tar.gz

# Move binary to PATH
mkdir -p ~/.local/bin
mv auto-nvm ~/.local/bin/

# Make executable
chmod +x ~/.local/bin/auto-nvm

# Add to PATH (if not already)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows:**
```powershell
# Extract archive (using built-in PowerShell)
Expand-Archive -Path auto-nvm-v{version}-x86_64-pc-windows-msvc.zip -DestinationPath .

# Create bin directory
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.local\bin"

# Move binary
Move-Item auto-nvm.exe "$env:USERPROFILE\.local\bin\"

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
[Environment]::SetEnvironmentVariable("PATH", "$env:USERPROFILE\.local\bin;$userPath", "User")

# Refresh current session
$env:PATH = "$env:USERPROFILE\.local\bin;$env:PATH"
```

#### Step 3: Configure Shell Integration

```bash
auto-nvm setup
```

### 3. Build from Source

For developers or if pre-built binaries aren't available for your platform:

#### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- Git

#### Build Steps

```bash
# Clone repository
git clone https://github.com/zerosrat/auto-nvm.git
cd auto-nvm

# Build release binary
cargo build --release

# Install binary
mkdir -p ~/.local/bin
cp target/release/auto-nvm ~/.local/bin/

# Add to PATH (if needed)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Configure shell integration
auto-nvm setup
```

## Platform-Specific Instructions

### Linux

#### Supported Distributions
- Ubuntu 18.04+
- Debian 10+
- CentOS 7+
- Fedora 30+
- Arch Linux
- Alpine Linux

#### Dependencies
- `curl` or `wget` (for install script)
- `tar` (for extracting archives)
- A supported shell (bash, zsh, fish)

#### Installation Locations
- **User install** (default): `~/.local/bin/auto-nvm`
- **System install**: `/usr/local/bin/auto-nvm` (requires sudo)

#### Shell Configuration Files
- **Bash**: `~/.bashrc` or `~/.bash_profile`
- **Zsh**: `~/.zshrc`
- **Fish**: `~/.config/fish/config.fish`

### macOS

#### Supported Versions
- macOS 10.15 (Catalina) or later
- Both Intel and Apple Silicon Macs

#### Dependencies
- `curl` (pre-installed on macOS)
- A supported shell (bash, zsh, fish)

#### Installation via Homebrew (Future)
```bash
# Coming soon
brew install auto-nvm
auto-nvm setup
```

#### Shell Configuration Files
- **Bash**: `~/.bash_profile` or `~/.bashrc`
- **Zsh**: `~/.zshrc` (default shell on macOS 10.15+)
- **Fish**: `~/.config/fish/config.fish`

### Windows

#### Supported Versions
- Windows 10 version 1903 or later
- Windows 11
- Windows Server 2019 or later

#### Supported Shells
- **PowerShell 5.1** (Windows PowerShell)
- **PowerShell 7+** (PowerShell Core)

#### Dependencies
- PowerShell (pre-installed on Windows)
- Internet connection for downloading

#### Installation Locations
- **User install** (default): `%USERPROFILE%\.local\bin\auto-nvm.exe`
- **System install**: `C:\Program Files\auto-nvm\auto-nvm.exe` (requires admin)

## Shell Configuration

Auto-NVM integrates with your shell by adding a wrapper around the `cd` command. Here's what gets added to each shell:

### Bash Integration (`~/.bashrc`)

```bash
# AUTO_NVM_START
# Auto-NVM: Automatic Node.js version switching
# This function wraps the cd command to automatically switch Node.js versions
# when entering directories containing .nvmrc files.
function cd() {
    builtin cd "$@"
    if command -v auto-nvm >/dev/null 2>&1; then
        eval "$(auto-nvm --quiet switch)"
    fi
}

# Check for .nvmrc on shell startup
if command -v auto-nvm >/dev/null 2>&1; then
    eval "$(auto-nvm --quiet switch)"
fi
# AUTO_NVM_END
```

### Zsh Integration (`~/.zshrc`)

```zsh
# AUTO_NVM_START
# Auto-NVM: Automatic Node.js version switching
function cd() {
    builtin cd "$@"
    if command -v auto-nvm >/dev/null 2>&1; then
        eval "$(auto-nvm --quiet switch)"
    fi
}

# Check for .nvmrc on shell startup
if command -v auto-nvm >/dev/null 2>&1; then
    eval "$(auto-nvm --quiet switch)"
fi
# AUTO_NVM_END
```

### Fish Integration (`~/.config/fish/config.fish`)

```fish
# AUTO_NVM_START
# Auto-NVM: Automatic Node.js version switching
function cd
    builtin cd $argv
    if command -v auto-nvm >/dev/null 2>&1
        eval (auto-nvm --quiet switch)
    end
end

# Check for .nvmrc on shell startup
if command -v auto-nvm >/dev/null 2>&1
    eval (auto-nvm --quiet switch)
end
# AUTO_NVM_END
```

### PowerShell Integration

```powershell
# AUTO_NVM_START
# Auto-NVM: Automatic Node.js version switching
function Set-Location {
    [CmdletBinding()]
    param([string]$Path = "")

    if ($Path) {
        Microsoft.PowerShell.Management\Set-Location $Path
    } else {
        Microsoft.PowerShell.Management\Set-Location
    }

    if (Get-Command auto-nvm -ErrorAction SilentlyContinue) {
        try {
            $result = auto-nvm --quiet switch
            if ($result) {
                Invoke-Expression $result
            }
        } catch {
            # Silently ignore errors in quiet mode
        }
    }
}

# Alias cd to Set-Location for compatibility
Set-Alias -Name cd -Value Set-Location -Force

# Check for .nvmrc on shell startup
if (Get-Command auto-nvm -ErrorAction SilentlyContinue) {
    try {
        $result = auto-nvm --quiet switch
        if ($result) {
            Invoke-Expression $result
        }
    } catch {
        # Silently ignore errors in quiet mode
    }
}
# AUTO_NVM_END
```

## Verification

After installation, verify that auto-nvm is working correctly:

### 1. Check Installation

```bash
# Verify binary is in PATH
which auto-nvm
# Output: /home/user/.local/bin/auto-nvm

# Check version
auto-nvm --version
# Output: auto-nvm 0.1.1
```

### 2. Test Basic Functionality

```bash
# Check current directory
auto-nvm check
# Output: No .nvmrc file found in current directory or parent directories

# Create test .nvmrc
echo "18.17.0" > .nvmrc

# Check again
auto-nvm check
# Output: Found .nvmrc with version: 18.17.0
#         Current Node.js version: v16.20.1
#         Switch needed: yes

# Test manual switching
auto-nvm switch
# Output: Switched to Node.js v18.17.0
```

### 3. Test Automatic Switching

```bash
# Create test directory with .nvmrc
mkdir test-auto-nvm
echo "16.20.1" > test-auto-nvm/.nvmrc

# Navigate to directory (should auto-switch)
cd test-auto-nvm/
# Output: Switched to Node.js v16.20.1

# Verify version
node --version
# Output: v16.20.1
```

### 4. Verify Shell Integration

Check that your shell configuration file contains the auto-nvm integration:

```bash
# Bash/Zsh
grep -A 10 "AUTO_NVM_START" ~/.bashrc  # or ~/.zshrc

# Fish
grep -A 10 "AUTO_NVM_START" ~/.config/fish/config.fish

# PowerShell
Get-Content $PROFILE | Select-String -Pattern "AUTO_NVM_START" -Context 5
```

## Uninstallation

### Remove Shell Integration

```bash
# Remove shell integration (keeps binary)
auto-nvm uninstall
```

This removes the auto-nvm configuration from your shell RC file but keeps the binary installed.

### Complete Removal

#### Unix (Linux/macOS)

```bash
# Remove shell integration
auto-nvm uninstall

# Remove binary
rm ~/.local/bin/auto-nvm

# Remove from PATH (edit your shell RC file)
# Remove the line: export PATH="$HOME/.local/bin:$PATH"
```

#### Windows

```powershell
# Remove shell integration
auto-nvm uninstall

# Remove binary
Remove-Item "$env:USERPROFILE\.local\bin\auto-nvm.exe"

# Remove from PATH
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$newPath = $userPath -replace [regex]::Escape("$env:USERPROFILE\.local\bin;"), ""
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
```

## Troubleshooting

### Common Issues

#### 1. Command Not Found

**Symptoms:**
```bash
auto-nvm: command not found
```

**Solutions:**

1. **Check if binary exists:**
   ```bash
   ls -la ~/.local/bin/auto-nvm
   ```

2. **Check PATH:**
   ```bash
   echo $PATH | grep -o "[^:]*local/bin[^:]*"
   ```

3. **Add to PATH manually:**
   ```bash
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

4. **Reinstall:**
   ```bash
   curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
   ```

#### 2. Auto-switching Not Working

**Symptoms:**
- Manual `auto-nvm switch` works
- But `cd` doesn't trigger auto-switching

**Solutions:**

1. **Check shell integration:**
   ```bash
   grep "AUTO_NVM" ~/.bashrc  # or appropriate shell RC file
   ```

2. **Re-run setup:**
   ```bash
   auto-nvm setup
   ```

3. **Restart shell:**
   ```bash
   exec $SHELL  # or open new terminal
   ```

4. **Check for conflicts:**
   ```bash
   type cd  # Should show function, not builtin
   ```

#### 3. Permission Errors

**Symptoms:**
```
Permission denied: /usr/local/bin/auto-nvm
```

**Solutions:**

1. **Install to user directory:**
   ```bash
   curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | AUTO_NVM_INSTALL_DIR=~/.local/bin bash
   ```

2. **Create directory first:**
   ```bash
   mkdir -p ~/.local/bin
   ```

3. **Check directory permissions:**
   ```bash
   ls -ld ~/.local/bin
   ```

#### 4. Download Failures

**Symptoms:**
```
Failed to download auto-nvm
```

**Solutions:**

1. **Check internet connection:**
   ```bash
   curl -I https://github.com
   ```

2. **Check GitHub API:**
   ```bash
   curl -s https://api.github.com/repos/zerosrat/auto-nvm/releases/latest
   ```

3. **Manual download:**
   - Visit [GitHub Releases](https://github.com/zerosrat/auto-nvm/releases)
   - Download manually and follow manual installation steps

4. **Corporate firewall:**
   - Check proxy settings
   - Contact IT department

#### 5. Shell-Specific Issues

**Fish Shell:**
```bash
# If fish doesn't recognize the function
funcsave cd
```

**PowerShell:**
```powershell
# If PowerShell execution policy blocks scripts
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

**Zsh with Oh My Zsh:**
```bash
# If Oh My Zsh conflicts with cd function
# Add to ~/.zshrc before Oh My Zsh initialization
export AUTO_NVM_SETUP_DONE=1
```

### Debug Mode

Enable debug logging to troubleshoot issues:

```bash
export AUTO_NVM_LOG_LEVEL=debug
auto-nvm check
```

### Getting Help

If you're still having issues:

1. **Check existing issues:** [GitHub Issues](https://github.com/zerosrat/auto-nvm/issues)
2. **Create new issue:** Include your platform, shell, and error messages
3. **Join discussions:** [GitHub Discussions](https://github.com/zerosrat/auto-nvm/discussions)

### System Information

When reporting issues, please include:

```bash
# Platform info
uname -a

# Shell info
echo $SHELL
$SHELL --version

# Auto-NVM info
auto-nvm --version

# PATH info
echo $PATH

# NVM info
nvm --version
which nvm
```

This information helps maintainers diagnose and fix issues quickly.