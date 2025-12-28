use anyhow::Result;
use chrono::{Local};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::{prelude::*, widgets::*};
use serde::Deserialize;
use std::process::Stdio;
use std::str::FromStr;
use std::{fs, io::{self, Write}, path::PathBuf, time::SystemTime};

#[derive(Clone, Copy, PartialEq)]
enum AppMode {
    Normal,
    DeleteConfirm,
}

// Data model (same as before)
#[derive(Clone)]
struct TryEntry {
    name: String,
    modified: SystemTime,
    created: SystemTime,
    score: i64,
    is_git: bool,
}

#[derive(Clone)]
struct Theme {
    title_try: Color,
    title_rs: Color,
    search_box: Color,
    list_date: Color,
    list_highlight_bg: Color,
    list_highlight_fg: Color,
    help_text: Color,
    status_message: Color,
    popup_bg: Color,
    popup_text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Catppuccin Mocha Palette
            title_try: Color::Rgb(137, 180, 250),         // Blue
            title_rs: Color::Rgb(243, 139, 168),          // Red
            search_box: Color::Rgb(250, 179, 135),        // Peach
            list_date: Color::Rgb(166, 173, 200),         // Subtext0
            list_highlight_bg: Color::Rgb(88, 91, 112),   // Surface2
            list_highlight_fg: Color::Rgb(205, 214, 244), // Text
            help_text: Color::Rgb(147, 153, 178),         // Overlay2
            status_message: Color::Rgb(249, 226, 175),    // Yellow
            popup_bg: Color::Rgb(30, 30, 46),             // Base
            popup_text: Color::Rgb(243, 139, 168),        // Red
        }
    }
}

// Our TUI state
struct App {
    query: String,                   // What the user typed
    all_entries: Vec<TryEntry>,      // All directories found
    filtered_entries: Vec<TryEntry>, // Directories filtered by search
    selected_index: usize,           // Which item is currently selected in the list
    should_quit: bool,               // Flag to exit the loop
    final_selection: Option<String>, // The final result (for the shell)
    mode: AppMode,
    status_message: Option<String>, // Feedback message for the user
    base_path: PathBuf,             // Base directory for tries
    theme: Theme,                   // Application colors
    editor_cmd: Option<String>,     // Editor command (e.g., "code", "nvim")
    wants_editor: bool,             // Flag to indicate if we should open the editor
}

impl App {
    fn new(path: PathBuf, theme: Theme, editor_cmd: Option<String>) -> Self {
        let mut entries = Vec::new();
        if let Ok(read_dir) = fs::read_dir(&path) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let is_git = entry.path().join(".git").exists();
                        entries.push(TryEntry {
                            name,
                            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                            created: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
                            score: 0,
                            is_git,
                        });
                    }
                }
            }
        }
        // Initial sort: most recent first
        entries.sort_by(|a, b| b.modified.cmp(&a.modified));

        Self {
            query: String::new(),
            all_entries: entries.clone(),
            filtered_entries: entries,
            selected_index: 0,
            should_quit: false,
            final_selection: None,
            mode: AppMode::Normal,
            status_message: None,
            base_path: path,
            theme,
            editor_cmd,
            wants_editor: false,
        }
    }

    // Filter update logic
    fn update_search(&mut self) {
        let matcher = SkimMatcherV2::default();

        if self.query.is_empty() {
            self.filtered_entries = self.all_entries.clone();
        } else {
            self.filtered_entries = self
                .all_entries
                .iter()
                .filter_map(|entry| {
                    matcher.fuzzy_match(&entry.name, &self.query).map(|score| {
                        let mut e = entry.clone();
                        e.score = score;
                        e
                    })
                })
                .collect();

            // Sort by fuzzy score
            self.filtered_entries.sort_by(|a, b| b.score.cmp(&a.score));
        }
        self.selected_index = 0; // Resets the selection to the top
    }

    // Function to delete the selected item
    fn delete_selected(&mut self) {
        if let Some(entry_name) = self
            .filtered_entries
            .get(self.selected_index)
            .map(|e| e.name.clone())
        {
            let path_to_remove = self.base_path.join(&entry_name);

            match fs::remove_dir_all(&path_to_remove) {
                Ok(_) => {
                    self.all_entries.retain(|e| e.name != entry_name);
                    self.update_search();
                    self.status_message = Some(format!("Deleted: {}", path_to_remove.display()));
                }
                Err(e) => {
                    self.status_message = Some(format!("Error deleting: {}", e));
                }
            }
        }
        self.mode = AppMode::Normal;
    }
}

fn draw_popup(f: &mut Frame, title: &str, message: &str, theme: &Theme) {
    let area = f.area();

    // 1. Define an area in the center (60% width, 20% height)
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(3),
            Constraint::Percentage(40),
        ])
        .split(area);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(popup_layout[1])[1];

    // 2. Clears the popup area (so the background text doesn't show through)
    f.render_widget(Clear, popup_area);

    // 3. Creates the block with a red border (alert)
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.popup_bg));

    let paragraph = Paragraph::new(message)
        .block(block)
        .style(
            Style::default()
                .fg(theme.popup_text)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, popup_area);
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    mut app: App,
) -> Result<(Option<String>, bool)> {
    while !app.should_quit {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(f.area());

            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[2]);

            let title = Paragraph::new(Line::from(vec![
                Span::styled(
                    "🦀 try",
                    Style::default()
                        .fg(app.theme.title_try)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("-", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "rs",
                    Style::default()
                        .fg(app.theme.title_rs)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" v{} ", env!("CARGO_PKG_VERSION")),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    "🦀",
                    Style::default()
                        .fg(app.theme.title_rs)
                        .add_modifier(Modifier::BOLD),
                ),
            ]))
            .alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            let search_text = Paragraph::new(app.query.clone())
                .style(Style::default().fg(app.theme.search_box))
                .block(Block::default().borders(Borders::ALL).title(" Search/New "));
            f.render_widget(search_text, chunks[1]);

            let items: Vec<ListItem> = app
                .filtered_entries
                .iter()
                .map(|entry| {
                    let now = SystemTime::now();
                    let elapsed = now.duration_since(entry.modified).unwrap_or(std::time::Duration::ZERO);
                    let secs = elapsed.as_secs();
                    let days = secs / 86400;
                    let hours = (secs % 86400) / 3600;
                    let minutes = (secs % 3600) / 60;
                    let date_str = format!("({:02} days {:02}h {:02}m)", days, hours, minutes);

                    // Calculate available width (block borders take 2 columns)
                    let width = content_chunks[0].width.saturating_sub(5) as usize;

                    let date_text = format!("{}", date_str);
                    let date_width = date_text.chars().count();
                    let git_icon = if entry.is_git { " " } else { "" };
                    let git_width = if entry.is_git { 2 } else { 0 };
                    let icon_width = 2; // "📁" takes 2 columns
                    
                    let created_dt: chrono::DateTime<Local> = entry.created.into();
                    let created_text = created_dt.format("%Y-%m-%d").to_string();
                    let created_width = created_text.chars().count();

                    // Calculate space for name
                    let reserved = date_width + git_width + icon_width + created_width + 2; // +2 for gaps
                    let available_for_name = width.saturating_sub(reserved);
                    let name_len = entry.name.chars().count();

                    let (display_name, padding) = if name_len > available_for_name {
                        let safe_len = available_for_name.saturating_sub(3);
                        let truncated: String = entry.name.chars().take(safe_len).collect();
                        (format!("{}...", truncated), 1)
                    } else {
                        (
                            entry.name.clone(),
                            width.saturating_sub(icon_width + created_width + 1 + name_len + date_width + git_width),
                        )
                    };

                    let content = Line::from(vec![
                        Span::raw("📁"),
                        Span::styled(created_text, Style::default().fg(app.theme.list_date)),
                        Span::raw(format!(" {}", display_name)),
                        Span::raw(" ".repeat(padding)),
                        Span::styled(git_icon, Style::default().fg(Color::Rgb(240, 80, 50))),
                        Span::styled(date_text, Style::default().fg(app.theme.list_date)),
                    ]);
                    ListItem::new(content)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Folders "))
                .highlight_style(
                    Style::default()
                        .bg(app.theme.list_highlight_bg)
                        .fg(app.theme.list_highlight_fg)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("→ ");

            let mut state = ListState::default();
            state.select(Some(app.selected_index));
            f.render_stateful_widget(list, content_chunks[0], &mut state);

            // Preview Widget
            if let Some(selected) = app.filtered_entries.get(app.selected_index) {
                let preview_path = app.base_path.join(&selected.name);
                let mut preview_lines = Vec::new();

                if let Ok(entries) = fs::read_dir(&preview_path) {
                    // Limit items to height of block to avoid reading too much
                    for entry in entries.take(content_chunks[1].height.saturating_sub(2) as usize) {
                        if let Ok(e) = entry {
                            let file_name = e.file_name().to_string_lossy().to_string();
                            let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                            let icon = if is_dir { "📁 " } else { "📄 " };
                            preview_lines.push(Line::from(vec![
                                Span::styled(icon, Style::default().fg(app.theme.title_try)),
                                Span::raw(file_name),
                            ]));
                        }
                    }
                }

                if preview_lines.is_empty() {
                    preview_lines.push(Line::from(Span::styled(
                        " (empty) ",
                        Style::default().fg(Color::DarkGray),
                    )));
                }

                let preview = Paragraph::new(preview_lines)
                    .block(Block::default().borders(Borders::ALL).title(" Preview "));
                f.render_widget(preview, content_chunks[1]);
            } else {
                let preview = Block::default().borders(Borders::ALL).title(" Preview ");
                f.render_widget(preview, content_chunks[1]);
            }

            // --- Footer Widget (Help) ---
            // If there is a status message, show it instead of help, or alongside it.
            let help_text = if let Some(msg) = &app.status_message {
                Line::from(vec![Span::styled(
                    msg,
                    Style::default()
                        .fg(app.theme.status_message)
                        .add_modifier(Modifier::BOLD),
                )])
            } else {
                Line::from(vec![
                    Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Navigate  "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Select  "),
                    Span::styled("Ctrl-D", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Delete  "),
                    Span::styled("Ctrl-E", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Edit    "),
                    Span::styled("Esc/Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Exit"),
                ])
            };

            let help_message = Paragraph::new(help_text)
                .style(Style::default().fg(app.theme.help_text))
                .alignment(Alignment::Center);

            f.render_widget(help_message, chunks[3]);

            // --- DRAWING THE POPUP (If in DeleteConfirm mode) ---
            if app.mode == AppMode::DeleteConfirm {
                if let Some(selected) = app.filtered_entries.get(app.selected_index) {
                    let msg = format!("Delete '{}'? (y/n)", selected.name);
                    draw_popup(f, " WARNING ", &msg, &app.theme);
                }
            }
        })?;

        // --- KEY HANDLING ---
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Behavior depends on the mode
                match app.mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char(c) => {
                            // Ctrl+C to quit
                            if c == 'c' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                                app.should_quit = true;
                            }
                            // Ctrl+D to delete
                            else if c == 'd' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                                // Only enter delete mode if something is selected
                                if !app.filtered_entries.is_empty() {
                                    app.mode = AppMode::DeleteConfirm;
                                }
                            } else if c == 'e'
                                && key.modifiers.contains(event::KeyModifiers::CONTROL)
                            {
                                // Ctrl+E to open editor
                                if app.editor_cmd.is_some() {
                                    if !app.filtered_entries.is_empty() {
                                        app.final_selection = Some(
                                            app.filtered_entries[app.selected_index].name.clone(),
                                        );
                                        app.wants_editor = true;
                                        app.should_quit = true;
                                    } else if !app.query.is_empty() {
                                        app.final_selection = Some(app.query.clone());
                                        app.wants_editor = true;
                                        app.should_quit = true;
                                    }
                                } else {
                                    app.status_message =
                                        Some("No editor configured in config.toml".to_string());
                                }
                            } else {
                                app.query.push(c);
                                app.status_message = None; // Clear status on type
                                app.update_search();
                            }
                        }
                        KeyCode::Backspace => {
                            app.query.pop();
                            app.update_search();
                        }
                        KeyCode::Up => {
                            if app.selected_index > 0 {
                                app.selected_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.selected_index < app.filtered_entries.len().saturating_sub(1) {
                                app.selected_index += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if !app.filtered_entries.is_empty() {
                                app.final_selection =
                                    Some(app.filtered_entries[app.selected_index].name.clone());
                            } else if !app.query.is_empty() {
                                app.final_selection = Some(app.query.clone());
                            }
                            app.should_quit = true;
                        }
                        KeyCode::Esc => app.should_quit = true,
                        _ => {}
                    },

                    AppMode::DeleteConfirm => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            app.delete_selected();
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                        }
                        KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            app.should_quit = true;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    Ok((app.final_selection, app.wants_editor))
}

// Representation of our TOML file
#[derive(Deserialize)]
struct ThemeConfig {
    title_try: Option<String>,
    title_rs: Option<String>,
    search_box: Option<String>,
    list_date: Option<String>,
    list_highlight_bg: Option<String>,
    list_highlight_fg: Option<String>,
    help_text: Option<String>,
    status_message: Option<String>,
    popup_bg: Option<String>,
    popup_text: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    tries_path: Option<String>,
    colors: Option<ThemeConfig>,
    editor: Option<String>,
}

// Helper function to replace "~" with the actual home path
fn expand_path(path_str: &str) -> PathBuf {
    if path_str.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            // Remove "~/" (first 2 chars) and join with home
            return home.join(&path_str[2..]);
        }
    }
    PathBuf::from(path_str)
}

fn load_configuration() -> (PathBuf, Theme, Option<String>, bool) {
    // 1. Try to find the default config directory (~/.config)
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        // Fallback if not found
        dirs::home_dir().expect("Folder not found").join(".config")
    });

    // 2. Build the path: ~/.config/try-rs/config.toml
    let app_config_dir = config_dir.join("try-rs");
    let config_file = app_config_dir
        .join(std::env::var_os("TRY_CONFIG").unwrap_or_else(|| "config".into()))
        .with_extension("toml");

    // 3. Define the old default (fallback)
    let default_path = dirs::home_dir()
        .expect("Folder not found")
        .join("work/tries");

    let mut theme = Theme::default();
    let try_path = std::env::var_os("TRY_PATH");
    let try_path_specified = try_path.is_some();
    let mut final_path = try_path.map(PathBuf::from).unwrap_or(default_path);
    let mut editor_cmd = std::env::var("VISUAL")
        .ok()
        .or_else(|| std::env::var("EDITOR").ok());
    let mut is_first_run = false;

    // 4. If the file exists, try to read it
    if config_file.exists() {
        if let Ok(contents) = fs::read_to_string(&config_file) {
            if let Ok(config) = toml::from_str::<Config>(&contents) {
                if let Some(path_str) = config.tries_path
                    && !try_path_specified
                {
                    final_path = expand_path(&path_str);
                }
                if let Some(editor) = config.editor {
                    editor_cmd = Some(editor);
                }
                if let Some(colors) = config.colors {
                    // Helper to parse color string to Color enum
                    let parse = |opt: Option<String>, def: Color| -> Color {
                        opt.and_then(|s| Color::from_str(&s).ok()).unwrap_or(def)
                    };

                    let def = Theme::default();
                    theme = Theme {
                        title_try: parse(colors.title_try, def.title_try),
                        title_rs: parse(colors.title_rs, def.title_rs),
                        search_box: parse(colors.search_box, def.search_box),
                        list_date: parse(colors.list_date, def.list_date),
                        list_highlight_bg: parse(colors.list_highlight_bg, def.list_highlight_bg),
                        list_highlight_fg: parse(colors.list_highlight_fg, def.list_highlight_fg),
                        help_text: parse(colors.help_text, def.help_text),
                        status_message: parse(colors.status_message, def.status_message),
                        popup_bg: parse(colors.popup_bg, def.popup_bg),
                        popup_text: parse(colors.popup_text, def.popup_text),
                    };
                }
            }
        }
    } else {
        // Create default config if it doesn't exist
        if fs::create_dir_all(&app_config_dir).is_ok() {
            let default_content = format!("tries_path = {final_path:?}");
            let _ = fs::write(&config_file, default_content);
            is_first_run = true;
        }
    }

    // If nothing works or there is no config, return the default
    (final_path, theme, editor_cmd, is_first_run)
}

fn setup_fish() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    });

    let fish_functions_dir = config_dir.join("fish").join("functions");

    if !fish_functions_dir.exists() {
        fs::create_dir_all(&fish_functions_dir)?;
    }

    let file_path = fish_functions_dir.join("try-rs.fish");
    let content = r#"function try-rs
    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    set command (command try-rs $argv | string collect)

    if test -n "$command"
        eval $command
    end
end
"#;

    fs::write(&file_path, content)?;
    eprintln!("Fish function created at: {}", file_path.display());
    eprintln!(
        "You may need to restart your shell or run 'source {}' to apply changes.",
        file_path.display()
    );

    Ok(())
}

fn setup_zsh() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    });

    let app_config_dir = config_dir.join("try-rs");

    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let file_path = app_config_dir.join("try-rs.zsh");
    let content = r#"try-rs() {
    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    local output
    output=$(command try-rs "$@")

    if [ -n "$output" ]; then
        eval "$output"
    fi
}
"#;

    fs::write(&file_path, content)?;
    eprintln!("ZSH function file created at: {}", file_path.display());

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let zshrc_path = home_dir.join(".zshrc");
    let source_cmd = format!("source {}", file_path.display());

    if zshrc_path.exists() {
        let zshrc_content = fs::read_to_string(&zshrc_path)?;
        if !zshrc_content.contains(&source_cmd) {
            use std::io::Write;
            let mut file = fs::OpenOptions::new().append(true).open(&zshrc_path)?;
            writeln!(file, "\n# try-rs integration")?;
            writeln!(file, "{}", source_cmd)?;
            eprintln!("Added configuration to ~/.zshrc");
        } else {
            eprintln!("Configuration already present in ~/.zshrc");
        }
    } else {
        eprintln!("You need to source this file in your ~/.zshrc:");
        eprintln!("{}", source_cmd);
    }

    Ok(())
}

fn setup_bash() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    });

    let app_config_dir = config_dir.join("try-rs");

    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let file_path = app_config_dir.join("try-rs.bash");
    let content = r#"try-rs() {
    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    local output
    output=$(command try-rs "$@")

    if [ -n "$output" ]; then
        eval "$output"
    fi
}
"#;

    fs::write(&file_path, content)?;
    eprintln!("Bash function file created at: {}", file_path.display());

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let bashrc_path = home_dir.join(".bashrc");
    let source_cmd = format!("source {}", file_path.display());

    if bashrc_path.exists() {
        let bashrc_content = fs::read_to_string(&bashrc_path)?;
        if !bashrc_content.contains(&source_cmd) {
            use std::io::Write;
            let mut file = fs::OpenOptions::new().append(true).open(&bashrc_path)?;
            writeln!(file, "\n# try-rs integration")?;
            writeln!(file, "{}", source_cmd)?;
            eprintln!("Added configuration to ~/.bashrc");
        } else {
            eprintln!("Configuration already present in ~/.bashrc");
        }
    } else {
        eprintln!("You need to source this file in your ~/.bashrc:");
        eprintln!("{}", source_cmd);
    }

    Ok(())
}

fn print_help() {
    eprintln!("🦀 try-rs {}", env!("CARGO_PKG_VERSION"));
    eprintln!("A blazing fast, Rust-based workspace manager for your temporary experiments.");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  try-rs [NAME|URL]     Create or jump to an experiment / Clone a repo");
    eprintln!("  try-rs                Open the TUI (Terminal User Interface)");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --setup <shell>       Generate shell integration code (fish, zsh, bash)");
    eprintln!("  --version             Show version information");
    eprintln!("  --help, -h            Show this help message");
}

fn main() -> Result<()> {
    let (tries_dir, theme, editor_cmd, is_first_run) = load_configuration();

    // Ensure the directory exists (either from config or default)
    if !tries_dir.exists() {
        fs::create_dir_all(&tries_dir)?;
    }

    // 2. Check command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return Ok(());
    }

    if args.len() > 1 && args[1] == "--version" {
        eprintln!("try-rs {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if is_first_run {
        let is_setup = args.len() > 1 && args[1] == "--setup";
        if !is_setup {
            let shell = std::env::var("SHELL").unwrap_or_default();
            let shell_type = if shell.contains("fish") {
                Some("fish")
            } else if shell.contains("zsh") {
                Some("zsh")
            } else if shell.contains("bash") {
                Some("bash")
            } else {
                None
            };

            if let Some(s) = shell_type {
                eprintln!("Detected shell: {}", s);
                eprint!("Shell integration not configured. Do you want to set it up for {}? [Y/n] ", s);
                io::stderr().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y") {
                    match s {
                        "fish" => setup_fish()?,
                        "zsh" => setup_zsh()?,
                        "bash" => setup_bash()?,
                        _ => {}
                    }
                }
            }
        }
    }

    if args.len() > 2 && args[1] == "--setup" {
        match args[2].as_str() {
            "fish" => return setup_fish(),
            "zsh" => return setup_zsh(),
            "bash" => return setup_bash(),
            _ => {}
        }
    }

    // The 'selection' variable will hold the chosen name or URL.
    // It can come from arguments (CLI) or the interface (TUI).
    let selection_result: Option<String>;
    let mut open_editor = false;

    if args.len() > 1 {
        // CLI MODE: The user passed an argument (e.g., try-rs https://...)
        // We skip the graphical interface entirely.
        selection_result = Some(args[1].clone());
    } else {
        // TUI MODE: No arguments, open the visual interface.

        enable_raw_mode()?;
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend)?;

        let app = App::new(tries_dir.clone(), theme, editor_cmd.clone());
        // Run the app and capture the result
        (selection_result, open_editor) = run_app(&mut terminal, app)?;

        // Restore the terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
    }

    // 3. Process the result (Common for both modes)
    if let Some(selection) = selection_result {
        let target_path = tries_dir.join(&selection);

        // CASE 1: Does the folder already exist? Enter it.
        if target_path.exists() {
            if open_editor && editor_cmd.is_some() {
                println!(
                    "{} '{}'",
                    editor_cmd.unwrap(),
                    target_path.to_string_lossy()
                );
            } else {
                println!("cd '{}'", target_path.to_string_lossy());
            }
        } else {
            // CASE 2: Is it a Git URL? Clone it!
            if is_git_url(&selection) {
                let repo_name = extract_repo_name(&selection);

                let now = Local::now();
                let date_prefix = now.format("%Y-%m-%d").to_string();
                let folder_name = format!("{}-{}", date_prefix, repo_name);
                let new_path = tries_dir.join(&folder_name);

                eprintln!("A clonar {} para {}...", selection, folder_name);

                let status = std::process::Command::new("git")
                    .arg("clone")
                    .arg(&selection)
                    .arg(&new_path)
                    .stdout(Stdio::null())
                    .stderr(Stdio::inherit())
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        if open_editor && editor_cmd.is_some() {
                            println!("{} '{}'", editor_cmd.unwrap(), new_path.to_string_lossy());
                        } else {
                            println!("cd '{}'", new_path.to_string_lossy());
                        }
                    }
                    _ => {
                        eprintln!("Error: Failed to clone the repository.");
                    }
                }
            } else {
                // CASE 3: Create an empty folder
                let now = Local::now();
                let date_prefix = now.format("%Y-%m-%d").to_string();

                let new_name = if selection.starts_with(&date_prefix) {
                    selection
                } else {
                    format!("{}-{}", date_prefix, selection)
                };

                let new_path = tries_dir.join(&new_name);
                fs::create_dir_all(&new_path)?;
                if open_editor && editor_cmd.is_some() {
                    println!("{} '{}'", editor_cmd.unwrap(), new_path.to_string_lossy());
                } else {
                    println!("cd '{}'", new_path.to_string_lossy());
                }
            }
        }
    }

    Ok(())
}

// Checks if the string looks like a Git URL
fn is_git_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("git@")
        || s.starts_with("ssh://")
}

// Extracts a clean repository name (e.g., "github.com/tobi/try.git" -> "try")
fn extract_repo_name(url: &str) -> String {
    // Remove the .git suffix if it exists
    let clean_url = url.trim_end_matches(".git");

    // Get the last part after the '/' or ':' (common in ssh)
    if let Some(last_part) = clean_url.rsplit(|c| c == '/' || c == ':').next() {
        if !last_part.is_empty() {
            return last_part.to_string();
        }
    }
    // Generic name if detection fails
    "cloned-repo".to_string()
}
