#!/bin/bash
# Shelltape bash hook with PTY-based output capture
# Source this file in your ~/.bashrc to enable command recording

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

# Enable command wrapping mode
export SHELLTAPE_WRAP=1

# Function to wrap command execution
__shelltape_wrap_command() {
    local cmd="$BASH_COMMAND"

    # Skip if already being wrapped or if it's a shelltape command
    if [[ "$SHELLTAPE_WRAPPING" == "1" ]] || [[ "$cmd" == shelltape* ]] || [[ "$cmd" == __shelltape* ]]; then
        return 0
    fi

    # Don't wrap certain commands
    case "$cmd" in
        cd|cd\ *|pwd|exit|logout|clear|history|alias|unalias|source|.|export|unset)
            return 0
            ;;
    esac

    # Mark that we're wrapping to prevent recursion
    SHELLTAPE_WRAPPING=1

    # Execute through shelltape wrapper
    shelltape exec --session-id "$SHELLTAPE_SESSION_ID" -- bash -c "$cmd"
    local exit_code=$?

    # Unmark wrapping
    SHELLTAPE_WRAPPING=0

    # Prevent the original command from executing
    return $exit_code
}

# Set up the DEBUG trap for command wrapping
trap '__shelltape_wrap_command' DEBUG
