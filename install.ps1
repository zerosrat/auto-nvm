# Auto-NVM Universal Install Script for Windows PowerShell
# Usage: iwr -useb https://raw.githubusercontent.com/user/auto-nvm/main/install.ps1 | iex

[CmdletBinding()]
param(
    [switch]$Help,
    [switch]$DryRun,
    [switch]$TestInstall,
    [switch]$Force,
    [switch]$NoSetup,
    [switch]$Quiet,
    [string]$InstallDir = ""
)

# Configuration
$RepoUrl = "https://github.com/zerosrat/auto-nvm"
$DefaultInstallDir = "$env:USERPROFILE\.local\bin"
$InstallDirectory = if ($InstallDir) { $InstallDir } else { $env:AUTO_NVM_INSTALL_DIR ?? $DefaultInstallDir }
$ForceInstall = $Force -or ($env:AUTO_NVM_FORCE -eq "true")
$AutoSetup = -not $NoSetup -and ($env:AUTO_NVM_AUTO_SETUP -ne "false")
$QuietMode = $Quiet -or ($env:AUTO_NVM_QUIET -eq "true")

# Test mode for CI/development
$TestMode = ($env:AUTO_NVM_TEST_MODE -eq "true")
$TestBinaryPath = $env:AUTO_NVM_TEST_BINARY_PATH

# Colors for output (Windows PowerShell compatible)
$Colors = @{
    Red = "Red"
    Green = "Green"
    Yellow = "Yellow"
    Blue = "Blue"
    Cyan = "Cyan"
    White = "White"
}

# Logging functions
function Write-Log {
    param([string]$Message, [string]$Color = "Blue")
    if (-not $QuietMode) {
        Write-Host "[INFO] $Message" -ForegroundColor $Colors[$Color]
    }
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor $Colors["Yellow"]
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor $Colors["Red"]
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor $Colors["Green"]
}

# Cleanup function
$TempDir = ""
function Cleanup {
    if ($TempDir -and (Test-Path $TempDir)) {
        try {
            Remove-Item $TempDir -Recurse -Force
        }
        catch {
            Write-Warning "Failed to cleanup temp directory: $TempDir"
        }
    }
}

# Platform detection
function Get-Platform {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x86_64-pc-windows-msvc" }
        "ARM64" { return "aarch64-pc-windows-msvc" }
        default {
            Write-Error "Unsupported architecture: $arch"
            exit 1
        }
    }
}

# Get latest version from GitHub API
function Get-LatestVersion {
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/zerosrat/auto-nvm/releases/latest" -ErrorAction Stop
        return $response.tag_name -replace '^v', ''
    }
    catch {
        Write-Error "Failed to get latest version: $_"
        exit 1
    }
}

# Download and extract binary
function Download-Binary {
    param(
        [string]$Version,
        [string]$Platform
    )

    if ($TestMode) {
        if (-not $TestBinaryPath -or -not (Test-Path $TestBinaryPath)) {
            Write-Error "Test mode enabled but TEST_BINARY_PATH not set or file doesn't exist"
            exit 1
        }
        Write-Log "Test mode: copying binary from $TestBinaryPath"
        Copy-Item $TestBinaryPath "$TempDir\auto-nvm.exe"
        return
    }

    $archiveName = "auto-nvm-v$Version-$Platform.zip"
    $downloadUrl = "$RepoUrl/releases/download/v$Version/$archiveName"
    $archivePath = "$TempDir\$archiveName"

    Write-Log "Downloading auto-nvm v$Version for $Platform..."
    Write-Log "URL: $downloadUrl"

    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -ErrorAction Stop
    }
    catch {
        Write-Error "Failed to download auto-nvm: $_"
        exit 1
    }

    Write-Log "Extracting archive..."
    try {
        Expand-Archive -Path $archivePath -DestinationPath $TempDir -Force -ErrorAction Stop
    }
    catch {
        Write-Error "Failed to extract archive: $_"
        exit 1
    }

    $binaryPath = "$TempDir\auto-nvm.exe"
    if (-not (Test-Path $binaryPath)) {
        Write-Error "Binary not found in archive"
        exit 1
    }
}

# Install binary to target directory
function Install-Binary {
    $targetPath = "$InstallDirectory\auto-nvm.exe"

    # Check if already installed
    if ((Test-Path $targetPath) -and -not $ForceInstall) {
        try {
            $currentVersion = & $targetPath --version 2>$null | ForEach-Object { ($_ -split ' ')[1] }
        }
        catch {
            $currentVersion = "unknown"
        }

        Write-Warning "auto-nvm is already installed at $targetPath (version: $currentVersion)"
        Write-Warning "Use -Force to overwrite, or remove the existing installation first"

        # Still try to run setup if AutoSetup is enabled
        if ($AutoSetup) {
            Write-Log "Running setup for existing installation..."
            Invoke-Setup $targetPath
        }
        return
    }

    # Create install directory
    if (-not (Test-Path $InstallDirectory)) {
        Write-Log "Creating install directory: $InstallDirectory"
        New-Item -ItemType Directory -Path $InstallDirectory -Force | Out-Null
    }

    # Copy binary
    Write-Log "Installing auto-nvm to $targetPath..."
    Copy-Item "$TempDir\auto-nvm.exe" $targetPath

    # Verify installation
    try {
        & $targetPath --version | Out-Null
    }
    catch {
        Write-Error "Installation verification failed"
        exit 1
    }

    Write-Success "auto-nvm installed successfully to $targetPath"
}

# Add to PATH if needed
function Set-PathEnvironment {
    # Check if already in PATH
    try {
        Get-Command auto-nvm -ErrorAction Stop | Out-Null
        Write-Log "auto-nvm is already in PATH"
        return
    }
    catch {
        # Not in PATH, continue
    }

    # Check if install directory is in PATH
    $currentPath = $env:PATH -split ';'
    if ($InstallDirectory -in $currentPath) {
        Write-Log "Install directory is already in PATH"
        return
    }

    Write-Log "Adding $InstallDirectory to PATH..."

    # Add to user PATH permanently
    try {
        $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($userPath) {
            $newPath = "$InstallDirectory;$userPath"
        } else {
            $newPath = $InstallDirectory
        }
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

        # Update current session PATH
        $env:PATH = "$InstallDirectory;$env:PATH"

        Write-Log "Added $InstallDirectory to user PATH"
        Write-Log "Please restart PowerShell or open a new terminal to refresh PATH"
    }
    catch {
        Write-Warning "Failed to update PATH automatically: $_"
        Write-Warning "Please manually add $InstallDirectory to your PATH"
    }
}

# Run auto-nvm setup
function Invoke-Setup {
    param([string]$BinaryPath)

    Write-Log "Running auto-nvm setup..."

    if ($TestMode) {
        Write-Log "Test mode: skipping actual setup"
        Write-Log "In production, this would run: $BinaryPath setup"
        return
    }

    try {
        & $BinaryPath setup
        Write-Success "Shell integration configured successfully!"
        Write-Success "auto-nvm will now automatically switch Node.js versions when you cd into directories with .nvmrc files"
    }
    catch {
        Write-Warning "Setup failed. You can run 'auto-nvm setup' manually later."
        Write-Warning "Error: $_"
    }
}

# Display usage information
function Show-Usage {
    @"
Auto-NVM Universal Install Script for Windows

Usage: .\install.ps1 [OPTIONS]

Options:
    -Help              Show this help message
    -DryRun           Show what would be done without making changes
    -TestInstall      Test installation in current environment
    -Force            Force installation even if already installed
    -NoSetup          Skip automatic shell setup
    -Quiet            Quiet output
    -InstallDir DIR   Custom installation directory (default: %USERPROFILE%\.local\bin)

Environment Variables:
    AUTO_NVM_INSTALL_DIR    Installation directory
    AUTO_NVM_FORCE          Force installation (true/false)
    AUTO_NVM_AUTO_SETUP     Run setup automatically (true/false)
    AUTO_NVM_QUIET          Quiet mode (true/false)

Examples:
    # Standard installation
    iwr -useb https://raw.githubusercontent.com/user/auto-nvm/main/install.ps1 | iex

    # Custom install directory
    `$env:AUTO_NVM_INSTALL_DIR="C:\tools\bin"; iwr -useb https://raw.githubusercontent.com/user/auto-nvm/main/install.ps1 | iex

    # Skip automatic setup
    `$env:AUTO_NVM_AUTO_SETUP="false"; iwr -useb https://raw.githubusercontent.com/user/auto-nvm/main/install.ps1 | iex
"@
}

# Main installation function
function Main {
    if ($Help) {
        Show-Usage
        return
    }

    Write-Host ""
    Write-Host "Auto-NVM Universal Installer" -ForegroundColor White -BackgroundColor Blue
    Write-Log "Installing auto-nvm - Cross-platform Node.js version auto-switcher"
    Write-Host ""

    # Check for dry run
    if ($DryRun) {
        Write-Log "DRY RUN: Would detect platform and install auto-nvm to $InstallDirectory"
        Write-Log "DRY RUN: Would add $InstallDirectory to PATH if needed"
        if ($AutoSetup) {
            Write-Log "DRY RUN: Would run 'auto-nvm setup' to configure shell integration"
        }
        Write-Log "DRY RUN: Installation complete!"
        return
    }

    # Check for test install mode
    if ($TestInstall) {
        Write-Log "TEST INSTALL MODE - Using test binary for installation"
        if (-not $TestBinaryPath -or -not (Test-Path $TestBinaryPath)) {
            Write-Error "Test install mode requires TEST_BINARY_PATH to be set to a valid binary"
            exit 1
        }
        # Override test mode to use the test binary
        $script:TestMode = $true
        Write-Log "Test binary path: $TestBinaryPath"
    }

    # Create temporary directory
    $script:TempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }

    try {
        # Detect platform
        $platform = Get-Platform
        Write-Log "Detected platform: $platform"

        # Get latest version
        if ($TestMode) {
            $version = "0.1.0-alpha.0"  # Use current version from Cargo.toml for testing
            Write-Log "Using test version: v$version"
        } else {
            Write-Log "Fetching latest version from GitHub..."
            $version = Get-LatestVersion
            if (-not $version) {
                Write-Error "Failed to get latest version"
                exit 1
            }
            Write-Log "Latest version: v$version"
        }

        # Download binary
        Download-Binary $version $platform

        # Install binary
        Install-Binary

        # Setup PATH
        Set-PathEnvironment

        # Run setup if enabled
        if ($AutoSetup) {
            Invoke-Setup "$InstallDirectory\auto-nvm.exe"
        }

        Write-Host ""
        Write-Success "Installation complete!" -Color "White"
        Write-Host ""
        Write-Success "auto-nvm v$version has been installed to $InstallDirectory\auto-nvm.exe"

        if ($AutoSetup) {
            Write-Success "Shell integration has been configured automatically."
            Write-Success "Restart PowerShell or open a new terminal to start using auto-nvm."
        } else {
            Write-Success "Run 'auto-nvm setup' to configure shell integration."
        }

        Write-Host ""
        Write-Success "Usage:"
        Write-Success "  auto-nvm check    - Check current directory for .nvmrc"
        Write-Success "  auto-nvm switch   - Switch to .nvmrc version"
        Write-Success "  auto-nvm setup    - Configure shell integration"
        Write-Success "  auto-nvm uninstall - Remove shell integration"
        Write-Host ""
        Write-Success "For more information, visit: $RepoUrl"
    }
    finally {
        Cleanup
    }
}

# Handle being invoked via Invoke-Expression
if ($MyInvocation.InvocationName -eq '&') {
    Main
} else {
    # Called directly, parse parameters and run
    Main
}