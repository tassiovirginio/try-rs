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
    pub search_title: Option<String>,
    pub search_border: Option<String>,
    pub folder_title: Option<String>,
    pub folder_border: Option<String>,
    pub disk_title: Option<String>,
    pub disk_border: Option<String>,
    pub preview_title: Option<String>,
    pub preview_border: Option<String>,
    pub legends_title: Option<String>,
    pub legends_border: Option<String>,
    pub list_date: Option<String>,
    pub list_highlight_bg: Option<String>,
    pub list_highlight_fg: Option<String>,
    pub helpers_colors: Option<String>,
    pub status_message: Option<String>,
    pub popup_bg: Option<String>,
    pub popup_text: Option<String>,
    // Icon colors
    pub icon_rust: Option<String>,
    pub icon_maven: Option<String>,
    pub icon_flutter: Option<String>,
    pub icon_go: Option<String>,
    pub icon_python: Option<String>,
    pub icon_mise: Option<String>,
    pub icon_worktree: Option<String>,
    pub icon_worktree_lock: Option<String>,
    pub icon_gitmodules: Option<String>,
    pub icon_git: Option<String>,
    pub icon_folder: Option<String>,
    pub icon_file: Option<String>,
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
                search_title: parse(colors.search_title, def.search_title),
                search_border: parse(colors.search_border, def.search_border),
                folder_title: parse(colors.folder_title, def.folder_title),
                folder_border: parse(colors.folder_border, def.folder_border),
                disk_title: parse(colors.disk_title, def.disk_title),
                disk_border: parse(colors.disk_border, def.disk_border),
                preview_title: parse(colors.preview_title, def.preview_title),
                preview_border: parse(colors.preview_border, def.preview_border),
                legends_title: parse(colors.legends_title, def.legends_title),
                legends_border: parse(colors.legends_border, def.legends_border),
                list_date: parse(colors.list_date, def.list_date),
                list_highlight_bg: parse(colors.list_highlight_bg, def.list_highlight_bg),
                list_highlight_fg: parse(colors.list_highlight_fg, def.list_highlight_fg),
                helpers_colors: parse(colors.helpers_colors, def.helpers_colors),
                status_message: parse(colors.status_message, def.status_message),
                popup_bg: parse(colors.popup_bg, def.popup_bg),
                popup_text: parse(colors.popup_text, def.popup_text),
                icon_rust: parse(colors.icon_rust, def.icon_rust),
                icon_maven: parse(colors.icon_maven, def.icon_maven),
                icon_flutter: parse(colors.icon_flutter, def.icon_flutter),
                icon_go: parse(colors.icon_go, def.icon_go),
                icon_python: parse(colors.icon_python, def.icon_python),
                icon_mise: parse(colors.icon_mise, def.icon_mise),
                icon_worktree: parse(colors.icon_worktree, def.icon_worktree),
                icon_worktree_lock: parse(colors.icon_worktree_lock, def.icon_worktree_lock),
                icon_gitmodules: parse(colors.icon_gitmodules, def.icon_gitmodules),
                icon_git: parse(colors.icon_git, def.icon_git),
                icon_folder: parse(colors.icon_folder, def.icon_folder),
                icon_file: parse(colors.icon_file, def.icon_file),
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
        search_title: Some(color_to_string(theme.search_title)),
        search_border: Some(color_to_string(theme.search_border)),
        folder_title: Some(color_to_string(theme.folder_title)),
        folder_border: Some(color_to_string(theme.folder_border)),
        disk_title: Some(color_to_string(theme.disk_title)),
        disk_border: Some(color_to_string(theme.disk_border)),
        preview_title: Some(color_to_string(theme.preview_title)),
        preview_border: Some(color_to_string(theme.preview_border)),
        legends_title: Some(color_to_string(theme.legends_title)),
        legends_border: Some(color_to_string(theme.legends_border)),
        list_date: Some(color_to_string(theme.list_date)),
        list_highlight_bg: Some(color_to_string(theme.list_highlight_bg)),
        list_highlight_fg: Some(color_to_string(theme.list_highlight_fg)),
        helpers_colors: Some(color_to_string(theme.helpers_colors)),
        status_message: Some(color_to_string(theme.status_message)),
        popup_bg: Some(color_to_string(theme.popup_bg)),
        popup_text: Some(color_to_string(theme.popup_text)),
        icon_rust: Some(color_to_string(theme.icon_rust)),
        icon_maven: Some(color_to_string(theme.icon_maven)),
        icon_flutter: Some(color_to_string(theme.icon_flutter)),
        icon_go: Some(color_to_string(theme.icon_go)),
        icon_python: Some(color_to_string(theme.icon_python)),
        icon_mise: Some(color_to_string(theme.icon_mise)),
        icon_worktree: Some(color_to_string(theme.icon_worktree)),
        icon_worktree_lock: Some(color_to_string(theme.icon_worktree_lock)),
        icon_gitmodules: Some(color_to_string(theme.icon_gitmodules)),
        icon_git: Some(color_to_string(theme.icon_git)),
        icon_folder: Some(color_to_string(theme.icon_folder)),
        icon_file: Some(color_to_string(theme.icon_file)),
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
