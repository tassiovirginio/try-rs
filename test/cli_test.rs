use clap::Parser;
use try_rs::cli::{Cli, Shell};

#[test]
fn cli_default_values() {
    let cli = Cli::try_parse_from(["try-rs"]).unwrap();
    assert!(cli.name_or_url.is_none());
    assert!(cli.destination.is_none());
    assert!(cli.setup.is_none());
    assert!(cli.setup_stdout.is_none());
    assert!(cli.completions.is_none());
    assert!(!cli.shallow_clone);
    assert!(cli.worktree.is_none());
}

#[test]
fn cli_with_name() {
    let cli = Cli::try_parse_from(["try-rs", "my-project"]).unwrap();
    assert_eq!(cli.name_or_url, Some("my-project".to_string()));
}

#[test]
fn cli_with_git_url() {
    let cli = Cli::try_parse_from(["try-rs", "https://github.com/user/repo.git"]).unwrap();
    assert_eq!(
        cli.name_or_url,
        Some("https://github.com/user/repo.git".to_string())
    );
}

#[test]
fn cli_with_name_and_destination() {
    let cli =
        Cli::try_parse_from(["try-rs", "https://github.com/user/repo.git", "my-folder"]).unwrap();
    assert_eq!(
        cli.name_or_url,
        Some("https://github.com/user/repo.git".to_string())
    );
    assert_eq!(cli.destination, Some("my-folder".to_string()));
}

#[test]
fn cli_setup_flag_fish() {
    let cli = Cli::try_parse_from(["try-rs", "--setup", "fish"]).unwrap();
    assert_eq!(cli.setup, Some(Shell::Fish));
}

#[test]
fn cli_setup_flag_zsh() {
    let cli = Cli::try_parse_from(["try-rs", "--setup", "zsh"]).unwrap();
    assert_eq!(cli.setup, Some(Shell::Zsh));
}

#[test]
fn cli_setup_flag_bash() {
    let cli = Cli::try_parse_from(["try-rs", "--setup", "bash"]).unwrap();
    assert_eq!(cli.setup, Some(Shell::Bash));
}

#[test]
fn cli_setup_flag_powershell() {
    let cli = Cli::try_parse_from(["try-rs", "--setup", "power-shell"]).unwrap();
    assert_eq!(cli.setup, Some(Shell::PowerShell));
}

#[test]
fn cli_setup_flag_nushell() {
    let cli = Cli::try_parse_from(["try-rs", "--setup", "nu-shell"]).unwrap();
    assert_eq!(cli.setup, Some(Shell::NuShell));
}

#[test]
fn cli_setup_stdout_flag() {
    let cli = Cli::try_parse_from(["try-rs", "--setup-stdout", "zsh"]).unwrap();
    assert_eq!(cli.setup_stdout, Some(Shell::Zsh));
}

#[test]
fn cli_completions_flag() {
    let cli = Cli::try_parse_from(["try-rs", "--completions", "bash"]).unwrap();
    assert_eq!(cli.completions, Some(Shell::Bash));
}

#[test]
fn cli_shallow_clone_flag() {
    let cli = Cli::try_parse_from([
        "try-rs",
        "--shallow-clone",
        "https://github.com/user/repo.git",
    ])
    .unwrap();
    assert!(cli.shallow_clone);
}

#[test]
fn cli_shorthand_clone_flag() {
    let cli = Cli::try_parse_from(["try-rs", "-s", "https://github.com/user/repo.git"]).unwrap();
    assert!(cli.shallow_clone);
}

#[test]
fn cli_worktree_flag() {
    let cli = Cli::try_parse_from(["try-rs", "--worktree", "feature-branch"]).unwrap();
    assert_eq!(cli.worktree, Some("feature-branch".to_string()));
}

#[test]
fn cli_worktree_shorthand() {
    let cli = Cli::try_parse_from(["try-rs", "-w", "feature-branch"]).unwrap();
    assert_eq!(cli.worktree, Some("feature-branch".to_string()));
}

#[test]
fn cli_multiple_flags() {
    let cli = Cli::try_parse_from([
        "try-rs",
        "--shallow-clone",
        "https://github.com/user/repo.git",
        "my-dest",
    ])
    .unwrap();
    assert!(cli.shallow_clone);
    assert_eq!(
        cli.name_or_url,
        Some("https://github.com/user/repo.git".to_string())
    );
    assert_eq!(cli.destination, Some("my-dest".to_string()));
}

#[test]
fn shell_enum_equality() {
    assert_eq!(Shell::Fish, Shell::Fish);
    assert_eq!(Shell::Zsh, Shell::Zsh);
    assert_eq!(Shell::Bash, Shell::Bash);
    assert_eq!(Shell::PowerShell, Shell::PowerShell);
    assert_eq!(Shell::NuShell, Shell::NuShell);
    assert_ne!(Shell::Fish, Shell::Zsh);
    assert_ne!(Shell::Bash, Shell::PowerShell);
}

#[test]
fn shell_enum_clone() {
    let shell = Shell::Zsh;
    let cloned = shell.clone();
    assert_eq!(shell, cloned);
}

#[test]
fn shell_enum_copy() {
    let shell = Shell::Fish;
    let copied: Shell = shell;
    assert_eq!(shell, copied);
}

#[test]
fn cli_version_flag() {
    let result = Cli::try_parse_from(["try-rs", "--version"]);
    assert!(result.is_err());
}

#[test]
fn cli_help_flag() {
    let result = Cli::try_parse_from(["try-rs", "--help"]);
    assert!(result.is_err());
}

#[test]
fn cli_empty_string_name() {
    let cli = Cli::try_parse_from(["try-rs", ""]).unwrap();
    assert_eq!(cli.name_or_url, Some("".to_string()));
}

#[test]
fn cli_complex_project_name() {
    let cli = Cli::try_parse_from(["try-rs", "my-awesome-project-v2"]).unwrap();
    assert_eq!(cli.name_or_url, Some("my-awesome-project-v2".to_string()));
}

#[test]
fn cli_ssh_git_url() {
    let cli = Cli::try_parse_from(["try-rs", "git@github.com:user/repo.git"]).unwrap();
    assert_eq!(
        cli.name_or_url,
        Some("git@github.com:user/repo.git".to_string())
    );
}
