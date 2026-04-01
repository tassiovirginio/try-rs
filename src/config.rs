use crate::tui::Theme;
use crate::utils::expand_path;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub tries_paths: Option<String>,
    pub tries_path: Option<String>,
    pub theme: Option<String>,
    pub editor: Option<String>,
    pub apply_date_prefix: Option<bool>,
    pub transparent_background: Option<bool>,
    pub show_disk: Option<bool>,
    pub show_preview: Option<bool>,
    pub show_legend: Option<bool>,
    pub show_right_panel: Option<bool>,
    pub right_panel_width: Option<u16>,
}

pub fn get_file_config_toml_name() -> String {
    std::env::var("TRY_CONFIG").unwrap_or("config.toml".to_string())
}

pub fn get_config_dir() -> PathBuf {
    std::env::var_os("TRY_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| get_base_config_dir().join("try-rs"))
}

/// Returns the base configuration directory.
/// Respects $XDG_CONFIG_HOME on all platforms (including macOS),
/// falling back to the platform-specific default from `dirs::config_dir()`,
/// and finally to `~/.config`.
pub fn get_base_config_dir() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(dirs::config_dir)
        .unwrap_or_else(|| {
            dirs::home_dir()
                .expect("Could not find home directory")
                .join(".config")
        })
}

/// Returns candidate config file paths in priority order.
fn config_candidates() -> Vec<PathBuf> {
    let config_name = get_file_config_toml_name();
    let mut candidates = Vec::new();

    if let Some(env_dir) = std::env::var_os("TRY_CONFIG_DIR") {
        candidates.push(PathBuf::from(env_dir).join(&config_name));
    }
    let base_dir = get_base_config_dir();
    candidates.push(base_dir.join("try-rs").join(&config_name));
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".config").join("try-rs").join(&config_name));
    }

    candidates
}

/// Finds the first existing config file path.
fn find_config_path() -> Option<PathBuf> {
    config_candidates().into_iter().find(|p| p.exists())
}

pub fn load_file_config_toml_if_exists() -> Option<Config> {
    let path = find_config_path()?;
    let contents = fs::read_to_string(&path).ok()?;
    toml::from_str::<Config>(&contents).ok()
}

pub struct AppConfig {
    pub tries_dirs: Vec<PathBuf>,
    pub active_tab: usize,
    pub theme: Theme,
    pub editor_cmd: Option<String>,
    pub config_path: Option<PathBuf>,
    pub apply_date_prefix: Option<bool>,
    pub transparent_background: Option<bool>,
    pub show_disk: Option<bool>,
    pub show_preview: Option<bool>,
    pub show_legend: Option<bool>,
    pub show_right_panel: Option<bool>,
    pub right_panel_width: Option<u16>,
}

pub fn load_configuration() -> AppConfig {
    let default_path = dirs::home_dir()
        .expect("Folder not found")
        .join("work")
        .join("tries");

    let mut theme = Theme::default();
    let try_path = std::env::var_os("TRY_PATH");
    let try_path_specified = try_path.is_some();
    let mut final_paths: Vec<PathBuf> = if let Some(path) = try_path {
        vec![path.into()]
    } else {
        vec![default_path]
    };
    let mut editor_cmd = std::env::var("VISUAL")
        .ok()
        .or_else(|| std::env::var("EDITOR").ok());
    let mut apply_date_prefix = None;
    let mut transparent_background = None;
    let mut show_disk = None;
    let mut show_preview = None;
    let mut show_legend = None;
    let mut show_right_panel = None;
    let mut right_panel_width = None;

    let loaded_config_path = find_config_path();

    if let Some(config) = load_file_config_toml_if_exists() {
        let paths_source = config.tries_paths.or(config.tries_path);
        
        if let Some(paths_str) = paths_source
            && !try_path_specified
        {
            final_paths = paths_str
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(expand_path)
                .collect();
        }
        if let Some(editor) = config.editor {
            editor_cmd = Some(editor);
        }
        if let Some(theme_name) = config.theme {
            if let Some(found_theme) = Theme::all().into_iter().find(|t| t.name == theme_name) {
                theme = found_theme;
            }
        }
        apply_date_prefix = config.apply_date_prefix;
        transparent_background = config.transparent_background;
        show_disk = config.show_disk;
        show_preview = config.show_preview;
        show_legend = config.show_legend;
        show_right_panel = config.show_right_panel;
        right_panel_width = config.right_panel_width;
    }

    AppConfig {
        tries_dirs: final_paths,
        active_tab: 0,
        theme,
        editor_cmd,
        config_path: loaded_config_path,
        apply_date_prefix,
        transparent_background,
        show_disk,
        show_preview,
        show_legend,
        show_right_panel,
        right_panel_width,
    }
}

pub fn save_config(
    path: &Path,
    theme: &Theme,
    tries_paths: &[PathBuf],
    editor: &Option<String>,
    apply_date_prefix: Option<bool>,
    transparent_background: Option<bool>,
    show_disk: Option<bool>,
    show_preview: Option<bool>,
    show_legend: Option<bool>,
    show_right_panel: Option<bool>,
    right_panel_width: Option<u16>,
) -> std::io::Result<()> {
    let paths_string = tries_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let config = Config {
        tries_paths: Some(paths_string),
        tries_path: tries_paths.first().map(|p| p.to_string_lossy().to_string()),
        theme: Some(theme.name.clone()),
        editor: editor.clone(),
        apply_date_prefix,
        transparent_background,
        show_disk,
        show_preview,
        show_legend,
        show_right_panel,
        right_panel_width,
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
