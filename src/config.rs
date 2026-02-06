use crate::tui::Theme;
use crate::utils::expand_path;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub tries_path: Option<String>,
    pub theme: Option<String>,
    pub editor: Option<String>,
    pub apply_date_prefix: Option<bool>,
    pub transparent_background: Option<bool>,
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

/// Returns candidate config file paths in priority order.
fn config_candidates() -> Vec<PathBuf> {
    let config_name = get_file_config_toml_name();
    let mut candidates = Vec::new();

    if let Some(env_dir) = std::env::var_os("TRY_CONFIG_DIR") {
        candidates.push(PathBuf::from(env_dir).join(&config_name));
    }
    if let Some(dir) = dirs::config_dir() {
        candidates.push(dir.join("try-rs").join(&config_name));
    }
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
    pub tries_dir: PathBuf,
    pub theme: Theme,
    pub editor_cmd: Option<String>,
    pub config_path: Option<PathBuf>,
    pub apply_date_prefix: Option<bool>,
    pub transparent_background: Option<bool>,
}

pub fn load_configuration() -> AppConfig {
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
    let mut apply_date_prefix = None;
    let mut transparent_background = None;

    let loaded_config_path = find_config_path();

    if let Some(config) = load_file_config_toml_if_exists() {
        if let Some(path_str) = config.tries_path
            && !try_path_specified
        {
            final_path = expand_path(&path_str);
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
    }

    AppConfig {
        tries_dir: final_path,
        theme,
        editor_cmd,
        config_path: loaded_config_path,
        apply_date_prefix,
        transparent_background,
    }
}

pub fn save_config(
    path: &Path,
    theme: &Theme,
    tries_path: &Path,
    editor: &Option<String>,
    apply_date_prefix: Option<bool>,
    transparent_background: Option<bool>,
) -> std::io::Result<()> {
    let config = Config {
        tries_path: Some(tries_path.to_string_lossy().to_string()),
        theme: Some(theme.name.clone()),
        editor: editor.clone(),
        apply_date_prefix,
        transparent_background,
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
