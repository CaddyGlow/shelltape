use crate::models::Command;
use crate::storage::Storage;
use anyhow::Result;
use std::collections::HashSet;

/// View mode for the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// List view showing all commands
    List,
    /// Detail view showing full command information
    Detail,
}

/// The main TUI application state
pub struct App {
    /// Storage instance
    #[allow(dead_code)]
    pub storage: Storage,
    /// All commands loaded from storage
    pub commands: Vec<Command>,
    /// Indices of filtered commands (into `commands` vec)
    pub filtered_commands: Vec<usize>,
    /// Currently selected index (into `filtered_commands`)
    pub selected: usize,
    /// Scroll offset for the list
    pub scroll: usize,
    /// Current search query
    pub search_query: String,
    /// Whether we're in search input mode
    pub search_mode: bool,
    /// Set of marked command indices (into `commands`)
    pub marked: HashSet<usize>,
    /// Current view mode
    pub view_mode: ViewMode,
    /// Whether to quit the app
    pub should_quit: bool,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Result<Self> {
        let storage = Storage::new()?;
        let mut commands = storage.read_all_commands()?;

        // Sort by most recent first
        commands.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        let filtered: Vec<usize> = (0..commands.len()).collect();

        Ok(Self {
            storage,
            commands,
            filtered_commands: filtered,
            selected: 0,
            scroll: 0,
            search_query: String::new(),
            search_mode: false,
            marked: HashSet::new(),
            view_mode: ViewMode::List,
            should_quit: false,
        })
    }

    /// Apply the current search filter
    pub fn apply_filter(&mut self) {
        if self.search_query.is_empty() {
            // No filter, show all commands
            self.filtered_commands = (0..self.commands.len()).collect();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_commands = self
                .commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    cmd.command.to_lowercase().contains(&query)
                        || cmd.cwd.to_lowercase().contains(&query)
                        || cmd.output.to_lowercase().contains(&query)
                })
                .map(|(i, _)| i)
                .collect();
        }

        // Reset selection and scroll
        self.selected = 0;
        self.scroll = 0;
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.filtered_commands.is_empty() {
            self.selected = (self.selected + 1).min(self.filtered_commands.len() - 1);
        }
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move selection down by page
    pub fn page_down(&mut self, page_size: usize) {
        if !self.filtered_commands.is_empty() {
            self.selected = (self.selected + page_size).min(self.filtered_commands.len() - 1);
        }
    }

    /// Move selection up by page
    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
    }

    /// Move to first item
    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    /// Move to last item
    pub fn select_last(&mut self) {
        if !self.filtered_commands.is_empty() {
            self.selected = self.filtered_commands.len() - 1;
        }
    }

    /// Toggle mark on currently selected command
    pub fn toggle_mark(&mut self) {
        if let Some(&cmd_idx) = self.filtered_commands.get(self.selected) {
            if self.marked.contains(&cmd_idx) {
                self.marked.remove(&cmd_idx);
            } else {
                self.marked.insert(cmd_idx);
            }
        }
    }

    /// Mark all filtered commands
    pub fn mark_all(&mut self) {
        for &idx in &self.filtered_commands {
            self.marked.insert(idx);
        }
    }

    /// Clear all marks
    pub fn clear_marks(&mut self) {
        self.marked.clear();
    }

    /// Get the currently selected command
    pub fn get_selected_command(&self) -> Option<&Command> {
        self.filtered_commands
            .get(self.selected)
            .and_then(|&idx| self.commands.get(idx))
    }

    /// Export marked commands to a file
    pub fn export_marked(&self, output_path: &str) -> Result<()> {
        use chrono::Utc;
        use std::fs;

        let marked_commands: Vec<&Command> = self
            .marked
            .iter()
            .filter_map(|&idx| self.commands.get(idx))
            .collect();

        if marked_commands.is_empty() {
            return Ok(());
        }

        // Build markdown
        let mut markdown = String::new();
        markdown.push_str("# Shelltape Command History (Marked Commands)\n\n");
        markdown.push_str(&format!(
            "Generated: {}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));
        markdown.push_str(&format!("Total commands: {}\n\n", marked_commands.len()));
        markdown.push_str("---\n\n");

        for cmd in marked_commands {
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

            markdown.push_str("**Command:**\n\n");
            markdown.push_str(&format!("```bash\n{}\n```\n\n", cmd.command));

            if !cmd.output.is_empty() {
                markdown.push_str("**Output:**\n\n");
                markdown.push_str(&format!("```\n{}\n```\n\n", cmd.output));
            }

            markdown.push_str("---\n\n");
        }

        fs::write(output_path, markdown)?;

        Ok(())
    }

    /// Toggle view mode
    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::List => ViewMode::Detail,
            ViewMode::Detail => ViewMode::List,
        };
    }

    /// Add character to search query
    pub fn search_input(&mut self, c: char) {
        self.search_query.push(c);
    }

    /// Remove last character from search query
    pub fn search_backspace(&mut self) {
        self.search_query.pop();
    }

    /// Clear search query
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.apply_filter();
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
