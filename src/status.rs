use crate::storage::Storage;
use anyhow::Result;
use std::fs;

/// Show status and storage information
pub fn show_status() -> Result<()> {
    let storage = Storage::new()?;
    let data_dir = storage.data_dir();

    println!("╔════════════════════════════════════════════════╗");
    println!("║          Shelltape Status                      ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();

    // Check if data directory exists
    let data_dir_exists = data_dir.exists();
    println!("📁 Data Directory:");
    println!("  • Location: {}", data_dir.display());
    println!("  • Exists: {}", if data_dir_exists { "✓" } else { "✗" });
    println!();

    if !data_dir_exists {
        println!("⚠️  Data directory does not exist yet.");
        println!("   Commands will be recorded once you execute some commands.");
        return Ok(());
    }

    // Check commands file
    let commands_file = data_dir.join("commands.jsonl");
    let commands_exists = commands_file.exists();

    println!("📝 Commands File:");
    println!("  • Path: {}", commands_file.display());
    println!("  • Exists: {}", if commands_exists { "✓" } else { "✗" });

    if commands_exists {
        if let Ok(metadata) = fs::metadata(&commands_file) {
            let size = metadata.len();
            let size_display = if size < 1024 {
                format!("{} B", size)
            } else if size < 1024 * 1024 {
                format!("{:.2} KB", size as f64 / 1024.0)
            } else {
                format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
            };
            println!("  • Size: {}", size_display);
        }

        // Count commands
        if let Ok(commands) = storage.read_all_commands() {
            println!("  • Total Commands: {}", commands.len());

            if !commands.is_empty() {
                if let Some(oldest) = commands.iter().min_by_key(|c| c.started_at) {
                    println!(
                        "  • Oldest: {}",
                        oldest.started_at.format("%Y-%m-%d %H:%M:%S")
                    );
                }
                if let Some(newest) = commands.iter().max_by_key(|c| c.started_at) {
                    println!(
                        "  • Newest: {}",
                        newest.started_at.format("%Y-%m-%d %H:%M:%S")
                    );
                }
            }
        }
    } else {
        println!("  ℹ No commands recorded yet");
    }
    println!();

    // Check sessions file
    let sessions_file = data_dir.join("sessions.jsonl");
    let sessions_exists = sessions_file.exists();

    println!("🖥️  Sessions File:");
    println!("  • Path: {}", sessions_file.display());
    println!("  • Exists: {}", if sessions_exists { "✓" } else { "✗" });

    if sessions_exists {
        if let Ok(metadata) = fs::metadata(&sessions_file) {
            let size = metadata.len();
            let size_display = if size < 1024 {
                format!("{} B", size)
            } else {
                format!("{:.2} KB", size as f64 / 1024.0)
            };
            println!("  • Size: {}", size_display);
        }

        if let Ok(sessions) = storage.read_all_sessions() {
            println!("  • Total Sessions: {}", sessions.len());
        }
    }
    println!();

    // Check if hooks are installed
    println!("🔧 Shell Integration:");
    check_shell_hooks();

    Ok(())
}

/// Check if shell hooks are installed
fn check_shell_hooks() {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            println!("  • Could not determine home directory");
            return;
        }
    };

    // Check bash
    let bashrc = home.join(".bashrc");
    if bashrc.exists() {
        if let Ok(content) = fs::read_to_string(&bashrc) {
            let installed = content.contains("shelltape") || content.contains("bash.sh");
            println!(
                "  • Bash (~/.bashrc): {}",
                if installed {
                    "✓ Installed"
                } else {
                    "✗ Not installed"
                }
            );
        }
    }

    // Check zsh
    let zshrc = home.join(".zshrc");
    if zshrc.exists() {
        if let Ok(content) = fs::read_to_string(&zshrc) {
            let installed = content.contains("shelltape") || content.contains("zsh.sh");
            println!(
                "  • Zsh (~/.zshrc): {}",
                if installed {
                    "✓ Installed"
                } else {
                    "✗ Not installed"
                }
            );
        }
    }

    // Check fish
    let fishrc = home.join(".config/fish/config.fish");
    if fishrc.exists() {
        if let Ok(content) = fs::read_to_string(&fishrc) {
            let installed = content.contains("shelltape") || content.contains("fish.fish");
            println!(
                "  • Fish (~/.config/fish/config.fish): {}",
                if installed {
                    "✓ Installed"
                } else {
                    "✗ Not installed"
                }
            );
        }
    }
}
