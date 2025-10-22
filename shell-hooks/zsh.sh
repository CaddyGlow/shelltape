#!/bin/zsh
# Shelltape zsh hook with PTY-based output capture
# Source this file in your ~/.zshrc to enable command recording

# Generate session ID once per shell session
if [ -z "$SHELLTAPE_SESSION_ID" ]; then
    if [ -f /proc/sys/kernel/random/uuid ]; then
        SHELLTAPE_SESSION_ID=$(cat /proc/sys/kernel/random/uuid)
    elif command -v uuidgen >/dev/null 2>&1; then
        SHELLTAPE_SESSION_ID=$(uuidgen)
    else
        # Fallback: use timestamp and random number
        SHELLTAPE_SESSION_ID="$(date +%s)-$$-$RANDOM"
    fi
    export SHELLTAPE_SESSION_ID
fi

# Configuration: Set to 1 for automatic capture, 0 for manual
: ${SHELLTAPE_AUTO_CAPTURE:=0}

# Helper function for wrapping commands
shelltape_exec() {
    local cmd="$*"

    # Don't wrap shelltape commands or certain simple patterns
    case "$cmd" in
        shelltape*|cd|cd\ *|pwd|clear|exit|logout)
            # Execute directly without wrapping
            eval "$cmd"
            ;;
        *)
            # Execute through shelltape wrapper for output capture
            command shelltape exec --session-id "$SHELLTAPE_SESSION_ID" -- "$@"
            ;;
    esac
}

# Automatic capture mode using function wrappers
# Note: This creates wrapper functions for common commands
# For full automatic capture of ALL commands, manual mode with aliases is recommended
if [[ "$SHELLTAPE_AUTO_CAPTURE" == "1" ]]; then
    # List of commands to auto-wrap
    __SHELLTAPE_AUTO_COMMANDS=(
        ls cat grep find df du ps top free
        git npm cargo pip python rustc
        make cmake gcc g++ clang
    )

    # Create wrapper functions for each command
    for cmd in "${__SHELLTAPE_AUTO_COMMANDS[@]}"; do
        # Only create wrapper if command exists
        if command -v "$cmd" >/dev/null 2>&1; then
            eval "
            function $cmd() {
                shelltape_exec $cmd \"\$@\"
            }
            "
        fi
    done

    unset __SHELLTAPE_AUTO_COMMANDS
fi

# Usage examples:
#
# Manual mode (default):
#   shelltape_exec ls -la
#   shelltape_exec git status
#
# Or create aliases:
#   alias ll='shelltape_exec ls -la'
#   alias gs='shelltape_exec git status'
#
# Automatic mode:
#   Set SHELLTAPE_AUTO_CAPTURE=1 before sourcing this file
#   This will auto-wrap common commands (ls, git, cat, grep, etc.)
#
#   To customize which commands are auto-wrapped, edit the
#   __SHELLTAPE_AUTO_COMMANDS array in this file
