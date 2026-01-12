#!/usr/bin/env bash
set -euo pipefail

# Auto-NVM Version Bump Script
# Usage: ./scripts/bump-version.sh [patch|minor|major|VERSION]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
CHANGELOG="$PROJECT_ROOT/CHANGELOG.md"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
RESET='\033[0m'

log() {
    echo -e "${BLUE}[INFO]${RESET} $*"
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

# Show usage information
show_usage() {
    cat << EOF
Auto-NVM Version Bump Script

Usage: $0 [COMMAND|VERSION]

Commands:
    patch       Increment patch version (0.1.0 -> 0.1.1)
    minor       Increment minor version (0.1.0 -> 0.2.0)
    major       Increment major version (0.1.0 -> 1.0.0)
    VERSION     Set specific version (e.g., 1.2.3)

Options:
    --help      Show this help message
    --dry-run   Show what would be done without making changes
    --no-git    Skip git operations (tag creation, commit)
    --no-changelog  Skip changelog update

Examples:
    $0 patch                    # 0.1.0 -> 0.1.1
    $0 minor                    # 0.1.0 -> 0.2.0
    $0 1.0.0                    # Set to 1.0.0
    $0 patch --dry-run          # Preview changes
    $0 minor --no-git           # Update version without git operations

Environment Variables:
    DRY_RUN         Set to 'true' for dry run mode
    NO_GIT          Set to 'true' to skip git operations
    NO_CHANGELOG    Set to 'true' to skip changelog update
EOF
}

# Parse command line arguments
parse_args() {
    BUMP_TYPE=""
    DRY_RUN="${DRY_RUN:-false}"
    NO_GIT="${NO_GIT:-false}"
    NO_CHANGELOG="${NO_CHANGELOG:-false}"

    while [[ $# -gt 0 ]]; do
        case $1 in
            --help)
                show_usage
                exit 0
                ;;
            --dry-run)
                DRY_RUN=true
                ;;
            --no-git)
                NO_GIT=true
                ;;
            --no-changelog)
                NO_CHANGELOG=true
                ;;
            patch|minor|major)
                BUMP_TYPE="$1"
                ;;
            [0-9]*.[0-9]*.[0-9]*)
                BUMP_TYPE="version"
                NEW_VERSION="$1"
                ;;
            *)
                error "Unknown argument: $1"
                show_usage
                exit 1
                ;;
        esac
        shift
    done

    if [[ -z "$BUMP_TYPE" ]]; then
        error "No bump type specified"
        show_usage
        exit 1
    fi
}

# Get current version from Cargo.toml
get_current_version() {
    if [[ ! -f "$CARGO_TOML" ]]; then
        error "Cargo.toml not found at $CARGO_TOML"
        exit 1
    fi

    grep '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/'
}

# Validate semantic version format
validate_version() {
    local version="$1"
    if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        error "Invalid version format: $version (expected: X.Y.Z)"
        exit 1
    fi
}

# Calculate new version based on bump type
calculate_new_version() {
    local current="$1"
    local bump_type="$2"

    # Split version into parts
    local major minor patch
    IFS='.' read -r major minor patch <<< "$current"

    case "$bump_type" in
        patch)
            NEW_VERSION="$major.$minor.$((patch + 1))"
            ;;
        minor)
            NEW_VERSION="$major.$((minor + 1)).0"
            ;;
        major)
            NEW_VERSION="$((major + 1)).0.0"
            ;;
        version)
            # NEW_VERSION already set from command line
            validate_version "$NEW_VERSION"
            ;;
        *)
            error "Unknown bump type: $bump_type"
            exit 1
            ;;
    esac
}

# Update version in Cargo.toml
update_cargo_toml() {
    local old_version="$1"
    local new_version="$2"

    log "Updating Cargo.toml: $old_version -> $new_version"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "DRY RUN: Would update version in $CARGO_TOML"
        return
    fi

    # Create backup
    cp "$CARGO_TOML" "$CARGO_TOML.bak"

    # Update version
    sed -i.tmp "s/^version = \"$old_version\"/version = \"$new_version\"/" "$CARGO_TOML"
    rm "$CARGO_TOML.tmp"

    # Verify update
    local updated_version
    updated_version=$(get_current_version)
    if [[ "$updated_version" != "$new_version" ]]; then
        error "Failed to update version in Cargo.toml"
        # Restore backup
        mv "$CARGO_TOML.bak" "$CARGO_TOML"
        exit 1
    fi

    rm "$CARGO_TOML.bak"
    success "Updated Cargo.toml version to $new_version"
}

# Update changelog
update_changelog() {
    local new_version="$1"
    local date
    date=$(date +%Y-%m-%d)

    if [[ "$NO_CHANGELOG" == "true" ]]; then
        log "Skipping changelog update (--no-changelog)"
        return
    fi

    log "Updating CHANGELOG.md for version $new_version"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "DRY RUN: Would update CHANGELOG.md with version $new_version"
        return
    fi

    if [[ ! -f "$CHANGELOG" ]]; then
        warn "CHANGELOG.md not found, skipping changelog update"
        return
    fi

    # Create backup
    cp "$CHANGELOG" "$CHANGELOG.bak"

    # Update changelog
    sed -i.tmp "s/## \[Unreleased\]/## [Unreleased]\n\n## [$new_version] - $date/" "$CHANGELOG"
    rm "$CHANGELOG.tmp"

    success "Updated CHANGELOG.md for version $new_version"
}

# Verify git repository
check_git_repo() {
    if [[ "$NO_GIT" == "true" ]]; then
        return
    fi

    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        error "Not in a git repository"
        exit 1
    fi

    # Check for uncommitted changes
    if [[ -n "$(git status --porcelain)" ]]; then
        warn "Working directory has uncommitted changes"
        warn "Consider committing or stashing changes before version bump"

        if [[ "$DRY_RUN" != "true" ]]; then
            read -p "Continue anyway? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                exit 1
            fi
        fi
    fi
}

# Create git commit and tag
create_git_commit() {
    local version="$1"

    if [[ "$NO_GIT" == "true" ]]; then
        log "Skipping git operations (--no-git)"
        return
    fi

    log "Creating git commit and tag for version $version"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "DRY RUN: Would create git commit and tag v$version"
        return
    fi

    # Stage changes
    git add "$CARGO_TOML"
    if [[ -f "$CHANGELOG" ]] && [[ "$NO_CHANGELOG" != "true" ]]; then
        git add "$CHANGELOG"
    fi

    # Create commit
    git commit -m "chore(release): bump version to $version

- Update Cargo.toml version to $version
- Update CHANGELOG.md with release date

Co-Authored-By: Auto-NVM Version Bump Script"

    # Create tag
    git tag -a "v$version" -m "Release v$version

Auto-NVM version $version

See CHANGELOG.md for detailed changes."

    success "Created git commit and tag v$version"
    log "To push changes and trigger release:"
    log "  git push origin $(git branch --show-current)"
    log "  git push origin v$version"
}

# Verify tools are available
check_dependencies() {
    local missing_tools=()

    if ! command -v sed >/dev/null 2>&1; then
        missing_tools+=("sed")
    fi

    if [[ "$NO_GIT" != "true" ]] && ! command -v git >/dev/null 2>&1; then
        missing_tools+=("git")
    fi

    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        error "Missing required tools: ${missing_tools[*]}"
        exit 1
    fi
}

# Main function
main() {
    log "${BOLD}Auto-NVM Version Bump Script${RESET}"
    log ""

    # Parse arguments
    parse_args "$@"

    # Check dependencies
    check_dependencies

    # Get current version
    local current_version
    current_version=$(get_current_version)
    log "Current version: $current_version"

    # Calculate new version
    calculate_new_version "$current_version" "$BUMP_TYPE"
    log "New version: $NEW_VERSION"

    # Validate version change
    if [[ "$current_version" == "$NEW_VERSION" ]]; then
        warn "Version unchanged: $NEW_VERSION"
        exit 0
    fi

    # Check git repository
    check_git_repo

    if [[ "$DRY_RUN" == "true" ]]; then
        log ""
        log "${YELLOW}DRY RUN MODE - No changes will be made${RESET}"
        log ""
        log "Would perform the following operations:"
        log "  1. Update Cargo.toml: $current_version -> $NEW_VERSION"
        if [[ "$NO_CHANGELOG" != "true" ]]; then
            log "  2. Update CHANGELOG.md with release date"
        fi
        if [[ "$NO_GIT" != "true" ]]; then
            log "  3. Create git commit with changes"
            log "  4. Create git tag v$NEW_VERSION"
        fi
        log ""
        log "To execute these changes, run without --dry-run"
        exit 0
    fi

    # Confirm changes
    log ""
    log "About to bump version from $current_version to $NEW_VERSION"
    read -p "Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log "Aborted by user"
        exit 0
    fi

    # Perform version bump
    log ""
    log "Performing version bump..."

    # Update files
    update_cargo_toml "$current_version" "$NEW_VERSION"
    update_changelog "$NEW_VERSION"

    # Create git commit and tag
    create_git_commit "$NEW_VERSION"

    # Success message
    success ""
    success "${BOLD}Version bump completed successfully!${RESET}"
    success ""
    success "Version: $current_version -> $NEW_VERSION"
    success ""

    if [[ "$NO_GIT" != "true" ]]; then
        success "Next steps:"
        success "  1. Review the changes: git show HEAD"
        success "  2. Push to trigger release: git push origin $(git branch --show-current) && git push origin v$NEW_VERSION"
        success "  3. Monitor CI/CD pipeline for release build"
    fi
}

# Run main function with all arguments
main "$@"