use crate::cli::Shell;
use anyhow::{Context, Result, anyhow};
use std::fs;

/// Uninstall shell hooks
pub fn uninstall(shell: Option<Shell>) -> Result<()> {
    let shell = shell.or_else(Shell::detect).ok_or_else(|| {
        anyhow!(
            "Could not detect shell. Please specify explicitly with --shell (bash, zsh, fish, or powershell)"
        )
    })?;

    println!("Uninstalling shelltape hooks for {:?}...", shell);

    // Remove source line from RC file
    remove_from_rc_file(shell)?;

    // Optional: Remove hook files from ~/.shelltape/
    let shelltape_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?
        .join(".shelltape");

    if shelltape_dir.exists() {
        let hook_file_path = shelltape_dir.join(shell.hook_file());
        if hook_file_path.exists() {
            fs::remove_file(&hook_file_path).with_context(|| {
                format!("Failed to remove hook file: {}", hook_file_path.display())
            })?;
            println!("  [OK] Removed hook file from {}", hook_file_path.display());
        }
    }

    println!("\nShelltape uninstalled successfully!");
    println!("\nTo complete the uninstall:");
    println!(
        "  1. Restart your shell or run: source ~/{}",
        shell.rc_file()
    );
    println!("  2. Optionally remove data: rm -rf ~/.shelltape/");

    Ok(())
}

/// Remove source line from the shell's RC file
fn remove_from_rc_file(shell: Shell) -> Result<()> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
    let rc_path = home_dir.join(shell.rc_file());

    if !rc_path.exists() {
        println!("  [INFO] RC file not found: {}", rc_path.display());
        return Ok(());
    }

    // Read existing content
    let content = fs::read_to_string(&rc_path)
        .with_context(|| format!("Failed to read: {}", rc_path.display()))?;

    let hook_line = match shell {
        Shell::Bash | Shell::Zsh => format!("source ~/.shelltape/{}", shell.hook_file()),
        Shell::Fish => format!("source ~/.shelltape/{}", shell.hook_file()),
        Shell::Powershell => format!(". ~\\.shelltape\\{}", shell.hook_file()),
    };

    // Check if hook line exists
    if !content.contains(&hook_line) {
        println!(
            "  [INFO] Shelltape hooks not found in {}",
            rc_path.display()
        );
        return Ok(());
    }

    // Remove the hook line and the comment line before it
    let mut new_lines = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // Check if this is the shelltape comment line
        if line.contains("# Shelltape") && i + 1 < lines.len() {
            let next_line = lines[i + 1];
            // If the next line is the hook source, skip both
            if next_line.contains(&hook_line) {
                i += 2; // Skip comment and hook line
                // Also skip any empty lines after
                while i < lines.len() && lines[i].trim().is_empty() {
                    i += 1;
                }
                continue;
            }
        }

        // Check if this is the hook line without comment
        if line.contains(&hook_line) {
            i += 1;
            continue;
        }

        new_lines.push(line);
        i += 1;
    }

    // Write back to file
    let new_content = new_lines.join("\n");
    fs::write(&rc_path, new_content)
        .with_context(|| format!("Failed to write to: {}", rc_path.display()))?;

    println!("  [OK] Removed hooks from {}", rc_path.display());

    Ok(())
}
