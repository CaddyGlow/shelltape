use crate::models::Command;
use crate::storage::Storage;
use anyhow::{Context, Result};
use chrono::DateTime;

/// Command recorder that captures command execution details
pub struct Recorder {
    storage: Storage,
    max_output_size: usize,
}

impl Recorder {
    /// Create a new Recorder with default settings
    pub fn new() -> Result<Self> {
        Ok(Self {
            storage: Storage::new()?,
            max_output_size: 100_000, // 100KB default
        })
    }

    /// Create a new Recorder with custom storage
    #[allow(dead_code)]
    pub fn with_storage(storage: Storage) -> Self {
        Self {
            storage,
            max_output_size: 100_000,
        }
    }

    /// Set the maximum output size in bytes
    #[allow(dead_code)]
    pub fn with_max_output_size(mut self, size: usize) -> Self {
        self.max_output_size = size;
        self
    }

    /// Record a command execution
    #[allow(clippy::too_many_arguments)]
    pub fn record(
        &self,
        command: String,
        output: String,
        exit_code: i32,
        start_time: i64, // nanoseconds since epoch
        end_time: i64,   // nanoseconds since epoch
        cwd: String,
        session_id: String,
    ) -> Result<()> {
        // Convert nanoseconds to DateTime
        let started_at = DateTime::from_timestamp_nanos(start_time);

        // Calculate duration in milliseconds
        let duration_ms = ((end_time - start_time) / 1_000_000) as u64;

        // Get system information
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        let cmd = Command {
            id: uuid::Uuid::new_v4().to_string(),
            command,
            output: self.truncate_output(output),
            exit_code,
            cwd,
            started_at,
            duration_ms,
            session_id,
            shell,
            hostname,
            username,
        };

        self.storage
            .append_command(&cmd)
            .with_context(|| "Failed to record command")?;

        Ok(())
    }

    /// Truncate output to maximum size
    fn truncate_output(&self, output: String) -> String {
        if output.len() <= self.max_output_size {
            output
        } else {
            let truncated = &output[..self.max_output_size];
            format!(
                "{}...\n[Output truncated: {} bytes total]",
                truncated,
                output.len()
            )
        }
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new().expect("Failed to create default recorder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_record_command() {
        let dir = tempdir().unwrap();
        let storage = Storage::with_dir(dir.path().to_path_buf()).unwrap();
        let recorder = Recorder::with_storage(storage);

        let start = Utc::now().timestamp_nanos_opt().unwrap();
        let end = start + 10_000_000; // 10ms later

        recorder
            .record(
                "echo test".to_string(),
                "test\n".to_string(),
                0,
                start,
                end,
                "/tmp".to_string(),
                "session-1".to_string(),
            )
            .unwrap();

        let commands = recorder.storage.read_all_commands().unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo test");
        assert_eq!(commands[0].exit_code, 0);
    }

    #[test]
    fn test_truncate_output() {
        let dir = tempdir().unwrap();
        let storage = Storage::with_dir(dir.path().to_path_buf()).unwrap();
        let recorder = Recorder::with_storage(storage).with_max_output_size(100);

        let large_output = "a".repeat(200);
        let start = Utc::now().timestamp_nanos_opt().unwrap();
        let end = start + 10_000_000;

        recorder
            .record(
                "echo test".to_string(),
                large_output,
                0,
                start,
                end,
                "/tmp".to_string(),
                "session-1".to_string(),
            )
            .unwrap();

        let commands = recorder.storage.read_all_commands().unwrap();
        assert_eq!(commands.len(), 1);
        assert!(commands[0].output.contains("[Output truncated"));
    }
}
