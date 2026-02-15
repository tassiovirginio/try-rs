use ratatui::style::Color;
use try_rs::themes::Theme;

#[test]
fn all_themes_have_unique_names() {
    let themes = Theme::all();
    let mut names: Vec<&str> = themes.iter().map(|t| t.name.as_str()).collect();
    let total = names.len();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), total, "theme names must be unique");
}

#[test]
fn all_themes_non_empty() {
    assert!(!Theme::all().is_empty());
}

#[test]
fn all_themes_have_non_empty_names() {
    for theme in Theme::all() {
        assert!(!theme.name.is_empty(), "theme name must not be empty");
    }
}

#[test]
fn default_theme_is_first() {
    assert_eq!(Theme::all()[0].name, "Default");
}

#[test]
fn default_theme_has_no_background() {
    assert!(Theme::default().background.is_none());
}

#[test]
fn default_theme_name_is_default() {
    assert_eq!(Theme::default().name, "Default");
}

#[test]
fn themed_themes_have_background() {
    let named = [
        "Catppuccin Mocha",
        "Dracula",
        "Gruvbox Dark",
        "Nord",
        "Tokyo Night",
    ];
    let all = Theme::all();
    for name in named {
        let t = all.iter().find(|t| t.name == name);
        assert!(t.is_some(), "theme '{}' should exist", name);
        assert!(
            t.unwrap().background.is_some(),
            "theme '{}' should have a background",
            name
        );
    }
}

#[test]
fn known_themes_exist() {
    let names: Vec<String> = Theme::all().iter().map(|t| t.name.clone()).collect();
    let expected = [
        "Default",
        "Catppuccin Mocha",
        "Catppuccin Macchiato",
        "Dracula",
        "JetBrains Darcula",
        "Gruvbox Dark",
        "Nord",
        "Tokyo Night",
        "One Dark Pro",
        "Everforest",
        "SynthWave '84",
        "OLED True Black",
        "Silver Gray",
        "Black & White",
        "Matrix",
        "Tron",
    ];
    for name in expected {
        assert!(
            names.contains(&name.to_string()),
            "missing theme '{}'",
            name
        );
    }
}

#[test]
fn theme_count() {
    assert_eq!(Theme::all().len(), 16);
}

#[test]
fn clone_preserves_theme() {
    let theme = Theme::default();
    let cloned = theme.clone();
    assert_eq!(cloned.name, theme.name);
    assert_eq!(cloned.title_try, theme.title_try);
    assert_eq!(cloned.title_rs, theme.title_rs);
    assert_eq!(cloned.popup_bg, theme.popup_bg);
}

#[test]
fn oled_theme_has_black_background() {
    let oled = Theme::oled_true_black();
    assert_eq!(oled.background, Some(Color::Rgb(0, 0, 0)));
}

#[test]
fn each_theme_constructor_does_not_panic() {
    let _ = Theme::default_theme();
    let _ = Theme::catppuccin_mocha();
    let _ = Theme::catppuccin_macchiato();
    let _ = Theme::dracula();
    let _ = Theme::jetbrains_darcula();
    let _ = Theme::gruvbox_dark();
    let _ = Theme::nord();
    let _ = Theme::tokyo_night();
    let _ = Theme::one_dark_pro();
    let _ = Theme::everforest();
    let _ = Theme::synthwave_84();
    let _ = Theme::oled_true_black();
    let _ = Theme::silver_gray();
    let _ = Theme::black_and_white();
    let _ = Theme::matrix();
    let _ = Theme::tron();
}
