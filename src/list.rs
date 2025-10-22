use crate::storage::Storage;
use anyhow::Result;

/// List recent commands
pub fn list_commands(limit: usize, filter: Option<String>) -> Result<()> {
    let storage = Storage::new()?;

    let commands = if let Some(query) = filter {
        storage.search_commands(&query, limit)?
    } else {
        storage.get_recent_commands(limit)?
    };

    if commands.is_empty() {
        println!("No commands found");
        return Ok(());
    }

    // Print header
    println!("{:<20} {:<8} {:<50} DIRECTORY", "TIME", "STATUS", "COMMAND");
    println!("{}", "─".repeat(100));

    // Print commands
    for cmd in &commands {
        let time = cmd.started_at.format("%Y-%m-%d %H:%M:%S");

        let status_display = if cmd.exit_code == 0 {
            "✓".to_string()
        } else {
            format!("✗ {}", cmd.exit_code)
        };

        let command_display = if cmd.command.len() > 50 {
            format!("{}...", &cmd.command[..47])
        } else {
            cmd.command.clone()
        };

        let cwd_display = if cmd.cwd.len() > 30 {
            format!("...{}", &cmd.cwd[cmd.cwd.len() - 27..])
        } else {
            cmd.cwd.clone()
        };

        println!(
            "{:<20} {:<8} {:<50} {}",
            time, status_display, command_display, cwd_display
        );
    }

    println!("\nTotal: {} commands", commands.len());

    Ok(())
}
