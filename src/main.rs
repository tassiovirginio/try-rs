use anyhow::Result;

use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::process::Stdio;
use std::{
    fs,
    io::{self, Write},
};

mod cli;
mod config;
mod shell;
mod themes;
mod tui;
mod utils;

use cli::{Cli, Shell};
use config::{AppConfig, load_configuration};
use shell::{generate_completions, get_shell_content, setup_shell};
use tui::{App, run_app};

use crate::utils::{SelectionResult, generate_prefix_date};

/// Prints the cd/editor command to stdout for the shell wrapper to eval.
fn print_cd_or_editor(path: &std::path::Path, open_editor: bool, editor_cmd: &Option<String>) {
    if open_editor && let Some(cmd) = editor_cmd {
        println!("{} '{}'", cmd, path.to_string_lossy());
    } else {
        println!("cd '{}'", path.to_string_lossy());
    }
}

/// Handles the --worktree flag: creates a git worktree in the tries dir.
fn handle_worktree(
    branch_name: &str,
    tries_dir: &std::path::Path,
    apply_date_prefix: Option<bool>,
) -> Result<()> {
    if !utils::is_inside_git_repo(".") {
        eprintln!("Error: Not inside a git repository.");
        eprintln!("The -w/--worktree option only works inside a git repository.");
        std::process::exit(1);
    }

    let mut folder_name = branch_name.to_string();
    if Some(true) == apply_date_prefix {
        folder_name = format!("{} {}", generate_prefix_date(), folder_name);
    }

    let new_path = tries_dir.join(&folder_name);

    if new_path.exists() {
        eprintln!("Worktree at '{}' already exists.", folder_name);
        println!("cd '{}'", new_path.to_string_lossy());
        return Ok(());
    }

    eprintln!(
        "Creating worktree '{}' at {}...",
        branch_name,
        new_path.display()
    );

    let branch_exists = std::process::Command::new("git")
        .args(["show-ref", &format!("refs/heads/{branch_name}")])
        .stdout(std::io::stderr())
        .stderr(Stdio::inherit())
        .status()
        .map(|s| s.success())
        .unwrap_or_else(|_| {
            eprintln!("Error: Failed to create worktree.");
            std::process::exit(1);
        });

    let new_path_str = new_path.to_string_lossy();
    let status = if branch_exists {
        std::process::Command::new("git")
            .args(["worktree", "add", &new_path_str, branch_name])
            .stdout(std::io::stderr())
            .stderr(Stdio::inherit())
            .status()
    } else {
        std::process::Command::new("git")
            .args(["worktree", "add", "-b", branch_name, &new_path_str])
            .stdout(std::io::stderr())
            .stderr(Stdio::inherit())
            .status()
    };

    match status {
        Ok(s) if s.success() => {
            println!("cd '{}'", new_path.to_string_lossy());
        }
        _ => {
            eprintln!("Error: Failed to create worktree.");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Detects the current shell and offers to set up shell integration if not configured.
fn detect_and_setup_shell() -> Result<()> {
    let shell_type = if cfg!(windows) {
        Some(Shell::PowerShell)
    } else if std::env::var("NU_VERSION").is_ok() {
        Some(Shell::NuShell)
    } else {
        let shell = std::env::var("SHELL").unwrap_or_default();
        if shell.contains("fish") {
            Some(Shell::Fish)
        } else if shell.contains("zsh") {
            Some(Shell::Zsh)
        } else if shell.contains("bash") {
            Some(Shell::Bash)
        } else {
            None
        }
    };

    if let Some(ref s) = shell_type {
        if !shell::is_shell_integration_configured(s) {
            eprintln!("Detected shell: {:?}", s);
            eprint!("Shell integration not configured. Do you want to set it up? [Y/n] ");
            io::stderr().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y") {
                setup_shell(s)?;
            }
        }
    }

    Ok(())
}

/// Clones a git repository into the tries directory.
fn handle_clone(
    url: &str,
    destination: Option<String>,
    shallow: bool,
    tries_dir: &std::path::Path,
    apply_date_prefix: Option<bool>,
    open_editor: bool,
    editor_cmd: &Option<String>,
) -> Result<()> {
    let repo_name = utils::extract_repo_name(url);
    let mut folder_name = destination.unwrap_or(repo_name);
    if Some(true) == apply_date_prefix {
        folder_name = format!("{} {}", generate_prefix_date(), folder_name);
    }

    let new_path = tries_dir.join(&folder_name);
    eprintln!("Cloning {} into {}...", url, folder_name);

    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone");
    if shallow {
        cmd.arg("--depth").arg("1");
    }

    let status = cmd
        .arg(url)
        .arg(&new_path)
        .arg("--recurse-submodules")
        .arg("--no-single-branch")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => print_cd_or_editor(&new_path, open_editor, editor_cmd),
        _ => eprintln!("Error: Failed to clone the repository."),
    }

    Ok(())
}

/// Creates a new folder in the tries directory.
fn handle_new_folder(
    name: &str,
    tries_dir: &std::path::Path,
    apply_date_prefix: Option<bool>,
    open_editor: bool,
    editor_cmd: &Option<String>,
) -> Result<()> {
    let mut new_name = name.to_string();
    let date_prefix = generate_prefix_date();
    if Some(true) == apply_date_prefix && !new_name.starts_with(&date_prefix) {
        new_name = format!("{date_prefix} {new_name}");
    }

    let new_path = tries_dir.join(&new_name);
    fs::create_dir_all(&new_path)?;
    print_cd_or_editor(&new_path, open_editor, editor_cmd);
    Ok(())
}

fn main() -> Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            let mut stdout = std::io::stdout();
            write!(stdout, "{}", err).unwrap();
            std::process::exit(if err.use_stderr() { 1 } else { 0 });
        }
    };
    let AppConfig {
        tries_dir,
        theme,
        editor_cmd,
        config_path,
        apply_date_prefix,
        transparent_background,
    } = load_configuration();

    if !tries_dir.exists() {
        fs::create_dir_all(&tries_dir)?;
    }

    if let Some(shell) = cli.setup {
        setup_shell(&shell)?;
        return Ok(());
    }

    if let Some(shell) = cli.setup_stdout {
        print!("{}", get_shell_content(&shell));
        return Ok(());
    }

    if let Some(shell) = cli.completions {
        generate_completions(&shell)?;
        return Ok(());
    }

    if let Some(ref worktree_branch_name) = cli.worktree {
        handle_worktree(worktree_branch_name, &tries_dir, apply_date_prefix)?;
        return Ok(());
    }

    if cli.setup.is_none() {
        detect_and_setup_shell()?;
    }

    let selection_result: SelectionResult;
    let mut open_editor = false;

    let (matching_folders, query) = match &cli.name_or_url {
        Some(name) => {
            let folder_name = if utils::is_git_url(name) {
                let repo_name = utils::extract_repo_name(name);
                &cli.destination.clone().unwrap_or(repo_name)
            } else {
                name
            };

            (
                utils::matching_folders(folder_name, &tries_dir),
                Some(folder_name.to_string()),
            )
        }
        None => (vec![], None),
    };

    if let Some(name) = &cli.name_or_url
        && matching_folders.len() <= 1
    {
        if matching_folders.is_empty() {
            selection_result = SelectionResult::New(name.clone());
        } else {
            selection_result = SelectionResult::Folder(
                matching_folders
                    .into_iter()
                    .next()
                    .expect("must have exactly 1 items here"),
            );
        }
    } else {
        enable_raw_mode()?;
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend)?;

        let app = App::new(
            tries_dir.clone(),
            theme,
            editor_cmd.clone(),
            config_path.clone(),
            apply_date_prefix,
            transparent_background.unwrap_or(true),
            query,
        );
        let res = run_app(&mut terminal, app);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        (selection_result, open_editor) = res?;
    }

    match selection_result {
        SelectionResult::Folder(selection) => {
            let target_path = tries_dir.join(&selection);
            print_cd_or_editor(&target_path, open_editor, &editor_cmd);
        }
        SelectionResult::New(selection) => {
            if utils::is_git_url(&selection) {
                handle_clone(
                    &selection,
                    cli.destination.clone(),
                    cli.shallow_clone,
                    &tries_dir,
                    apply_date_prefix,
                    open_editor,
                    &editor_cmd,
                )?;
            } else {
                handle_new_folder(
                    &selection,
                    &tries_dir,
                    apply_date_prefix,
                    open_editor,
                    &editor_cmd,
                )?;
            }
        }
        SelectionResult::None => {}
    }

    Ok(())
}
