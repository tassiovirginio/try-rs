use std::path::{Path, PathBuf};

use chrono::Local;
use tempdir::TempDir;
use try_rs::utils::*;

#[test]
fn is_git_url_valid_urls() {
    assert!(is_git_url("https://github.com/user/repo.git"));
    assert!(is_git_url("http://github.com/user/repo"));
    assert!(is_git_url("git@github.com:user/repo.git"));
    assert!(is_git_url("ssh://git@github.com/user/repo"));
    assert!(is_git_url("some-repo.git"));
}

#[test]
fn is_git_url_rejects_plain_names() {
    assert!(!is_git_url("my-project"));
    assert!(!is_git_url("foo/bar"));
    assert!(!is_git_url(""));
}

#[test]
fn is_git_url_edge_cases() {
    assert!(is_git_url("https://gitlab.com/group/subgroup/repo.git"));
    assert!(is_git_url("git@gitlab.com:group/repo.git"));
    assert!(is_git_url("ssh://user@host:22/repo"));
    assert!(!is_git_url("ftp://server/file"));
    assert!(!is_git_url("just-a-name"));
    assert!(!is_git_url("path/to/dir"));
}

#[test]
fn extract_repo_name_from_https() {
    assert_eq!(
        extract_repo_name("https://github.com/user/repo.git"),
        "repo"
    );
    assert_eq!(extract_repo_name("https://github.com/user/repo"), "repo");
}

#[test]
fn extract_repo_name_from_ssh() {
    assert_eq!(extract_repo_name("git@github.com:user/repo.git"), "repo");
}

#[test]
fn extract_repo_name_trailing_slash() {
    assert_eq!(extract_repo_name("https://github.com/user/repo/"), "repo");
}

#[test]
fn extract_repo_name_edge_cases() {
    assert_eq!(
        extract_repo_name("https://github.com/user/my-repo.git/"),
        "my-repo"
    );
    assert_eq!(extract_repo_name("git@github.com:org/project"), "project");
    assert_eq!(
        extract_repo_name("ssh://git@host/deep/nested/repo.git"),
        "repo"
    );
}

#[test]
fn extract_repo_name_fallback() {
    assert_eq!(extract_repo_name(""), "cloned-repo");
}

#[test]
fn extract_prefix_date_valid() {
    let result = extract_prefix_date("2024-06-15 my-project");
    assert!(result.is_some());
    let (_, name) = result.unwrap();
    assert_eq!(name, "my-project");
}

#[test]
fn extract_prefix_date_invalid() {
    assert!(extract_prefix_date("not-a-date project").is_none());
    assert!(extract_prefix_date("nodate").is_none());
}

#[test]
fn extract_prefix_date_only_date_no_name() {
    assert!(extract_prefix_date("2024-06-15").is_none());
}

#[test]
fn extract_prefix_date_with_spaces_in_name() {
    let result = extract_prefix_date("2024-01-01 my cool project");
    assert!(result.is_some());
    let (_, name) = result.unwrap();
    assert_eq!(name, "my cool project");
}

#[test]
fn extract_prefix_date_wrong_format() {
    assert!(extract_prefix_date("01-01-2024 project").is_none());
    assert!(extract_prefix_date("2024/01/01 project").is_none());
}

#[test]
fn generate_prefix_date_format() {
    let date = generate_prefix_date();
    assert_eq!(date.len(), 10);
    assert_eq!(&date[4..5], "-");
    assert_eq!(&date[7..8], "-");
}

#[test]
fn generate_prefix_date_matches_today() {
    let date = generate_prefix_date();
    let today = Local::now().format("%Y-%m-%d").to_string();
    assert_eq!(date, today);
}

#[test]
fn expand_path_tilde() {
    let expanded = expand_path("~/some/dir");
    assert!(!expanded.starts_with("~"));
    assert!(expanded.to_string_lossy().ends_with("some/dir"));
}

#[test]
fn expand_path_absolute() {
    let expanded = expand_path("/absolute/path");
    assert_eq!(expanded, PathBuf::from("/absolute/path"));
}

#[test]
fn expand_path_relative() {
    let expanded = expand_path("relative/path");
    assert_eq!(expanded, PathBuf::from("relative/path"));
}

#[test]
fn expand_path_just_tilde() {
    let expanded = expand_path("~");
    assert_eq!(expanded, PathBuf::from("~"));
}

#[test]
fn matching_folders_exact_and_dated() {
    let tmp = TempDir::new("match-test").unwrap();
    let base = tmp.path();
    std::fs::create_dir(base.join("foo")).unwrap();
    std::fs::create_dir(base.join("2024-01-15 foo")).unwrap();
    std::fs::create_dir(base.join("bar")).unwrap();

    let matches = matching_folders("foo", &base.to_path_buf());
    assert!(matches.contains(&"foo".to_string()));
    assert!(matches.contains(&"2024-01-15 foo".to_string()));
    assert!(!matches.contains(&"bar".to_string()));
}

#[test]
fn matching_folders_empty_dir() {
    let tmp = TempDir::new("match-empty").unwrap();
    let matches = matching_folders("anything", &tmp.path().to_path_buf());
    assert!(matches.is_empty());
}

#[test]
fn matching_folders_no_match() {
    let tmp = TempDir::new("match-none").unwrap();
    std::fs::create_dir(tmp.path().join("alpha")).unwrap();
    std::fs::create_dir(tmp.path().join("beta")).unwrap();
    let matches = matching_folders("gamma", &tmp.path().to_path_buf());
    assert!(matches.is_empty());
}

#[test]
fn matching_folders_ignores_files() {
    let tmp = TempDir::new("match-files").unwrap();
    std::fs::write(tmp.path().join("foo"), "not a dir").unwrap();
    let matches = matching_folders("foo", &tmp.path().to_path_buf());
    assert!(matches.is_empty());
}

#[test]
fn matching_folders_multiple_dated() {
    let tmp = TempDir::new("match-multi-date").unwrap();
    std::fs::create_dir(tmp.path().join("2024-01-01 proj")).unwrap();
    std::fs::create_dir(tmp.path().join("2024-06-15 proj")).unwrap();
    std::fs::create_dir(tmp.path().join("other")).unwrap();
    let matches = matching_folders("proj", &tmp.path().to_path_buf());
    assert_eq!(matches.len(), 2);
    assert!(matches.iter().all(|m| m.contains("proj")));
}

#[test]
fn matching_folders_nonexistent_path() {
    let matches = matching_folders("foo", &PathBuf::from("/nonexistent/dir"));
    assert!(matches.is_empty());
}

#[test]
fn get_folder_size_mb_empty() {
    let tmp = TempDir::new("size-test").unwrap();
    assert_eq!(get_folder_size_mb(tmp.path()), 0);
}

#[test]
fn get_folder_size_mb_nonexistent() {
    assert_eq!(get_folder_size_mb(Path::new("/nonexistent/path")), 0);
}

#[test]
fn get_folder_size_mb_with_files() {
    let tmp = TempDir::new("size-with-files").unwrap();
    std::fs::write(tmp.path().join("small.txt"), "hello").unwrap();
    assert_eq!(get_folder_size_mb(tmp.path()), 0);
}

#[test]
fn is_git_worktree_not_worktree() {
    let tmp = TempDir::new("worktree-test").unwrap();
    assert!(!is_git_worktree(tmp.path()));
}

#[test]
fn is_git_worktree_with_git_dir() {
    let tmp = TempDir::new("worktree-dir").unwrap();
    std::fs::create_dir(tmp.path().join(".git")).unwrap();
    assert!(!is_git_worktree(tmp.path()));
}

#[test]
fn is_git_worktree_locked_no_git() {
    let tmp = TempDir::new("lock-test").unwrap();
    assert!(!is_git_worktree_locked(tmp.path()));
}

#[test]
fn first_line_basic() {
    let input = b"gitdir: /some/path/to/worktree\n";
    let result = first_line(input);
    assert_eq!(result.to_string_lossy(), "/some/path/to/worktree");
}

#[test]
fn first_line_no_newline() {
    let input = b"gitdir: /path/without/newline";
    let result = first_line(input);
    assert_eq!(result.to_string_lossy(), "/path/without/newline");
}

#[cfg(unix)]
#[test]
fn get_free_disk_space_mb_returns_some_for_root() {
    let space = get_free_disk_space_mb(Path::new("/"));
    assert!(space.is_some());
    assert!(space.unwrap() > 0);
}

#[cfg(unix)]
#[test]
fn get_free_disk_space_mb_returns_none_for_invalid() {
    let space = get_free_disk_space_mb(Path::new("/nonexistent/path/xyz"));
    assert!(space.is_none());
}

#[test]
fn is_inside_git_repo_inside_repo() {
    let tmp = TempDir::new("git-repo").unwrap();

    // Initialize a git repo
    let _ = std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp.path())
        .output();

    assert!(is_inside_git_repo(tmp.path()));
}

#[test]
fn is_inside_git_repo_outside_repo() {
    let tmp = TempDir::new("no-git").unwrap();
    assert!(!is_inside_git_repo(tmp.path()));
}

#[test]
fn remove_git_worktree_returns_output() {
    let tmp = TempDir::new("worktree-remove").unwrap();

    // Initialize a git repo first
    let _ = std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp.path())
        .output();

    let result = remove_git_worktree(tmp.path());
    // Result should be Ok even if it fails (command ran)
    assert!(result.is_ok());
}

#[test]
fn first_line_with_multiple_lines() {
    let input = b"gitdir: /path/to/worktree\nother: content\nmore: lines";
    let result = first_line(input);
    assert_eq!(result.to_string_lossy(), "/path/to/worktree");
}

#[test]
fn first_line_empty_after_space() {
    let input = b"gitdir: ";
    let result = first_line(input);
    assert_eq!(result.to_string_lossy(), "");
}

#[test]
fn is_git_url_various_protocols() {
    assert!(is_git_url("https://gitlab.com/user/repo.git"));
    assert!(is_git_url("http://bitbucket.org/user/repo"));
    assert!(is_git_url("git@github.com:user/repo.git"));
    assert!(is_git_url("ssh://git@gitlab.com/user/repo"));
}

#[test]
fn is_git_url_local_paths() {
    assert!(is_git_url("/path/to/repo.git"));
    assert!(is_git_url("./relative/repo.git"));
    assert!(is_git_url("../parent/repo.git"));
}

#[test]
fn extract_repo_name_complex_urls() {
    assert_eq!(
        extract_repo_name("https://host.com/org/team/repo.git"),
        "repo"
    );
    assert_eq!(extract_repo_name("git@host.com:org/team/repo"), "repo");
    assert_eq!(extract_repo_name("/deep/nested/path/repo.git"), "repo");
}

#[test]
fn extract_repo_name_single_word() {
    // Single words that look like repo names (not URLs) are returned as-is
    assert_eq!(extract_repo_name("myrepo"), "myrepo");
    assert_eq!(extract_repo_name("repo.git"), "repo");
}

#[test]
fn expand_path_with_tilde_and_complex_path() {
    let expanded = expand_path("~/work/projects/my-project");
    assert!(!expanded.starts_with("~"));
    assert!(expanded
        .to_string_lossy()
        .contains("work/projects/my-project"));
}

#[test]
fn expand_path_empty_string() {
    let expanded = expand_path("");
    assert_eq!(expanded, PathBuf::from(""));
}

#[test]
fn extract_prefix_date_with_different_dates() {
    let result = extract_prefix_date("2023-12-25 christmas-project");
    assert!(result.is_some());
    let (_, name) = result.unwrap();
    assert_eq!(name, "christmas-project");
}

#[test]
fn extract_prefix_date_invalid_formats() {
    assert!(extract_prefix_date("2023-13-01 invalid-month").is_none());
    assert!(extract_prefix_date("2023-00-15 invalid-day").is_none());
    // Note: "23-01-01" might be parsed as year 23 (0023), which is technically valid
    // We focus on clearly invalid dates
    assert!(extract_prefix_date("not-a-date project").is_none());
    assert!(extract_prefix_date("01-01-2023 wrong-format").is_none());
}

#[test]
fn extract_prefix_date_edge_cases() {
    assert!(extract_prefix_date("2024-02-29 leap-year").is_some());
    assert!(extract_prefix_date("2023-02-29 not-leap-year").is_none());
}

#[test]
fn matching_folders_with_special_chars() {
    let tmp = TempDir::new("match-special").unwrap();
    std::fs::create_dir(tmp.path().join("my-project_v2")).unwrap();
    std::fs::create_dir(tmp.path().join("2024-01-15 my-project_v2")).unwrap();

    let matches = matching_folders("my-project_v2", &tmp.path().to_path_buf());
    assert_eq!(matches.len(), 2);
}

#[test]
fn matching_folders_nested_search() {
    let tmp = TempDir::new("match-nested").unwrap();
    let nested = tmp.path().join("parent").join("child");
    std::fs::create_dir_all(&nested).unwrap();

    // matching_folders only searches immediate children
    let matches = matching_folders("child", &tmp.path().to_path_buf());
    assert!(matches.is_empty());
}

#[test]
fn get_folder_size_mb_large_files() {
    let tmp = TempDir::new("size-large").unwrap();
    // Create a 2MB file
    let content = vec![0u8; 2 * 1024 * 1024];
    std::fs::write(tmp.path().join("large.bin"), content).unwrap();

    let size = get_folder_size_mb(tmp.path());
    assert!(size >= 1);
}

#[test]
fn get_folder_size_mb_nested_dirs() {
    let tmp = TempDir::new("size-nested").unwrap();
    let nested = tmp.path().join("level1").join("level2");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::write(nested.join("file.txt"), "content").unwrap();

    let size = get_folder_size_mb(tmp.path());
    // Should be 0 MB since file is small
    assert_eq!(size, 0);
}

#[test]
fn get_folder_size_mb_with_symlinks() {
    let tmp = TempDir::new("size-symlinks").unwrap();
    let real_file = tmp.path().join("real.txt");
    std::fs::write(&real_file, "content").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link = tmp.path().join("link.txt");
        let _ = symlink(&real_file, &link);
    }

    // Symlinks should not be followed
    let size = get_folder_size_mb(tmp.path());
    assert_eq!(size, 0);
}

#[test]
fn selection_result_variants() {
    let folder = SelectionResult::Folder("test".to_string());
    let new = SelectionResult::New("new-folder".to_string());
    let none = SelectionResult::None;

    match folder {
        SelectionResult::Folder(name) => assert_eq!(name, "test"),
        _ => panic!("Expected Folder variant"),
    }

    match new {
        SelectionResult::New(name) => assert_eq!(name, "new-folder"),
        _ => panic!("Expected New variant"),
    }

    match none {
        SelectionResult::None => (),
        _ => panic!("Expected None variant"),
    }
}

#[test]
fn generate_prefix_date_consistency() {
    let date1 = generate_prefix_date();
    let date2 = generate_prefix_date();
    // Same call should produce same result
    assert_eq!(date1, date2);

    // Check format is YYYY-MM-DD
    assert_eq!(date1.len(), 10);
    assert!(date1.chars().nth(4).unwrap() == '-');
    assert!(date1.chars().nth(7).unwrap() == '-');
}
