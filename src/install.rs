use crate::cli::Shell;
use anyhow::{Context, Result, anyhow};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Install shell hooks for automatic command recording
pub fn install(shell: Option<Shell>) -> Result<()> {
    let shell = shell.or_else(Shell::detect).ok_or_else(|| {
        anyhow!(
            "Could not detect shell. Please specify explicitly with --shell (bash, zsh, fish, or powershell)"
        )
    })?;

    println!("Installing shelltape hooks for {:?}...", shell);

    // Create ~/.shelltape directory if it doesn't exist
    let shelltape_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?
        .join(".shelltape");

    fs::create_dir_all(&shelltape_dir)
        .with_context(|| format!("Failed to create directory: {}", shelltape_dir.display()))?;

    // Copy hook file to ~/.shelltape/
    copy_hook_file(&shelltape_dir, shell)?;

    // Add source line to RC file
    add_to_rc_file(shell)?;

    println!("\nShelltape installed successfully!");
    println!("\nTo start recording commands, either:");
    println!("  1. Restart your shell");
    println!("  2. Run: source ~/{}", shell.rc_file());
    println!("\nThen use:");
    println!("  - shelltape list          - View recent commands");
    println!("  - shelltape browse        - Interactive browser (TUI)");
    println!("  - shelltape stats         - Show statistics");
    println!("  - shelltape export -o file.md - Export to markdown");

    Ok(())
}

/// Copy the appropriate hook file to ~/.shelltape/
fn copy_hook_file(shelltape_dir: &Path, shell: Shell) -> Result<()> {
    let hook_content = match shell {
        Shell::Bash => include_str!("../shell-hooks/bash.sh"),
        Shell::Zsh => include_str!("../shell-hooks/zsh.sh"),
        Shell::Fish => include_str!("../shell-hooks/fish.fish"),
        Shell::Powershell => include_str!("../shell-hooks/powershell.ps1"),
    };

    let hook_file_path = shelltape_dir.join(shell.hook_file());

    fs::write(&hook_file_path, hook_content)
        .with_context(|| format!("Failed to write hook file to: {}", hook_file_path.display()))?;

    println!("  [OK] Copied hook file to {}", hook_file_path.display());

    Ok(())
}

/// Add source line to the shell's RC file
fn add_to_rc_file(shell: Shell) -> Result<()> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;

    let rc_path = home_dir.join(shell.rc_file());

    // Create the RC file if it doesn't exist
    if !rc_path.exists() {
        fs::write(&rc_path, "")
            .with_context(|| format!("Failed to create RC file: {}", rc_path.display()))?;
    }

    // Read existing content to check if already installed
    let content = fs::read_to_string(&rc_path)
        .with_context(|| format!("Failed to read: {}", rc_path.display()))?;

    let hook_line = match shell {
        Shell::Bash | Shell::Zsh => format!("source ~/.shelltape/{}", shell.hook_file()),
        Shell::Fish => format!("source ~/.shelltape/{}", shell.hook_file()),
        Shell::Powershell => format!(". ~\\.shelltape\\{}", shell.hook_file()),
    };

    // Check if already installed
    if content.contains(&hook_line) {
        println!(
            "  [INFO] Shelltape hooks already present in {}",
            rc_path.display()
        );
        return Ok(());
    }

    // Append the hook
    let mut file = OpenOptions::new()
        .append(true)
        .open(&rc_path)
        .with_context(|| format!("Failed to open {} for appending", rc_path.display()))?;

    writeln!(file, "\n# Shelltape - Terminal command history recorder")?;
    writeln!(file, "{}", hook_line)?;

    println!("  [OK] Added hooks to {}", rc_path.display());

    Ok(())
}
