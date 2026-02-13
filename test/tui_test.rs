use std::time::SystemTime;
use tempdir::TempDir;
use try_rs::themes::Theme;
use try_rs::tui::{App, TryEntry};

#[test]
fn try_entry_default_values() {
    let entry = TryEntry {
        name: "test".to_string(),
        display_name: "test".to_string(),
        modified: SystemTime::UNIX_EPOCH,
        created: SystemTime::UNIX_EPOCH,
        score: 0,
        is_git: false,
        is_worktree: false,
        is_worktree_locked: false,
        is_gitmodules: false,
        is_mise: false,
        is_cargo: false,
        is_maven: false,
        is_flutter: false,
        is_go: false,
        is_python: false,
    };

    assert_eq!(entry.name, "test");
    assert_eq!(entry.display_name, "test");
    assert_eq!(entry.score, 0);
    assert!(!entry.is_git);
    assert!(!entry.is_worktree);
}

#[test]
fn try_entry_clone() {
    let entry = TryEntry {
        name: "project".to_string(),
        display_name: "My Project".to_string(),
        modified: SystemTime::UNIX_EPOCH,
        created: SystemTime::UNIX_EPOCH,
        score: 100,
        is_git: true,
        is_worktree: false,
        is_worktree_locked: false,
        is_gitmodules: true,
        is_mise: false,
        is_cargo: true,
        is_maven: false,
        is_flutter: false,
        is_go: false,
        is_python: false,
    };

    let cloned = entry.clone();
    assert_eq!(cloned.name, entry.name);
    assert_eq!(cloned.score, entry.score);
    assert_eq!(cloned.is_git, entry.is_git);
    assert_eq!(cloned.is_cargo, entry.is_cargo);
}

#[test]
fn try_entry_with_flags() {
    let entry = TryEntry {
        name: "rust-project".to_string(),
        display_name: "rust-project".to_string(),
        modified: SystemTime::now(),
        created: SystemTime::now(),
        score: 50,
        is_git: true,
        is_worktree: false,
        is_worktree_locked: false,
        is_gitmodules: false,
        is_mise: true,
        is_cargo: true,
        is_maven: false,
        is_flutter: false,
        is_go: false,
        is_python: false,
    };

    assert!(entry.is_git);
    assert!(entry.is_cargo);
    assert!(entry.is_mise);
    assert!(!entry.is_maven);
}

#[test]
fn app_new_with_empty_directory() {
    let tmp = TempDir::new("app-empty").unwrap();
    let theme = Theme::default();

    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert!(app.all_entries.is_empty());
    assert!(app.filtered_entries.is_empty());
    assert_eq!(app.selected_index, 0);
}

#[test]
fn app_new_with_folders() {
    let tmp = TempDir::new("app-folders").unwrap();
    std::fs::create_dir(tmp.path().join("folder1")).unwrap();
    std::fs::create_dir(tmp.path().join("folder2")).unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 2);
    assert_eq!(app.filtered_entries.len(), 2);
}

#[test]
fn app_new_detects_git() {
    let tmp = TempDir::new("app-git").unwrap();
    let git_dir = tmp.path().join("git-project");
    std::fs::create_dir(&git_dir).unwrap();
    std::fs::create_dir(git_dir.join(".git")).unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_git);
}

#[test]
fn app_new_detects_cargo() {
    let tmp = TempDir::new("app-cargo").unwrap();
    let cargo_dir = tmp.path().join("rust-project");
    std::fs::create_dir(&cargo_dir).unwrap();
    std::fs::write(cargo_dir.join("Cargo.toml"), "[package]").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_cargo);
}

#[test]
fn app_new_detects_python() {
    let tmp = TempDir::new("app-python").unwrap();
    let py_dir = tmp.path().join("python-project");
    std::fs::create_dir(&py_dir).unwrap();
    std::fs::write(py_dir.join("pyproject.toml"), "[project]").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_python);
}

#[test]
fn app_new_with_date_prefix() {
    let tmp = TempDir::new("app-dated").unwrap();
    std::fs::create_dir(tmp.path().join("2024-01-15 my-project")).unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert_eq!(app.all_entries[0].display_name, "my-project");
    assert_eq!(app.all_entries[0].name, "2024-01-15 my-project");
}

#[test]
fn app_new_with_query() {
    let tmp = TempDir::new("app-query").unwrap();
    std::fs::create_dir(tmp.path().join("alpha")).unwrap();
    std::fs::create_dir(tmp.path().join("beta")).unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        Some("alpha".to_string()),
    );

    assert_eq!(app.query, "alpha");
    assert_eq!(app.filtered_entries.len(), 1);
    assert_eq!(app.filtered_entries[0].name, "alpha");
}

#[test]
fn app_update_search_empty_query() {
    let tmp = TempDir::new("app-search").unwrap();
    std::fs::create_dir(tmp.path().join("folder1")).unwrap();
    std::fs::create_dir(tmp.path().join("folder2")).unwrap();

    let theme = Theme::default();
    let mut app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        Some("test".to_string()),
    );

    app.query.clear();
    app.update_search();

    assert_eq!(app.filtered_entries.len(), 2);
}

#[test]
fn app_update_search_with_match() {
    let tmp = TempDir::new("app-search-match").unwrap();
    std::fs::create_dir(tmp.path().join("alpha-test")).unwrap();
    std::fs::create_dir(tmp.path().join("beta")).unwrap();

    let theme = Theme::default();
    let mut app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    app.query = "alpha".to_string();
    app.update_search();

    assert_eq!(app.filtered_entries.len(), 1);
    assert_eq!(app.filtered_entries[0].name, "alpha-test");
}

#[test]
fn app_update_search_no_match() {
    let tmp = TempDir::new("app-search-none").unwrap();
    std::fs::create_dir(tmp.path().join("folder1")).unwrap();

    let theme = Theme::default();
    let mut app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    app.query = "nonexistent".to_string();
    app.update_search();

    assert!(app.filtered_entries.is_empty());
}

#[test]
fn app_update_search_resets_index() {
    let tmp = TempDir::new("app-index").unwrap();
    std::fs::create_dir(tmp.path().join("a")).unwrap();
    std::fs::create_dir(tmp.path().join("b")).unwrap();
    std::fs::create_dir(tmp.path().join("c")).unwrap();

    let theme = Theme::default();
    let mut app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    app.selected_index = 2;
    app.query = "a".to_string();
    app.update_search();

    assert_eq!(app.selected_index, 0);
}

#[test]
fn app_delete_selected_removes_entry() {
    let tmp = TempDir::new("app-delete").unwrap();
    std::fs::create_dir(tmp.path().join("to-delete")).unwrap();

    let theme = Theme::default();
    let mut app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    app.delete_selected();

    assert!(app.all_entries.is_empty());
    assert!(!tmp.path().join("to-delete").exists());
}

#[test]
fn app_delete_selected_sets_status() {
    let tmp = TempDir::new("app-delete-status").unwrap();
    std::fs::create_dir(tmp.path().join("deleted")).unwrap();

    let theme = Theme::default();
    let mut app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    app.delete_selected();

    assert!(app.status_message.is_some());
    assert!(app.status_message.as_ref().unwrap().contains("Deleted"));
}

#[test]
fn app_has_all_themes() {
    let tmp = TempDir::new("app-themes").unwrap();
    let theme = Theme::default();

    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.available_themes.len(), 16);
}

#[test]
fn app_detects_mise() {
    let tmp = TempDir::new("app-mise").unwrap();
    let dir = tmp.path().join("mise-project");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("mise.toml"), "[tools]").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_mise);
}

#[test]
fn app_detects_maven() {
    let tmp = TempDir::new("app-maven").unwrap();
    let dir = tmp.path().join("java-project");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("pom.xml"), "<project></project>").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_maven);
}

#[test]
fn app_detects_go() {
    let tmp = TempDir::new("app-go").unwrap();
    let dir = tmp.path().join("go-project");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("go.mod"), "module test").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_go);
}

#[test]
fn app_detects_flutter() {
    let tmp = TempDir::new("app-flutter").unwrap();
    let dir = tmp.path().join("flutter-project");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("pubspec.yaml"), "name: test").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_flutter);
}

#[test]
fn app_detects_python_requirements() {
    let tmp = TempDir::new("app-py-req").unwrap();
    let dir = tmp.path().join("python-project");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("requirements.txt"), "requests").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_python);
}

#[test]
fn app_detects_gitmodules() {
    let tmp = TempDir::new("app-gitmodules").unwrap();
    let dir = tmp.path().join("project-with-submodules");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join(".gitmodules"), "[submodule]").unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert!(app.all_entries[0].is_gitmodules);
}

#[test]
fn app_sorted_by_modified() {
    let tmp = TempDir::new("app-sorted").unwrap();
    let dir1 = tmp.path().join("older");
    let dir2 = tmp.path().join("newer");
    std::fs::create_dir(&dir1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    std::fs::create_dir(&dir2).unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries[0].name, "newer");
    assert_eq!(app.all_entries[1].name, "older");
}

#[test]
fn app_ignores_files() {
    let tmp = TempDir::new("app-files").unwrap();
    std::fs::write(tmp.path().join("not-a-dir.txt"), "content").unwrap();
    std::fs::create_dir(tmp.path().join("actual-dir")).unwrap();

    let theme = Theme::default();
    let app = App::new(
        tmp.path().to_path_buf(),
        theme,
        None,
        None,
        None,
        false,
        None,
    );

    assert_eq!(app.all_entries.len(), 1);
    assert_eq!(app.all_entries[0].name, "actual-dir");
}
