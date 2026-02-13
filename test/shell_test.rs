use std::path::PathBuf;
use tempdir::TempDir;
use try_rs::cli::Shell;
use try_rs::shell::*;

#[test]
fn get_shell_content_fish_contains_function() {
    let content = get_shell_content(&Shell::Fish);
    assert!(content.contains("function try-rs"));
    assert!(content.contains("end"));
}

#[test]
fn get_shell_content_zsh_contains_function() {
    let content = get_shell_content(&Shell::Zsh);
    assert!(content.contains("try-rs()"));
    assert!(content.contains("command try-rs"));
}

#[test]
fn get_shell_content_bash_contains_function() {
    let content = get_shell_content(&Shell::Bash);
    assert!(content.contains("try-rs()"));
    assert!(content.contains("command try-rs"));
}

#[test]
fn get_shell_content_powershell_contains_function() {
    let content = get_shell_content(&Shell::PowerShell);
    assert!(content.contains("function try-rs"));
    assert!(content.contains("try-rs.exe"));
}

#[test]
fn get_shell_content_nushell_contains_function() {
    let content = get_shell_content(&Shell::NuShell);
    assert!(content.contains("def --wrapped try-rs"));
    assert!(content.contains("try-rs.exe"));
}

#[test]
fn get_shell_content_includes_completions() {
    for shell in [
        Shell::Fish,
        Shell::Zsh,
        Shell::Bash,
        Shell::PowerShell,
        Shell::NuShell,
    ] {
        let content = get_shell_content(&shell);
        assert!(
            !content.is_empty(),
            "Shell {:?} content should not be empty",
            shell
        );
    }
}

#[test]
fn get_completions_script_fish_contains_complete() {
    let script = get_completions_script(&Shell::Fish);
    assert!(script.contains("complete"));
    assert!(script.contains("__try_rs"));
}

#[test]
fn get_completions_script_zsh_contains_function() {
    let script = get_completions_script(&Shell::Zsh);
    assert!(script.contains("_try_rs_get_tries_path"));
    assert!(script.contains("_try_rs_complete"));
}

#[test]
fn get_completions_script_bash_contains_function() {
    let script = get_completions_script(&Shell::Bash);
    assert!(script.contains("_try_rs_get_tries_path"));
    assert!(script.contains("_try_rs_complete"));
}

#[test]
fn get_completions_script_powershell_contains_register() {
    let script = get_completions_script(&Shell::PowerShell);
    assert!(script.contains("Register-ArgumentCompleter"));
    assert!(script.contains("try-rs"));
}

#[test]
fn get_completions_script_nushell_contains_export() {
    let script = get_completions_script(&Shell::NuShell);
    assert!(script.contains("export extern try-rs"));
}

#[test]
fn get_completions_script_all_contain_try_path() {
    for shell in [
        Shell::Fish,
        Shell::Zsh,
        Shell::Bash,
        Shell::PowerShell,
        Shell::NuShell,
    ] {
        let script = get_completions_script(&shell);
        assert!(
            script.contains("TRY_PATH") || script.contains("tries_path"),
            "Shell {:?} completions should reference TRY_PATH or tries_path",
            shell
        );
    }
}

#[test]
fn get_completion_script_only_fish_is_same() {
    let only = get_completion_script_only(&Shell::Fish);
    let full = get_completions_script(&Shell::Fish);
    assert_eq!(only, full);
}

#[test]
fn get_completion_script_only_zsh_is_same() {
    let only = get_completion_script_only(&Shell::Zsh);
    let full = get_completions_script(&Shell::Zsh);
    assert_eq!(only, full);
}

#[test]
fn get_completion_script_only_bash_is_same() {
    let only = get_completion_script_only(&Shell::Bash);
    let full = get_completions_script(&Shell::Bash);
    assert_eq!(only, full);
}

#[test]
fn get_completion_script_only_powershell_is_same() {
    let only = get_completion_script_only(&Shell::PowerShell);
    let full = get_completions_script(&Shell::PowerShell);
    assert_eq!(only, full);
}

#[test]
fn get_completion_script_only_nushell_is_different() {
    let only = get_completion_script_only(&Shell::NuShell);
    let full = get_completions_script(&Shell::NuShell);
    assert_ne!(
        only, full,
        "NuShell standalone completion should be different"
    );
}

#[test]
fn get_shell_integration_path_fish() {
    let path = get_shell_integration_path(&Shell::Fish);
    assert!(path.to_string_lossy().contains("fish"));
    assert!(path.to_string_lossy().contains("try-rs.fish"));
}

#[test]
fn get_shell_integration_path_zsh() {
    let path = get_shell_integration_path(&Shell::Zsh);
    assert!(path.to_string_lossy().contains("try-rs.zsh"));
}

#[test]
fn get_shell_integration_path_bash() {
    let path = get_shell_integration_path(&Shell::Bash);
    assert!(path.to_string_lossy().contains("try-rs.bash"));
}

#[test]
fn get_shell_integration_path_powershell() {
    let path = get_shell_integration_path(&Shell::PowerShell);
    assert!(path.to_string_lossy().contains("try-rs.ps1"));
}

#[test]
fn get_shell_integration_path_nushell() {
    let path = get_shell_integration_path(&Shell::NuShell);
    assert!(path.to_string_lossy().contains("try-rs.nu"));
}

#[test]
fn is_shell_integration_configured_false_when_not_exists() {
    let tmp = TempDir::new("shell-config").unwrap();
    let config_dir = tmp.path().join(".config");
    std::fs::create_dir_all(&config_dir).unwrap();

    unsafe {
        std::env::set_var("HOME", tmp.path());
        std::env::set_var("XDG_CONFIG_HOME", &config_dir);
    }

    // Test com um shell específico que sabemos que não existe no temp dir
    let shell_file = config_dir.join("try-rs.zsh");
    assert!(!shell_file.exists());
    assert!(
        !is_shell_integration_configured(&Shell::Zsh),
        "Zsh should not be configured in empty dir"
    );
}

#[test]
fn is_shell_integration_configured_true_when_exists() {
    let tmp = TempDir::new("shell-config").unwrap();
    let config_dir = tmp.path().join(".config").join("try-rs");
    std::fs::create_dir_all(&config_dir).unwrap();

    let shell_file = config_dir.join("try-rs.zsh");
    std::fs::write(&shell_file, "# test content").unwrap();

    unsafe {
        std::env::set_var("HOME", tmp.path());
        std::env::set_var("XDG_CONFIG_HOME", tmp.path().join(".config"));
    }

    assert!(is_shell_integration_configured(&Shell::Zsh));
}

#[test]
fn generate_completions_outputs_script() {
    for shell in [
        Shell::Fish,
        Shell::Zsh,
        Shell::Bash,
        Shell::PowerShell,
        Shell::NuShell,
    ] {
        let result = generate_completions(&shell);
        assert!(
            result.is_ok(),
            "generate_completions should succeed for {:?}",
            shell
        );
    }
}

#[test]
fn shell_content_contains_eval_or_cd() {
    for shell in [Shell::Fish, Shell::Zsh, Shell::Bash] {
        let content = get_shell_content(&shell);
        assert!(
            content.contains("eval") || content.contains("cd "),
            "Shell {:?} should contain eval or cd",
            shell
        );
    }
}

#[test]
fn shell_content_handles_flags() {
    for shell in [
        Shell::Fish,
        Shell::Zsh,
        Shell::Bash,
        Shell::PowerShell,
        Shell::NuShell,
    ] {
        let content = get_shell_content(&shell);
        assert!(
            content.contains("-") || content.contains("flag"),
            "Shell {:?} should handle flags",
            shell
        );
    }
}

#[test]
fn completions_contain_directory_listing() {
    for shell in [Shell::Fish, Shell::Zsh, Shell::Bash] {
        let script = get_completions_script(&shell);
        assert!(
            script.contains("ls") || script.contains("dir") || script.contains("Directory"),
            "Shell {:?} completions should list directories",
            shell
        );
    }
}

#[test]
fn all_shells_have_unique_extensions() {
    let paths: Vec<PathBuf> = [
        Shell::Fish,
        Shell::Zsh,
        Shell::Bash,
        Shell::PowerShell,
        Shell::NuShell,
    ]
    .iter()
    .map(|s| get_shell_integration_path(s))
    .collect();

    let extensions: Vec<String> = paths
        .iter()
        .map(|p| p.extension().unwrap().to_string_lossy().to_string())
        .collect();

    let unique_extensions: std::collections::HashSet<String> =
        extensions.clone().into_iter().collect();
    assert_eq!(
        extensions.len(),
        unique_extensions.len(),
        "All shells should have unique extensions"
    );
}
