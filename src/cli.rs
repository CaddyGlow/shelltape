use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "shelltape")]
#[command(about = "Record and browse your terminal command history", long_about = None)]
#[command(version = env!("GDL_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install shell hooks for automatic command recording
    Install {
        /// Shell to install hooks for (auto-detected if not specified)
        #[arg(short, long)]
        shell: Option<Shell>,
    },

    /// Uninstall shell hooks
    Uninstall {
        /// Shell to uninstall hooks from (auto-detected if not specified)
        #[arg(short, long)]
        shell: Option<Shell>,
    },

    /// Execute a command with output capture (wrapper mode)
    Exec {
        /// The command to execute
        #[arg(required = true)]
        command: Vec<String>,

        /// Session ID for this shell session
        #[arg(long)]
        session_id: String,
    },

    /// Record a command (called by shell hooks)
    Record {
        /// The command that was executed
        #[arg(long)]
        command: String,

        /// Exit code from the command
        #[arg(long)]
        exit_code: i32,

        /// Start time in nanoseconds since epoch
        #[arg(long)]
        start_time: i64,

        /// End time in nanoseconds since epoch
        #[arg(long)]
        end_time: i64,

        /// Working directory when command was executed
        #[arg(long)]
        cwd: String,

        /// Session ID for this shell session
        #[arg(long)]
        session_id: String,

        /// Command output (optional)
        #[arg(long, default_value = "")]
        output: String,
    },

    /// Browse commands interactively (TUI)
    Browse,

    /// List recent commands
    List {
        /// Maximum number of commands to display
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Filter commands by query string
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Export commands to markdown
    Export {
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Filter by session ID
        #[arg(short, long)]
        session: Option<String>,

        /// Filter by query string
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Show statistics about command history
    Stats,

    /// Clean old commands from history
    Clean {
        /// Remove commands older than this many days
        #[arg(long, default_value = "90")]
        older_than_days: u64,

        /// Don't ask for confirmation
        #[arg(short, long)]
        yes: bool,
    },

    /// Show status and storage information
    Status,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Shell {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// PowerShell (Windows)
    Powershell,
}

impl Shell {
    /// Get the RC file path for this shell
    pub fn rc_file(&self) -> &'static str {
        match self {
            Shell::Bash => ".bashrc",
            Shell::Zsh => ".zshrc",
            Shell::Fish => ".config/fish/config.fish",
            Shell::Powershell => "Documents/PowerShell/Microsoft.PowerShell_profile.ps1",
        }
    }

    /// Get the hook file name for this shell
    pub fn hook_file(&self) -> &'static str {
        match self {
            Shell::Bash => "bash.sh",
            Shell::Zsh => "zsh.sh",
            Shell::Fish => "fish.fish",
            Shell::Powershell => "powershell.ps1",
        }
    }

    /// Detect the current shell from environment
    pub fn detect() -> Option<Self> {
        // On Windows, check for PowerShell first
        #[cfg(target_os = "windows")]
        {
            // Check if we're running in PowerShell
            if std::env::var("PSModulePath").is_ok() {
                return Some(Shell::Powershell);
            }
        }

        // Unix shells use SHELL environment variable
        let shell = std::env::var("SHELL").ok()?;

        if shell.contains("bash") {
            Some(Shell::Bash)
        } else if shell.contains("zsh") {
            Some(Shell::Zsh)
        } else if shell.contains("fish") {
            Some(Shell::Fish)
        } else {
            None
        }
    }
}
