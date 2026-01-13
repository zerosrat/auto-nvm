#!/usr/bin/env bash
set -euo pipefail

# Auto-NVM Universal Install Script for Unix (Linux/macOS)
# Usage: curl -fsSL https://raw.githubusercontent.com/user/auto-nvm/main/install.sh | bash

# Configuration
REPO_URL="https://github.com/zerosrat/auto-nvm"
INSTALL_DIR="${AUTO_NVM_INSTALL_DIR:-$HOME/.local/bin}"
FORCE_INSTALL="${AUTO_NVM_FORCE:-false}"
AUTO_SETUP="${AUTO_NVM_AUTO_SETUP:-true}"
QUIET="${AUTO_NVM_QUIET:-false}"

# Test mode for CI/development
TEST_MODE="${AUTO_NVM_TEST_MODE:-false}"
TEST_BINARY_PATH="${AUTO_NVM_TEST_BINARY_PATH:-}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
RESET='\033[0m'

# Logging functions
log() {
    if [[ "$QUIET" != "true" ]]; then
        echo -e "${BLUE}[INFO]${RESET} $*"
    fi
}

warn() {
    echo -e "${YELLOW}[WARN]${RESET} $*" >&2
}

error() {
    echo -e "${RED}[ERROR]${RESET} $*" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${RESET} $*"
}

# Cleanup function
cleanup() {
    if [[ -n "${TEMP_DIR:-}" ]] && [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

# Platform detection
detect_platform() {
    local os arch

    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)

    case "$arch" in
        x86_64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)
            error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac

    case "$os" in
        linux)
            echo "${arch}-unknown-linux-gnu"
            ;;
        darwin)
            echo "${arch}-apple-darwin"
            ;;
        *)
            error "Unsupported operating system: $os"
            exit 1
            ;;
    esac
}

# Get latest version from GitHub API
get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        curl -s "https://api.github.com/repos/zerosrat/auto-nvm/releases/latest" | \
            grep '"tag_name":' | \
            sed -E 's/.*"([^"]+)".*/\1/' | \
            sed 's/^v//'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/zerosrat/auto-nvm/releases/latest" | \
            grep '"tag_name":' | \
            sed -E 's/.*"([^"]+)".*/\1/' | \
            sed 's/^v//'
    else
        error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

# Download and extract binary
download_binary() {
    local version="$1"
    local platform="$2"
    local download_url archive_name

    if [[ "$TEST_MODE" == "true" ]]; then
        if [[ -z "$TEST_BINARY_PATH" ]] || [[ ! -f "$TEST_BINARY_PATH" ]]; then
            error "Test mode enabled but TEST_BINARY_PATH not set or file doesn't exist"
            exit 1
        fi
        log "Test mode: copying binary from $TEST_BINARY_PATH"
        cp "$TEST_BINARY_PATH" "$TEMP_DIR/auto-nvm"
        chmod +x "$TEMP_DIR/auto-nvm"
        return
    fi

    archive_name="auto-nvm-v${version}-${platform}.tar.gz"
    download_url="${REPO_URL}/releases/download/v${version}/${archive_name}"

    log "Downloading auto-nvm v${version} for ${platform}..."
    log "URL: $download_url"

    if command -v curl >/dev/null 2>&1; then
        if ! curl -fsSL "$download_url" -o "$TEMP_DIR/$archive_name"; then
            error "Failed to download auto-nvm"
            exit 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget -q "$download_url" -O "$TEMP_DIR/$archive_name"; then
            error "Failed to download auto-nvm"
            exit 1
        fi
    else
        error "Neither curl nor wget is available"
        exit 1
    fi

    log "Extracting archive..."
    if ! tar -xzf "$TEMP_DIR/$archive_name" -C "$TEMP_DIR"; then
        error "Failed to extract archive"
        exit 1
    fi

    if [[ ! -f "$TEMP_DIR/auto-nvm" ]]; then
        error "Binary not found in archive"
        exit 1
    fi

    chmod +x "$TEMP_DIR/auto-nvm"
}

# Install binary to target directory
install_binary() {
    local target_path="$INSTALL_DIR/auto-nvm"

    # Check if already installed
    if [[ -f "$target_path" ]] && [[ "$FORCE_INSTALL" != "true" ]]; then
        local current_version
        current_version=$("$target_path" --version 2>/dev/null | awk '{print $2}' || echo "unknown")
        warn "auto-nvm is already installed at $target_path (version: $current_version)"
        warn "Use AUTO_NVM_FORCE=true to overwrite, or remove the existing installation first"

        # Still try to run setup if AUTO_SETUP is enabled
        if [[ "$AUTO_SETUP" == "true" ]]; then
            log "Running setup for existing installation..."
            run_setup "$target_path"
        fi
        return 0
    fi

    # Create install directory
    if [[ ! -d "$INSTALL_DIR" ]]; then
        log "Creating install directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
    fi

    # Copy binary
    log "Installing auto-nvm to $target_path..."
    cp "$TEMP_DIR/auto-nvm" "$target_path"

    # Verify installation
    if ! "$target_path" --version >/dev/null 2>&1; then
        error "Installation verification failed"
        exit 1
    fi

    success "auto-nvm installed successfully to $target_path"
}

# Add to PATH if needed
setup_path() {
    local shell_rc

    # Check if already in PATH
    if command -v auto-nvm >/dev/null 2>&1; then
        log "auto-nvm is already in PATH"
        return 0
    fi

    # Check if install directory is in PATH
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
        log "Install directory is already in PATH"
        return 0
    fi

    log "Adding $INSTALL_DIR to PATH..."

    # Detect shell and appropriate RC file
    case "$SHELL" in
        */bash)
            if [[ -f "$HOME/.bashrc" ]]; then
                shell_rc="$HOME/.bashrc"
            elif [[ -f "$HOME/.bash_profile" ]]; then
                shell_rc="$HOME/.bash_profile"
            else
                shell_rc="$HOME/.bashrc"
                touch "$shell_rc"
            fi
            ;;
        */zsh)
            shell_rc="$HOME/.zshrc"
            [[ ! -f "$shell_rc" ]] && touch "$shell_rc"
            ;;
        */fish)
            # Fish uses a different PATH setup
            local fish_config_dir="$HOME/.config/fish"
            mkdir -p "$fish_config_dir"
            echo "set -gx PATH $INSTALL_DIR \$PATH" >> "$fish_config_dir/config.fish"
            log "Added $INSTALL_DIR to Fish PATH in $fish_config_dir/config.fish"
            return 0
            ;;
        *)
            warn "Unknown shell: $SHELL"
            warn "Please manually add $INSTALL_DIR to your PATH"
            return 0
            ;;
    esac

    # Add to shell RC file
    if [[ -n "$shell_rc" ]]; then
        echo "" >> "$shell_rc"
        echo "# Added by auto-nvm installer" >> "$shell_rc"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_rc"
        log "Added PATH export to $shell_rc"
        log "Please restart your shell or run: source $shell_rc"
    fi
}

# Run auto-nvm setup
run_setup() {
    local binary_path="$1"

    log "Running auto-nvm setup..."

    if [[ "$TEST_MODE" == "true" ]]; then
        log "Test mode: skipping actual setup"
        log "In production, this would run: $binary_path setup"
        return 0
    fi

    if "$binary_path" setup; then
        success "Shell integration configured successfully!"
        success "auto-nvm will now automatically switch Node.js versions when you cd into directories with .nvmrc files"
    else
        warn "Setup failed. You can run 'auto-nvm setup' manually later."
        return 1
    fi
}

# Display usage information
show_usage() {
    cat << EOF
Auto-NVM Universal Install Script

Usage: $0 [OPTIONS]

Options:
    --help              Show this help message
    --dry-run          Show what would be done without making changes
    --test-install     Test installation in current environment
    --force            Force installation even if already installed
    --no-setup         Skip automatic shell setup
    --quiet            Quiet output
    --install-dir DIR  Custom installation directory (default: ~/.local/bin)

Environment Variables:
    AUTO_NVM_INSTALL_DIR    Installation directory
    AUTO_NVM_FORCE          Force installation (true/false)
    AUTO_NVM_AUTO_SETUP     Run setup automatically (true/false)
    AUTO_NVM_QUIET          Quiet mode (true/false)

Examples:
    # Standard installation
    curl -fsSL https://raw.githubusercontent.com/user/auto-nvm/main/install.sh | bash

    # Custom install directory
    curl -fsSL https://raw.githubusercontent.com/user/auto-nvm/main/install.sh | AUTO_NVM_INSTALL_DIR=/usr/local/bin bash

    # Skip automatic setup
    curl -fsSL https://raw.githubusercontent.com/user/auto-nvm/main/install.sh | AUTO_NVM_AUTO_SETUP=false bash
EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help)
                show_usage
                exit 0
                ;;
            --dry-run)
                log "DRY RUN MODE - No changes will be made"
                export DRY_RUN=true
                ;;
            --test-install)
                export TEST_INSTALL=true
                ;;
            --force)
                export FORCE_INSTALL=true
                ;;
            --no-setup)
                export AUTO_SETUP=false
                ;;
            --quiet)
                export QUIET=true
                ;;
            --install-dir)
                export INSTALL_DIR="$2"
                shift
                ;;
            *)
                error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
        shift
    done
}

# Main installation function
main() {
    parse_args "$@"

    log "${BOLD}Auto-NVM Universal Installer${RESET}"
    log "Installing auto-nvm - Cross-platform Node.js version auto-switcher"
    log ""

    # Check for dry run
    if [[ "${DRY_RUN:-false}" == "true" ]]; then
        log "DRY RUN: Would detect platform and install auto-nvm to $INSTALL_DIR"
        log "DRY RUN: Would add $INSTALL_DIR to PATH if needed"
        if [[ "$AUTO_SETUP" == "true" ]]; then
            log "DRY RUN: Would run 'auto-nvm setup' to configure shell integration"
        fi
        log "DRY RUN: Installation complete!"
        return 0
    fi

    # Check for test install mode
    if [[ "${TEST_INSTALL:-false}" == "true" ]]; then
        log "TEST INSTALL MODE - Using test binary for installation"
        if [[ -z "$TEST_BINARY_PATH" ]] || [[ ! -f "$TEST_BINARY_PATH" ]]; then
            error "Test install mode requires TEST_BINARY_PATH to be set to a valid binary"
            exit 1
        fi
        # Override test mode to use the test binary
        TEST_MODE="true"
        log "Test binary path: $TEST_BINARY_PATH"
    fi

    # Create temporary directory
    TEMP_DIR=$(mktemp -d)

    # Detect platform
    local platform
    platform=$(detect_platform)
    log "Detected platform: $platform"

    # Get latest version
    local version
    if [[ "$TEST_MODE" == "true" ]]; then
        version="0.1.0-alpha.0"  # Use current version from Cargo.toml for testing
        log "Using test version: v$version"
    else
        log "Fetching latest version from GitHub..."
        version=$(get_latest_version)
        if [[ -z "$version" ]]; then
            error "Failed to get latest version"
            exit 1
        fi
        log "Latest version: v$version"
    fi

    # Download binary
    download_binary "$version" "$platform"

    # Install binary
    install_binary

    # Setup PATH
    setup_path

    # Run setup if enabled
    if [[ "$AUTO_SETUP" == "true" ]]; then
        run_setup "$INSTALL_DIR/auto-nvm"
    fi

    success ""
    success "${BOLD}Installation complete!${RESET}"
    success ""
    success "auto-nvm v$version has been installed to $INSTALL_DIR/auto-nvm"

    if [[ "$AUTO_SETUP" == "true" ]]; then
        success "Shell integration has been configured automatically."
        success "Restart your shell or open a new terminal to start using auto-nvm."
    else
        success "Run 'auto-nvm setup' to configure shell integration."
    fi

    success ""
    success "Usage:"
    success "  auto-nvm check    - Check current directory for .nvmrc"
    success "  auto-nvm switch   - Switch to .nvmrc version"
    success "  auto-nvm setup    - Configure shell integration"
    success "  auto-nvm uninstall - Remove shell integration"
    success ""
    success "For more information, visit: $REPO_URL"
}

# Run main function with all arguments
main "$@"