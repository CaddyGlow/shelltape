# Shelltape for PowerShell (Windows)

Complete guide for using Shelltape with PowerShell on Windows.

## Installation

### Step 1: Install Shelltape

```powershell
# Download the Windows binary or build from source
# Add shelltape.exe to your PATH

# Verify installation
shelltape --version
```

### Step 2: Install PowerShell Hooks

```powershell
shelltape install --shell powershell
```

This will:
- Create `~\.shelltape\` directory
- Copy `powershell.ps1` to `~\.shelltape\`
- Add integration to your PowerShell profile

### Step 3: Reload Your Profile

```powershell
# Option 1: Restart PowerShell

# Option 2: Reload profile
. $PROFILE
```

## Usage

### Manual Mode (Default)

Use the `stexec` alias or `Invoke-ShelltapeExec`:

```powershell
# Using the alias
stexec git status
stexec ls -la
stexec Get-Process

# Using the full function name
Invoke-ShelltapeExec npm test
Invoke-ShelltapeExec cargo build
```

### Automatic Mode

Enable automatic capture for common commands:

```powershell
# Enable for current session
$env:SHELLTAPE_AUTO_CAPTURE = "1"
. $PROFILE

# Make it permanent: Add to your profile BEFORE the shelltape line
# Edit profile:
notepad $PROFILE

# Add this line at the top:
$env:SHELLTAPE_AUTO_CAPTURE = "1"
```

When automatic mode is enabled, these commands are captured automatically:
- **File operations:** ls, dir, cat, type, Get-Content, Get-ChildItem
- **Search:** Select-String, findstr, where, sort
- **Development:** git, npm, pip, python, node, cargo, rustc
- **Containers:** docker, kubectl, terraform
- **Network:** ping, ipconfig, netstat, tracert
- **PowerShell cmdlets:** Get-Process, Get-Service, Get-EventLog

### Creating Aliases

Add to your PowerShell profile for quick access:

```powershell
# Git aliases
function gs { stexec git status $args }
function gd { stexec git diff $args }
function gl { stexec git log $args }
function ga { stexec git add $args }
function gc { stexec git commit $args }
function gp { stexec git push $args }

# Common commands
function ll { stexec ls -la $args }
function cat { stexec Get-Content $args }
function grep { stexec Select-String $args }

# Development
function npm { stexec npm $args }
function cargo { stexec cargo $args }
function python { stexec python $args }
```

## Viewing History

### List Commands

```powershell
shelltape list                  # Last 20 commands
shelltape list -l 50            # Last 50 commands
shelltape list -f "git"         # Search for git commands
shelltape list -f "error"       # Find commands with errors in output
```

### Interactive Browser (TUI)

```powershell
shelltape browse
```

**Navigation:**
- `j`/`k` or `↑`/`↓` - Move up/down
- `/` - Search commands and output
- `Space` - Mark command for export
- `a` - Mark all
- `c` - Clear marks
- `Enter` - View full details
- `e` - Export marked commands
- `q` - Quit

### Export to Markdown

```powershell
shelltape export -o history.md                    # Export all
shelltape export -o git-log.md -f "git"          # Export git commands only
shelltape export -o session.md -s $env:SHELLTAPE_SESSION_ID  # Current session
```

### View Statistics

```powershell
shelltape stats
```

Shows:
- Total commands executed
- Success/failure rate
- Most frequently used commands
- Average execution time

## Tips & Tricks

### Check Your Session ID

```powershell
$env:SHELLTAPE_SESSION_ID
```

### View Raw Data

All data is stored in `~\.shelltape\commands.jsonl`:

```powershell
# View all commands (requires jq or ConvertFrom-Json)
Get-Content ~\.shelltape\commands.jsonl | ForEach-Object { $_ | ConvertFrom-Json }

# Count commands
(Get-Content ~\.shelltape\commands.jsonl).Count

# Last command
Get-Content ~\.shelltape\commands.jsonl | Select-Object -Last 1 | ConvertFrom-Json
```

### Test Capture

```powershell
# Run a test command
stexec echo "Hello Shelltape"

# Verify it was captured
shelltape list -l 1

# Check output
Get-Content ~\.shelltape\commands.jsonl | Select-Object -Last 1 | ConvertFrom-Json | Select-Object -ExpandProperty output
```

### Customize Auto-Capture Commands

Edit `~\.shelltape\powershell.ps1` and modify the `$AutoCaptureCommands` array:

```powershell
$AutoCaptureCommands = @(
    'git', 'npm', 'cargo',
    # Add your own commands
    'dotnet', 'msbuild', 'nuget'
)
```

Then reload: `. $PROFILE`

## Maintenance

### Clean Old Data

```powershell
# Remove commands older than 30 days
shelltape clean --older-than-days 30

# Remove without confirmation
shelltape clean --older-than-days 90 -y
```

### Check Status

```powershell
shelltape status
```

### Uninstall

```powershell
shelltape uninstall --shell powershell

# Optionally remove all data
Remove-Item -Recurse -Force ~\.shelltape\
```

## Troubleshooting

### Commands Not Being Captured?

**Manual mode:**
```powershell
# Make sure you're using stexec
stexec git status  # ✓ Captured
git status         # ✗ Not captured (unless auto-capture is on)
```

**Automatic mode:**
```powershell
# Check if auto-capture is enabled
$env:SHELLTAPE_AUTO_CAPTURE  # Should be "1"

# Enable it
$env:SHELLTAPE_AUTO_CAPTURE = "1"
. $PROFILE
```

### Check Installation

```powershell
# Verify hooks are installed
shelltape status

# Check profile integration
Get-Content $PROFILE | Select-String "shelltape"
```

### Execution Policy Issues

If you get execution policy errors:

```powershell
# Check current policy
Get-ExecutionPolicy

# Set to RemoteSigned (recommended)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Profile Not Found

```powershell
# Create profile directory if it doesn't exist
New-Item -Path (Split-Path $PROFILE) -ItemType Directory -Force

# Create profile file
New-Item -Path $PROFILE -ItemType File -Force
```

## What Gets Captured?

For each command executed through Shelltape:

- **Command text** - Full command with arguments
- **Output** - Complete stdout and stderr (merged)
- **Exit code** - Success/failure status
- **Timing** - Nanosecond-precision start/end times
- **Working directory** - Where command was run
- **Session ID** - Groups commands by PowerShell session
- **Environment** - Shell type, hostname, username
- **Metadata** - Timestamps, duration

## Example Workflows

### Development Session

```powershell
# Start working
cd C:\Projects\MyApp

# Run and capture build commands
stexec dotnet build
stexec dotnet test
stexec git status

# Review what you did
shelltape list -l 10

# Export session for documentation
shelltape export -o build-log.md -s $env:SHELLTAPE_SESSION_ID
```

### Debugging Session

```powershell
# Run commands with auto-capture on
$env:SHELLTAPE_AUTO_CAPTURE = "1"
. $PROFILE

# Run various debug commands
Get-Process myapp
netstat -ano
ipconfig /all
Get-EventLog -LogName Application -Newest 10

# Later, find what you ran
shelltape list -f "myapp"
shelltape browse  # Search for specific errors
```

### Git Workflow

```powershell
# Create git aliases (add to profile)
function gs { stexec git status $args }
function gd { stexec git diff $args }
function gl { stexec git log --oneline -10 $args }

# Use them
gs
gd
gl

# Export git history
shelltape export -o git-session.md -f "git"
```

## Advanced Configuration

### Silent Loading

Suppress the welcome message when PowerShell starts:

```powershell
# Add to profile BEFORE sourcing shelltape
$env:SHELLTAPE_QUIET = "1"
. ~\.shelltape\powershell.ps1
```

### Custom Storage Location

Shelltape stores data in `~\.shelltape\` by default. To change this, you'll need to set `SHELLTAPE_DATA_DIR` before running commands (this requires modifying the source code currently).

### Integration with PSReadLine

Shelltape works seamlessly with PSReadLine:

```powershell
# Your PSReadLine config works normally
Set-PSReadLineOption -PredictionSource History
Set-PSReadLineOption -EditMode Emacs
```

## PowerShell-Specific Features

### Cmdlet Support

PowerShell cmdlets work great with shelltape:

```powershell
stexec Get-Process | Where-Object CPU -gt 100
stexec Get-Service | Where-Object Status -eq "Running"
stexec Get-ChildItem -Recurse -Filter *.ps1
```

### Pipeline Capture

Full PowerShell pipelines are captured:

```powershell
stexec Get-Process | Sort-Object CPU -Descending | Select-Object -First 10
```

### Module Functions

Your custom module functions can be wrapped:

```powershell
function MyCustomFunction { stexec My-ModuleCommand $args }
```

---

## See Also

- [USAGE.md](USAGE.md) - General usage guide (Unix shells)
- [README.md](README.md) - Project overview
- [PLAN.md](PLAN.md) - Development roadmap

---

Happy command tracking on Windows!
