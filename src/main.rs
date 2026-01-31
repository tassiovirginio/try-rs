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
use config::load_configuration;
use shell::{setup_bash, setup_fish, setup_nushell, setup_powershell, setup_zsh};
use tui::{App, run_app};

use crate::utils::{SelectionResult, generate_prefix_date};

fn main() -> Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            let mut stderr = std::io::stderr();
            write!(stderr, "{}", err).unwrap();
            std::process::exit(if err.use_stderr() { 1 } else { 0 });
        }
    };
    let (
        tries_dir,
        theme,
        editor_cmd,
        _is_first_run,
        config_path,
        apply_date_prefix,
        transparent_background,
    ) = load_configuration();

    if !tries_dir.exists() {
        fs::create_dir_all(&tries_dir)?;
    }

    if let Some(shell) = cli.setup {
        match shell {
            Shell::Fish => setup_fish()?,
            Shell::Zsh => setup_zsh()?,
            Shell::Bash => setup_bash()?,
            Shell::PowerShell => setup_powershell()?,
            Shell::NuShell => setup_nushell()?,
        }
        return Ok(());
    }

    if let Some(worktree_branch_name) = cli.worktree {
        if !utils::is_inside_git_repo(".") {
            eprintln!("Error: Not inside a git repository.");
            eprintln!("The -w/--worktree option only works inside a git repository.");
            std::process::exit(1);
        }

        let mut folder_name = worktree_branch_name.clone();
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
            &worktree_branch_name,
            new_path.display()
        );

        let branch_exists_status = std::process::Command::new("git")
            .args(["show-ref", &format!("refs/heads/{worktree_branch_name}")])
            .stdout(std::io::stderr())
            .stderr(Stdio::inherit())
            .status();

        let branch_exists = match branch_exists_status {
            Ok(s) => s.success(),
            _ => {
                eprintln!("Error: Failed to create worktree.");
                std::process::exit(1);
            }
        };

        let status = if branch_exists {
            std::process::Command::new("git")
                .args([
                    "worktree",
                    "add",
                    new_path.to_str().unwrap(),
                    &worktree_branch_name,
                ])
                .stdout(std::io::stderr())
                .stderr(Stdio::inherit())
                .status()
        } else {
            std::process::Command::new("git")
                .args([
                    "worktree",
                    "add",
                    "-b",
                    &worktree_branch_name,
                    new_path.to_str().unwrap(),
                ])
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

        return Ok(());
    }

    if cli.setup.is_none() {
        let shell_type = if cfg!(windows) {
            Some(shell::ShellType::PowerShell)
        } else {
            if std::env::var("NU_VERSION").is_ok() {
                Some(shell::ShellType::NuShell)
            } else {
                let shell = std::env::var("SHELL").unwrap_or_default();
                if shell.contains("fish") {
                    Some(shell::ShellType::Fish)
                } else if shell.contains("zsh") {
                    Some(shell::ShellType::Zsh)
                } else if shell.contains("bash") {
                    Some(shell::ShellType::Bash)
                } else {
                    None
                }
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
                    match s {
                        shell::ShellType::Fish => setup_fish()?,
                        shell::ShellType::Zsh => setup_zsh()?,
                        shell::ShellType::Bash => setup_bash()?,
                        shell::ShellType::PowerShell => setup_powershell()?,
                        shell::ShellType::NuShell => setup_nushell()?,
                    }
                }
            }
        }
    }

    let selection_result: SelectionResult;
    let mut open_editor = false;

    let (matching_folders, query) = match &cli.name_or_url {
        Some(name) => {
            let folder_name = if utils::is_git_url(&name) {
                let repo_name = utils::extract_repo_name(&name);
                &cli.destination.clone().unwrap_or(repo_name)
            } else {
                name
            };

            (
                utils::matching_folders(&folder_name, &tries_dir),
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

    if let SelectionResult::Folder(selection) = selection_result {
        let target_path = tries_dir.join(&selection);
        if open_editor && let Some(cmd) = editor_cmd {
            println!("{} '{}'", cmd, target_path.to_string_lossy());
        } else {
            println!("cd '{}'", target_path.to_string_lossy());
        }
    } else if let SelectionResult::New(selection) = selection_result {
        if utils::is_git_url(&selection) {
            let repo_name = utils::extract_repo_name(&selection);

            let mut folder_name = cli.destination.clone().unwrap_or(repo_name);
            if Some(true) == apply_date_prefix {
                folder_name = format!("{} {}", generate_prefix_date(), folder_name);
            }

            let new_path = tries_dir.join(&folder_name);

            eprintln!("Cloning {} into {}...", selection, folder_name);

            let mut cmd = std::process::Command::new("git");
            cmd.arg("clone");

            if cli.shallow_clone {
                cmd.arg("--depth").arg("1");
            }

            let status = cmd
                .arg(&selection)
                .arg(&new_path)
                .arg("--recurse-submodules")
                .arg("--no-single-branch")
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .status();

            match status {
                Ok(s) if s.success() => {
                    if open_editor && let Some(cmd) = editor_cmd {
                        println!("{} '{}'", cmd, new_path.to_string_lossy());
                    } else {
                        println!("cd '{}'", new_path.to_string_lossy());
                    }
                }
                _ => {
                    eprintln!("Error: Failed to clone the repository.");
                }
            }
        } else {
            let mut new_name = selection;
            let date_prefix = generate_prefix_date();
            if Some(true) == apply_date_prefix && !new_name.starts_with(&date_prefix) {
                new_name = format!("{date_prefix} {new_name}");
            }

            let new_path = tries_dir.join(&new_name);
            fs::create_dir_all(&new_path)?;
            if open_editor && let Some(cmd) = editor_cmd {
                println!("{} '{}'", cmd, new_path.to_string_lossy());
            } else {
                println!("cd '{}'", new_path.to_string_lossy());
            }
        }
    }

    Ok(())
}
