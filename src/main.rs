use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{prelude::*, widgets::*};
use serde::Deserialize;
use std::process::Stdio;
use std::{fs, io, path::PathBuf, time::SystemTime};

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
    score: i64,
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
    status_message: Option<String>,  // Feedback message for the user
}

impl App {
    fn new(path: PathBuf) -> Self {
        let mut entries = Vec::new();
        if let Ok(read_dir) = fs::read_dir(path) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        entries.push(TryEntry {
                            name: entry.file_name().to_string_lossy().to_string(),
                            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                            score: 0,
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
    fn delete_selected(&mut self, base_path: &std::path::Path) {
    if let Some(entry_name) = self
        .filtered_entries
        .get(self.selected_index)
        .map(|e| e.name.clone())
    {
        let path_to_remove = base_path.join(&entry_name);

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

fn draw_popup(f: &mut Frame, title: &str, message: &str) {
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
        .style(Style::default().bg(Color::DarkGray));

    let paragraph = Paragraph::new(message)
        .block(block)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, popup_area);
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    mut app: App,
) -> Result<Option<String>> {

    let tries_dir = get_configuration_path();

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

            let title = Paragraph::new(Line::from(vec![
                Span::styled("TRY-RS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]))
            .alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            let search_text = Paragraph::new(app.query.clone())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title(" Search/New "));
            f.render_widget(search_text, chunks[1]);

            let items: Vec<ListItem> = app
                .filtered_entries
                .iter()
                .map(|entry| {
                    let date: DateTime<Local> = entry.modified.into();
                    let date_str = date.format("%Y-%m-%d %H:%M");
                    let content = Line::from(vec![
                        Span::raw(format!("📁{:<30}", entry.name)),
                        Span::styled(
                            format!("({})", date_str),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]);
                    ListItem::new(content)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Folders "))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("→ ");

            let mut state = ListState::default();
            state.select(Some(app.selected_index));
            f.render_stateful_widget(list, chunks[2], &mut state);

            // --- Footer Widget (Help) ---
            // If there is a status message, show it instead of help, or alongside it.
            let help_text = if let Some(msg) = &app.status_message {
                 Line::from(vec![
                    Span::styled(msg, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                ])
            } else {
                Line::from(vec![
                    Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Navigate  "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Select  "),
                    Span::styled("Ctrl-D", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Delete  "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": Cancel"),
                ])
            };

            let help_message = Paragraph::new(help_text)
                .style(Style::default().fg(Color::DarkGray)) 
                .alignment(Alignment::Center);

            f.render_widget(help_message, chunks[3]);

            // --- DRAWING THE POPUP (If in DeleteConfirm mode) ---
            if app.mode == AppMode::DeleteConfirm {
                if let Some(selected) = app.filtered_entries.get(app.selected_index) {
                    let msg = format!("Delete '{}'? (y/n)", selected.name);
                    draw_popup(f, " WARNING ", &msg);
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
                            // Ctrl+D to delete
                            if c == 'd' && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                                // Only enter delete mode if something is selected
                                if !app.filtered_entries.is_empty() {
                                    app.mode = AppMode::DeleteConfirm;
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
                            app.delete_selected(&tries_dir);
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                        }
                        _ => {} 
                    },
                }
            }
        }
    }

    Ok(app.final_selection)
}

// Representation of our TOML file
#[derive(Deserialize)]
struct Config {
    tries_path: Option<String>,
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

fn get_configuration_path() -> PathBuf {
    // 1. Try to find the default config directory (~/.config)
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        // Fallback if not found
        dirs::home_dir().expect("Folder not found").join(".config")
    });

    // 2. Build the path: ~/.config/try-rs/config.toml
    let app_config_dir = config_dir.join("try-rs");
    let config_file = app_config_dir.join("config.toml");

    // 3. Define the old default (fallback)
    let default_path = dirs::home_dir()
        .expect("Folder not found")
        .join("work/tries");

    // 4. If the file exists, try to read it
    if config_file.exists() {
        if let Ok(contents) = fs::read_to_string(&config_file) {
            if let Ok(config) = toml::from_str::<Config>(&contents) {
                if let Some(path_str) = config.tries_path {
                    return expand_path(&path_str);
                }
            }
        }
    } else {
        // Create default config if it doesn't exist
        if fs::create_dir_all(&app_config_dir).is_ok() {
            let default_content = "tries_path = \"~/work/tries\"";
            let _ = fs::write(&config_file, default_content);
        }
    }

    // If nothing works or there is no config, return the default
    default_path
}

fn setup_fish() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir().expect("Could not find home directory").join(".config")
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
    println!("Fish function created at: {}", file_path.display());
    println!("You may need to restart your shell or run 'source {}' to apply changes.", file_path.display());

    Ok(())
}

fn setup_zsh() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir().expect("Could not find home directory").join(".config")
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
    println!("ZSH function file created at: {}", file_path.display());
    println!("You need to source this file in your ~/.zshrc:");
    println!("source {}", file_path.display());

    Ok(())
}

fn main() -> Result<()> {
    let tries_dir = get_configuration_path();

    // Ensure the directory exists (either from config or default)
    if !tries_dir.exists() {
        fs::create_dir_all(&tries_dir)?;
    }

    // 2. Check command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 && args[1] == "--setup" {
        match args[2].as_str() {
            "fish" => return setup_fish(),
            "zsh" => return setup_zsh(),
            _ => {}
        }
    }

    // The 'selection' variable will hold the chosen name or URL.
    // It can come from arguments (CLI) or the interface (TUI).
    let selection_result: Option<String>;

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

        let app = App::new(tries_dir.clone());
        // Run the app and capture the result
        selection_result = run_app(&mut terminal, app)?;

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
            println!("cd '{}'", target_path.to_string_lossy());
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
                        println!("cd '{}'", new_path.to_string_lossy());
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
                println!("cd '{}'", new_path.to_string_lossy());
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
