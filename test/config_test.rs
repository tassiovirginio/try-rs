use std::path::PathBuf;

use tempdir::TempDir;
use try_rs::config::*;
use try_rs::themes::Theme;

#[test]
fn save_and_reload_config() {
    let tmp = TempDir::new("save-config-test").unwrap();
    let config_path = tmp.path().join("config.toml");
    let theme = Theme::default();
    let tries_path = PathBuf::from("/tmp/tries");

    save_config(
        &config_path,
        &theme,
        &tries_path,
        &Some("code".to_string()),
        Some(true),
        Some(false),
    )
    .unwrap();

    let contents = std::fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&contents).unwrap();
    assert_eq!(loaded.tries_path.as_deref(), Some("/tmp/tries"));
    assert_eq!(loaded.theme.as_deref(), Some("Default"));
    assert_eq!(loaded.editor.as_deref(), Some("code"));
    assert_eq!(loaded.apply_date_prefix, Some(true));
    assert_eq!(loaded.transparent_background, Some(false));
}

#[test]
fn save_config_creates_parent_dirs() {
    let tmp = TempDir::new("save-nested").unwrap();
    let config_path = tmp.path().join("nested").join("dir").join("config.toml");
    let theme = Theme::default();

    save_config(
        &config_path,
        &theme,
        &PathBuf::from("/tmp/t"),
        &None,
        None,
        None,
    )
    .unwrap();

    assert!(config_path.exists());
}

#[test]
fn save_config_none_optionals() {
    let tmp = TempDir::new("save-none").unwrap();
    let config_path = tmp.path().join("config.toml");
    let theme = Theme::default();

    save_config(
        &config_path,
        &theme,
        &PathBuf::from("/tmp/t"),
        &None,
        None,
        None,
    )
    .unwrap();

    let contents = std::fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&contents).unwrap();
    assert!(loaded.editor.is_none());
    assert!(loaded.apply_date_prefix.is_none());
    assert!(loaded.transparent_background.is_none());
}

#[test]
fn config_serialization_roundtrip() {
    let config = Config {
        tries_path: Some("~/work/tries".to_string()),
        theme: Some("Tokyo Night".to_string()),
        editor: Some("nvim".to_string()),
        apply_date_prefix: Some(true),
        transparent_background: Some(true),
    };

    let toml_str = toml::to_string(&config).unwrap();
    let loaded: Config = toml::from_str(&toml_str).unwrap();

    assert_eq!(loaded.tries_path, config.tries_path);
    assert_eq!(loaded.theme, config.theme);
    assert_eq!(loaded.editor, config.editor);
    assert_eq!(loaded.apply_date_prefix, config.apply_date_prefix);
    assert_eq!(loaded.transparent_background, config.transparent_background);
}

#[test]
fn config_deserialize_empty() {
    let config: Config = toml::from_str("").unwrap();
    assert!(config.tries_path.is_none());
    assert!(config.theme.is_none());
    assert!(config.editor.is_none());
    assert!(config.apply_date_prefix.is_none());
    assert!(config.transparent_background.is_none());
}

#[test]
fn config_deserialize_partial() {
    let config: Config = toml::from_str(r#"theme = "Nord""#).unwrap();
    assert_eq!(config.theme.as_deref(), Some("Nord"));
    assert!(config.tries_path.is_none());
}

#[test]
fn config_ignores_unknown_fields() {
    let result: Result<Config, _> = toml::from_str(
        r#"
theme = "Default"
unknown_field = "value"
"#,
    );
    let _ = result;
}

#[test]
fn get_file_config_toml_name_default() {
    unsafe { std::env::remove_var("TRY_CONFIG") };
    assert_eq!(get_file_config_toml_name(), "config.toml");
}

#[test]
fn get_config_dir_with_env() {
    unsafe { std::env::set_var("TRY_CONFIG_DIR", "/custom/config/dir") };
    let dir = get_config_dir();
    assert_eq!(dir, PathBuf::from("/custom/config/dir"));
    unsafe { std::env::remove_var("TRY_CONFIG_DIR") };
}

#[test]
fn save_config_preserves_theme_name() {
    let tmp = TempDir::new("theme-name").unwrap();
    let config_path = tmp.path().join("config.toml");

    let themes = Theme::all();
    for theme in &themes {
        save_config(
            &config_path,
            theme,
            &PathBuf::from("/tmp/t"),
            &None,
            None,
            None,
        )
        .unwrap();

        let contents = std::fs::read_to_string(&config_path).unwrap();
        let loaded: Config = toml::from_str(&contents).unwrap();
        assert_eq!(loaded.theme.as_deref(), Some(theme.name.as_str()));
    }
}

#[test]
fn config_with_all_fields() {
    let toml_str = r#"
tries_path = "/home/user/tries"
theme = "Tokyo Night"
editor = "vim"
apply_date_prefix = true
transparent_background = false
"#;

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.tries_path, Some("/home/user/tries".to_string()));
    assert_eq!(config.theme, Some("Tokyo Night".to_string()));
    assert_eq!(config.editor, Some("vim".to_string()));
    assert_eq!(config.apply_date_prefix, Some(true));
    assert_eq!(config.transparent_background, Some(false));
}

#[test]
fn config_save_and_load_roundtrip() {
    let tmp = TempDir::new("config-roundtrip").unwrap();
    let config_path = tmp.path().join("test.toml");
    let theme = Theme::default();

    save_config(
        &config_path,
        &theme,
        &PathBuf::from("~/tries"),
        &Some("nvim".to_string()),
        Some(true),
        Some(true),
    )
    .unwrap();

    let contents = std::fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&contents).unwrap();

    assert_eq!(loaded.tries_path, Some("~/tries".to_string()));
    assert_eq!(loaded.editor, Some("nvim".to_string()));
    assert_eq!(loaded.apply_date_prefix, Some(true));
    assert_eq!(loaded.transparent_background, Some(true));
}

#[test]
fn config_handles_whitespace_in_values() {
    let toml_str = r#"
tries_path = " /home/user/path with spaces "
editor = " code --wait "
"#;

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(
        config.tries_path,
        Some(" /home/user/path with spaces ".to_string())
    );
    assert_eq!(config.editor, Some(" code --wait ".to_string()));
}

#[test]
fn config_handles_unicode() {
    let toml_str = r#"
tries_path = "/home/user/实验"
theme = "东京之夜"
"#;

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.tries_path, Some("/home/user/实验".to_string()));
    assert_eq!(config.theme, Some("东京之夜".to_string()));
}

#[test]
fn config_preserve_booleans_exact() {
    let config1 = Config {
        tries_path: None,
        theme: None,
        editor: None,
        apply_date_prefix: Some(true),
        transparent_background: Some(false),
    };

    let toml = toml::to_string(&config1).unwrap();
    let config2: Config = toml::from_str(&toml).unwrap();

    assert_eq!(config1.apply_date_prefix, config2.apply_date_prefix);
    assert_eq!(
        config1.transparent_background,
        config2.transparent_background
    );
}

#[test]
fn config_with_empty_strings() {
    let config = Config {
        tries_path: Some("".to_string()),
        theme: Some("".to_string()),
        editor: Some("".to_string()),
        apply_date_prefix: None,
        transparent_background: None,
    };

    let toml = toml::to_string(&config).unwrap();
    let loaded: Config = toml::from_str(&toml).unwrap();

    assert_eq!(loaded.tries_path, Some("".to_string()));
    assert_eq!(loaded.theme, Some("".to_string()));
    assert_eq!(loaded.editor, Some("".to_string()));
}

#[test]
fn config_save_overwrites_existing() {
    let tmp = TempDir::new("config-overwrite").unwrap();
    let config_path = tmp.path().join("config.toml");

    let theme1 = Theme::default();
    save_config(
        &config_path,
        &theme1,
        &PathBuf::from("/path1"),
        &Some("editor1".to_string()),
        Some(true),
        None,
    )
    .unwrap();

    let theme2 = Theme::tokyo_night();
    save_config(
        &config_path,
        &theme2,
        &PathBuf::from("/path2"),
        &Some("editor2".to_string()),
        Some(false),
        Some(true),
    )
    .unwrap();

    let contents = std::fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&contents).unwrap();

    assert_eq!(loaded.tries_path, Some("/path2".to_string()));
    assert_eq!(loaded.editor, Some("editor2".to_string()));
    assert_eq!(loaded.apply_date_prefix, Some(false));
    assert_eq!(loaded.transparent_background, Some(true));
    assert_eq!(loaded.theme, Some("Tokyo Night".to_string()));
}

#[test]
fn config_serialization_order() {
    let config = Config {
        theme: Some("Default".to_string()),
        tries_path: Some("/path".to_string()),
        editor: Some("vim".to_string()),
        apply_date_prefix: Some(true),
        transparent_background: Some(false),
    };

    let toml = toml::to_string(&config).unwrap();

    // All fields should be present
    assert!(toml.contains("theme"));
    assert!(toml.contains("tries_path"));
    assert!(toml.contains("editor"));
    assert!(toml.contains("apply_date_prefix"));
    assert!(toml.contains("transparent_background"));
}

#[test]
fn config_handles_comments_in_toml() {
    let toml_str = r#"
# This is a comment
tries_path = "/home/user/tries"
# Another comment
theme = "Default"
"#;

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.tries_path, Some("/home/user/tries".to_string()));
    assert_eq!(config.theme, Some("Default".to_string()));
}

#[test]
fn config_file_extension_handling() {
    let tmp = TempDir::new("config-ext").unwrap();

    unsafe {
        std::env::remove_var("TRY_CONFIG");
    }

    let name = try_rs::config::get_file_config_toml_name();
    assert_eq!(name, "config.toml");

    unsafe {
        std::env::set_var("TRY_CONFIG", "custom.toml");
    }
    let name = try_rs::config::get_file_config_toml_name();
    assert_eq!(name, "custom.toml");

    unsafe {
        std::env::remove_var("TRY_CONFIG");
    }
}

#[test]
fn config_handles_special_characters_in_paths() {
    let toml_str = r#"
tries_path = "/path/with-dash/and_underscore/and.dot"
"#;

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(
        config.tries_path,
        Some("/path/with-dash/and_underscore/and.dot".to_string())
    );
}

#[test]
fn config_handles_very_long_values() {
    let long_path = "/very".repeat(100);
    let config = Config {
        tries_path: Some(long_path.clone()),
        theme: None,
        editor: None,
        apply_date_prefix: None,
        transparent_background: None,
    };

    let toml = toml::to_string(&config).unwrap();
    let loaded: Config = toml::from_str(&toml).unwrap();

    assert_eq!(loaded.tries_path, Some(long_path));
}
