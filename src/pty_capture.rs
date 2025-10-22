use anyhow::{Context, Result};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// Result of command execution with captured output
pub struct ExecutionResult {
    pub output: String,
    pub exit_code: i32,
    pub start_time: i64,
    pub end_time: i64,
}

/// Execute a command in a PTY and capture its output
pub fn execute_with_capture(command: &str, cwd: &str) -> Result<ExecutionResult> {
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Failed to get start time")?
        .as_nanos() as i64;

    let pty_system = NativePtySystem::default();

    // Get current terminal size or use defaults
    let (rows, cols) = if let Ok((w, h)) = crossterm::terminal::size() {
        (h, w)
    } else {
        (24, 80)
    };

    // Create a PTY with terminal size
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("Failed to open PTY")?;

    // Parse the command into program and args
    let (program, args) = parse_command(command);

    // Build the command
    let mut cmd = CommandBuilder::new(program);
    cmd.args(args);
    cmd.cwd(cwd);

    // Spawn the command in the PTY
    let mut child = pair
        .slave
        .spawn_command(cmd)
        .context("Failed to spawn command")?;

    // Drop the slave side so we can read from master
    drop(pair.slave);

    // Read output from PTY master
    let mut reader = pair.master.try_clone_reader().context("Failed to clone reader")?;
    let output = Arc::new(Mutex::new(Vec::new()));
    let output_clone = Arc::clone(&output);

    // Spawn thread to read output and display it in real-time
    let read_thread = thread::spawn(move || {
        let mut buffer = [0u8; 8192];
        let mut stdout = std::io::stdout();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    // Write to stdout for user to see
                    let _ = stdout.write_all(&buffer[..n]);
                    let _ = stdout.flush();

                    // Also save to buffer
                    if let Ok(mut out) = output_clone.lock() {
                        out.extend_from_slice(&buffer[..n]);
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Handle stdin forwarding for interactive apps
    let mut writer = pair
        .master
        .take_writer()
        .context("Failed to get writer")?;

    // Spawn thread to forward stdin to PTY (for interactive commands)
    let stdin_thread = thread::spawn(move || {
        let mut stdin = std::io::stdin();
        let mut buffer = [0u8; 8192];
        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if writer.write_all(&buffer[..n]).is_err() {
                        break;
                    }
                    if writer.flush().is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Wait for child to exit
    let exit_status = child.wait().context("Failed to wait for child")?;

    // Wait for read thread to finish
    let _ = read_thread.join();

    // stdin_thread will stop when child exits and writer is dropped
    drop(stdin_thread);

    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Failed to get end time")?
        .as_nanos() as i64;

    // Convert output to string
    let output_bytes = output.lock().unwrap();
    let output_string = String::from_utf8_lossy(&output_bytes).to_string();

    // Get exit code
    let exit_code = exit_status.exit_code() as i32;

    Ok(ExecutionResult {
        output: output_string,
        exit_code,
        start_time,
        end_time,
    })
}

/// Parse a command string into program and arguments
/// This is a simple parser - for complex commands, consider using a shell
fn parse_command(command: &str) -> (&str, Vec<&str>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return ("", vec![]);
    }

    let program = parts[0];
    let args = parts[1..].to_vec();

    (program, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_echo() {
        let result = execute_with_capture("echo hello", "/tmp").unwrap();
        assert!(result.output.contains("hello"));
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_execute_with_args() {
        let result = execute_with_capture("echo foo bar baz", "/tmp").unwrap();
        assert!(result.output.contains("foo"));
        assert!(result.output.contains("bar"));
        assert!(result.output.contains("baz"));
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_failed_command() {
        let result = execute_with_capture("false", "/tmp").unwrap();
        assert_eq!(result.exit_code, 1);
    }

    #[test]
    fn test_parse_command() {
        let (prog, args) = parse_command("echo hello world");
        assert_eq!(prog, "echo");
        assert_eq!(args, vec!["hello", "world"]);
    }
}
