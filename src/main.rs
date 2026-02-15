use anyhow::{Result, anyhow, bail};

use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{TerminalOptions, Viewport, prelude::*};
use std::process::Stdio;
use std::{
    fs,
    io::{self, IsTerminal, Write},
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

const DSR: &str = "\x1b[6n";

fn get_cursor_position_for_inline_picker() -> io::Result<(u16, u16)> {
    #[cfg(windows)]
    {
        return crossterm::cursor::position();
    }

    #[cfg(not(windows))]
    {
        use std::fs::OpenOptions;
        use std::io::{BufReader, Read};

        let mut tty = OpenOptions::new().read(true).write(true).open("/dev/tty")?;
        write!(tty, "{DSR}")?;
        tty.flush()?;

        let mut response = Vec::new();
        for byte in BufReader::new(tty).bytes() {
            match byte {
                Ok(b'R') => break,
                Ok(b'\x1b' | b'[') => {}
                Ok(b) => response.push(b),
                Err(e) => return Err(e),
            }
        }

        let mut parts = response.split(|b| *b == b';');
        let row = parts
            .next()
            .and_then(|p| std::str::from_utf8(p).ok())
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(1);
        let col = parts
            .next()
            .and_then(|p| std::str::from_utf8(p).ok())
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(1);

        Ok((col.saturating_sub(1), row.saturating_sub(1)))
    }
}

fn compute_inline_picker_area(
    backend: &mut CrosstermBackend<io::Stderr>,
    requested_height: u16,
) -> io::Result<Rect> {
    let terminal_size = backend.size()?;
    let (_, mut cursor_y) = get_cursor_position_for_inline_picker()?;

    cursor_y = cursor_y.min(terminal_size.height.saturating_sub(1));

    let desired_height = requested_height.clamp(1, terminal_size.height.max(1));
    let available_height = terminal_size.height.saturating_sub(cursor_y);

    if available_height < desired_height {
        let scroll_amount = desired_height - available_height;

        if scroll_amount > 0 {
            execute!(backend, ScrollUp(scroll_amount))?;
        }

        cursor_y = cursor_y.saturating_sub(scroll_amount);
    }

    let final_height = terminal_size.height.saturating_sub(cursor_y).max(1);
    Ok(Rect::new(
        0,
        cursor_y,
        terminal_size.width,
        desired_height.min(final_height),
    ))
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
        const DEFAULT_INLINE_PICKER_HEIGHT: u16 = 18;
        const MIN_INLINE_PICKER_HEIGHT: u16 = 8;

        if cli.inline_picker && (!io::stdin().is_terminal() || !io::stderr().is_terminal()) {
            bail!("--inline-picker requires an interactive terminal session");
        }

        enable_raw_mode()?;
        let mut stderr = io::stderr();
        let mut inline_picker_area = None;

        if !cli.inline_picker {
            execute!(stderr, EnterAlternateScreen)?;
        }

        let backend = CrosstermBackend::new(stderr);
        let mut terminal = if cli.inline_picker {
            let inline_height = cli
                .inline_height
                .unwrap_or(DEFAULT_INLINE_PICKER_HEIGHT)
                .max(MIN_INLINE_PICKER_HEIGHT);

            let mut backend = backend;
            let picker_area = compute_inline_picker_area(&mut backend, inline_height).map_err(
                |err| anyhow!("--inline-picker requires an interactive terminal session ({err})"),
            )?;
            inline_picker_area = Some(picker_area);

            Terminal::with_options(
                backend,
                TerminalOptions {
                    viewport: Viewport::Fixed(picker_area),
                },
            )?
        } else {
            Terminal::new(backend)?
        };

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
        if cli.inline_picker {
            if let Some(area) = inline_picker_area {
                let end_y = area.y.saturating_add(area.height);
                for row in area.y..end_y {
                    execute!(
                        terminal.backend_mut(),
                        MoveTo(0, row),
                        Clear(ClearType::CurrentLine)
                    )?;
                }
                execute!(terminal.backend_mut(), MoveTo(0, area.y))?;
            } else {
                terminal.clear()?;
            }
        } else {
            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        }
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
