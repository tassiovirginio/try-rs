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
use utils::{extract_repo_name, is_git_url};

fn main() -> Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            let mut stderr = std::io::stderr();
            write!(stderr, "{}", err).unwrap();
            std::process::exit(if err.use_stderr() { 1 } else { 0 });
        }
    };
    let (tries_dir, theme, editor_cmd, is_first_run, config_path) = load_configuration();

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

    if is_first_run && cli.setup.is_none() {
        let shell_type = if cfg!(windows) {
            Some(Shell::PowerShell)
        } else {
            if std::env::var("NU_VERSION").is_ok() {
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
            }
        };

        if let Some(s) = shell_type {
            eprintln!("Detected shell: {:?}", s);
            eprint!(
                "Shell integration not configured. Do you want to set it up for {:?}? [Y/n] ",
                s
            );
            io::stderr().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y") {
                match s {
                    Shell::Fish => setup_fish()?,
                    Shell::Zsh => setup_zsh()?,
                    Shell::Bash => setup_bash()?,
                    Shell::PowerShell => setup_powershell()?,
                    Shell::NuShell => setup_nushell()?,
                }
            }
        }
    }

    let selection_result: Option<String>;
    let mut open_editor = false;

    if let Some(name) = cli.name_or_url {
        selection_result = Some(name);
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
        );
        let res = run_app(&mut terminal, app);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        (selection_result, open_editor) = res?;
    }

    if let Some(selection) = selection_result {
        let target_path = tries_dir.join(&selection);

        if target_path.exists() {
            if open_editor && let Some(cmd) = editor_cmd {
                println!("{} '{}'", cmd, target_path.to_string_lossy());
            } else {
                println!("cd '{}'", target_path.to_string_lossy());
            }
        } else {
            if is_git_url(&selection) {
                let repo_name = extract_repo_name(&selection);

                let folder_name = cli.destination.clone().unwrap_or(repo_name);
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
                let new_name = selection;

                let new_path = tries_dir.join(&new_name);
                fs::create_dir_all(&new_path)?;
                if open_editor && let Some(cmd) = editor_cmd {
                    println!("{} '{}'", cmd, new_path.to_string_lossy());
                } else {
                    println!("cd '{}'", new_path.to_string_lossy());
                }
            }
        }
    }

    Ok(())
}
