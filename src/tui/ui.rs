use crate::tui::app::{App, ViewMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

/// Draw the entire UI
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Command list or detail view
            Constraint::Length(2), // Status bar
        ])
        .split(f.area());

    draw_search_bar(f, app, chunks[0]);

    match app.view_mode {
        ViewMode::List => {
            // Split main area for list and preview
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(60), // Command list
                    Constraint::Percentage(40), // Preview
                ])
                .split(chunks[1]);

            draw_command_list(f, app, main_chunks[0]);
            draw_preview(f, app, main_chunks[1]);
        }
        ViewMode::Detail => {
            draw_detail_view(f, app, chunks[1]);
        }
    }

    draw_status_bar(f, app, chunks[2]);
}

/// Draw the search bar
fn draw_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.search_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let text = if app.search_mode {
        format!("Search: {}_", app.search_query)
    } else if app.search_query.is_empty() {
        "Press / to search".to_string()
    } else {
        format!("Filter: {} (press / to edit)", app.search_query)
    };

    let paragraph = Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title(" Search "));

    f.render_widget(paragraph, area);
}

/// Draw the command list
fn draw_command_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_commands
        .iter()
        .enumerate()
        .map(|(display_idx, &cmd_idx)| {
            let cmd = &app.commands[cmd_idx];

            let mark = if app.marked.contains(&cmd_idx) {
                "●"
            } else {
                " "
            };

            let exit = if cmd.exit_code == 0 { "✓" } else { "✗" };
            let time = cmd.started_at.format("%m-%d %H:%M:%S");

            // Truncate command for display
            let cmd_display = if cmd.command.len() > 60 {
                format!("{}...", &cmd.command[..57])
            } else {
                cmd.command.clone()
            };

            let content = format!("{} {} {} {}", mark, exit, time, cmd_display);

            let style = if display_idx == app.selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!(
                " Commands ({}/{}) ",
                app.filtered_commands.len(),
                app.commands.len()
            ))
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

/// Draw the preview pane (shows selected command details)
fn draw_preview(f: &mut Frame, app: &App, area: Rect) {
    let content = if let Some(cmd) = app.get_selected_command() {
        let duration_display = if cmd.duration_ms < 1000 {
            format!("{}ms", cmd.duration_ms)
        } else {
            format!("{:.2}s", cmd.duration_ms as f64 / 1000.0)
        };

        let output_display = if cmd.output.trim().is_empty() {
            "(no output captured)".to_string()
        } else if cmd.output.len() > 200 {
            let preview = cmd.output.chars().take(200).collect::<String>();
            format!("{}... (truncated)", preview.trim())
        } else {
            cmd.output.trim().to_string()
        };

        let session_display = if cmd.session_id.len() >= 8 {
            &cmd.session_id[..8]
        } else {
            &cmd.session_id
        };

        format!(
            "Command: {}\n\nDirectory: {}\nDuration: {}\nExit Code: {}\nSession: {}\n\nOutput:\n{}",
            cmd.command, cmd.cwd, duration_display, cmd.exit_code, session_display, output_display
        )
    } else {
        "No command selected".to_string()
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().title(" Preview ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// Draw the full detail view
fn draw_detail_view(f: &mut Frame, app: &App, area: Rect) {
    let content = if let Some(cmd) = app.get_selected_command() {
        let duration_display = if cmd.duration_ms < 1000 {
            format!("{}ms", cmd.duration_ms)
        } else {
            format!("{:.2}s", cmd.duration_ms as f64 / 1000.0)
        };

        let status = if cmd.exit_code == 0 {
            "✓ Success"
        } else {
            "✗ Failed"
        };

        format!(
            "╔═══════════════════════════════════════════════════════════════╗\n\
             ║ COMMAND DETAILS                                               ║\n\
             ╚═══════════════════════════════════════════════════════════════╝\n\n\
             Time:      {}\n\
             Duration:  {}\n\
             Status:    {} (exit code: {})\n\
             Session:   {}\n\n\
             Shell:     {}\n\
             Hostname:  {}\n\
             User:      {}\n\n\
             Directory:\n  {}\n\n\
             Command:\n  {}\n\n\
             Output:\n{}",
            cmd.started_at.format("%Y-%m-%d %H:%M:%S"),
            duration_display,
            status,
            cmd.exit_code,
            cmd.session_id,
            cmd.shell,
            cmd.hostname,
            cmd.username,
            cmd.cwd,
            cmd.command,
            if cmd.output.trim().is_empty() {
                "  (no output captured)".to_string()
            } else {
                cmd.output
                    .trim()
                    .lines()
                    .map(|line| format!("  {}", line.trim_end()))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        )
    } else {
        "No command selected".to_string()
    };

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title(" Detail View (press Enter to return) ")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

/// Draw the status bar
fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let help_text = if app.search_mode {
        " ESC: exit search | Enter: apply | Type to search "
    } else {
        match app.view_mode {
            ViewMode::List => {
                " j/k/↑/↓: navigate | Space: mark | a: mark all | c: clear marks | /: search | Enter: detail | e: export | q: quit "
            }
            ViewMode::Detail => " Enter: back to list | q: quit ",
        }
    };

    let marked_count = app.marked.len();
    let marked_info = if marked_count > 0 {
        format!(" | {} marked", marked_count)
    } else {
        String::new()
    };

    let status_text = format!("{}{}", help_text, marked_info);

    let spans = vec![Span::styled(
        status_text,
        Style::default().bg(Color::DarkGray).fg(Color::White),
    )];

    let paragraph = Paragraph::new(Line::from(spans));

    f.render_widget(paragraph, area);
}
