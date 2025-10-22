#!/usr/bin/env fish
# Shelltape fish hook
# Source this file in your ~/.config/fish/config.fish to enable command recording

# Generate session ID once per shell session
if not set -q SHELLTAPE_SESSION_ID
    if test -f /proc/sys/kernel/random/uuid
        set -gx SHELLTAPE_SESSION_ID (cat /proc/sys/kernel/random/uuid)
    else if command -v uuidgen >/dev/null 2>&1
        set -gx SHELLTAPE_SESSION_ID (uuidgen)
    else
        # Fallback: use timestamp and random number
        set -gx SHELLTAPE_SESSION_ID (date +%s)"-"(random)
    end
end

# Temporary file for capturing output (per-shell instance)
set -gx SHELLTAPE_OUTPUT_FILE "/tmp/shelltape_$SHELLTAPE_SESSION_ID"_(fish -p %self)".log"

# Function called before each command execution
function __shelltape_preexec --on-event fish_preexec
    set -g SHELLTAPE_CMD $argv[1]
    set -g SHELLTAPE_START (date +%s%N)

    # Clear output file
    echo -n > $SHELLTAPE_OUTPUT_FILE
end

# Function called after each command execution
function __shelltape_postcmd --on-event fish_postexec
    set -l exit_code $status
    set -l end (date +%s%N)

    if set -q SHELLTAPE_CMD
        # Don't record shelltape commands or certain simple patterns
        switch $SHELLTAPE_CMD
            case 'shelltape*' '__shelltape*' 'cd' 'cd *' 'ls' 'ls *' 'pwd' 'clear'
                # Skip recording
            case '*'
                # Read captured output (if any)
                set -l output ""
                if test -f $SHELLTAPE_OUTPUT_FILE
                    set output (cat $SHELLTAPE_OUTPUT_FILE 2>/dev/null)
                end

                # Record the command in background to avoid blocking the shell
                fish -c "shelltape record \
                    --command '$SHELLTAPE_CMD' \
                    --exit-code $exit_code \
                    --start-time $SHELLTAPE_START \
                    --end-time $end \
                    --cwd '$PWD' \
                    --session-id '$SHELLTAPE_SESSION_ID' \
                    --output '$output' &" &
        end
    end

    set -e SHELLTAPE_CMD
end

# Clean up temporary files on exit
function __shelltape_cleanup --on-event fish_exit
    rm -f $SHELLTAPE_OUTPUT_FILE
end
