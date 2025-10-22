use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single command execution record
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    /// Unique identifier (UUID)
    pub id: String,
    /// The command that was executed
    pub command: String,
    /// Output from the command (may be truncated)
    pub output: String,
    /// Exit code from the command
    pub exit_code: i32,
    /// Working directory when command was executed
    pub cwd: String,
    /// Timestamp when command started
    pub started_at: DateTime<Utc>,
    /// Duration of command execution in milliseconds
    pub duration_ms: u64,
    /// Session ID this command belongs to
    pub session_id: String,
    /// Shell type (bash, zsh, fish, etc.)
    pub shell: String,
    /// Hostname where command was executed
    pub hostname: String,
    /// Username who executed the command
    pub username: String,
}

/// A shell session record
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    /// Unique identifier (UUID)
    pub id: String,
    /// Timestamp when session started
    pub started_at: DateTime<Utc>,
    /// Timestamp when session ended (None if still active)
    pub ended_at: Option<DateTime<Utc>>,
    /// Hostname where session was started
    pub hostname: String,
    /// Shell type (bash, zsh, fish, etc.)
    pub shell: String,
    /// Number of commands executed in this session
    pub command_count: u32,
}

/// Optional search index for fast queries
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Total number of commands in the index
    pub commands_count: usize,
    /// When the index was last updated
    pub last_updated: DateTime<Utc>,
    /// List of all session IDs
    pub sessions: Vec<String>,
}

/// Statistics about command history
#[derive(Debug, Clone)]
pub struct Stats {
    /// Total number of commands recorded
    pub total_commands: usize,
    /// Total number of sessions
    pub total_sessions: usize,
    /// Success rate (percentage of commands with exit code 0)
    pub success_rate: f64,
    /// Most frequently used commands
    pub most_used_commands: Vec<(String, usize)>,
}
