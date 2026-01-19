use crate::tui::Theme;
use crate::utils::expand_path;
use ratatui::style::Color;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub tries_path: Option<String>,
    pub colors: Option<ThemeConfig>,
    pub editor: Option<String>,
}

pub fn get_file_config_toml_name() -> String {
    std::env::var("TRY_CONFIG").unwrap_or("config.toml".to_string())
}

pub fn get_config_dir() -> PathBuf {
    std::env::var_os("TRY_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| get_base_config_dir().join("try-rs"))
}

pub fn get_base_config_dir() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    })
}

pub fn load_file_config_toml_if_exists() -> Option<Config> {
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

pub fn load_configuration() -> (PathBuf, Theme, Option<String>, bool, Option<PathBuf>) {
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

    let loaded_config_path = if let Some(path) = std::env::var_os("TRY_CONFIG_DIR")
        .map(|p| PathBuf::from(p).join(get_file_config_toml_name()))
        .filter(|p| p.exists())
    {
        Some(path)
    } else if let Some(path) = dirs::config_dir()
        .map(|p| p.join("try-rs").join(get_file_config_toml_name()))
        .filter(|p| p.exists())
    {
        Some(path)
    } else {
        dirs::home_dir()
            .map(|p| {
                p.join(".config")
                    .join("try-rs")
                    .join(get_file_config_toml_name())
            })
            .filter(|p| p.exists())
    };

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
            let parse = |opt: Option<String>, def: Color| -> Color {
                opt.and_then(|s| Color::from_str(&s).ok()).unwrap_or(def)
            };

            let def = Theme::default();
            theme = Theme {
                name: "Custom".to_string(),
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
        is_first_run = true;
    }

    (
        final_path,
        theme,
        editor_cmd,
        is_first_run,
        loaded_config_path,
    )
}

fn color_to_string(c: Color) -> String {
    match c {
        Color::Reset => "Reset".to_string(),
        Color::Black => "Black".to_string(),
        Color::Red => "Red".to_string(),
        Color::Green => "Green".to_string(),
        Color::Yellow => "Yellow".to_string(),
        Color::Blue => "Blue".to_string(),
        Color::Magenta => "Magenta".to_string(),
        Color::Cyan => "Cyan".to_string(),
        Color::Gray => "Gray".to_string(),
        Color::DarkGray => "DarkGray".to_string(),
        Color::LightRed => "LightRed".to_string(),
        Color::LightGreen => "LightGreen".to_string(),
        Color::LightYellow => "LightYellow".to_string(),
        Color::LightBlue => "LightBlue".to_string(),
        Color::LightMagenta => "LightMagenta".to_string(),
        Color::LightCyan => "LightCyan".to_string(),
        Color::White => "White".to_string(),
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        Color::Indexed(i) => format!("{i}"),
    }
}

pub fn save_config(
    path: &Path,
    theme: &Theme,
    tries_path: &Path,
    editor: &Option<String>,
) -> std::io::Result<()> {
    let theme_config = ThemeConfig {
        title_try: Some(color_to_string(theme.title_try)),
        title_rs: Some(color_to_string(theme.title_rs)),
        search_box: Some(color_to_string(theme.search_box)),
        list_date: Some(color_to_string(theme.list_date)),
        list_highlight_bg: Some(color_to_string(theme.list_highlight_bg)),
        list_highlight_fg: Some(color_to_string(theme.list_highlight_fg)),
        help_text: Some(color_to_string(theme.help_text)),
        status_message: Some(color_to_string(theme.status_message)),
        popup_bg: Some(color_to_string(theme.popup_bg)),
        popup_text: Some(color_to_string(theme.popup_text)),
    };

    let config = Config {
        tries_path: Some(tries_path.to_string_lossy().to_string()), // Persist current path
        colors: Some(theme_config),
        editor: editor.clone(),
    };

    let toml_string =
        toml::to_string(&config).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = fs::File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
