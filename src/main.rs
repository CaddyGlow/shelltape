mod clean;
mod cli;
mod export;
mod install;
mod list;
mod models;
mod pty_capture;
mod recorder;
mod stats;
mod status;
mod storage;
mod tui;
mod uninstall;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { shell } => {
            install::install(shell)?;
        }
        Commands::Uninstall { shell } => {
            uninstall::uninstall(shell)?;
        }
        Commands::Exec { command, session_id } => {
            // Join command parts
            let command_str = command.join(" ");
            let cwd = std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // Execute with PTY capture (output is displayed in real-time by PTY)
            let result = pty_capture::execute_with_capture(&command_str, &cwd)?;

            // Record the command
            let recorder = recorder::Recorder::new()?;
            recorder.record(
                command_str,
                result.output,
                result.exit_code,
                result.start_time,
                result.end_time,
                cwd,
                session_id,
            )?;

            // Exit with same code as command
            std::process::exit(result.exit_code);
        }
        Commands::Record {
            command,
            exit_code,
            start_time,
            end_time,
            cwd,
            session_id,
            output,
        } => {
            let recorder = recorder::Recorder::new()?;
            recorder.record(command, output, exit_code, start_time, end_time, cwd, session_id)?;
        }
        Commands::Browse => {
            tui::run()?;
        }
        Commands::List { limit, filter } => {
            list::list_commands(limit, filter)?;
        }
        Commands::Export {
            output,
            session,
            filter,
        } => {
            export::export_commands(output, session, filter)?;
        }
        Commands::Stats => {
            stats::show_stats()?;
        }
        Commands::Clean {
            older_than_days,
            yes,
        } => {
            clean::clean_commands(older_than_days, yes)?;
        }
        Commands::Status => {
            status::show_status()?;
        }
    }

    Ok(())
}
