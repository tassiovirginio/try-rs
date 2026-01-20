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

use crate::config::{get_file_config_toml_name, save_config};
pub use crate::themes::Theme;

#[derive(Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    DeleteConfirm,
    ThemeSelect,
    ConfigSavePrompt,
    ConfigSaveLocationSelect,
    About,
}

#[derive(Clone)]
pub struct TryEntry {
    pub name: String,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub score: i64,
    pub is_git: bool,
    pub is_worktree: bool,
    pub is_mise: bool,
    pub is_cargo: bool,
    pub is_maven: bool,
    pub is_flutter: bool,
    pub is_go: bool,
    pub is_python: bool,
}

pub struct App {
    pub query: String,
    pub all_entries: Vec<TryEntry>,
    pub filtered_entries: Vec<TryEntry>,
    pub selected_index: usize,
    pub should_quit: bool,
    pub final_selection: Option<String>,
    pub mode: AppMode,
    pub status_message: Option<String>,
    pub base_path: PathBuf,
    pub theme: Theme,
    pub editor_cmd: Option<String>,
    pub wants_editor: bool,

    pub available_themes: Vec<Theme>,
    pub theme_list_state: ListState,

    pub config_path: Option<PathBuf>,
    pub config_location_state: ListState,
}

impl App {
    pub fn new(
        path: PathBuf,
        theme: Theme,
        editor_cmd: Option<String>,
        config_path: Option<PathBuf>,
    ) -> Self {
        let mut entries = Vec::new();
        if let Ok(read_dir) = fs::read_dir(&path) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata()
                    && metadata.is_dir()
                {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let git_path = entry.path().join(".git");
                    let is_git = git_path.exists();
                    let is_worktree = git_path.is_file();
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
                        is_worktree,
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
        entries.sort_by(|a, b| b.modified.cmp(&a.modified));

        let themes = Theme::all();

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
            config_path,
            config_location_state: ListState::default(),
        }
    }

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

            self.filtered_entries.sort_by(|a, b| b.score.cmp(&a.score));
        }
        self.selected_index = 0;
    }

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

    f.render_widget(Clear, popup_area);

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

fn draw_config_location_select(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(8),
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

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Select Config Location ")
        .borders(Borders::ALL)
        .style(Style::default().bg(app.theme.popup_bg));

    let config_name = get_file_config_toml_name();
    let items = vec![
        ListItem::new(format!("System Config (~/.config/try-rs/{})", config_name))
            .style(Style::default().fg(app.theme.list_highlight_fg)),
        ListItem::new(format!("Home Directory (~/{})", config_name))
            .style(Style::default().fg(app.theme.list_highlight_fg)),
    ];

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(app.theme.list_highlight_bg)
                .fg(app.theme.list_highlight_fg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, popup_area, &mut app.config_location_state);
}

fn draw_about_popup(f: &mut Frame, theme: &Theme) {
    let area = f.area();
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Length(12),
            Constraint::Percentage(25),
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
        .title(" About ")
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.popup_bg));

    let text = vec![
        Line::from(vec![
            Span::styled(
                "🦀 try",
                Style::default()
                    .fg(theme.title_try)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("-", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "rs",
                Style::default()
                    .fg(theme.title_rs)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" v{}", env!("CARGO_PKG_VERSION")),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "try-rs.org",
            Style::default().fg(theme.search_box),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "github.com/tassiovirginio/try-rs",
            Style::default().fg(theme.search_box),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("📜 License: ", Style::default().fg(theme.help_text)),
            Span::styled(
                "MIT",
                Style::default()
                    .fg(theme.status_message)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc to close",
            Style::default().fg(theme.help_text),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, popup_area);
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
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(f.area());

            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[1]);

            let search_text = Paragraph::new(app.query.clone())
                .style(Style::default().fg(app.theme.search_box))
                .block(Block::default().borders(Borders::ALL).title(" Search/New "));
            f.render_widget(search_text, chunks[0]);

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

                    let width = content_chunks[0].width.saturating_sub(5) as usize;

                    let date_text = date_str.to_string();
                    let date_width = date_text.chars().count();
                    let git_icon = if entry.is_git { " " } else { "" };
                    let git_width = if entry.is_git { 2 } else { 0 };
                    let worktree_icon = if entry.is_worktree { "󰙅" } else { "" };
                    let worktree_width = if entry.is_worktree { 2 } else { 0 };
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
                    let icon_width = 2;

                    let created_dt: chrono::DateTime<Local> = entry.created.into();
                    let created_text = created_dt.format("%Y-%m-%d").to_string();
                    let created_width = created_text.chars().count();

                    let reserved = date_width
                        + git_width
                        + worktree_width
                        + mise_width
                        + cargo_width
                        + maven_width
                        + flutter_width
                        + go_width
                        + python_width
                        + icon_width
                        + created_width
                        + 2;
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
                                    + worktree_width
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
                        Span::styled(
                            worktree_icon,
                            Style::default().fg(Color::Rgb(100, 180, 100)),
                        ),
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
                    Span::styled("Ctrl-A", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" About | "),
                    Span::styled("Esc/Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Quit"),
                ])
            };

            let help_message = Paragraph::new(help_text)
                .style(Style::default().fg(app.theme.help_text))
                .alignment(Alignment::Center);

            f.render_widget(help_message, chunks[2]);

            if app.mode == AppMode::DeleteConfirm
                && let Some(selected) = app.filtered_entries.get(app.selected_index)
            {
                let msg = format!("Delete '{}'? (y/n)", selected.name);
                draw_popup(f, " WARNING ", &msg, &app.theme);
            }

            if app.mode == AppMode::ThemeSelect {
                draw_theme_select(f, &mut app);
            }

            if app.mode == AppMode::ConfigSavePrompt {
                draw_popup(
                    f,
                    " Create Config? ",
                    "Config file not found.\nCreate one now to save theme? (y/n)",
                    &app.theme,
                );
            }

            if app.mode == AppMode::ConfigSaveLocationSelect {
                draw_config_location_select(f, &mut app);
            }

            if app.mode == AppMode::About {
                draw_about_popup(f, &app.theme);
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
                        } else if c == 'a' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            app.mode = AppMode::About;
                        } else if matches!(c, 'k' | 'p')
                            && key.modifiers.contains(event::KeyModifiers::CONTROL)
                        {
                            if app.selected_index > 0 {
                                app.selected_index -= 1;
                            }
                        } else if matches!(c, 'j' | 'n')
                            && key.modifiers.contains(event::KeyModifiers::CONTROL)
                        {
                            if app.selected_index < app.filtered_entries.len().saturating_sub(1) {
                                app.selected_index += 1;
                            }
                        } else if c == 'u' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            app.query.clear();
                            app.update_search();
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
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Up | KeyCode::Char('k' | 'p') => {
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
                    KeyCode::Down | KeyCode::Char('j' | 'n') => {
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

                                if let Some(ref path) = app.config_path {
                                    if let Err(e) = save_config(
                                        path,
                                        &app.theme,
                                        &app.base_path,
                                        &app.editor_cmd,
                                    ) {
                                        app.status_message = Some(format!("Error saving: {}", e));
                                    } else {
                                        app.status_message = Some("Theme saved.".to_string());
                                    }
                                    app.mode = AppMode::Normal;
                                } else {
                                    app.mode = AppMode::ConfigSavePrompt;
                                }
                            } else {
                                app.mode = AppMode::Normal;
                            }
                        } else {
                            app.mode = AppMode::Normal;
                        }
                    }
                    _ => {}
                },
                AppMode::ConfigSavePrompt => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                        app.mode = AppMode::ConfigSaveLocationSelect;
                        app.config_location_state.select(Some(0));
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    _ => {}
                },

                AppMode::ConfigSaveLocationSelect => match key.code {
                    KeyCode::Esc | KeyCode::Char('c')
                        if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Up | KeyCode::Char('k' | 'p') => {
                        let i = match app.config_location_state.selected() {
                            Some(i) => {
                                if i > 0 {
                                    i - 1
                                } else {
                                    i
                                }
                            }
                            None => 0,
                        };
                        app.config_location_state.select(Some(i));
                    }
                    KeyCode::Down | KeyCode::Char('j' | 'n') => {
                        let i = match app.config_location_state.selected() {
                            Some(i) => {
                                if i < 1 {
                                    i + 1
                                } else {
                                    i
                                }
                            }
                            None => 0,
                        };
                        app.config_location_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        if let Some(i) = app.config_location_state.selected() {
                            let config_name = get_file_config_toml_name();
                            let path = if i == 0 {
                                dirs::config_dir()
                                    .unwrap_or_else(|| {
                                        dirs::home_dir().expect("Folder not found").join(".config")
                                    })
                                    .join("try-rs")
                                    .join(&config_name)
                            } else {
                                dirs::home_dir()
                                    .expect("Folder not found")
                                    .join(&config_name)
                            };

                            if let Err(e) =
                                save_config(&path, &app.theme, &app.base_path, &app.editor_cmd)
                            {
                                app.status_message = Some(format!("Error saving config: {}", e));
                            } else {
                                app.config_path = Some(path);
                                app.status_message = Some("Theme saved!".to_string());
                            }
                        }
                        app.mode = AppMode::Normal;
                    }
                    _ => {}
                },
                AppMode::About => match key.code {
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char(' ') => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        app.mode = AppMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }

    Ok((app.final_selection, app.wants_editor))
}
