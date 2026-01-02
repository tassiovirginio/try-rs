use anyhow::Result;
use chrono::Local;
use crossterm::event::{self, Event, KeyCode};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::{prelude::*, widgets::*};

use std::{
    fs,
    io::{self},
    path::PathBuf,
    time::SystemTime,
};

#[derive(Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    DeleteConfirm,
    ThemeSelect,
}

// Data model (same as before)
#[derive(Clone)]
pub struct TryEntry {
    pub name: String,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub score: i64,
    pub is_git: bool,
    pub is_mise: bool,
    pub is_cargo: bool,
    pub is_maven: bool,
    pub is_flutter: bool,
    pub is_go: bool,
    pub is_python: bool,
}

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub title_try: Color,
    pub title_rs: Color,
    pub search_box: Color,
    pub list_date: Color,
    pub list_highlight_bg: Color,
    pub list_highlight_fg: Color,
    pub help_text: Color,
    pub status_message: Color,
    pub popup_bg: Color,
    pub popup_text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
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

    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            title_try: Color::Rgb(189, 147, 249),      // Purple
            title_rs: Color::Rgb(255, 121, 198),       // Pink
            search_box: Color::Rgb(255, 184, 108),     // Orange
            list_date: Color::Rgb(139, 233, 253),      // Cyan
            list_highlight_bg: Color::Rgb(68, 71, 90), // Selection
            list_highlight_fg: Color::Rgb(248, 248, 242), // Foreground
            help_text: Color::Rgb(98, 114, 164),       // Comment
            status_message: Color::Rgb(241, 250, 140), // Yellow
            popup_bg: Color::Rgb(40, 42, 54),          // Background
            popup_text: Color::Rgb(255, 85, 85),       // Red
        }
    }

    pub fn jetbrains_darcula() -> Self {
        Self {
            name: "JetBrains Darcula".to_string(),
            title_try: Color::Rgb(78, 124, 238),        // Blueish
            title_rs: Color::Rgb(204, 120, 50),         // Orange
            search_box: Color::Rgb(106, 135, 89),       // Green
            list_date: Color::Rgb(128, 128, 128),       // Grey
            list_highlight_bg: Color::Rgb(33, 66, 131), // Selection
            list_highlight_fg: Color::Rgb(187, 187, 187), // Light Grey
            help_text: Color::Rgb(128, 128, 128),
            status_message: Color::Rgb(255, 198, 109), // Gold
            popup_bg: Color::Rgb(60, 63, 65),          // Bg
            popup_text: Color::Rgb(204, 120, 50),
        }
    }

    pub fn gruvbox_dark() -> Self {
        Self {
            name: "Gruvbox Dark".to_string(),
            title_try: Color::Rgb(251, 73, 52),           // Red
            title_rs: Color::Rgb(250, 189, 47),           // Yellow
            search_box: Color::Rgb(184, 187, 38),         // Green
            list_date: Color::Rgb(146, 131, 116),         // Grey
            list_highlight_bg: Color::Rgb(80, 73, 69),    // Bg2
            list_highlight_fg: Color::Rgb(235, 219, 178), // Fg
            help_text: Color::Rgb(168, 153, 132),
            status_message: Color::Rgb(215, 153, 33), // Orange
            popup_bg: Color::Rgb(40, 40, 40),         // Bg0
            popup_text: Color::Rgb(251, 73, 52),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".to_string(),
            title_try: Color::Rgb(136, 192, 208),  // Frost Cyan
            title_rs: Color::Rgb(191, 97, 106),    // Aurora Red
            search_box: Color::Rgb(163, 190, 140), // Aurora Green
            list_date: Color::Rgb(216, 222, 233),  // Snow Storm
            list_highlight_bg: Color::Rgb(67, 76, 94), // Polar Night 3
            list_highlight_fg: Color::Rgb(236, 239, 244), // Snow Storm 3
            help_text: Color::Rgb(76, 86, 106),    // Polar Night 2
            status_message: Color::Rgb(235, 203, 139), // Aurora Yellow
            popup_bg: Color::Rgb(46, 52, 64),      // Polar Night 0
            popup_text: Color::Rgb(191, 97, 106),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            title_try: Color::Rgb(122, 162, 247),         // Blue
            title_rs: Color::Rgb(247, 118, 142),          // Red
            search_box: Color::Rgb(158, 206, 106),        // Green
            list_date: Color::Rgb(169, 177, 214),         // Fg
            list_highlight_bg: Color::Rgb(65, 72, 104),   // Terminal Black
            list_highlight_fg: Color::Rgb(192, 202, 245), // Terminal White
            help_text: Color::Rgb(86, 95, 137),           // Comment
            status_message: Color::Rgb(224, 175, 104),    // Yellow
            popup_bg: Color::Rgb(26, 27, 38),             // Bg
            popup_text: Color::Rgb(247, 118, 142),
        }
    }
}

// Our TUI state
pub struct App {
    pub query: String,                   // What the user typed
    pub all_entries: Vec<TryEntry>,      // All directories found
    pub filtered_entries: Vec<TryEntry>, // Directories filtered by search
    pub selected_index: usize,           // Which item is currently selected in the list
    pub should_quit: bool,               // Flag to exit the loop
    pub final_selection: Option<String>, // The final result (for the shell)
    pub mode: AppMode,
    pub status_message: Option<String>, // Feedback message for the user
    pub base_path: PathBuf,             // Base directory for tries
    pub theme: Theme,                   // Application colors
    pub editor_cmd: Option<String>,     // Editor command (e.g., "code", "nvim")
    pub wants_editor: bool,             // Flag to indicate if we should open the editor

    // Theme switching
    pub available_themes: Vec<Theme>,
    pub theme_list_state: ListState,
}

impl App {
    pub fn new(path: PathBuf, theme: Theme, editor_cmd: Option<String>) -> Self {
        let mut entries = Vec::new();
        if let Ok(read_dir) = fs::read_dir(&path) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata()
                    && metadata.is_dir()
                {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_git = entry.path().join(".git").exists();
                    let is_mise = entry.path().join("mise.toml").exists();
                    let is_cargo = entry.path().join("Cargo.toml").exists();
                    let is_maven = entry.path().join("pom.xml").exists();
                    let is_flutter = entry.path().join("pubspec.yaml").exists();
                    let is_go = entry.path().join("go.mod").exists();
                    let is_python = entry.path().join("pyproject.toml").exists()
                        || entry.path().join("requirements.txt").exists();
                    entries.push(TryEntry {
                        name,
                        modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                        created: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
                        score: 0,
                        is_git,
                        is_mise,
                        is_cargo,
                        is_maven,
                        is_flutter,
                        is_go,
                        is_python,
                    });
                }
            }
        }
        // Initial sort: most recent first
        entries.sort_by(|a, b| b.modified.cmp(&a.modified));

        let themes = vec![
            Theme::catppuccin_mocha(),
            Theme::dracula(),
            Theme::jetbrains_darcula(),
            Theme::gruvbox_dark(),
            Theme::nord(),
            Theme::tokyo_night(),
        ];

        let mut theme_state = ListState::default();
        theme_state.select(Some(0));

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
            available_themes: themes,
            theme_list_state: theme_state,
        }
    }

    // Filter update logic
    pub fn update_search(&mut self) {
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
    pub fn delete_selected(&mut self) {
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

fn draw_theme_select(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(area);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(popup_layout[1])[1];

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Select Theme ")
        .borders(Borders::ALL)
        .style(Style::default().bg(app.theme.popup_bg));

    let items: Vec<ListItem> = app
        .available_themes
        .iter()
        .map(|t| {
            ListItem::new(t.name.clone()).style(Style::default().fg(app.theme.list_highlight_fg))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(app.theme.list_highlight_bg)
                .fg(app.theme.list_highlight_fg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, popup_area, &mut app.theme_list_state);
}

pub fn run_app(
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
                    let elapsed = now
                        .duration_since(entry.modified)
                        .unwrap_or(std::time::Duration::ZERO);
                    let secs = elapsed.as_secs();
                    let days = secs / 86400;
                    let hours = (secs % 86400) / 3600;
                    let minutes = (secs % 3600) / 60;
                    let date_str = format!("({:02}d {:02}h {:02}m)", days, hours, minutes);

                    // Calculate available width (block borders take 2 columns)
                    let width = content_chunks[0].width.saturating_sub(5) as usize;

                    let date_text = date_str.to_string();
                    let date_width = date_text.chars().count();
                    let git_icon = if entry.is_git { " " } else { "" };
                    let git_width = if entry.is_git { 2 } else { 0 };
                    let mise_icon = if entry.is_mise { "󰬔 " } else { "" };
                    let mise_width = if entry.is_mise { 2 } else { 0 };
                    let cargo_icon = if entry.is_cargo { " " } else { "" };
                    let cargo_width = if entry.is_cargo { 2 } else { 0 };
                    let maven_icon = if entry.is_maven { " " } else { "" };
                    let maven_width = if entry.is_maven { 2 } else { 0 };
                    let flutter_icon = if entry.is_flutter { " " } else { "" };
                    let flutter_width = if entry.is_flutter { 2 } else { 0 };
                    let go_icon = if entry.is_go { " " } else { "" };
                    let go_width = if entry.is_go { 2 } else { 0 };
                    let python_icon = if entry.is_python { " " } else { "" };
                    let python_width = if entry.is_python { 2 } else { 0 };
                    let icon_width = 2; // "📁" takes 2 columns

                    let created_dt: chrono::DateTime<Local> = entry.created.into();
                    let created_text = created_dt.format("%Y-%m-%d").to_string();
                    let created_width = created_text.chars().count();

                    // Calculate space for name
                    let reserved = date_width
                        + git_width
                        + mise_width
                        + cargo_width
                        + maven_width
                        + flutter_width
                        + go_width
                        + python_width
                        + icon_width
                        + created_width
                        + 2; // +2 for gaps
                    let available_for_name = width.saturating_sub(reserved);
                    let name_len = entry.name.chars().count();

                    let (display_name, padding) = if name_len > available_for_name {
                        let safe_len = available_for_name.saturating_sub(3);
                        let truncated: String = entry.name.chars().take(safe_len).collect();
                        (format!("{}...", truncated), 1)
                    } else {
                        (
                            entry.name.clone(),
                            width.saturating_sub(
                                icon_width
                                    + created_width
                                    + 1
                                    + name_len
                                    + date_width
                                    + git_width
                                    + mise_width
                                    + cargo_width
                                    + maven_width
                                    + flutter_width
                                    + go_width
                                    + python_width,
                            ),
                        )
                    };

                    let content = Line::from(vec![
                        Span::raw("📁"),
                        Span::styled(created_text, Style::default().fg(app.theme.list_date)),
                        Span::raw(format!(" {}", display_name)),
                        Span::raw(" ".repeat(padding)),
                        Span::styled(cargo_icon, Style::default().fg(Color::Rgb(230, 100, 50))),
                        Span::styled(maven_icon, Style::default().fg(Color::Rgb(255, 150, 50))),
                        Span::styled(flutter_icon, Style::default().fg(Color::Rgb(2, 123, 222))),
                        Span::styled(go_icon, Style::default().fg(Color::Rgb(0, 173, 216))),
                        Span::styled(python_icon, Style::default().fg(Color::Yellow)),
                        Span::styled(mise_icon, Style::default().fg(Color::Rgb(250, 179, 135))),
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

            if let Some(selected) = app.filtered_entries.get(app.selected_index) {
                let preview_path = app.base_path.join(&selected.name);
                let mut preview_lines = Vec::new();

                if let Ok(entries) = fs::read_dir(&preview_path) {
                    for e in entries
                        .take(content_chunks[1].height.saturating_sub(2) as usize)
                        .flatten()
                    {
                        let file_name = e.file_name().to_string_lossy().to_string();
                        let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                        let icon = if is_dir { "📁 " } else { "📄 " };
                        preview_lines.push(Line::from(vec![
                            Span::styled(icon, Style::default().fg(app.theme.title_try)),
                            Span::raw(file_name),
                        ]));
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
                    Span::raw(" Nav | "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Select | "),
                    Span::styled("Ctrl-D", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Del | "),
                    Span::styled("Ctrl-E", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Edit | "),
                    Span::styled("Ctrl-T", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Theme | "),
                    Span::styled("Esc/Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Quit"),
                ])
            };

            let help_message = Paragraph::new(help_text)
                .style(Style::default().fg(app.theme.help_text))
                .alignment(Alignment::Center);

            f.render_widget(help_message, chunks[3]);

            if app.mode == AppMode::DeleteConfirm
                && let Some(selected) = app.filtered_entries.get(app.selected_index)
            {
                let msg = format!("Delete '{}'? (y/n)", selected.name);
                draw_popup(f, " WARNING ", &msg, &app.theme);
            }

            if app.mode == AppMode::ThemeSelect {
                draw_theme_select(f, &mut app);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
            && key.is_press()
        {
            match app.mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char(c) => {
                        if c == 'c' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            app.should_quit = true;
                        } else if c == 'd' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            if !app.filtered_entries.is_empty() {
                                app.mode = AppMode::DeleteConfirm;
                            }
                        } else if c == 'e' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            if app.editor_cmd.is_some() {
                                if !app.filtered_entries.is_empty() {
                                    app.final_selection =
                                        Some(app.filtered_entries[app.selected_index].name.clone());
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
                        } else if c == 't' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            app.mode = AppMode::ThemeSelect;
                        } else {
                            app.query.push(c);
                            app.status_message = None;
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

                AppMode::ThemeSelect => match key.code {
                    KeyCode::Esc | KeyCode::Char('c')
                        if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Up => {
                        let i = match app.theme_list_state.selected() {
                            Some(i) => {
                                if i > 0 {
                                    i - 1
                                } else {
                                    i
                                }
                            }
                            None => 0,
                        };
                        app.theme_list_state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = match app.theme_list_state.selected() {
                            Some(i) => {
                                if i < app.available_themes.len() - 1 {
                                    i + 1
                                } else {
                                    i
                                }
                            }
                            None => 0,
                        };
                        app.theme_list_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        if let Some(i) = app.theme_list_state.selected() {
                            if let Some(theme) = app.available_themes.get(i) {
                                app.theme = theme.clone();
                            }
                        }
                        app.mode = AppMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }

    Ok((app.final_selection, app.wants_editor))
}
