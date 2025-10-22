use crate::tui::app::{App, ViewMode};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

/// Handle keyboard input events
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    // Global quit key
    if key.code == KeyCode::Char('q') && !app.search_mode {
        app.quit();
        return Ok(());
    }

    // Handle events based on current mode
    if app.search_mode {
        handle_search_mode(app, key)?;
    } else {
        match app.view_mode {
            ViewMode::List => handle_list_mode(app, key)?,
            ViewMode::Detail => handle_detail_mode(app, key)?,
        }
    }

    Ok(())
}

/// Handle key events in search mode
fn handle_search_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.search_mode = false;
        }
        KeyCode::Enter => {
            app.apply_filter();
            app.search_mode = false;
        }
        KeyCode::Char(c) => {
            app.search_input(c);
        }
        KeyCode::Backspace => {
            app.search_backspace();
        }
        _ => {}
    }

    Ok(())
}

/// Handle key events in list mode
fn handle_list_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        // Navigation
        KeyCode::Char('j') | KeyCode::Down => {
            app.select_next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.select_previous();
        }
        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::NONE) => {
            app.select_first();
        }
        KeyCode::Char('G') | KeyCode::End => {
            app.select_last();
        }
        KeyCode::PageDown | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let page_size = 10; // Could be calculated from terminal height
            app.page_down(page_size);
        }
        KeyCode::PageUp | KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let page_size = 10;
            app.page_up(page_size);
        }

        // Marking
        KeyCode::Char(' ') => {
            app.toggle_mark();
            app.select_next(); // Move to next after marking
        }
        KeyCode::Char('a') => {
            app.mark_all();
        }
        KeyCode::Char('c') => {
            app.clear_marks();
        }

        // Search
        KeyCode::Char('/') => {
            app.search_mode = true;
            app.search_query.clear();
        }
        KeyCode::Esc => {
            app.clear_search();
        }

        // View
        KeyCode::Enter => {
            app.toggle_view_mode();
        }

        // Export
        KeyCode::Char('e') => {
            if !app.marked.is_empty() {
                let home = dirs::home_dir().unwrap_or_default();
                let output_path = home.join("shelltape-export.md");

                if let Err(e) = app.export_marked(&output_path.to_string_lossy()) {
                    eprintln!("Export failed: {}", e);
                }
            }
        }

        _ => {}
    }

    Ok(())
}

/// Handle key events in detail mode
fn handle_detail_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter | KeyCode::Esc => {
            app.toggle_view_mode();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.select_next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.select_previous();
        }
        _ => {}
    }

    Ok(())
}

/// Read the next event from the terminal
pub fn read_event() -> Result<Event> {
    Ok(event::read()?)
}
