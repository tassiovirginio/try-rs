use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use chrono::Local;
use tempdir::TempDir;

fn cd_command(path: &Path) -> String {
    if cfg!(windows) {
        format!(
            "Set-Location -LiteralPath '{}'",
            path.display().to_string().replace('\'', "''")
        )
    } else {
        format!("cd '{}'", path.display())
    }
}

fn listed_worktrees(output: &str) -> Vec<PathBuf> {
    output
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(PathBuf::from)
        .map(|path| path.canonicalize().unwrap())
        .collect()
}

fn path_config(path: &Path) -> String {
    toml::to_string(&std::collections::BTreeMap::from([(
        "tries_path",
        path.to_string_lossy().into_owned(),
    )]))
    .unwrap()
}

#[test]
fn shows_help() {
    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--help")
        .output()
        .expect("failed to spawn process");

    let output = String::from_utf8(p.stdout).unwrap();

    assert!(output.contains("Usage: try-rs"));
}

#[test]
fn shows_version() {
    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--version")
        .output()
        .expect("failed to spawn process");

    let stdout = String::from_utf8(p.stdout).unwrap();
    let stderr = String::from_utf8(p.stderr).unwrap();

    assert!(
        stdout.contains("try-rs"),
        "version output should go to stdout"
    );
    assert!(
        !stderr.contains("try-rs 0."),
        "version should not appear on stderr"
    );
}

#[test]
fn invalid_shell_flag() {
    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--shell")
        .arg("invalid_shell")
        .output()
        .expect("failed to spawn process");

    assert!(
        !p.status.success(),
        "invalid shell should exit with non-zero"
    );
}

#[test]
fn new_name() {
    // given
    let h = Harness::new(false);

    // when
    let p = h.run_try(&["foo"]);

    // then
    let expected_dir = h.tries_path().join("foo");
    assert_eq!(
        cd_command(&expected_dir),
        p.stdout.trim(),
        "printed command is correct"
    );
    assert!(expected_dir.exists(), "try directory created");
    assert!(expected_dir.is_dir(), "try dir is a directory");
}

#[test]
fn exact_folder_exists() {
    // given
    let h = Harness::new(false);
    let existing = "existing";
    h.create_try_folder(existing);

    // when
    let p = h.run_try(&[existing]);

    // then
    let expected_dir = h.tries_path().join(existing);
    assert_eq!(
        cd_command(&expected_dir),
        p.stdout.trim(),
        "printed command is correct"
    );
}

#[test]
fn folder_with_date_exists() {
    // given
    let h = Harness::new(false);
    let name = "2020-02-02 existing";
    h.create_try_folder(name);

    // when
    let p = h.run_try(&["existing"]);

    // then
    let expected_dir = h.tries_path().join(name);
    assert_eq!(
        cd_command(&expected_dir),
        p.stdout.trim(),
        "printed command is correct"
    );
}

#[test]
fn new_name_with_date() {
    // given
    let h = Harness::new(true);
    let name = "new-name";

    // when
    let p = h.run_try(&["new-name"]);

    // then
    let expected_dir = h
        .tries_path()
        .join(format!("{} {}", Local::now().format("%Y-%m-%d"), name));
    assert_eq!(
        cd_command(&expected_dir),
        p.stdout.trim(),
        "printed command is correct"
    );
}

#[test]
fn checkout_git_with_destination() {
    // given
    let h = Harness::new(false);

    let git_dir = create_git_origin(&h).expect("could not setup git origin");

    // when
    let p = h.run_try(&[&git_dir.display().to_string(), "foo"]);

    // then
    assert!(p.status.success());
    let expected_dir = h.tries_path().join("foo");
    assert_eq!(
        cd_command(&expected_dir),
        p.stdout.trim(),
        "printed command is correct"
    );
    assert!(expected_dir.exists(), "try directory created");
    assert!(expected_dir.is_dir(), "try dir is a directory");
    assert!(
        command(&expected_dir, "git", &["status"])
            .unwrap()
            .status
            .success(),
        "new directory should be a git repo"
    );
}

#[test]
fn checkout_git_without_destination() {
    // given
    let h = Harness::new(false);

    let git_dir = create_git_origin(&h).expect("could not setup git origin");

    // when
    let p = h.run_try(&[&git_dir.display().to_string()]);

    // then
    assert!(p.status.success());
    let expected_dir = h.tries_path().join(EXISTING_GIT_REPO_NAME);
    assert_eq!(
        cd_command(&expected_dir),
        p.stdout.trim(),
        "printed command is correct"
    );
    assert!(expected_dir.exists(), "try directory created");
    assert!(expected_dir.is_dir(), "try dir is a directory");
    assert!(
        command(&expected_dir, "git", &["status"])
            .unwrap()
            .status
            .success(),
        "new directory should be a git repo"
    );
}

#[test]
fn new_worktree() {
    // given
    let h = Harness::new(false);
    let git_dir = create_git_origin(&h).expect("could not setup git origin");

    // when
    let try_exe = std::path::PathBuf::from(env!("CARGO_BIN_EXE_try-rs"));

    let output = Command::new(format!("{}", try_exe.display()))
        .current_dir(git_dir)
        .args(["--worktree", "my-brand-new-feature"])
        .env("SHELL", "")
        .env_remove("TRY_PATH")
        .env("TRY_CONFIG_DIR", h.dir.path())
        .output()
        .map(|output| Output {
            status: output.status,
            stderr: String::from_utf8(output.stderr).expect("couldn't read stderr to string"),
            stdout: String::from_utf8(output.stdout).expect("couldn't read stdout to string"),
        })
        .expect("failed to spawn process");

    // then
    let expected_dir = h.tries_path().join("my-brand-new-feature");
    assert!(output.status.success());
    assert!(output.stdout.contains(&cd_command(&expected_dir)));
    assert!(expected_dir.exists());
    assert!(expected_dir.is_dir());
    assert!(
        output
            .stderr
            .starts_with("Creating worktree 'my-brand-new-feature'")
    );
    let git_status = command(&expected_dir, "git", &["worktree", "list", "--porcelain"]).unwrap();
    assert!(
        git_status.status.success(),
        "new directory should be a git repo"
    );
    assert!(listed_worktrees(&git_status.stdout).contains(&expected_dir.canonicalize().unwrap()));
    assert!(
        git_status
            .stdout
            .lines()
            .any(|line| line == "branch refs/heads/my-brand-new-feature")
    );
}

const EXISTING_GIT_REPO_NAME: &str = "git-origin";

fn create_git_origin(h: &Harness) -> Result<PathBuf, String> {
    let dir = h.dir.path().join(format!("{}.git", EXISTING_GIT_REPO_NAME));
    fs::DirBuilder::new().create(&dir).unwrap();

    let content_file = dir.join("content.txt");
    let content = "this is some file content";
    fs::write(content_file, content).expect("could not create content file");

    command(&dir, "git", &["init"])?;
    command(&dir, "git", &["config", "user.email", "test@test.internal"])?;
    command(&dir, "git", &["config", "user.name", "Test"])?;
    command(&dir, "git", &["add", "."])?;
    command(&dir, "git", &["commit", "-m", "init"])?;

    Ok(dir)
}

fn command(dir: &PathBuf, cmd: &str, args: &[&str]) -> Result<Output, String> {
    let result = Command::new(cmd)
        .current_dir(dir)
        .args(args)
        .output()
        .map(|output| Output {
            status: output.status,
            stderr: String::from_utf8(output.stderr).expect("couldn't read stderr to string"),
            stdout: String::from_utf8(output.stdout).expect("couldn't read stdout to string"),
        });
    let result = match result {
        Ok(ok) => {
            if ok.status.success() {
                Ok(ok)
            } else {
                {
                    #[cfg(unix)]
                    let signal = ok.status.signal();
                    #[cfg(not(unix))]
                    let signal: Option<i32> = None;
                    Err(format!(
                        "command not successful (exit code {:?}, signal {:?}):\nstdout::{}\nstderr:\n{}",
                        ok.status.code(),
                        signal,
                        ok.stdout,
                        ok.stderr,
                    ))
                }
            }
        }
        Err(err) => Err(err.to_string()),
    };
    result.map_err(|err| {
        format!(
            "failed to run '{} {}' in '{}': {}",
            cmd,
            args.join(" "),
            dir.display(),
            err
        )
    })
}

#[derive(Debug)]
struct Output {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

struct Harness {
    dir: TempDir,
}

impl Harness {
    fn new(with_date_prefix: bool) -> Self {
        let dir = TempDir::new("try-test").expect("couldn't generate temp directory for test");

        let mut config = String::new();
        config.push_str(&path_config(&dir.path().join("tries")));
        if with_date_prefix {
            config.push_str("apply_date_prefix = true\n");
        }
        fs::write(dir.path().join("config.toml"), config).expect("could not create config file");

        Harness { dir }
    }

    fn run_try(&self, args: &[&str]) -> Output {
        Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .env("SHELL", "")
            .env_remove("TRY_PATH")
            .env("TRY_CONFIG_DIR", self.dir.path())
            .output()
            .map(|output| Output {
                status: output.status,
                stderr: String::from_utf8(output.stderr).expect("couldn't read stderr to string"),
                stdout: String::from_utf8(output.stdout).expect("couldn't read stdout to string"),
            })
            .expect("failed to spawn process")
    }

    fn tries_path(&self) -> PathBuf {
        self.dir.path().join("tries").to_path_buf()
    }

    fn run_try_with_env(&self, args: &[&str], env_key: &str, env_val: &str) -> Output {
        Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .env_remove("TRY_PATH")
            .env("TRY_CONFIG_DIR", self.dir.path())
            .env(env_key, env_val)
            .output()
            .map(|output| Output {
                status: output.status,
                stderr: String::from_utf8(output.stderr).expect("couldn't read stderr to string"),
                stdout: String::from_utf8(output.stdout).expect("couldn't read stdout to string"),
            })
            .expect("failed to spawn process")
    }

    fn create_try_folder(&self, name: &str) {
        fs::DirBuilder::new()
            .recursive(true)
            .create(self.tries_path().join(name))
            .expect("couldn't create 'existing' try");
    }
}

// ── Additional integration tests ───────────────────────────────────

#[test]
fn new_folder_is_created_on_disk() {
    let h = Harness::new(false);
    let p = h.run_try(&["brand-new"]);
    assert!(p.status.success());
    assert!(h.tries_path().join("brand-new").exists());
    assert!(h.tries_path().join("brand-new").is_dir());
}

#[test]
fn new_name_with_date_creates_dated_folder() {
    let h = Harness::new(true);
    let today = Local::now().format("%Y-%m-%d").to_string();
    let _ = h.run_try(&["dated-test"]);
    let expected = h.tries_path().join(format!("{} dated-test", today));
    assert!(expected.exists(), "dated folder should be created");
}

#[test]
fn multiple_new_folders() {
    let h = Harness::new(false);
    h.run_try(&["proj-a"]);
    h.run_try(&["proj-b"]);
    h.run_try(&["proj-c"]);
    assert!(h.tries_path().join("proj-a").exists());
    assert!(h.tries_path().join("proj-b").exists());
    assert!(h.tries_path().join("proj-c").exists());
}

#[test]
fn help_contains_expected_flags() {
    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--help")
        .output()
        .expect("failed to spawn");

    let output = String::from_utf8(p.stdout).unwrap();
    assert!(output.contains("--setup"), "should document --setup flag");
    assert!(
        output.contains("--setup-stdout"),
        "should document --setup-stdout flag"
    );
    assert!(
        output.contains("--full-clone") || output.contains("-f"),
        "should document full clone"
    );
    assert!(
        output.contains("--worktree") || output.contains("-w"),
        "should document worktree flag"
    );
    assert!(
        output.contains("--inline-picker"),
        "should document inline picker flag"
    );
}

#[test]
fn inline_height_requires_inline_picker() {
    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--inline-height")
        .arg("22")
        .output()
        .expect("failed to spawn");

    assert!(
        !p.status.success(),
        "--inline-height should require --inline-picker"
    );

    let stdout = String::from_utf8(p.stdout).unwrap();
    let stderr = String::from_utf8(p.stderr).unwrap();
    let output = format!("{stdout}\n{stderr}");
    assert!(
        output.contains("--inline-picker"),
        "error should mention missing --inline-picker"
    );
}

#[test]
fn existing_folder_cd_does_not_recreate() {
    let h = Harness::new(false);
    h.create_try_folder("already");

    // Put a marker file inside
    fs::write(h.tries_path().join("already").join("marker.txt"), "test").unwrap();

    let p = h.run_try(&["already"]);

    assert!(
        p.stdout
            .contains(&cd_command(&h.tries_path().join("already")))
    );
    // Marker file should still be there (folder wasn't recreated)
    assert!(h.tries_path().join("already").join("marker.txt").exists());
}

#[test]
fn config_with_editor_field() {
    let dir = TempDir::new("try-editor-test").unwrap();
    let config = path_config(&dir.path().join("tries")) + "editor = \"vim\"\n";
    fs::write(dir.path().join("config.toml"), config).unwrap();

    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("editor-proj")
        .env("TRY_CONFIG_DIR", dir.path())
        .output()
        .expect("failed to spawn");

    // It should create the folder successfully
    assert!(p.status.success() || !String::from_utf8(p.stdout).unwrap().is_empty());
}

#[test]
fn config_with_theme() {
    let dir = TempDir::new("try-theme-test").unwrap();
    let config = path_config(&dir.path().join("tries")) + "theme = \"Dracula\"\n";
    fs::write(dir.path().join("config.toml"), config).unwrap();

    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("themed-proj")
        .env("TRY_CONFIG_DIR", dir.path())
        .output()
        .expect("failed to spawn");

    let output = String::from_utf8(p.stdout).unwrap();
    assert!(
        output.contains(if cfg!(windows) { "Set-Location" } else { "cd" }),
        "should produce navigation command"
    );
}

#[test]
fn config_with_invalid_theme_falls_back() {
    let dir = TempDir::new("try-bad-theme").unwrap();
    let config = path_config(&dir.path().join("tries")) + "theme = \"NonExistentTheme\"\n";
    fs::write(dir.path().join("config.toml"), config).unwrap();

    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("fallback-proj")
        .env("TRY_CONFIG_DIR", dir.path())
        .output()
        .expect("failed to spawn");

    let output = String::from_utf8(p.stdout).unwrap();
    // Should still work, just using default theme
    assert!(
        output.contains(if cfg!(windows) { "Set-Location" } else { "cd" }),
        "should still produce navigation command"
    );
}

#[test]
fn try_path_env_overrides_config() {
    let h = Harness::new(false);
    let override_dir = TempDir::new("try-override-path").unwrap();

    let p = h.run_try_with_env(
        &["env-proj"],
        "TRY_PATH",
        override_dir.path().to_str().unwrap(),
    );

    // The cd should point to the TRY_PATH override dir, not the config's tries_path
    assert!(
        p.stdout
            .contains(&override_dir.path().to_string_lossy().to_string()),
        "TRY_PATH env should override config tries_path"
    );
}

#[test]
fn empty_workspace_configuration_falls_back_without_panicking() {
    const CHILD: &str = "TRY_RS_EMPTY_CONFIG_TEST_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let config = try_rs::config::load_configuration();
        assert_eq!(
            config.tries_dirs,
            vec![dirs::home_dir().unwrap().join("work").join("tries")]
        );
        return;
    }
    for paths in ["", " , , "] {
        let h = Harness::new(false);
        fs::write(
            h.dir.path().join("config.toml"),
            format!("tries_path = {paths:?}\n"),
        )
        .unwrap();
        let p = h.run_try(&["--setup-stdout", "bash"]);
        assert!(p.status.success(), "{}", p.stderr);
        let p = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "empty_workspace_configuration_falls_back_without_panicking",
            ])
            .env(CHILD, "1")
            .env("TRY_CONFIG_DIR", h.dir.path())
            .env_remove("TRY_PATH")
            .output()
            .unwrap();
        assert!(
            p.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&p.stdout),
            String::from_utf8_lossy(&p.stderr)
        );
    }
}

#[test]
fn failed_clone_returns_failure_and_no_shell_command() {
    let h = Harness::new(false);
    let missing = h.dir.path().join("missing.git");
    let p = h.run_try(&[missing.to_str().unwrap()]);
    assert!(!p.status.success());
    assert!(p.stdout.is_empty());
    assert!(p.stderr.contains("Failed to clone"));
}

#[cfg(unix)]
#[test]
fn wrappers_navigate_with_flags_and_treat_path_as_data() {
    for shell in ["bash", "zsh", "fish"] {
        if Command::new(shell).arg("--version").output().is_err() {
            continue;
        }
        let h = Harness::new(false);
        let bin_dir = h.dir.path().join("bin");
        fs::create_dir(&bin_dir).unwrap();
        std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_try-rs"), bin_dir.join("try-rs")).unwrap();
        let script = h.run_try(&["--setup-stdout", shell]);
        let init = h.dir.path().join("init");
        fs::write(&init, script.stdout).unwrap();
        let name = "O'Brien; $(touch INJECTED) \\ test\nnext";
        // Pass the name via an environment variable, never shell source.
        let code = format!(
            "source '{}'; try-rs --hide-disk \"$TRY_TEST_NAME\"; pwd",
            init.display()
        );
        let result = Command::new(shell)
            .args(match shell {
                "fish" => vec!["--no-config"],
                "zsh" => vec!["-f"],
                _ => vec!["--noprofile", "--norc"],
            })
            .arg("-c")
            .arg(code)
            .current_dir(h.dir.path())
            .env("TRY_TEST_NAME", name)
            .env("TRY_CONFIG_DIR", h.dir.path())
            .env_remove("TRY_PATH")
            .env(
                "PATH",
                format!("{}:{}", bin_dir.display(), std::env::var("PATH").unwrap()),
            )
            .env("SHELL", shell)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&result.stdout).trim(),
            h.tries_path().join(name).to_str().unwrap(),
            "{shell}"
        );
        assert!(!h.dir.path().join("INJECTED").exists());
        let result = Command::new(shell)
            .args(match shell {
                "fish" => vec!["--no-config"],
                "zsh" => vec!["-f"],
                _ => vec!["--noprofile", "--norc"],
            })
            .args([
                "-c",
                &format!(
                    "source '{}'; try-rs /nonexistent/review-missing.git",
                    init.display()
                ),
            ])
            .env("TRY_CONFIG_DIR", h.dir.path())
            .env_remove("TRY_PATH")
            .env(
                "PATH",
                format!("{}:{}", bin_dir.display(), std::env::var("PATH").unwrap()),
            )
            .output()
            .unwrap();
        assert!(!result.status.success(), "{shell} must preserve failures");
        assert!(result.stdout.is_empty());
    }
}

#[test]
fn moving_and_renaming_worktrees_preserves_git_registration() {
    let h = Harness::new(false);
    let origin = create_git_origin(&h).unwrap();
    let initial = h.dir.path().join("original");
    command(
        &origin,
        "git",
        &[
            "worktree",
            "add",
            "-b",
            "feature",
            initial.to_str().unwrap(),
        ],
    )
    .unwrap();
    let renamed = h.dir.path().join("renamed");
    let mut app = try_rs::tui::App::new(
        h.dir.path().to_path_buf(),
        try_rs::tui::Theme::default(),
        None,
        None,
        None,
        None,
        true,
        Some("original".into()),
        vec![h.dir.path().to_path_buf()],
        0,
        false,
    );
    app.rename_input = "renamed".into();
    app.rename_selected();
    assert!(renamed.exists(), "{:?}", app.status_message);
    let destination = h.dir.path().join("other-workspace");
    fs::create_dir(&destination).unwrap();
    let moved = destination.join("renamed");
    try_rs::utils::move_workspace(&renamed, &moved).unwrap();
    command(&moved, "git", &["status", "--porcelain"]).unwrap();
    let listing = command(&origin, "git", &["worktree", "list", "--porcelain"]).unwrap();
    let paths = listed_worktrees(&listing.stdout);
    assert!(paths.contains(&moved.canonicalize().unwrap()));
    assert_eq!(paths.len(), 2);
    command(
        &origin,
        "git",
        &["worktree", "lock", moved.to_str().unwrap()],
    )
    .unwrap();
    assert!(try_rs::utils::move_workspace(&moved, &initial).is_err());
    assert!(moved.exists());
    command(
        &origin,
        "git",
        &["worktree", "unlock", moved.to_str().unwrap()],
    )
    .unwrap();
    let mut app = try_rs::tui::App::new(
        destination.clone(),
        try_rs::tui::Theme::default(),
        None,
        None,
        None,
        None,
        true,
        Some("renamed".into()),
        vec![destination],
        0,
        false,
    );
    app.delete_selected();
    assert!(!moved.exists(), "{:?}", app.status_message);
}

#[cfg(unix)]
#[test]
fn bash_wrapper_enters_created_worktree_and_preserves_failures() {
    let h = Harness::new(false);
    let origin = create_git_origin(&h).unwrap();
    let bin_dir = h.dir.path().join("bin");
    fs::create_dir(&bin_dir).unwrap();
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_try-rs"), bin_dir.join("try-rs")).unwrap();
    let init = h.dir.path().join("init");
    fs::write(&init, h.run_try(&["--setup-stdout", "bash"]).stdout).unwrap();
    let result = Command::new("bash").args(["--noprofile", "--norc", "-c"])
        .arg(format!("source '{}'; try-rs -w feature && pwd; try-rs /nonexistent/review-missing.git; exit $?", init.display()))
        .current_dir(origin).env("TRY_CONFIG_DIR", h.dir.path()).env_remove("TRY_PATH")
        .env("PATH", format!("{}:{}", bin_dir.display(), std::env::var("PATH").unwrap()))
        .output().unwrap();
    assert!(!result.status.success());
    assert_eq!(
        String::from_utf8_lossy(&result.stdout).trim(),
        h.tries_path().join("feature").to_str().unwrap()
    );
}

#[test]
fn powershell_and_nushell_wrappers_navigate_and_report_failures() {
    for (shell, setup) in [
        ("pwsh", "power-shell"),
        ("powershell", "power-shell"),
        ("nu", "nu-shell"),
    ] {
        let available =
            std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()).any(|directory| {
                directory.join(shell).is_file()
                    || cfg!(windows) && directory.join(format!("{shell}.exe")).is_file()
            });
        if !available {
            continue;
        }
        let h = Harness::new(false);
        let origin = create_git_origin(&h).unwrap();
        let bin_dir = h.dir.path().join("bin");
        fs::create_dir(&bin_dir).unwrap();
        fs::copy(
            env!("CARGO_BIN_EXE_try-rs"),
            bin_dir.join(if cfg!(windows) {
                "try-rs.exe"
            } else {
                "try-rs"
            }),
        )
        .unwrap();
        let init = h
            .dir
            .path()
            .join(if shell != "nu" { "init.ps1" } else { "init.nu" });
        fs::write(&init, h.run_try(&["--setup-stdout", setup]).stdout).unwrap();
        let name = if cfg!(windows) {
            "ação 实验 O'Brien; $(touch INJECTED) '# [test]"
        } else {
            "ação 实验 O'Brien; $(touch INJECTED) '# [test]\nnext"
        };
        let prefix = if shell != "nu" {
            format!(". '{}'", init.display())
        } else {
            format!("source '{}'", init.display())
        };
        let run = |code: &str| {
            let mut cmd = Command::new(shell);
            #[cfg(windows)]
            if shell == "powershell" {
                use std::os::windows::process::CommandExt;
                // Give legacy PowerShell its own console so code-page changes
                // cannot affect other tests running in parallel.
                cmd.creation_flags(0x00000010);
            }
            cmd.args(if shell != "nu" {
                vec![
                    "-NoLogo",
                    "-NoProfile",
                    "-NonInteractive",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-Command",
                ]
            } else {
                vec!["--no-config-file", "-c"]
            });
            let encoding = if shell == "powershell" {
                "[Console]::OutputEncoding = [Text.Encoding]::GetEncoding(437);"
            } else {
                ""
            };
            cmd.arg(format!("{prefix}; {encoding} {code}"))
                .current_dir(&origin)
                .env("TRY_TEST_NAME", name)
                .env("TRY_CONFIG_DIR", h.dir.path())
                .env_remove("TRY_PATH")
                .env("XDG_DATA_HOME", h.dir.path().join("data"))
                .env("XDG_CACHE_HOME", h.dir.path().join("cache"))
                .env("XDG_CONFIG_HOME", h.dir.path().join("config"))
                .env(
                    "PATH",
                    std::env::join_paths(std::iter::once(bin_dir.clone()).chain(
                        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()),
                    ))
                    .unwrap(),
                );
            cmd.output().unwrap()
        };
        let result = run(if shell != "nu" {
            "try-rs --hide-disk $env:TRY_TEST_NAME; if ($PSVersionTable.PSVersion.Major -eq 5 -and [Console]::OutputEncoding.CodePage -ne 437) { throw 'Console encoding was not restored' }; [Console]::OutputEncoding = [Text.UTF8Encoding]::new($false); [Console]::Write((Get-Location).Path)"
        } else {
            "try-rs --hide-disk $env.TRY_TEST_NAME; print --no-newline $env.PWD"
        });
        assert!(
            result.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&result.stdout),
            h.tries_path().join(name).to_str().unwrap(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(!origin.join("INJECTED").exists());
        let result = run(if shell != "nu" {
            "try-rs -w feature; [Console]::Write((Get-Location).Path)"
        } else {
            "try-rs -w feature; print --no-newline $env.PWD"
        });
        assert!(
            result.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&result.stdout),
            h.tries_path().join("feature").to_str().unwrap(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let result = run(if shell != "nu" {
            "try-rs /nonexistent/review-missing.git; exit $LASTEXITCODE"
        } else {
            "try-rs /nonexistent/review-missing.git; exit $env.LAST_EXIT_CODE"
        });
        assert!(
            !result.status.success(),
            "{shell} must report clone failure"
        );
        assert!(
            result.stdout.is_empty(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stdout)
        );
        let result = run("try-rs --help");
        assert!(
            result.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(String::from_utf8_lossy(&result.stdout).contains("Usage: try-rs"));
    }
}
