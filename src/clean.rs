use crate::storage::Storage;
use anyhow::Result;
use std::io::{self, Write};

/// Clean old commands from history
pub fn clean_commands(older_than_days: u64, yes: bool) -> Result<()> {
    let storage = Storage::new()?;

    // Get count before cleaning
    let commands_before = storage.read_all_commands()?;
    let total_before = commands_before.len();

    if total_before == 0 {
        println!("No commands to clean");
        return Ok(());
    }

    // Count how many would be removed
    let cutoff = chrono::Utc::now() - chrono::Duration::days(older_than_days as i64);
    let would_remove = commands_before
        .iter()
        .filter(|cmd| cmd.started_at < cutoff)
        .count();

    if would_remove == 0 {
        println!("No commands older than {} days found", older_than_days);
        return Ok(());
    }

    println!(
        "⚠️  This will remove {} out of {} commands (older than {} days)",
        would_remove, total_before, older_than_days
    );

    // Ask for confirmation unless --yes flag is set
    if !yes {
        print!("Continue? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled");
            return Ok(());
        }
    }

    // Perform cleanup
    let removed = storage.cleanup_old_commands(older_than_days)?;

    println!("✓ Removed {} commands", removed);
    println!("  Remaining: {} commands", total_before - removed);

    Ok(())
}
