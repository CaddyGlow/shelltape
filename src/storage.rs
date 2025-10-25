use crate::models::{Command, Session, Stats};
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/// Storage manager for shelltape data
pub struct Storage {
    data_dir: PathBuf,
    commands_file: PathBuf,
    sessions_file: PathBuf,
}

impl Storage {
    /// Create a new Storage instance using the default data directory (~/.shelltape/)
    pub fn new() -> Result<Self> {
        let data_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?
            .join(".shelltape");

        Self::with_dir(data_dir)
    }

    /// Create a new Storage instance with a custom data directory
    pub fn with_dir(data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&data_dir)
            .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;

        let commands_file = data_dir.join("commands.jsonl");
        let sessions_file = data_dir.join("sessions.jsonl");

        Ok(Self {
            data_dir,
            commands_file,
            sessions_file,
        })
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Append a command to the commands file
    pub fn append_command(&self, cmd: &Command) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.commands_file)
            .with_context(|| {
                format!(
                    "Failed to open commands file: {}",
                    self.commands_file.display()
                )
            })?;

        let json =
            serde_json::to_string(cmd).with_context(|| "Failed to serialize command to JSON")?;

        writeln!(file, "{}", json).with_context(|| "Failed to write command to file")?;

        Ok(())
    }

    /// Read all commands from the commands file
    pub fn read_all_commands(&self) -> Result<Vec<Command>> {
        if !self.commands_file.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.commands_file).with_context(|| {
            format!(
                "Failed to open commands file: {}",
                self.commands_file.display()
            )
        })?;

        let reader = BufReader::new(file);
        let mut commands = Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.with_context(|| {
                format!("Failed to read line {} from commands file", line_num + 1)
            })?;

            if line.trim().is_empty() {
                continue;
            }

            let cmd: Command = serde_json::from_str(&line).with_context(|| {
                format!(
                    "Failed to parse command from line {} in commands file",
                    line_num + 1
                )
            })?;

            commands.push(cmd);
        }

        Ok(commands)
    }

    /// Search for commands matching a query string
    pub fn search_commands(&self, query: &str, limit: usize) -> Result<Vec<Command>> {
        let all_commands = self.read_all_commands()?;
        let query_lower = query.to_lowercase();

        let mut results: Vec<Command> = all_commands
            .into_iter()
            .filter(|cmd| {
                cmd.command.to_lowercase().contains(&query_lower)
                    || cmd.cwd.to_lowercase().contains(&query_lower)
                    || cmd.output.to_lowercase().contains(&query_lower)
            })
            .collect();

        // Sort by most recent first
        results.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        results.truncate(limit);

        Ok(results)
    }

    /// Get the most recent commands
    pub fn get_recent_commands(&self, limit: usize) -> Result<Vec<Command>> {
        let mut commands = self.read_all_commands()?;
        commands.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        commands.truncate(limit);
        Ok(commands)
    }

    /// Append a session to the sessions file
    #[allow(dead_code)]
    pub fn append_session(&self, session: &Session) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.sessions_file)
            .with_context(|| {
                format!(
                    "Failed to open sessions file: {}",
                    self.sessions_file.display()
                )
            })?;

        let json = serde_json::to_string(session)
            .with_context(|| "Failed to serialize session to JSON")?;

        writeln!(file, "{}", json).with_context(|| "Failed to write session to file")?;

        Ok(())
    }

    /// Read all sessions from the sessions file
    pub fn read_all_sessions(&self) -> Result<Vec<Session>> {
        if !self.sessions_file.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.sessions_file).with_context(|| {
            format!(
                "Failed to open sessions file: {}",
                self.sessions_file.display()
            )
        })?;

        let reader = BufReader::new(file);
        let mut sessions = Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.with_context(|| {
                format!("Failed to read line {} from sessions file", line_num + 1)
            })?;

            if line.trim().is_empty() {
                continue;
            }

            let session: Session = serde_json::from_str(&line).with_context(|| {
                format!(
                    "Failed to parse session from line {} in sessions file",
                    line_num + 1
                )
            })?;

            sessions.push(session);
        }

        Ok(sessions)
    }

    /// Update a session's end time
    #[allow(dead_code)]
    pub fn update_session(&self, session_id: &str, ended_at: DateTime<Utc>) -> Result<()> {
        let mut sessions = self.read_all_sessions()?;

        // Find and update the matching session
        let updated = sessions.iter_mut().any(|session| {
            if session.id == session_id {
                session.ended_at = Some(ended_at);
                true
            } else {
                false
            }
        });

        if !updated {
            return Err(anyhow!("Session not found: {}", session_id));
        }

        // Rewrite the entire sessions file
        self.rewrite_sessions(&sessions)?;

        Ok(())
    }

    /// Rewrite the sessions file with the provided sessions
    #[allow(dead_code)]
    fn rewrite_sessions(&self, sessions: &[Session]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.sessions_file)
            .with_context(|| {
                format!(
                    "Failed to open sessions file for writing: {}",
                    self.sessions_file.display()
                )
            })?;

        for session in sessions {
            let json = serde_json::to_string(session)
                .with_context(|| "Failed to serialize session to JSON")?;
            writeln!(file, "{}", json).with_context(|| "Failed to write session to file")?;
        }

        Ok(())
    }

    /// Rewrite the commands file with the provided commands
    fn rewrite_commands(&self, commands: &[Command]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.commands_file)
            .with_context(|| {
                format!(
                    "Failed to open commands file for writing: {}",
                    self.commands_file.display()
                )
            })?;

        for cmd in commands {
            let json = serde_json::to_string(cmd)
                .with_context(|| "Failed to serialize command to JSON")?;
            writeln!(file, "{}", json).with_context(|| "Failed to write command to file")?;
        }

        Ok(())
    }

    /// Clean up old commands older than the specified number of days
    pub fn cleanup_old_commands(&self, days: u64) -> Result<usize> {
        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        let commands = self.read_all_commands()?;

        let (keep, remove): (Vec<_>, Vec<_>) = commands
            .into_iter()
            .partition(|cmd| cmd.started_at > cutoff);

        // Rewrite file with only kept commands
        self.rewrite_commands(&keep)?;

        Ok(remove.len())
    }

    /// Get statistics about the command history
    pub fn get_stats(&self) -> Result<Stats> {
        let commands = self.read_all_commands()?;
        let sessions = self.read_all_sessions()?;

        let total_commands = commands.len();
        let total_sessions = sessions.len();

        // Calculate success rate
        let successful = commands.iter().filter(|cmd| cmd.exit_code == 0).count();
        let success_rate = if total_commands > 0 {
            (successful as f64 / total_commands as f64) * 100.0
        } else {
            0.0
        };

        // Calculate most used commands
        let mut command_counts: HashMap<String, usize> = HashMap::new();
        for cmd in &commands {
            *command_counts.entry(cmd.command.clone()).or_insert(0) += 1;
        }

        let mut most_used: Vec<(String, usize)> = command_counts.into_iter().collect();
        most_used.sort_by(|a, b| b.1.cmp(&a.1));
        most_used.truncate(10);

        Ok(Stats {
            total_commands,
            total_sessions,
            success_rate,
            most_used_commands: most_used,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_storage_append_and_read() {
        let dir = tempdir().unwrap();
        let storage = Storage::with_dir(dir.path().to_path_buf()).unwrap();

        let cmd = Command {
            id: "test-1".to_string(),
            command: "echo hello".to_string(),
            output: "hello\n".to_string(),
            exit_code: 0,
            cwd: "/tmp".to_string(),
            started_at: Utc::now(),
            duration_ms: 10,
            session_id: "session-1".to_string(),
            shell: "bash".to_string(),
            hostname: "localhost".to_string(),
            username: "testuser".to_string(),
        };

        storage.append_command(&cmd).unwrap();
        let commands = storage.read_all_commands().unwrap();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo hello");
    }

    #[test]
    fn test_search() {
        let dir = tempdir().unwrap();
        let storage = Storage::with_dir(dir.path().to_path_buf()).unwrap();

        let cmd1 = Command {
            id: "test-1".to_string(),
            command: "echo hello".to_string(),
            output: "hello\n".to_string(),
            exit_code: 0,
            cwd: "/tmp".to_string(),
            started_at: Utc::now(),
            duration_ms: 10,
            session_id: "session-1".to_string(),
            shell: "bash".to_string(),
            hostname: "localhost".to_string(),
            username: "testuser".to_string(),
        };

        let cmd2 = Command {
            id: "test-2".to_string(),
            command: "ls -la".to_string(),
            output: "total 0\n".to_string(),
            exit_code: 0,
            cwd: "/tmp".to_string(),
            started_at: Utc::now(),
            duration_ms: 5,
            session_id: "session-1".to_string(),
            shell: "bash".to_string(),
            hostname: "localhost".to_string(),
            username: "testuser".to_string(),
        };

        storage.append_command(&cmd1).unwrap();
        storage.append_command(&cmd2).unwrap();

        let results = storage.search_commands("echo", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].command, "echo hello");
    }
}
