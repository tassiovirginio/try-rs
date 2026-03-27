use anyhow::Result;
use chrono::Local;
use crossterm::event::{self, Event, KeyCode};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::{prelude::*, widgets::*};

use std::{
    collections::HashSet,
    fs,
    io::{self},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::SystemTime,
};

pub use crate::themes::Theme;
use crate::{
    config::{get_file_config_toml_name, save_config},
    utils::{self, SelectionResult},
};

#[derive(Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    DeleteConfirm,
    RenamePrompt,
    ThemeSelect,
    ConfigSavePrompt,
    ConfigSaveLocationSelect,
    About,
}

#[derive(Clone)]
pub struct TryEntry {
    pub name: String,
    pub display_name: String,
    pub display_offset: usize,
    pub match_indices: Vec<usize>,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub score: i64,
    pub is_git: bool,
    pub is_worktree: bool,
    pub is_worktree_locked: bool,
    pub is_gitmodules: bool,
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
    pub final_selection: SelectionResult,
    pub mode: AppMode,
    pub status_message: Option<String>,
    pub base_path: PathBuf,
    pub theme: Theme,
    pub editor_cmd: Option<String>,
    pub wants_editor: bool,
    pub apply_date_prefix: Option<bool>,
    pub transparent_background: bool,
    pub show_new_option: bool,
    pub show_disk: bool,
    pub show_preview: bool,
    pub show_legend: bool,
    pub right_panel_visible: bool,
    pub right_panel_width: u16,

    pub tries_dirs: Vec<PathBuf>,
    pub active_tab: usize,
    pub tab_list_state: ListState,

    pub available_themes: Vec<Theme>,
    pub theme_list_state: ListState,
    pub original_theme: Option<Theme>,
    pub original_transparent_background: Option<bool>,

    pub config_path: Option<PathBuf>,
    pub config_location_state: ListState,

    pub cached_free_space_mb: Option<u64>,
    pub folder_size_mb: Arc<AtomicU64>,

    pub rename_input: String,

    current_entries: HashSet<String>,
    matcher: SkimMatcherV2,
}

impl App {
    fn is_current_entry(
        entry_path: &Path,
        entry_name: &str,
        is_symlink: bool,
        cwd_unresolved: &Path,
        cwd_real: &Path,
        base_real: &Path,
    ) -> bool {
        if cwd_unresolved.starts_with(entry_path) {
            return true;
        }

        if is_symlink {
            if let Ok(target) = entry_path.canonicalize()
                && cwd_real.starts_with(&target)
            {
                return true;
            }
        } else {
            let resolved_entry = base_real.join(entry_name);
            if cwd_real.starts_with(&resolved_entry) {
                return true;
            }
        }

        false
    }

    pub fn new(
        path: PathBuf,
        theme: Theme,
        editor_cmd: Option<String>,
        config_path: Option<PathBuf>,
        apply_date_prefix: Option<bool>,
        transparent_background: bool,
        query: Option<String>,
        tries_dirs: Vec<PathBuf>,
        active_tab: usize,
    ) -> Self {
        let mut entries = Vec::new();
        let mut current_entries = HashSet::new();
        let cwd_unresolved = std::env::var_os("PWD")
            .map(PathBuf::from)
            .filter(|p| !p.as_os_str().is_empty())
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        let cwd_real = std::env::current_dir()
            .ok()
            .and_then(|cwd| cwd.canonicalize().ok())
            .unwrap_or_else(|| cwd_unresolved.clone());
        let base_real = path.canonicalize().unwrap_or_else(|_| path.clone());

        if let Ok(read_dir) = fs::read_dir(&path) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata()
                    && metadata.is_dir()
                {
                    let entry_path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let git_path = entry_path.join(".git");
                    let is_git = git_path.exists();
                    let is_worktree = git_path.is_file();
                    let is_worktree_locked = utils::is_git_worktree_locked(&entry_path);
                    let is_gitmodules = entry_path.join(".gitmodules").exists();
                    let is_mise = entry_path.join("mise.toml").exists();
                    let is_cargo = entry_path.join("Cargo.toml").exists();
                    let is_maven = entry_path.join("pom.xml").exists();
                    let is_symlink = entry
                        .file_type()
                        .map(|kind| kind.is_symlink())
                        .unwrap_or(false);
                    let is_current = Self::is_current_entry(
                        &entry_path,
                        &name,
                        is_symlink,
                        &cwd_unresolved,
                        &cwd_real,
                        &base_real,
                    );
                    if is_current {
                        current_entries.insert(name.clone());
                    }

                    let created;
                    let display_name;
                    if let Some((date_prefix, remainder)) = utils::extract_prefix_date(&name) {
                        created = date_prefix;
                        display_name = remainder;
                    } else {
                        created = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
                        display_name = name.clone();
                    }
                    let display_offset = name
                        .chars()
                        .count()
                        .saturating_sub(display_name.chars().count());
                    let is_flutter = entry_path.join("pubspec.yaml").exists();
                    let is_go = entry_path.join("go.mod").exists();
                    let is_python = entry_path.join("pyproject.toml").exists()
                        || entry_path.join("requirements.txt").exists();
                    entries.push(TryEntry {
                        name,
                        display_name,
                        display_offset,
                        match_indices: Vec::new(),
                        modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                        created,
                        score: 0,
                        is_git,
                        is_worktree,
                        is_worktree_locked,
                        is_gitmodules,
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

        let mut app = Self {
            query: query.unwrap_or_else(|| String::new()),
            all_entries: entries.clone(),
            filtered_entries: entries,
            selected_index: 0,
            should_quit: false,
            final_selection: SelectionResult::None,
            mode: AppMode::Normal,
            status_message: None,
            base_path: path.clone(),
            theme,
            editor_cmd,
            wants_editor: false,
            apply_date_prefix,
            transparent_background,
            show_new_option: false,
            show_disk: true,
            show_preview: true,
            show_legend: true,
            right_panel_visible: true,
            right_panel_width: 25,
            tries_dirs: tries_dirs.clone(),
            active_tab,
            tab_list_state: ListState::default(),
            available_themes: themes,
            theme_list_state: theme_state,
            original_theme: None,
            original_transparent_background: None,
            config_path,
            config_location_state: ListState::default(),
            cached_free_space_mb: utils::get_free_disk_space_mb(&path),
            folder_size_mb: Arc::new(AtomicU64::new(0)),
            rename_input: String::new(),
            current_entries,
            matcher: SkimMatcherV2::default(),
        };

        // Spawn background thread to calculate folder size
        let folder_size_arc = Arc::clone(&app.folder_size_mb);
        let path_clone = path.clone();
        thread::spawn(move || {
            let size = utils::get_folder_size_mb(&path_clone);
            folder_size_arc.store(size, Ordering::Relaxed);
        });

        app.update_search();
        app
    }

    pub fn switch_tab(&mut self, new_tab: usize) {
        if new_tab >= self.tries_dirs.len() {
            return;
        }
        self.active_tab = new_tab;
        self.base_path = self.tries_dirs[new_tab].clone();
        self.cached_free_space_mb = utils::get_free_disk_space_mb(&self.base_path);
        self.folder_size_mb = Arc::new(AtomicU64::new(0));
        
        let path_clone = self.base_path.clone();
        let folder_size_arc = Arc::clone(&self.folder_size_mb);
        thread::spawn(move || {
            let size = utils::get_folder_size_mb(&path_clone);
            folder_size_arc.store(size, Ordering::Relaxed);
        });

        self.query.clear();
        self.load_entries();
        self.update_search();
    }

    fn load_entries(&mut self) {
        self.all_entries.clear();
        let cwd_unresolved = std::env::var_os("PWD")
            .map(PathBuf::from)
            .filter(|p| !p.as_os_str().is_empty())
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        let cwd_real = std::env::current_dir()
            .ok()
            .and_then(|cwd| cwd.canonicalize().ok())
            .unwrap_or_else(|| cwd_unresolved.clone());
        let base_real = self.base_path.canonicalize().unwrap_or_else(|_| self.base_path.clone());

        if let Ok(read_dir) = fs::read_dir(&self.base_path) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata()
                    && metadata.is_dir()
                {
                    let entry_path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let git_path = entry_path.join(".git");
                    let is_git = git_path.exists();
                    let is_worktree = git_path.is_file();
                    let is_worktree_locked = utils::is_git_worktree_locked(&entry_path);
                    let is_gitmodules = entry_path.join(".gitmodules").exists();
                    let is_mise = entry_path.join("mise.toml").exists();
                    let is_cargo = entry_path.join("Cargo.toml").exists();
                    let is_maven = entry_path.join("pom.xml").exists();
                    let is_symlink = entry
                        .file_type()
                        .map(|kind| kind.is_symlink())
                        .unwrap_or(false);
                    let is_current = Self::is_current_entry(
                        &entry_path,
                        &name,
                        is_symlink,
                        &cwd_unresolved,
                        &cwd_real,
                        &base_real,
                    );

                    let created;
                    let display_name;
                    if let Some((date_prefix, remainder)) = utils::extract_prefix_date(&name) {
                        created = date_prefix;
                        display_name = remainder;
                    } else {
                        created = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
                        display_name = name.clone();
                    }
                    let display_offset = name
                        .chars()
                        .count()
                        .saturating_sub(display_name.chars().count());
                    let is_flutter = entry_path.join("pubspec.yaml").exists();
                    let is_go = entry_path.join("go.mod").exists();
                    let is_python = entry_path.join("pyproject.toml").exists()
                        || entry_path.join("requirements.txt").exists();
                    self.all_entries.push(TryEntry {
                        name,
                        display_name,
                        display_offset,
                        match_indices: Vec::new(),
                        modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                        created,
                        score: 0,
                        is_git,
                        is_worktree,
                        is_worktree_locked,
                        is_gitmodules,
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
        self.all_entries.sort_by(|a, b| b.modified.cmp(&a.modified));
    }

    pub fn has_exact_match(&self) -> bool {
        self.all_entries.iter().any(|e| e.name == self.query)
    }

    pub fn update_search(&mut self) {
        if self.query.is_empty() {
            self.filtered_entries = self.all_entries.clone();
        } else {
            self.filtered_entries = self
                .all_entries
                .iter()
                .filter_map(|entry| {
                    self.matcher
                        .fuzzy_indices(&entry.name, &self.query)
                        .map(|(score, indices)| {
                            let mut e = entry.clone();
                            e.score = score;
                            if entry.display_offset == 0 {
                                e.match_indices = indices;
                            } else {
                                e.match_indices = indices
                                    .into_iter()
                                    .filter_map(|idx| idx.checked_sub(entry.display_offset))
                                    .collect();
                            }
                            e
                        })
                })
                .collect();

            self.filtered_entries.sort_by(|a, b| b.score.cmp(&a.score));
        }
        self.show_new_option = !self.query.is_empty() && !self.has_exact_match();
        self.selected_index = 0;
    }

    pub fn delete_selected(&mut self) {
        if let Some(entry_name) = self
            .filtered_entries
            .get(self.selected_index)
            .map(|e| e.name.clone())
        {
            let path_to_remove = self.base_path.join(&entry_name);

            // Only use git worktree remove if it's actually a worktree (not main working tree)
            if utils::is_git_worktree(&path_to_remove) {
                match utils::remove_git_worktree(&path_to_remove) {
                    Ok(output) => {
                        if output.status.success() {
                            self.all_entries.retain(|e| e.name != entry_name);
                            self.update_search();
                            self.status_message =
                                Some(format!("Worktree removed: {path_to_remove:?}"));
                        } else {
                            self.status_message = Some(format!(
                                "Error deleting: {}",
                                String::from_utf8_lossy(&output.stderr)
                                    .lines()
                                    .take(1)
                                    .collect::<String>()
                            ));
                        }
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error removing worktree: {}", e));
                    }
                };
            } else {
                // Regular directory or main git repo - just delete it
                match fs::remove_dir_all(&path_to_remove) {
                    Ok(_) => {
                        self.all_entries.retain(|e| e.name != entry_name);
                        self.update_search();
                        self.status_message =
                            Some(format!("Deleted: {}", path_to_remove.display()));
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error deleting: {}", e));
                    }
                }
            };
        }
        self.mode = AppMode::Normal;
    }

    pub fn rename_selected(&mut self) {
        let new_name = self.rename_input.trim().to_string();
        if new_name.is_empty() {
            self.status_message = Some("Rename cancelled: name is empty".to_string());
            self.mode = AppMode::Normal;
            return;
        }

        let Some(entry) = self.filtered_entries.get(self.selected_index) else {
            self.mode = AppMode::Normal;
            return;
        };
        let old_name = entry.name.clone();
        if new_name == old_name {
            self.mode = AppMode::Normal;
            return;
        }

        let old_path = self.base_path.join(&old_name);
        let new_path = self.base_path.join(&new_name);

        if new_path.exists() {
            self.status_message = Some(format!("Error: '{}' already exists", new_name));
            self.mode = AppMode::Normal;
            return;
        }

        if let Err(e) = fs::rename(&old_path, &new_path) {
            self.status_message = Some(format!("Error renaming: {}", e));
            self.mode = AppMode::Normal;
            return;
        }

        for e in &mut self.all_entries {
            if e.name != old_name {
                continue;
            }
            e.name = new_name.clone();
            let display_name =
                if let Some((_date, remainder)) = utils::extract_prefix_date(&new_name) {
                    remainder
                } else {
                    new_name.clone()
                };
            e.display_offset = new_name
                .chars()
                .count()
                .saturating_sub(display_name.chars().count());
            e.display_name = display_name;
            break;
        }
        self.update_search();
        self.status_message = Some(format!("Renamed '{}' → '{}'", old_name, new_name));
        self.mode = AppMode::Normal;
    }
}

fn draw_popup(f: &mut Frame, title: &str, message: &str, theme: &Theme) {
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
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(popup_layout[1])[1];

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(theme.popup_bg));

    // Vertically center the text inside the popup
    let inner_height = popup_area.height.saturating_sub(2) as usize; // subtract borders
    let text_lines = message.lines().count();
    let top_padding = inner_height.saturating_sub(text_lines) / 2;
    let padded_message = format!("{}{}", "\n".repeat(top_padding), message);

    let paragraph = Paragraph::new(padded_message)
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
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(popup_layout[1])[1];

    f.render_widget(Clear, popup_area);

    // Split popup into theme list and transparency option
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(popup_area);

    let block = Block::default()
        .title(" Select Theme ")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
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
                .fg(app.theme.list_selected_fg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, inner_layout[0], &mut app.theme_list_state);

    // Draw transparency checkbox
    let checkbox = if app.transparent_background {
        "[x]"
    } else {
        "[ ]"
    };
    let transparency_text = format!(" {} Transparent Background (Space to toggle)", checkbox);
    let transparency_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(app.theme.popup_bg));
    let transparency_paragraph = Paragraph::new(transparency_text)
        .style(Style::default().fg(app.theme.list_highlight_fg))
        .block(transparency_block);
    f.render_widget(transparency_paragraph, inner_layout[1]);
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
        .padding(Padding::horizontal(1))
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
                .fg(app.theme.list_selected_fg)
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
        .padding(Padding::horizontal(1))
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
            Style::default().fg(theme.search_title),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "github.com/tassiovirginio/try-rs",
            Style::default().fg(theme.search_title),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("󰈙 License: ", Style::default().fg(theme.helpers_colors)),
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
            Style::default().fg(theme.helpers_colors),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, popup_area);
}

fn build_highlighted_name_spans(
    text: &str,
    match_indices: &[usize],
    highlight_style: Style,
) -> Vec<Span<'static>> {
    if text.is_empty() {
        return Vec::new();
    }

    if match_indices.is_empty() {
        return vec![Span::raw(text.to_string())];
    }

    let chars = text.chars().collect::<Vec<_>>();
    let mut spans = Vec::new();
    let mut cursor = 0usize;
    let mut idx = 0usize;

    while idx < match_indices.len() {
        let start = match_indices[idx];
        if start >= chars.len() {
            break;
        }

        if cursor < start {
            spans.push(Span::raw(chars[cursor..start].iter().collect::<String>()));
        }

        let mut end = start + 1;
        idx += 1;
        while idx < match_indices.len() && match_indices[idx] == end {
            end += 1;
            idx += 1;
        }

        let end = end.min(chars.len());

        spans.push(Span::styled(
            chars[start..end].iter().collect::<String>(),
            highlight_style,
        ));
        cursor = end;
    }

    if cursor < chars.len() {
        spans.push(Span::raw(chars[cursor..].iter().collect::<String>()));
    }

    spans
}

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    mut app: App,
) -> Result<(SelectionResult, bool, usize)> {
    while !app.should_quit {
        terminal.draw(|f| {
            // Render background if not transparent
            if !app.transparent_background {
                if let Some(bg_color) = app.theme.background {
                    let background = Block::default().style(Style::default().bg(bg_color));
                    f.render_widget(background, f.area());
                }
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(f.area());

            let show_disk_panel = app.show_disk;
            let show_preview_panel = app.show_preview;
            let show_legend_panel = app.show_legend;
            let has_right_panel_content =
                show_disk_panel || show_preview_panel || show_legend_panel;
            let show_right_panel = app.right_panel_visible && has_right_panel_content;

            let right_panel_width = app.right_panel_width.clamp(20, 80);
            let content_constraints = if !show_right_panel {
                [Constraint::Percentage(100), Constraint::Percentage(0)]
            } else {
                [
                    Constraint::Percentage(100 - right_panel_width),
                    Constraint::Percentage(right_panel_width),
                ]
            };

            let show_tabs = app.tries_dirs.len() > 1;
            let tab_height = 1;
            let content_with_tabs = if show_tabs {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(1),
                        Constraint::Length(tab_height),
                    ])
                    .split(chunks[0])
            } else {
                Rc::new([chunks[0], chunks[0]])
            };

            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(content_constraints)
                .split(if show_tabs { content_with_tabs[0] } else { chunks[0] });

            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .split(content_chunks[0]);

            if show_tabs {
                let tab_names: Vec<Span> = app
                    .tries_dirs
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let name = p.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| p.to_string_lossy().to_string());
                        if i == app.active_tab {
                            Span::styled(
                                format!("[{}]", name),
                                Style::default()
                                    .fg(app.theme.list_highlight_fg)
                                    .add_modifier(Modifier::BOLD),
                            )
                        } else {
                            Span::raw(format!(" {}", name))
                        }
                    })
                    .collect();
                
                let tab_line = Paragraph::new(Line::from(tab_names))
                    .style(Style::default().fg(app.theme.helpers_colors))
                    .alignment(Alignment::Left);
                f.render_widget(tab_line, content_with_tabs[1]);
            }

            let search_text = Paragraph::new(app.query.clone())
                .style(Style::default().fg(app.theme.search_title))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .padding(Padding::horizontal(1))
                        .title(Span::styled(
                            " Search/New ",
                            Style::default().fg(app.theme.search_title),
                        ))
                        .border_style(Style::default().fg(app.theme.search_border)),
                );
            f.render_widget(search_text, left_chunks[0]);

            let matched_char_style = Style::default()
                .fg(app.theme.list_match_fg)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

            let now = SystemTime::now();

            let mut items: Vec<ListItem> = app
                .filtered_entries
                .iter()
                .map(|entry| {
                    let elapsed = now
                        .duration_since(entry.modified)
                        .unwrap_or(std::time::Duration::ZERO);
                    let secs = elapsed.as_secs();
                    let days = secs / 86400;
                    let hours = (secs % 86400) / 3600;
                    let minutes = (secs % 3600) / 60;
                    let date_str = format!("({:02}d {:02}h {:02}m)", days, hours, minutes);

                    let width = left_chunks[1].width.saturating_sub(7) as usize;

                    let date_width = date_str.chars().count();

                    // Build icon list: (flag, icon_str, color)
                    let icons: &[(bool, &str, Color)] = &[
                        (entry.is_cargo, " ", app.theme.icon_rust),
                        (entry.is_maven, " ", app.theme.icon_maven),
                        (entry.is_flutter, " ", app.theme.icon_flutter),
                        (entry.is_go, " ", app.theme.icon_go),
                        (entry.is_python, " ", app.theme.icon_python),
                        (entry.is_mise, "󰬔 ", app.theme.icon_mise),
                        (entry.is_worktree, "󰙅 ", app.theme.icon_worktree),
                        (entry.is_worktree_locked, " ", app.theme.icon_worktree_lock),
                        (entry.is_gitmodules, " ", app.theme.icon_gitmodules),
                        (entry.is_git, " ", app.theme.icon_git),
                    ];
                    let icons_width: usize = icons.iter().filter(|(f, _, _)| *f).count() * 2;
                    let icon_width = 3; // folder icon

                    let created_dt: chrono::DateTime<Local> = entry.created.into();
                    let created_text = created_dt.format("%Y-%m-%d").to_string();
                    let created_width = created_text.chars().count();

                    let reserved = date_width + icons_width + icon_width + created_width + 2;
                    let available_for_name = width.saturating_sub(reserved);
                    let name_len = entry.display_name.chars().count();

                    let (display_name, display_match_indices, is_truncated, padding) = if name_len
                        > available_for_name
                    {
                        let safe_len = available_for_name.saturating_sub(3);
                        let truncated: String = entry.display_name.chars().take(safe_len).collect();
                        (
                            truncated,
                            entry
                                .match_indices
                                .iter()
                                .copied()
                                .filter(|idx| *idx < safe_len)
                                .collect::<Vec<_>>(),
                            true,
                            1,
                        )
                    } else {
                        (
                            entry.display_name.clone(),
                            entry.match_indices.clone(),
                            false,
                            width.saturating_sub(
                                icon_width
                                    + created_width
                                    + 1
                                    + name_len
                                    + date_width
                                    + icons_width,
                            ),
                        )
                    };

                    let is_current = app.current_entries.contains(&entry.name);
                    let marker = if is_current { "* " } else { "  " };
                    let marker_style = if is_current {
                        Style::default()
                            .fg(app.theme.list_match_fg)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    let mut spans = vec![
                        Span::styled(marker, marker_style),
                        Span::styled("󰝰 ", Style::default().fg(app.theme.icon_folder)),
                        Span::styled(created_text, Style::default().fg(app.theme.list_date)),
                        Span::raw(" "),
                    ];
                    spans.extend(build_highlighted_name_spans(
                        &display_name,
                        &display_match_indices,
                        matched_char_style,
                    ));
                    if is_truncated {
                        spans.push(Span::raw("..."));
                    }
                    spans.push(Span::raw(" ".repeat(padding)));
                    for &(flag, icon, color) in icons {
                        if flag {
                            spans.push(Span::styled(icon, Style::default().fg(color)));
                        }
                    }
                    spans.push(Span::styled(
                        date_str,
                        Style::default().fg(app.theme.list_date),
                    ));

                    ListItem::new(Line::from(spans))
                        .style(Style::default().fg(app.theme.list_highlight_fg))
                })
                .collect();

            // Append "new" option when no exact match
            if app.show_new_option {
                let new_item = ListItem::new(Line::from(vec![
                    Span::styled("  ", Style::default().fg(app.theme.search_title)),
                    Span::styled(
                        format!("Create new: {}", app.query),
                        Style::default()
                            .fg(app.theme.search_title)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ]));
                items.push(new_item);
            }

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .padding(Padding::horizontal(1))
                        .title(Span::styled(
                            " Folders ",
                            Style::default().fg(app.theme.folder_title),
                        ))
                        .border_style(Style::default().fg(app.theme.folder_border)),
                )
                .highlight_style(
                    Style::default()
                        .bg(app.theme.list_highlight_bg)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("→ ");

            let mut state = ListState::default();
            state.select(Some(app.selected_index));
            f.render_stateful_widget(list, left_chunks[1], &mut state);

            if show_right_panel {
                let free_space = app
                    .cached_free_space_mb
                    .map(|s| {
                        if s >= 1000 {
                            format!("{:.1} GB", s as f64 / 1024.0)
                        } else {
                            format!("{} MB", s)
                        }
                    })
                    .unwrap_or_else(|| "N/A".to_string());

                let folder_size = app.folder_size_mb.load(Ordering::Relaxed);
                let folder_size_str = if folder_size == 0 {
                    "---".to_string()
                } else if folder_size >= 1000 {
                    format!("{:.1} GB", folder_size as f64 / 1024.0)
                } else {
                    format!("{} MB", folder_size)
                };

                let legend_items: [(&str, Color, &str); 10] = [
                    ("", app.theme.icon_rust, "Rust"),
                    ("", app.theme.icon_maven, "Maven"),
                    ("", app.theme.icon_flutter, "Flutter"),
                    ("", app.theme.icon_go, "Go"),
                    ("", app.theme.icon_python, "Python"),
                    ("󰬔", app.theme.icon_mise, "Mise"),
                    ("", app.theme.icon_worktree_lock, "Locked"),
                    ("󰙅", app.theme.icon_worktree, "Worktree"),
                    ("", app.theme.icon_gitmodules, "Submodule"),
                    ("", app.theme.icon_git, "Git"),
                ];

                let legend_required_lines = if show_legend_panel {
                    let legend_inner_width = content_chunks[1].width.saturating_sub(4).max(1);
                    let mut lines: u16 = 1;
                    let mut used: u16 = 0;

                    for (idx, (icon, _, label)) in legend_items.iter().enumerate() {
                        let item_width = (icon.chars().count() + 1 + label.chars().count()) as u16;
                        let separator_width = if idx == 0 { 0 } else { 2 };

                        if used > 0 && used + separator_width + item_width > legend_inner_width {
                            lines += 1;
                            used = item_width;
                        } else {
                            used += separator_width + item_width;
                        }
                    }

                    lines
                } else {
                    0
                };

                let legend_height = legend_required_lines.saturating_add(2).max(3);

                let right_constraints = if show_disk_panel {
                    if show_preview_panel && show_legend_panel {
                        [
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(legend_height),
                        ]
                    } else if show_preview_panel {
                        [
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(0),
                        ]
                    } else if show_legend_panel {
                        [
                            Constraint::Length(3),
                            Constraint::Length(0),
                            Constraint::Min(1),
                        ]
                    } else {
                        [
                            Constraint::Length(3),
                            Constraint::Length(0),
                            Constraint::Length(0),
                        ]
                    }
                } else if show_preview_panel && show_legend_panel {
                    [
                        Constraint::Length(0),
                        Constraint::Min(1),
                        Constraint::Length(legend_height),
                    ]
                } else if show_preview_panel {
                    [
                        Constraint::Length(0),
                        Constraint::Min(1),
                        Constraint::Length(0),
                    ]
                } else {
                    [
                        Constraint::Length(0),
                        Constraint::Length(0),
                        Constraint::Min(1),
                    ]
                };
                let right_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(right_constraints)
                    .split(content_chunks[1]);

                if show_disk_panel {
                    let memory_info = Paragraph::new(Line::from(vec![
                        Span::styled("󰋊 ", Style::default().fg(app.theme.title_rs)),
                        Span::styled("Used: ", Style::default().fg(app.theme.helpers_colors)),
                        Span::styled(
                            folder_size_str,
                            Style::default().fg(app.theme.status_message),
                        ),
                        Span::styled(" | ", Style::default().fg(app.theme.helpers_colors)),
                        Span::styled("Free: ", Style::default().fg(app.theme.helpers_colors)),
                        Span::styled(free_space, Style::default().fg(app.theme.status_message)),
                    ]))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .padding(Padding::horizontal(1))
                            .title(Span::styled(
                                " Disk ",
                                Style::default().fg(app.theme.disk_title),
                            ))
                            .border_style(Style::default().fg(app.theme.disk_border)),
                    )
                    .alignment(Alignment::Center);
                    f.render_widget(memory_info, right_chunks[0]);
                }

                if show_preview_panel {
                    // Check if "new" option is currently selected
                    let is_new_selected =
                        app.show_new_option && app.selected_index == app.filtered_entries.len();

                    if is_new_selected {
                        // Show "new folder" preview
                        let preview_lines = vec![Line::from(Span::styled(
                            "(new folder)",
                            Style::default()
                                .fg(app.theme.search_title)
                                .add_modifier(Modifier::ITALIC),
                        ))];
                        let preview = Paragraph::new(preview_lines).block(
                            Block::default()
                                .borders(Borders::ALL)
                                .padding(Padding::horizontal(1))
                                .title(Span::styled(
                                    " Preview ",
                                    Style::default().fg(app.theme.preview_title),
                                ))
                                .border_style(Style::default().fg(app.theme.preview_border)),
                        );
                        f.render_widget(preview, right_chunks[1]);
                    } else if let Some(selected) = app.filtered_entries.get(app.selected_index) {
                        let preview_path = app.base_path.join(&selected.name);
                        let mut preview_lines = Vec::new();

                        if let Ok(entries) = fs::read_dir(&preview_path) {
                            for e in entries
                                .take(right_chunks[1].height.saturating_sub(2) as usize)
                                .flatten()
                            {
                                let file_name = e.file_name().to_string_lossy().to_string();
                                let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                                let (icon, color) = if is_dir {
                                    ("󰝰 ", app.theme.icon_folder)
                                } else {
                                    ("󰈙 ", app.theme.icon_file)
                                };
                                preview_lines.push(Line::from(vec![
                                    Span::styled(icon, Style::default().fg(color)),
                                    Span::raw(file_name),
                                ]));
                            }
                        }

                        if preview_lines.is_empty() {
                            preview_lines.push(Line::from(Span::styled(
                                " (empty) ",
                                Style::default().fg(app.theme.helpers_colors),
                            )));
                        }

                        let preview = Paragraph::new(preview_lines).block(
                            Block::default()
                                .borders(Borders::ALL)
                                .padding(Padding::horizontal(1))
                                .title(Span::styled(
                                    " Preview ",
                                    Style::default().fg(app.theme.preview_title),
                                ))
                                .border_style(Style::default().fg(app.theme.preview_border)),
                        );
                        f.render_widget(preview, right_chunks[1]);
                    } else {
                        let preview = Block::default()
                            .borders(Borders::ALL)
                            .padding(Padding::horizontal(1))
                            .title(Span::styled(
                                " Preview ",
                                Style::default().fg(app.theme.preview_title),
                            ))
                            .border_style(Style::default().fg(app.theme.preview_border));
                        f.render_widget(preview, right_chunks[1]);
                    }
                }

                if show_legend_panel {
                    // Icon legend
                    let mut legend_spans = Vec::with_capacity(legend_items.len() * 4);
                    for (idx, (icon, color, label)) in legend_items.iter().enumerate() {
                        if idx > 0 {
                            legend_spans.push(Span::raw("  "));
                        }
                        legend_spans.push(Span::styled(*icon, Style::default().fg(*color)));
                        legend_spans.push(Span::styled(
                            "\u{00A0}",
                            Style::default().fg(app.theme.helpers_colors),
                        ));
                        legend_spans.push(Span::styled(
                            *label,
                            Style::default().fg(app.theme.helpers_colors),
                        ));
                    }
                    let legend_lines = vec![Line::from(legend_spans)];

                    let legend = Paragraph::new(legend_lines)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .padding(Padding::horizontal(1))
                                .title(Span::styled(
                                    " Legends ",
                                    Style::default().fg(app.theme.legends_title),
                                ))
                                .border_style(Style::default().fg(app.theme.legends_border)),
                        )
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true });
                    f.render_widget(legend, right_chunks[2]);
                }
            }

            let help_text = if let Some(msg) = &app.status_message {
                Line::from(vec![Span::styled(
                    msg,
                    Style::default()
                        .fg(app.theme.status_message)
                        .add_modifier(Modifier::BOLD),
                )])
            } else {
                let mut help_parts = vec![
                    Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Nav | "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Select | "),
                    Span::styled("Ctrl-D", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Del | "),
                    Span::styled("Ctrl-R", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Rename | "),
                    Span::styled("Ctrl-E", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Edit | "),
                    Span::styled("Ctrl-T", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Theme | "),
                ];

                if app.tries_dirs.len() > 1 {
                    help_parts.extend(vec![
                        Span::styled("←→", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" Tab | "),
                    ]);
                }

                help_parts.extend(vec![
                    Span::styled("Ctrl+A", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" About | "),
                    Span::styled("Alt-P", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Panel | "),
                    Span::styled("Esc/Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" Quit"),
                ]);

                Line::from(help_parts)
            };

            let help_message = Paragraph::new(help_text)
                .style(Style::default().fg(app.theme.helpers_colors))
                .alignment(Alignment::Center);

            f.render_widget(help_message, chunks[1]);

            if app.mode == AppMode::DeleteConfirm
                && let Some(selected) = app.filtered_entries.get(app.selected_index)
            {
                let msg = format!("Delete '{}'?\n(y/n)", selected.name);
                draw_popup(f, " WARNING ", &msg, &app.theme);
            }

            if app.mode == AppMode::RenamePrompt {
                let msg = format!("{}_", app.rename_input);
                draw_popup(f, " Rename ", &msg, &app.theme);
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

        // Poll with 1-second timeout so the screen refreshes periodically
        if !event::poll(std::time::Duration::from_secs(1))? {
            continue;
        }
        if let Event::Key(key) = event::read()? {
            if !key.is_press() {
                continue;
            }
            // Clear status message on any key press so it disappears after one redraw
            app.status_message = None;
            match app.mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char(c) => {
                        if c == 'c' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            app.should_quit = true;
                        } else if c == 'd' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            let is_new_selected = app.show_new_option
                                && app.selected_index == app.filtered_entries.len();
                            if !app.filtered_entries.is_empty() && !is_new_selected {
                                app.mode = AppMode::DeleteConfirm;
                            }
                        } else if c == 'r' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            let is_new_selected = app.show_new_option
                                && app.selected_index == app.filtered_entries.len();
                            if !app.filtered_entries.is_empty() && !is_new_selected {
                                app.rename_input =
                                    app.filtered_entries[app.selected_index].name.clone();
                                app.mode = AppMode::RenamePrompt;
                            }
                        } else if c == 'e' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            if app.editor_cmd.is_some() {
                                let is_new_selected = app.show_new_option
                                    && app.selected_index == app.filtered_entries.len();
                                if is_new_selected {
                                    app.final_selection = SelectionResult::New(app.query.clone());
                                    app.wants_editor = true;
                                    app.should_quit = true;
                                } else if !app.filtered_entries.is_empty() {
                                    app.final_selection = SelectionResult::Folder(
                                        app.filtered_entries[app.selected_index].name.clone(),
                                    );
                                    app.wants_editor = true;
                                    app.should_quit = true;
                                } else if !app.query.is_empty() {
                                    app.final_selection = SelectionResult::New(app.query.clone());
                                    app.wants_editor = true;
                                    app.should_quit = true;
                                }
                            } else {
                                app.status_message =
                                    Some("No editor configured in config.toml".to_string());
                            }
                        } else if c == 't' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            // Save current theme and transparency before opening selector
                            app.original_theme = Some(app.theme.clone());
                            app.original_transparent_background = Some(app.transparent_background);
                            // Find and select current theme in the list
                            let current_idx = app
                                .available_themes
                                .iter()
                                .position(|t| t.name == app.theme.name)
                                .unwrap_or(0);
                            app.theme_list_state.select(Some(current_idx));
                            app.mode = AppMode::ThemeSelect;
                        } else if c == 'a' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            app.mode = AppMode::About;
                        } else if matches!(c, 'p')
                            && key.modifiers.contains(event::KeyModifiers::ALT)
                        {
                            app.right_panel_visible = !app.right_panel_visible;
                        } else if matches!(c, 'k' | 'p')
                            && key.modifiers.contains(event::KeyModifiers::CONTROL)
                        {
                            if app.selected_index > 0 {
                                app.selected_index -= 1;
                            }
                        } else if matches!(c, 'j' | 'n')
                            && key.modifiers.contains(event::KeyModifiers::CONTROL)
                        {
                            let max_index = if app.show_new_option {
                                app.filtered_entries.len()
                            } else {
                                app.filtered_entries.len().saturating_sub(1)
                            };
                            if app.selected_index < max_index {
                                app.selected_index += 1;
                            }
                        } else if c == 'u' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                            app.query.clear();
                            app.update_search();
                        } else if key.modifiers.is_empty()
                            || key.modifiers == event::KeyModifiers::SHIFT
                        {
                            app.query.push(c);
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
                        let max_index = if app.show_new_option {
                            app.filtered_entries.len()
                        } else {
                            app.filtered_entries.len().saturating_sub(1)
                        };
                        if app.selected_index < max_index {
                            app.selected_index += 1;
                        }
                    }
                    KeyCode::Left => {
                        if app.tries_dirs.len() > 1 {
                            let prev = if app.active_tab == 0 {
                                app.tries_dirs.len() - 1
                            } else {
                                app.active_tab - 1
                            };
                            app.switch_tab(prev);
                        }
                    }
                    KeyCode::Right => {
                        if app.tries_dirs.len() > 1 {
                            let next = (app.active_tab + 1) % app.tries_dirs.len();
                            app.switch_tab(next);
                        }
                    }
                    KeyCode::Enter => {
                        let is_new_selected =
                            app.show_new_option && app.selected_index == app.filtered_entries.len();
                        if is_new_selected {
                            app.final_selection = SelectionResult::New(app.query.clone());
                        } else if !app.filtered_entries.is_empty() {
                            app.final_selection = SelectionResult::Folder(
                                app.filtered_entries[app.selected_index].name.clone(),
                            );
                        } else if !app.query.is_empty() {
                            app.final_selection = SelectionResult::New(app.query.clone());
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

                AppMode::RenamePrompt => match key.code {
                    KeyCode::Enter => {
                        app.rename_selected();
                    }
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Backspace => {
                        app.rename_input.pop();
                    }
                    KeyCode::Char(c) => {
                        app.rename_input.push(c);
                    }
                    _ => {}
                },

                AppMode::ThemeSelect => match key.code {
                    KeyCode::Char(' ') => {
                        // Toggle transparent background
                        app.transparent_background = !app.transparent_background;
                    }
                    KeyCode::Esc => {
                        // Restore original theme and transparency
                        if let Some(original) = app.original_theme.take() {
                            app.theme = original;
                        }
                        if let Some(original_transparent) =
                            app.original_transparent_background.take()
                        {
                            app.transparent_background = original_transparent;
                        }
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        // Restore original theme and transparency
                        if let Some(original) = app.original_theme.take() {
                            app.theme = original;
                        }
                        if let Some(original_transparent) =
                            app.original_transparent_background.take()
                        {
                            app.transparent_background = original_transparent;
                        }
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
                        // Apply theme preview
                        if let Some(theme) = app.available_themes.get(i) {
                            app.theme = theme.clone();
                        }
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
                        // Apply theme preview
                        if let Some(theme) = app.available_themes.get(i) {
                            app.theme = theme.clone();
                        }
                    }
                    KeyCode::Enter => {
                        // Clear original theme and transparency (we're confirming the new values)
                        app.original_theme = None;
                        app.original_transparent_background = None;
                        if let Some(i) = app.theme_list_state.selected() {
                            if let Some(theme) = app.available_themes.get(i) {
                                app.theme = theme.clone();

                                if let Some(ref path) = app.config_path {
                                    if let Err(e) = save_config(
                                        path,
                                        &app.theme,
                                        &app.tries_dirs,
                                        &app.editor_cmd,
                                        app.apply_date_prefix,
                                        Some(app.transparent_background),
                                        Some(app.show_disk),
                                        Some(app.show_preview),
                                        Some(app.show_legend),
                                        Some(app.right_panel_visible),
                                        Some(app.right_panel_width),
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

                            if let Err(e) = save_config(
                                &path,
                                &app.theme,
                                &app.tries_dirs,
                                &app.editor_cmd,
                                app.apply_date_prefix,
                                Some(app.transparent_background),
                                Some(app.show_disk),
                                Some(app.show_preview),
                                Some(app.show_legend),
                                Some(app.right_panel_visible),
                                Some(app.right_panel_width),
                            ) {
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

    Ok((app.final_selection, app.wants_editor, app.active_tab))
}

#[cfg(test)]
mod tests {
    use super::App;
    use std::{fs, path::PathBuf};
    use tempdir::TempDir;

    #[test]
    fn current_entry_detects_nested_path() {
        let temp = TempDir::new("current-entry-nested").unwrap();
        let base_path = temp.path().to_path_buf();
        let entry_name = "2025-11-20-gamma";
        let entry_path = base_path.join(entry_name);
        let nested_path = entry_path.join("nested/deeper");

        fs::create_dir_all(&nested_path).unwrap();

        assert!(App::is_current_entry(
            &entry_path,
            entry_name,
            false,
            &nested_path,
            &nested_path,
            &base_path,
        ));
    }

    #[test]
    fn current_entry_detects_nested_path_with_stale_pwd() {
        let temp = TempDir::new("current-entry-script").unwrap();
        let base_path = temp.path().to_path_buf();
        let entry_name = "2025-11-20-gamma";
        let entry_path = base_path.join(entry_name);
        let nested_path = entry_path.join("nested/deeper");

        fs::create_dir_all(&nested_path).unwrap();

        let stale_pwd = PathBuf::from("/tmp/not-the-real-cwd");

        assert!(App::is_current_entry(
            &entry_path,
            entry_name,
            false,
            &stale_pwd,
            &nested_path,
            &base_path,
        ));
    }
}
