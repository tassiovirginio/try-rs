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
use std::{
    fs,
    io::{self, IsTerminal, Write},
    path::PathBuf,
    process::Stdio,
};

mod cli;
mod config;
mod shell;
mod themes;
mod tui;
mod utils;

use cli::{Cli, Shell};
use config::{AppConfig, load_configuration};
use shell::{clear_shell_setup, generate_completions, get_shell_content, setup_shell};
use tui::{App, run_app};

use crate::utils::{
    DEFAULT_NAME_SEPARATOR, SelectionResult, apply_name_separator, generate_prefix_date,
    prefix_with_date,
};

/// Prints the cd/editor command to stdout for the shell wrapper to eval.
fn print_cd_or_editor(path: &std::path::Path, open_editor: bool, editor_cmd: &Option<String>) {
    if open_editor && let Some(cmd) = editor_cmd {
        println!("{} {}", cmd, shell_path(path));
    } else if std::env::var("TRY_RS_SHELL").as_deref() == Ok("nu") {
        println!("cd {}", path.display());
    } else if std::env::var("TRY_RS_SHELL").as_deref() == Ok("powershell")
        || cfg!(windows) && std::env::var("TRY_RS_SHELL").is_err()
    {
        println!("Set-Location -LiteralPath {}", shell_path(path));
    } else {
        println!("cd {}", shell_path(path));
    }
}

/// Quote a path as literal data for the shell consuming stdout.
fn shell_path(path: &std::path::Path) -> String {
    let shell = std::env::var("TRY_RS_SHELL").unwrap_or_else(|_| {
        if cfg!(windows) {
            "powershell".into()
        } else {
            std::env::var("SHELL").unwrap_or_default()
        }
    });
    quote_shell_path(path, &shell)
}

fn quote_shell_path(path: &std::path::Path, shell: &str) -> String {
    let path = path.to_string_lossy();
    if shell.contains("nu") {
        let mut hashes = "#".to_string();
        while path.contains(&format!("'{hashes}")) {
            hashes.push('#');
        }
        return format!("r{hashes}'{path}'{hashes}");
    }
    if shell.contains("powershell") || shell.contains("pwsh") {
        return format!("'{}'", path.replace('\'', "''"));
    }
    let path = if shell.contains("fish") {
        path.replace('\\', "\\\\")
    } else {
        path.into_owned()
    };
    format!("'{}'", path.replace('\'', "'\\''"))
}

#[cfg(not(windows))]
const DSR: &str = "\x1b[6n";

/// Query the terminal for the current cursor position via DSR (Device Status Report).
///
/// On Unix, this reads directly from `/dev/tty`. On Windows, it delegates to
/// `crossterm::cursor::position()`.
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

/// Compute the terminal area for the inline picker, scrolling if necessary.
///
/// Ensures the picker fits below the current cursor line, scrolling the
/// terminal content up when there is not enough room.
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

struct TerminalGuard {
    alternate: bool,
    active: bool,
}
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = disable_raw_mode();
            if self.alternate {
                let _ = execute!(io::stderr(), LeaveAlternateScreen);
            }
            let _ = execute!(io::stderr(), crossterm::cursor::Show);
        }
    }
}

/// Handles the --worktree flag: creates a git worktree in the tries dir.
fn handle_worktree(
    branch_name: &str,
    tries_dir: &std::path::Path,
    apply_date_prefix: Option<bool>,
    date_prefix_format: Option<&str>,
    name_separator: &str,
) -> Result<()> {
    if !utils::is_inside_git_repo(".") {
        eprintln!("Error: Not inside a git repository.");
        eprintln!("The -w/--worktree option only works inside a git repository.");
        std::process::exit(1);
    }

    let mut folder_name = branch_name.to_string();
    if Some(true) == apply_date_prefix {
        folder_name = prefix_with_date(&folder_name, date_prefix_format, name_separator);
    }

    let new_path = tries_dir.join(&folder_name);

    if new_path.exists() {
        eprintln!("Worktree at '{}' already exists.", folder_name);
        print_cd_or_editor(&new_path, false, &None);
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
            print_cd_or_editor(&new_path, false, &None);
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

    if !io::stdin().is_terminal() {
        return Ok(());
    }

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
    full_clone: bool,
    tries_dir: &std::path::Path,
    apply_date_prefix: Option<bool>,
    date_prefix_format: Option<&str>,
    name_separator: &str,
    open_editor: bool,
    editor_cmd: &Option<String>,
) -> Result<()> {
    let repo_name = utils::extract_repo_name(url);
    let mut folder_name = destination
        .map(|d| apply_name_separator(&d, name_separator))
        .unwrap_or(repo_name);
    if Some(true) == apply_date_prefix {
        folder_name = prefix_with_date(&folder_name, date_prefix_format, name_separator);
    }

    let new_path = tries_dir.join(&folder_name);
    eprintln!("Cloning {} into {}...", url, folder_name);

    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone");
    if !full_clone {
        cmd.arg("--depth").arg("1");
        cmd.arg("--shallow-submodules");
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
        Ok(status) => bail!("Failed to clone the repository (exit status: {status})"),
        Err(err) => return Err(err.into()),
    }

    Ok(())
}

/// Creates a new folder in the tries directory.
fn handle_new_folder(
    name: &str,
    tries_dir: &std::path::Path,
    apply_date_prefix: Option<bool>,
    date_prefix_format: Option<&str>,
    name_separator: &str,
    open_editor: bool,
    editor_cmd: &Option<String>,
) -> Result<()> {
    let mut new_name = apply_name_separator(name, name_separator);
    let date_prefix = generate_prefix_date(date_prefix_format);
    if Some(true) == apply_date_prefix && !new_name.starts_with(&date_prefix) {
        new_name = prefix_with_date(&new_name, date_prefix_format, name_separator);
    }

    let new_path = tries_dir.join(&new_name);
    fs::create_dir_all(&new_path)?;
    print_cd_or_editor(&new_path, open_editor, editor_cmd);
    Ok(())
}

/// The program entry point.
///
/// Parses CLI arguments, loads configuration, and either handles a
/// short-circuit command (setup, completions, worktree) or launches
/// the TUI.
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
        tries_dirs,
        active_tab,
        theme,
        editor_cmd,
        config_path,
        apply_date_prefix,
        date_prefix_format,
        name_separator,
        transparent_background,
        show_disk,
        show_preview,
        show_legend,
        show_right_panel,
        right_panel_width,
    }: AppConfig = load_configuration();
    let config_name_separator = name_separator.clone();
    let name_separator = name_separator.unwrap_or_else(|| DEFAULT_NAME_SEPARATOR.to_string());

    let resolve_visibility = |cli_show: bool, cli_hide: bool, config_show: Option<bool>| -> bool {
        if !cli_hide {
            false
        } else if cli_show {
            true
        } else {
            config_show.unwrap_or(true)
        }
    };

    let show_disk = resolve_visibility(cli.show_disk, cli.hide_disk, show_disk);
    let show_preview = resolve_visibility(cli.show_preview, cli.hide_preview, show_preview);
    let show_legend = resolve_visibility(cli.show_legend, cli.hide_legend, show_legend);
    let show_right_panel =
        resolve_visibility(cli.show_right_panel, cli.hide_right_panel, show_right_panel);
    let right_panel_width = right_panel_width.unwrap_or(25).clamp(10, 90);

    // Handle output-only / config-only short-circuits before touching the
    // tries directory. These must work in environments where HOME is not
    // writable (e.g. the Nix build sandbox invoking `--setup-stdout` to
    // generate shell init scripts at build time).
    if let Some(shell) = cli.setup_stdout {
        print!("{}", get_shell_content(&shell));
        return Ok(());
    }

    if let Some(shell) = cli.completions {
        generate_completions(&shell)?;
        return Ok(());
    }

    if let Some(shell) = cli.setup {
        setup_shell(&shell)?;
        return Ok(());
    }

    if cli.setup_clear {
        clear_shell_setup()?;
        return Ok(());
    }

    let tries_dir = tries_dirs[active_tab].clone();

    if !tries_dir.exists() {
        fs::create_dir_all(&tries_dir)?;
    }

    if let Some(ref worktree_branch_name) = cli.worktree {
        handle_worktree(
            worktree_branch_name,
            &tries_dir,
            apply_date_prefix,
            date_prefix_format.as_deref(),
            &name_separator,
        )?;
        return Ok(());
    }

    if cli.setup.is_none() {
        detect_and_setup_shell()?;
    }

    let selection_result: SelectionResult;
    let mut open_editor = false;
    let mut selected_dir = tries_dirs[active_tab].clone();

    // Positional arguments: `<url> [destination]` when cloning, otherwise every
    // word forms a single experiment name (`try-rs my cool idea`).
    let (name_or_url, destination) = match &cli.name_or_url {
        Some(first) if utils::is_git_url(first) => {
            if !cli.words.is_empty() {
                eprintln!(
                    "Error: unexpected arguments after the destination: {}",
                    cli.words.join(" ")
                );
                std::process::exit(1);
            }
            (Some(first.clone()), cli.destination.clone())
        }
        Some(first) => {
            let words: Vec<&str> = std::iter::once(first.as_str())
                .chain(cli.destination.as_deref())
                .chain(cli.words.iter().map(String::as_str))
                .collect();
            (
                Some(apply_name_separator(&words.join(" "), &name_separator)),
                None,
            )
        }
        None => (None, None),
    };

    let (matching_folders, query) = match &name_or_url {
        Some(name) => {
            let folder_name = if utils::is_git_url(name) {
                let repo_name = utils::extract_repo_name(name);
                &destination.clone().unwrap_or(repo_name)
            } else {
                name
            };

            let mut all_matches: Vec<(PathBuf, String)> = Vec::new();
            for dir in &tries_dirs {
                let matches = utils::matching_folders(folder_name, dir);
                all_matches.extend(matches);
            }

            (all_matches, Some(folder_name.to_string()))
        }
        None => (Vec::<(PathBuf, String)>::new(), None),
    };

    if let Some(name) = &name_or_url
        && matching_folders.len() <= 1
    {
        if matching_folders.is_empty() {
            selection_result = SelectionResult::New(name.clone());
        } else {
            let (matched_dir, folder_name) = matching_folders
                .into_iter()
                .next()
                .expect("must have exactly 1 items here");
            selected_dir = matched_dir;
            selection_result = SelectionResult::Folder(folder_name);
        }
    } else {
        let has_multiple_tabs = tries_dirs.len() > 1;

        if !has_multiple_tabs && matching_folders.len() > 1 {
            selection_result = matching_folders
                .into_iter()
                .next()
                .map(|(_, n)| SelectionResult::Folder(n))
                .unwrap_or(SelectionResult::None);
        } else {
            const DEFAULT_INLINE_PICKER_HEIGHT: u16 = 18;
            const MIN_INLINE_PICKER_HEIGHT: u16 = 8;

            if cli.inline_picker && (!io::stdin().is_terminal() || !io::stderr().is_terminal()) {
                bail!("--inline_picker requires an interactive terminal session");
            }

            enable_raw_mode()?;
            let mut terminal_guard = TerminalGuard {
                alternate: !cli.inline_picker,
                active: true,
            };
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
                let picker_area =
                    compute_inline_picker_area(&mut backend, inline_height).map_err(|err| {
                        anyhow!("--inline_picker requires an interactive terminal session ({err})")
                    })?;
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

            let mut app = App::new(
                tries_dir.clone(),
                theme,
                editor_cmd.clone(),
                config_path.clone(),
                apply_date_prefix,
                date_prefix_format.clone(),
                transparent_background.unwrap_or(true),
                query,
                tries_dirs.clone(),
                active_tab,
                show_disk,
            );
            app.show_preview = show_preview;
            app.show_legend = show_legend;
            app.right_panel_visible = show_right_panel;
            app.right_panel_width = right_panel_width;
            app.name_separator = config_name_separator;
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
            terminal_guard.active = false;

            let (result_selection, result_editor, result_tab) = res?;
            selection_result = result_selection;
            open_editor = result_editor;
            selected_dir = tries_dirs[result_tab].clone();
        }
    }

    match selection_result {
        SelectionResult::Folder(selection) => {
            let target_path = selected_dir.join(&selection);
            print_cd_or_editor(&target_path, open_editor, &editor_cmd);
        }
        SelectionResult::New(selection) => {
            if utils::is_git_url(&selection) {
                handle_clone(
                    &selection,
                    destination.clone(),
                    cli.full_clone,
                    &selected_dir,
                    apply_date_prefix,
                    date_prefix_format.as_deref(),
                    &name_separator,
                    open_editor,
                    &editor_cmd,
                )?;
            } else {
                handle_new_folder(
                    &selection,
                    &selected_dir,
                    apply_date_prefix,
                    date_prefix_format.as_deref(),
                    &name_separator,
                    open_editor,
                    &editor_cmd,
                )?;
            }
        }
        SelectionResult::None => {}
    }

    Ok(())
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn quotes_powershell_and_nushell_paths() {
        let path = std::path::Path::new("O'Brien; $x '# end");
        assert_eq!(
            quote_shell_path(path, "powershell"),
            "'O''Brien; $x ''# end'"
        );
        assert_eq!(quote_shell_path(path, "nu"), "r##'O'Brien; $x '# end'##");
    }

    #[test]
    fn installed_powershell_and_nushell_keep_editor_arguments_literal() {
        let temporary = tempdir::TempDir::new("try-shell-literals").unwrap();
        let value = "O'Brien; $(touch INJECTED) '# '## [test] \\ end\nnext";
        for (program, shell) in [
            ("pwsh", "powershell"),
            ("powershell", "powershell"),
            ("nu", "nu"),
        ] {
            let available = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
                .any(|directory| {
                    directory.join(program).is_file()
                        || cfg!(windows) && directory.join(format!("{program}.exe")).is_file()
                });
            if !available {
                continue;
            }
            let quoted = quote_shell_path(std::path::Path::new(value), shell);
            let mut command = std::process::Command::new(program);
            if shell == "powershell" {
                command
                    .args(["-NoLogo", "-NoProfile", "-NonInteractive", "-Command"])
                    .arg(format!("[Console]::Write({quoted})"));
            } else {
                command
                    .args(["--no-config-file", "-c"])
                    .arg(format!("print --no-newline {quoted}"));
            }
            let output = command
                .current_dir(temporary.path())
                .env("XDG_DATA_HOME", temporary.path().join("data"))
                .env("XDG_CACHE_HOME", temporary.path().join("cache"))
                .env("XDG_CONFIG_HOME", temporary.path().join("config"))
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{program}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                value,
                "{program}"
            );
            assert!(!temporary.path().join("INJECTED").exists());
        }
    }

    #[cfg(unix)]
    #[test]
    fn terminal_guard_restores_real_terminal_after_error() {
        use std::os::{
            fd::{AsRawFd, FromRawFd},
            unix::process::CommandExt,
        };
        if std::env::var_os("TRY_RS_TEST_PTY_CHILD").is_some() {
            let flags = || unsafe {
                let mut value = std::mem::zeroed();
                assert_eq!(libc::tcgetattr(0, &mut value), 0);
                let value: libc::termios = value;
                (value.c_iflag, value.c_oflag, value.c_cflag, value.c_lflag)
            };
            let before = flags();
            let fail = || -> Result<()> {
                enable_raw_mode()?;
                let _guard = TerminalGuard {
                    alternate: true,
                    active: true,
                };
                assert_ne!(before, flags());
                bail!("simulated terminal initialization failure")
            };
            assert!(fail().is_err());
            assert_eq!(before, flags());
            return;
        }
        let mut master = -1;
        let mut slave = -1;
        unsafe {
            assert_eq!(
                libc::openpty(
                    &mut master,
                    &mut slave,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    std::ptr::null()
                ),
                0
            );
        }
        let _master = unsafe { fs::File::from_raw_fd(master) };
        let slave = unsafe { fs::File::from_raw_fd(slave) };
        let fd = slave.as_raw_fd();
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "regression_tests::terminal_guard_restores_real_terminal_after_error",
            ])
            .env("TRY_RS_TEST_PTY_CHILD", "1")
            .stdin(slave.try_clone().unwrap())
            .stderr(slave.try_clone().unwrap());
        unsafe {
            command.pre_exec(move || {
                if libc::setsid() < 0 || libc::ioctl(fd, libc::TIOCSCTTY as _, 0) < 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}
