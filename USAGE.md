# Shelltape Usage Guide

## Installation Complete!

You've successfully installed shelltape with PTY-based output capture!

## How to Use

### Method 1: Direct Command Wrapping (Most Reliable)

Execute commands through shelltape directly:

```bash
shelltape exec --session-id "$SHELLTAPE_SESSION_ID" -- ls -la
shelltape exec --session-id "$SHELLTAPE_SESSION_ID" -- git status
shelltape exec --session-id "$SHELLTAPE_SESSION_ID" -- cat file.txt
```

### Method 2: Using the Helper Function (Recommended)

The hooks provide a `shelltape_exec` function:

```bash
shelltape_exec ls -la
shelltape_exec git status
shelltape_exec cat file.txt
```

### Method 3: Create Aliases (Best User Experience)

Add these to your `~/.zshrc` (after the shelltape source line):

```bash
# Shelltape aliases - commands with output capture
alias ll='shelltape_exec ls -la'
alias cat='shelltape_exec cat'
alias less='shelltape_exec less'
alias grep='shelltape_exec grep'
alias find='shelltape_exec find'

# Git aliases with capture
alias gs='shelltape_exec git status'
alias gd='shelltape_exec git diff'
alias gl='shelltape_exec git log'
alias ga='shelltape_exec git add'
alias gc='shelltape_exec git commit'
alias gp='shelltape_exec git push'

# System commands
alias df='shelltape_exec df -h'
alias du='shelltape_exec du -h'
alias ps='shelltape_exec ps aux'
alias top='shelltape_exec top'
alias free='shelltape_exec free -h'

# Package managers
alias npm='shelltape_exec npm'
alias cargo='shelltape_exec cargo'
alias pip='shelltape_exec pip'
```

Then reload: `source ~/.zshrc`

## Viewing Your History

### List Recent Commands
```bash
shelltape list              # Show last 20 commands
shelltape list -l 50        # Show last 50 commands
shelltape list -f "git"     # Search for commands containing "git"
```

### Interactive Browser (TUI)
```bash
shelltape browse
```

**Keybindings in TUI:**
- `j`/`k` or `↑`/`↓` - Navigate
- `/` - Search (including output!)
- `Space` - Mark command
- `Enter` - View details
- `e` - Export marked commands
- `q` - Quit

### Search Output
Since output is captured, you can search through it:

```bash
shelltape list -f "error"       # Find commands with "error" in output
shelltape list -f "success"     # Find successful messages
shelltape list -f "npm install" # Find npm install commands
```

### Export to Markdown
```bash
shelltape export -o history.md                    # Export all
shelltape export -o git-history.md -f "git"      # Export git commands
shelltape export -o errors.md -f "error"         # Export errors
```

### Statistics
```bash
shelltape stats
```

Shows:
- Total commands executed
- Success rate
- Most used commands
- Average execution time

## Tips

### Check What's Being Captured

```bash
# Run a command with capture
shelltape_exec echo "Hello World"

# View it
shelltape list -l 1

# See the full output
tail -1 ~/.shelltape/commands.jsonl | jq .
```

### Selective Capture

You don't need to capture everything! Only wrap commands where you want output captured:

```bash
# Regular command (not captured)
cd /tmp

# Captured command
shelltape_exec ls -la

# Regular again
cd ~
```

### View Raw Data

All data is stored in human-readable JSONL:

```bash
# View all commands
cat ~/.shelltape/commands.jsonl | jq .

# Count commands
wc -l ~/.shelltape/commands.jsonl

# Search with jq
cat ~/.shelltape/commands.jsonl | jq 'select(.command | contains("git"))'
```

## Maintenance

### Clean Old Data
```bash
shelltape clean --older-than-days 30    # Remove commands older than 30 days
shelltape clean --older-than-days 90 -y # Remove without confirmation
```

### Check Status
```bash
shelltape status
```

### Uninstall
```bash
shelltape uninstall
rm -rf ~/.shelltape/  # Remove all data
```

## Troubleshooting

### Commands Not Being Captured?

Make sure you're using one of the capture methods:
- Direct: `shelltape exec --session-id "$SHELLTAPE_SESSION_ID" -- command`
- Helper: `shelltape_exec command`
- Aliases: `ll` (if you created the alias)

### Check Session ID
```bash
echo $SHELLTAPE_SESSION_ID
# Should show a UUID
```

### Verify Installation
```bash
shelltape status
# Should show hooks are installed
```

### Test Capture
```bash
# Test basic capture
shelltape_exec echo "test"

# Check if it worked
shelltape list -l 1 | grep "test"

# Verify output was captured
tail -1 ~/.shelltape/commands.jsonl | jq -r '.output'
```

## Example Workflow

```bash
# Start working on a project
cd ~/projects/myapp

# Run commands with capture
shelltape_exec npm install
shelltape_exec npm test
shelltape_exec git status

# Later, find what you did
shelltape list -f "npm"

# Export your session
shelltape export -o work-log.md -s "$SHELLTAPE_SESSION_ID"

# Or browse interactively
shelltape browse
```

## What Gets Captured?

For each wrapped command, shelltape captures:
- Command text
- Full output (stdout + stderr)
- Exit code
- Execution time (nanosecond precision)
- Working directory
- Session ID
- Timestamp
- Shell, hostname, username

## Advanced: Automatic Capture Mode

If you want common commands captured automatically, set `SHELLTAPE_AUTO_CAPTURE=1` **before** sourcing the hooks.

Add to `~/.zshrc` (before the shelltape line):

```bash
export SHELLTAPE_AUTO_CAPTURE=1
source ~/.shelltape/zsh.sh
```

This will automatically wrap these commands:
- **File ops:** ls, cat, grep, find
- **System:** df, du, ps, top, free
- **Development:** git, npm, cargo, pip, python, rustc, make, cmake, gcc, g++, clang

**Customizing Auto-Capture:**

Edit `~/.shelltape/zsh.sh` and modify the `__SHELLTAPE_AUTO_COMMANDS` array:

```bash
__SHELLTAPE_AUTO_COMMANDS=(
    ls cat grep find df du ps top free
    git npm cargo pip python rustc
    make cmake gcc g++ clang
    # Add your own commands here
    kubectl docker terraform
)
```

**Note:** For commands not in the auto-wrap list, you can still use:
- `shelltape_exec command` - manual wrapping
- Create aliases for frequently used commands

**Why not capture ALL commands?**

Zsh doesn't provide a mechanism to intercept and replace arbitrary command execution. The automatic mode works by creating function wrappers for specific commands. For truly universal capture, use manual mode with selective aliases for the commands you care about.

---

Happy command tracking!
