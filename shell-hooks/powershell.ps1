# Shelltape PowerShell Profile Integration
# Add this to your PowerShell profile to enable command recording

# Generate session ID once per PowerShell session
if (-not $env:SHELLTAPE_SESSION_ID) {
    $env:SHELLTAPE_SESSION_ID = [guid]::NewGuid().ToString()
}

# Configuration: Set to $true for automatic capture, $false for manual
if (-not $env:SHELLTAPE_AUTO_CAPTURE) {
    $env:SHELLTAPE_AUTO_CAPTURE = "0"
}

# Helper function for wrapping commands
function Invoke-ShelltapeExec {
    param(
        [Parameter(Mandatory=$true, Position=0)]
        [string]$Command,

        [Parameter(ValueFromRemainingArguments=$true)]
        [string[]]$Arguments
    )

    # Don't wrap shelltape commands or certain simple commands
    if ($Command -match '^(shelltape|cd|pwd|exit|cls|clear)$') {
        # Execute directly
        & $Command @Arguments
    }
    else {
        # Build the full command string
        $fullCommand = $Command
        if ($Arguments) {
            $fullCommand += " " + ($Arguments -join " ")
        }

        # Execute through shelltape wrapper for output capture
        & shelltape exec --session-id $env:SHELLTAPE_SESSION_ID -- $Command @Arguments
    }
}

# Alias for convenience
Set-Alias -Name stexec -Value Invoke-ShelltapeExec

# Automatic capture mode using proxy functions
if ($env:SHELLTAPE_AUTO_CAPTURE -eq "1") {
    # List of commands to auto-wrap
    $AutoCaptureCommands = @(
        'ls', 'dir', 'cat', 'type', 'Get-Content', 'Get-ChildItem',
        'Select-String', 'findstr', 'where', 'sort',
        'git', 'npm', 'pip', 'python', 'node', 'cargo', 'rustc',
        'docker', 'kubectl', 'terraform',
        'ping', 'ipconfig', 'netstat', 'tracert',
        'Get-Process', 'Get-Service', 'Get-EventLog'
    )

    foreach ($cmd in $AutoCaptureCommands) {
        # Check if command exists
        if (Get-Command $cmd -ErrorAction SilentlyContinue) {
            # Create a proxy function that wraps the original command
            $scriptBlock = {
                param([Parameter(ValueFromRemainingArguments=$true)]$args)

                # Get the original command name from the function name
                $cmdName = $MyInvocation.MyCommand.Name

                # Execute through shelltape
                & shelltape exec --session-id $env:SHELLTAPE_SESSION_ID -- $cmdName @args
            }.GetNewClosure()

            # Remove any existing alias or function with this name
            if (Get-Alias $cmd -ErrorAction SilentlyContinue) {
                Remove-Alias $cmd -Force
            }

            # Create the wrapper function
            New-Item -Path Function: -Name "global:$cmd" -Value $scriptBlock -Force | Out-Null
        }
    }
}

# Manual mode helper aliases
# You can create custom aliases for frequently used commands
# Examples:
#   Set-Alias -Name ll -Value Invoke-ShelltapeExec
#   function gs { Invoke-ShelltapeExec git status @args }
#   function gd { Invoke-ShelltapeExec git diff @args }

# Usage information
function Show-ShelltapeHelp {
    Write-Host @"
Shelltape PowerShell Integration
=================================

Manual Mode (default):
  stexec ls -la
  stexec git status
  Invoke-ShelltapeExec npm test

Automatic Mode (SHELLTAPE_AUTO_CAPTURE=1):
  ls               # Automatically captured
  git status       # Automatically captured
  Get-ChildItem    # Automatically captured

View Commands:
  shelltape list              # List recent commands
  shelltape browse            # Interactive browser (TUI)
  shelltape list -f "git"     # Search for git commands
  shelltape export -o log.md  # Export to markdown

Statistics:
  shelltape stats             # Show statistics

Session ID: $env:SHELLTAPE_SESSION_ID
Auto-Capture: $env:SHELLTAPE_AUTO_CAPTURE

To enable automatic capture for this session:
  `$env:SHELLTAPE_AUTO_CAPTURE = "1"
  . `$PROFILE

To make it permanent, add to your PowerShell profile:
  `$env:SHELLTAPE_AUTO_CAPTURE = "1"
"@
}

Set-Alias -Name shelltape-help -Value Show-ShelltapeHelp

# Display brief info on load (only in interactive mode)
if ($Host.UI.RawUI -and -not $env:SHELLTAPE_QUIET) {
    Write-Host "Shelltape loaded! " -NoNewline -ForegroundColor Green
    Write-Host "Type 'shelltape-help' for usage info. " -ForegroundColor Cyan
    if ($env:SHELLTAPE_AUTO_CAPTURE -eq "1") {
        Write-Host "Auto-capture is ENABLED" -ForegroundColor Yellow
    } else {
        Write-Host "Auto-capture is OFF (use 'stexec' or aliases)" -ForegroundColor Gray
    }
}

# Export functions for module-based loading
Export-ModuleMember -Function Invoke-ShelltapeExec, Show-ShelltapeHelp -Alias stexec, shelltape-help
