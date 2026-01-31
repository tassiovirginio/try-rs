use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

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
    let h = Harness::new();
    let p = h.run_try(&["foo"]);

    let output = String::from_utf8(p.stdout).unwrap();

    let expected_dir = h.path().join("foo");
    h.dir.into_path();
    assert_eq!(
        format!("cd '{}'", expected_dir.display()),
        output.trim(),
        "printed command is correct"
    );
    assert!(expected_dir.exists(), "try directory created");
    assert!(expected_dir.is_dir(), "try dir is a directory");
}

struct Harness {
    dir: TempDir,
}

impl Harness {
    fn new() -> Self {
        let dir = TempDir::new("try-test").expect("couldn't generate temp directory for test");
        fs::write(
            dir.path().join("config.toml"),
            format!("tries_path = \"{}\"", dir.path().join("tries").display()),
        )
        .expect("could not create config file");
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
}
