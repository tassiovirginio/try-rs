use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

use chrono::Local;
use tempdir::TempDir;

#[test]
fn shows_help() {
    let p = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--help")
        .output()
        .expect("failed to spawn process");

    let output = String::from_utf8(p.stdout).unwrap();
    let err = String::from_utf8(p.stderr).unwrap();

    assert!(output.is_empty());
    assert!(err.contains("Usage: try-rs"));
}

#[test]
fn new_name() {
    // given
    let h = Harness::new(false);

    // when
    let p = h.run_try(&["foo"]);
    let output = String::from_utf8(p.stdout).unwrap();

    // then
    let expected_dir = h.path().join("foo");
    assert_eq!(
        format!("cd '{}'", expected_dir.display()),
        output.trim(),
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
    let output = String::from_utf8(p.stdout).unwrap();

    // then
    let expected_dir = h.path().join(existing);
    assert_eq!(
        format!("cd '{}'", expected_dir.display()),
        output.trim(),
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
    let output = String::from_utf8(p.stdout).unwrap();

    // then
    let expected_dir = h.path().join(name);
    assert_eq!(
        format!("cd '{}'", expected_dir.display()),
        output.trim(),
        "printed command is correct"
    );
}

#[test]
fn new_name_with_date() {
    // given
    let h = Harness::new(true);
    let name = "existing";

    // when
    let p = h.run_try(&["existing"]);
    let output = String::from_utf8(p.stdout).unwrap();

    // then
    let expected_dir = h.path().join(format!(
        "{} {}",
        Local::now().format("%Y-%m-%d").to_string(),
        name
    ));
    assert_eq!(
        format!("cd '{}'", expected_dir.display()),
        output.trim(),
        "printed command is correct"
    );
}

struct Harness {
    dir: TempDir,
}

impl Harness {
    fn new(with_date_prefix: bool) -> Self {
        let dir = TempDir::new("try-test").expect("couldn't generate temp directory for test");

        let mut config = String::new();
        config.push_str(&format!(
            "tries_path = \"{}\"\n",
            dir.path().join("tries").display()
        ));
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
            .env("TRY_CONFIG_DIR", self.dir.path())
            .output()
            .expect("failed to spawn process")
    }

    fn path(&self) -> PathBuf {
        self.dir.path().join("tries").to_path_buf()
    }

    fn create_try_folder(&self, name: &str) {
        fs::DirBuilder::new()
            .recursive(true)
            .create(self.path().join(name))
            .expect("couldn't create 'existing' try");
    }
}
