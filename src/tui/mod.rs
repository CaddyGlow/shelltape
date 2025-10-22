mod app;
mod events;
mod ui;

pub use app::App;

use anyhow::{Context, Result};
use crossterm::{
    event::Event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Run the TUI application
pub fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // Create app
    let mut app = App::new()?;

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    // Handle any errors that occurred during the app run
    result?;

    // Print export message if any commands were marked
    if !app.marked.is_empty() {
        let home = dirs::home_dir().unwrap_or_default();
        let output_path = home.join("shelltape-export.md");
        println!(
            "\n{} commands marked. Press 'e' to export to {}",
            app.marked.len(),
            output_path.display()
        );
    }

    Ok(())
}

/// Main application loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events
        if let Event::Key(key) = events::read_event()? {
            events::handle_key_event(app, key)?;
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
