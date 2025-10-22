use crate::storage::Storage;
use anyhow::Result;

/// Show statistics about command history
pub fn show_stats() -> Result<()> {
    let storage = Storage::new()?;
    let stats = storage.get_stats()?;

    println!("╔════════════════════════════════════════════════╗");
    println!("║          Shelltape Statistics                  ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();

    println!("📊 Overview:");
    println!("  • Total Commands:  {}", stats.total_commands);
    println!("  • Total Sessions:  {}", stats.total_sessions);
    println!("  • Success Rate:    {:.1}%", stats.success_rate);
    println!();

    if !stats.most_used_commands.is_empty() {
        println!("🔥 Most Used Commands:");
        for (i, (cmd, count)) in stats.most_used_commands.iter().enumerate().take(10) {
            let cmd_display = if cmd.len() > 60 {
                format!("{}...", &cmd[..57])
            } else {
                cmd.clone()
            };
            println!("  {:2}. [{:4}×] {}", i + 1, count, cmd_display);
        }
        println!();
    }

    // Additional stats
    let commands = storage.read_all_commands()?;

    if !commands.is_empty() {
        // Calculate average duration
        let total_duration: u64 = commands.iter().map(|c| c.duration_ms).sum();
        let avg_duration = total_duration / commands.len() as u64;

        // Find longest running command
        let longest = commands.iter().max_by_key(|c| c.duration_ms);

        println!("⏱️  Performance:");
        println!("  • Average Duration: {}ms", avg_duration);

        if let Some(longest_cmd) = longest {
            let cmd_display = if longest_cmd.command.len() > 50 {
                format!("{}...", &longest_cmd.command[..47])
            } else {
                longest_cmd.command.clone()
            };
            println!("  • Longest Command:  {}ms - {}", longest_cmd.duration_ms, cmd_display);
        }
        println!();
    }

    // Storage info
    let data_dir = storage.data_dir();
    println!("💾 Storage:");
    println!("  • Location: {}", data_dir.display());

    if let Ok(metadata) = std::fs::metadata(data_dir.join("commands.jsonl")) {
        let size_kb = metadata.len() / 1024;
        println!("  • Size: {} KB", size_kb);
    }

    Ok(())
}
