use crate::storage::Storage;
use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

/// Export commands to markdown format
pub fn export_commands(
    output: PathBuf,
    session: Option<String>,
    filter: Option<String>,
) -> Result<()> {
    let storage = Storage::new()?;
    let mut commands = storage.read_all_commands()?;

    // Filter by session
    if let Some(sid) = &session {
        commands.retain(|cmd| &cmd.session_id == sid);
    }

    // Filter by query
    if let Some(query) = &filter {
        let query_lower = query.to_lowercase();
        commands.retain(|cmd| cmd.command.to_lowercase().contains(&query_lower));
    }

    // Sort chronologically (oldest first for export)
    commands.sort_by(|a, b| a.started_at.cmp(&b.started_at));

    // Build markdown content
    let mut markdown = String::new();

    // Header
    markdown.push_str("# Shelltape Command History\n\n");
    markdown.push_str(&format!(
        "Generated: {}\n\n",
        Utc::now().format("%Y-%m-%d %H:%M:%S")
    ));
    markdown.push_str(&format!("Total commands: {}\n\n", commands.len()));

    if let Some(sid) = &session {
        markdown.push_str(&format!("Session: `{}`\n\n", sid));
    }

    if let Some(query) = &filter {
        markdown.push_str(&format!("Filter: `{}`\n\n", query));
    }

    markdown.push_str("---\n\n");

    // Commands
    for cmd in &commands {
        markdown.push_str(&format!(
            "## {}\n\n",
            cmd.started_at.format("%Y-%m-%d %H:%M:%S")
        ));
        markdown.push_str(&format!("**Directory:** `{}`\n\n", cmd.cwd));
        markdown.push_str(&format!("**Duration:** {}ms\n\n", cmd.duration_ms));

        let status = if cmd.exit_code == 0 {
            "✓ Success"
        } else {
            "✗ Failed"
        };
        markdown.push_str(&format!(
            "**Exit Code:** {} ({})\n\n",
            cmd.exit_code, status
        ));

        markdown.push_str(&format!("**Shell:** {}\n\n", cmd.shell));
        markdown.push_str(&format!("**Hostname:** {}\n\n", cmd.hostname));
        markdown.push_str(&format!("**User:** {}\n\n", cmd.username));

        markdown.push_str("**Command:**\n\n");
        markdown.push_str(&format!("```bash\n{}\n```\n\n", cmd.command));

        if !cmd.output.is_empty() {
            markdown.push_str("**Output:**\n\n");
            markdown.push_str(&format!("```\n{}\n```\n\n", cmd.output));
        }

        markdown.push_str("---\n\n");
    }

    // Write to file
    fs::write(&output, markdown)
        .with_context(|| format!("Failed to write to: {}", output.display()))?;

    println!(
        "✓ Exported {} commands to {}",
        commands.len(),
        output.display()
    );

    Ok(())
}
