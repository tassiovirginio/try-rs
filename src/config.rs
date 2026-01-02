use crate::tui::Theme;
use crate::utils::expand_path;
use ratatui::style::Color;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ThemeConfig {
    pub title_try: Option<String>,
    pub title_rs: Option<String>,
    pub search_box: Option<String>,
    pub list_date: Option<String>,
    pub list_highlight_bg: Option<String>,
    pub list_highlight_fg: Option<String>,
    pub help_text: Option<String>,
    pub status_message: Option<String>,
    pub popup_bg: Option<String>,
    pub popup_text: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub tries_path: Option<String>,
    pub colors: Option<ThemeConfig>,
    pub editor: Option<String>,
}

pub fn get_file_config_toml_name() -> String {
    std::env::var("TRY_CONFIG").unwrap_or("config.toml".to_string())
}

pub fn load_file_config_toml_if_exists() -> Option<Config> {
    // 1. Check TRY_CONFIG_DIR environment variable
    if let Some(env_dir) = std::env::var_os("TRY_CONFIG_DIR") {
        let config_path = PathBuf::from(env_dir).join(get_file_config_toml_name());
        if config_path.exists() {
            if let Ok(contents) = fs::read_to_string(&config_path)
                && let Ok(config) = toml::from_str::<Config>(&contents)
            {
                return Some(config);
            }
        }
    }

    // 2. Check XDG config dir (~/.config/try-rs/config.toml)
    let config_dir_config_toml = dirs::config_dir()
        .expect("Folder not found")
        .join("try-rs")
        .join(get_file_config_toml_name());

    if config_dir_config_toml.exists() {
        if let Ok(contents) = fs::read_to_string(&config_dir_config_toml)
            && let Ok(config) = toml::from_str::<Config>(&contents)
        {
            return Some(config);
        }
    }

    // 3. Check ~/.try-rs/config.toml (legacy/alternative)
    let home_dir_config_toml = dirs::home_dir()
        .expect("Folder not found")
        .join(".config")
        .join("try-rs")
        .join(get_file_config_toml_name());

    if home_dir_config_toml.exists() {
        if let Ok(contents) = fs::read_to_string(&home_dir_config_toml)
            && let Ok(config) = toml::from_str::<Config>(&contents)
        {
            return Some(config);
        }
    }

    None
}

pub fn load_configuration() -> (PathBuf, Theme, Option<String>, bool) {
    // Default Path: Work/tries
    let default_path = dirs::home_dir()
        .expect("Folder not found")
        .join("work")
        .join("tries");

    let mut theme = Theme::default();
    let try_path = std::env::var_os("TRY_PATH");
    let try_path_specified = try_path.is_some();
    let mut final_path = try_path.map(PathBuf::from).unwrap_or(default_path);
    let mut editor_cmd = std::env::var("VISUAL")
        .ok()
        .or_else(|| std::env::var("EDITOR").ok());
    let mut is_first_run = false;

    // Try to load any existing config
    if let Some(config) = load_file_config_toml_if_exists() {
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
    } else {
        // No config found. We should create the default one.
        // Calculate the default location to write to: ~/.config/try-rs/config.toml
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().expect("Folder not found").join(".config"));
        let app_config_dir = config_dir.join("try-rs");
        let config_file = app_config_dir.join("config.toml");

        if fs::create_dir_all(&app_config_dir).is_ok() {
            let default_content = format!("tries_path = {final_path:?}");
            // We only write if the file really doesn't exist (double check to be safe)
            if !config_file.exists() {
                let _ = fs::write(&config_file, default_content);
                is_first_run = true;
            }
        }
    }

    (final_path, theme, editor_cmd, is_first_run)
}
