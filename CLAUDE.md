# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Auto-NVM is a cross-platform Node.js version auto-switcher that supports multiple shells (bash, zsh, fish, powershell) and automatically switches Node.js versions when users `cd` into directories containing `.nvmrc` files.

## Implementation Phases

The project is designed for phased implementation to maintain manageable context:

1. **Phase 1 (MVP)**: Core functionality + bash support only
2. **Phase 2**: Installation scripts + multi-shell support
3. **Phase 3**: Performance optimization + caching
4. **Phase 4**: Testing + documentation + release preparation

Refer to `docs/IMPLEMENTATION_PHASES.md` for detailed task breakdowns and success criteria for each phase.

## Key Configuration Points

- `AUTO_NVM_ENABLED`: Global enable/disable toggle
- `AUTO_NVM_PROMPT_INSTALL`: Whether to prompt for missing version installation
- `AUTO_NVM_CACHE_ENABLED`: Performance caching toggle
- `AUTO_NVM_EXCLUDE_DIRS`: Directories to skip processing
- `AUTO_NVM_LOG_LEVEL`: Logging verbosity (debug, info, warn, error, silent)

## Usage

The `switch` command outputs the nvm command for the current shell to execute via `eval`:

**Fish shell:**
```fish
eval (auto-nvm switch)
```

**Bash/Zsh:**
```bash
eval "$(auto-nvm switch)"
```

**PowerShell:**
```powershell
Invoke-Expression (auto-nvm switch)
```

## Critical Implementation Notes

- All shell scripts must be POSIX-compatible for maximum portability
- Error handling should be comprehensive with user-friendly messages
- Performance is critical - target < 500ms switching delay
- Cache mechanism uses directory-based keys with TTL expiration
- Each shell integration must preserve original cd functionality
- NVM abstraction layer must handle both Unix nvm and Windows nvm-windows