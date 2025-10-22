# Shelltape 📼

A powerful terminal command history recorder and browser written in Rust. Shelltape records every command you run with full context (output, timing, exit codes, working directory) and provides beautiful tools to search, browse, and export your command history.

## Features

- 🎯 **Automatic Recording** - Captures commands, output, timing, and context
- 📼 **Output Capture** - PTY-based output capture preserves colors and formatting
- 🔍 **Full-Text Search** - Search through commands and their output
- 🖥️ **Interactive TUI** - Beautiful terminal UI for browsing history
- 📊 **Statistics** - Analyze your command usage patterns
- 📝 **Export** - Export commands with output to markdown format
- 🚀 **Zero Dependencies** - No SQLite, uses JSONL for storage
- 🔒 **Privacy First** - All data stored locally in `~/.shelltape/`

## Installation

### From Source

```bash
git clone https://github.com/CaddyGlow/shelltape
cd shelltape
cargo build --release
sudo cp target/release/shelltape /usr/local/bin/
```

### Install Shell Hooks

After installing the binary, set up automatic command recording:

```bash
# Auto-detect your shell and install hooks
shelltape install

# Or specify your shell explicitly
shelltape install --shell bash
shelltape install --shell zsh
shelltape install --shell fish
```

Then restart your shell or run:
```bash
source ~/.bashrc  # or ~/.zshrc for zsh
```

### Uninstall

To remove shell hooks:

```bash
# Auto-detect and uninstall
shelltape uninstall

# Or specify shell
shelltape uninstall --shell bash
```

To completely remove all data:
```bash
rm -rf ~/.shelltape/
```

## Usage

### Interactive Browser (TUI)

Launch the interactive terminal UI to browse your command history:

```bash
shelltape browse
```

**Keybindings:**
- `j`/`k` or `↑`/`↓` - Navigate up/down
- `g` - Go to first command
- `G` - Go to last command
- `Ctrl-d` / `Ctrl-u` - Page down/up
- `/` - Search mode
- `Space` - Mark/unmark command
- `a` - Mark all filtered commands
- `c` - Clear all marks
- `Enter` - View command details
- `e` - Export marked commands
- `q` - Quit

### List Commands

View recent commands in your terminal:

```bash
# List 20 most recent commands (default)
shelltape list

# List 50 commands
shelltape list -l 50

# Search for specific commands
shelltape list -f "git"
shelltape list -f "cargo build"
```

### Statistics

View statistics about your command usage:

```bash
shelltape stats
```

Shows:
- Total commands and sessions
- Success rate
- Most used commands
- Average execution time
- Storage information

### Export

Export commands to markdown format:

```bash
# Export all commands
shelltape export -o history.md

# Export from specific session
shelltape export -o session.md -s SESSION_ID

# Export filtered commands
shelltape export -o git-cmds.md -f "git"
```

### Status

Check installation status and storage information:

```bash
shelltape status
```

### Cleanup

Remove old commands from history:

```bash
# Remove commands older than 90 days (with confirmation)
shelltape clean

# Remove commands older than 30 days
shelltape clean --older-than-days 30

# Skip confirmation prompt
shelltape clean --yes
```

## How It Works

### Storage

Shelltape uses JSONL (JSON Lines) format for storage:

```
~/.shelltape/
├── commands.jsonl    # All recorded commands
├── sessions.jsonl    # Shell session metadata
└── hooks/            # Shell integration scripts
```

Each command is stored as a JSON object with:
- Command text
- Output (captured via PTY)
- Exit code
- Working directory
- Start time and duration
- Session ID
- Shell, hostname, and username

**Output Capture:** Shelltape uses PTY (pseudo-terminal) wrapping to capture command output transparently, preserving colors and formatting just as they appear in your terminal.

### Shell Integration

To use shelltape with output capture, you can:

1. **Manual Wrapping**: Use `shelltape exec` to run commands:
   ```bash
   shelltape exec -- ls -la
   shelltape exec -- git status
   ```

2. **Alias Commands** (Recommended): Add to your shell RC file:
   ```bash
   # Add after sourcing shelltape hooks
   alias ll='shelltape_exec ls -la'
   alias gs='shelltape_exec git status'
   ```

3. **Automatic Wrapping** (Advanced): The shell hooks can intercept commands automatically, though this requires careful setup to avoid breaking shell built-ins.

## Configuration

Future versions will support a config file at `~/.shelltape/config.toml`:

```toml
[recording]
max_output_size = 100_000  # bytes
exclude_patterns = ["cd", "ls", "pwd"]

[storage]
retention_days = 90
auto_cleanup = true

[ui]
default_limit = 100
```

## Development

### Prerequisites

- Rust 1.70 or later
- Nix (optional, for reproducible builds)

### Building

```bash
# Using Cargo
cargo build
cargo test

# Using Nix
nix develop  # Enter dev shell
cargo build

# Cross-compilation with Nix
nix build .#windows-x86_64
nix build .#macos-aarch64
```

### Project Structure

```
src/
├── main.rs          # Entry point
├── cli.rs           # CLI definitions
├── models.rs        # Data models
├── storage.rs       # JSONL storage layer
├── recorder.rs      # Command recording
├── install.rs       # Hook installation
├── list.rs          # List command
├── export.rs        # Export command
├── stats.rs         # Statistics
├── clean.rs         # Cleanup
├── status.rs        # Status info
└── tui/             # Terminal UI
    ├── mod.rs       # TUI entry point
    ├── app.rs       # App state
    ├── ui.rs        # UI rendering
    └── events.rs    # Event handling
```

## Roadmap

- [x] Phase 1: Foundation & Storage
- [x] Phase 2: CLI Commands
- [x] Phase 3: TUI Browser
- [x] Phase 4: Output Capture (PTY-based)
- [ ] Phase 5: Optimization & Polish
  - [ ] Binary index cache for fast search
  - [ ] Configuration file support
  - [ ] Fuzzy search
  - [ ] Session management UI
- [ ] Phase 6: Distribution
  - [ ] Homebrew formula
  - [ ] AUR package
  - [ ] Debian package

## Comparison with Other Tools

| Feature | Shelltape | Atuin | McFly |
|---------|-----------|-------|-------|
| Storage | JSONL | SQLite | SQLite |
| Output Capture | ✓ (PTY) | ✗ | ✗ |
| TUI Browser | ✓ | ✓ | ✓ |
| Export | ✓ | Limited | ✗ |
| Zero C deps | ✓ | ✗ | ✗ |
| Sync Support | Planned | ✓ | ✗ |

## Privacy & Security

- All data stored locally in `~/.shelltape/`
- No telemetry or external connections
- Human-readable JSONL format
- Easy to backup, sync, or delete

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI
- Inspired by [Atuin](https://github.com/atuinsh/atuin) and [McFly](https://github.com/cantino/mcfly)
