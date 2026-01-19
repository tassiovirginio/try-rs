use crate::config::{get_base_config_dir, get_config_dir};
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ShellType {
    Fish,
    Zsh,
    Bash,
    PowerShell,
    NuShell,
}

pub fn get_shell_integration_path(shell: &ShellType) -> PathBuf {
    let config_dir = match shell {
        ShellType::Fish => get_base_config_dir(),
        _ => get_config_dir(),
    };

    match shell {
        ShellType::Fish => config_dir.join("fish").join("functions").join("try-rs.fish"),
        ShellType::Zsh => config_dir.join("try-rs.zsh"),
        ShellType::Bash => config_dir.join("try-rs.bash"),
        ShellType::PowerShell => config_dir.join("try-rs.ps1"),
        ShellType::NuShell => config_dir.join("try-rs.nu"),
    }
}

pub fn is_shell_integration_configured(shell: &ShellType) -> bool {
    get_shell_integration_path(shell).exists()
}

pub fn setup_fish() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    });

    let fish_functions_dir = config_dir.join("fish").join("functions");

    if !fish_functions_dir.exists() {
        fs::create_dir_all(&fish_functions_dir)?;
    }

    let file_path = fish_functions_dir.join("try-rs.fish");
    let content = r#"function try-rs
    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    set command (command try-rs $argv | string collect)

    if test -n "$command"
        eval $command
    end
end
"#;

    fs::write(&file_path, content)?;
    eprintln!("Fish function created at: {}", file_path.display());
    eprintln!(
        "You may need to restart your shell or run 'source {}' to apply changes.",
        file_path.display()
    );

    Ok(())
}

pub fn setup_zsh() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    });

    let app_config_dir = config_dir.join("try-rs");

    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let file_path = app_config_dir.join("try-rs.zsh");
    let content = r#"try-rs() {
    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    local output
    output=$(command try-rs "$@")

    if [ -n "$output" ]; then
        eval "$output"
    fi
}
"#;

    fs::write(&file_path, content)?;
    eprintln!("ZSH function file created at: {}", file_path.display());

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let zshrc_path = home_dir.join(".zshrc");
    let source_cmd = format!("source '{}'", file_path.display());

    if zshrc_path.exists() {
        let zshrc_content = fs::read_to_string(&zshrc_path)?;
        if !zshrc_content.contains(&source_cmd) {
            use std::io::Write;
            let mut file = fs::OpenOptions::new().append(true).open(&zshrc_path)?;
            writeln!(file, "\n# try-rs integration")?;
            writeln!(file, "{}", source_cmd)?;
            eprintln!("Added configuration to ~/.zshrc");
        } else {
            eprintln!("Configuration already present in ~/.zshrc");
        }
    } else {
        eprintln!("You need to source this file in your ~/.zshrc:");
        eprintln!("{}", source_cmd);
    }

    Ok(())
}

pub fn setup_bash() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    });

    let app_config_dir = config_dir.join("try-rs");

    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let file_path = app_config_dir.join("try-rs.bash");
    let content = r#"try-rs() {
    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    local output
    output=$(command try-rs "$@")

    if [ -n "$output" ]; then
        eval "$output"
    fi
}
"#;

    fs::write(&file_path, content)?;
    eprintln!("Bash function file created at: {}", file_path.display());

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let bashrc_path = home_dir.join(".bashrc");
    let source_cmd = format!("source '{}'", file_path.display());

    if bashrc_path.exists() {
        let bashrc_content = fs::read_to_string(&bashrc_path)?;
        if !bashrc_content.contains(&source_cmd) {
            use std::io::Write;
            let mut file = fs::OpenOptions::new().append(true).open(&bashrc_path)?;
            writeln!(file, "\n# try-rs integration")?;
            writeln!(file, "{}", source_cmd)?;
            eprintln!("Added configuration to ~/.bashrc");
        } else {
            eprintln!("Configuration already present in ~/.bashrc");
        }
    } else {
        eprintln!("You need to source this file in your ~/.bashrc:");
        eprintln!("{}", source_cmd);
    }

    Ok(())
}

pub fn setup_powershell() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
    });
    let app_config_dir = config_dir.join("try-rs");
    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let file_path = app_config_dir.join("try-rs.ps1");
    let content = r#"
# try-rs integration for PowerShell
function try-rs {
    # Captures the output of the binary (stdout) which is the "cd" or editor command
    # The TUI is rendered on stderr, so it doesn't interfere.
    $command = (try-rs.exe @args)

    if ($command) {
        Invoke-Expression $command
    }
}
"#;
    fs::write(&file_path, content.trim())?;
    eprintln!(
        "PowerShell function file created at: {}",
        file_path.display()
    );

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let profile_path_ps7 = home_dir
        .join("Documents")
        .join("PowerShell")
        .join("Microsoft.PowerShell_profile.ps1");
    let profile_path_ps5 = home_dir
        .join("Documents")
        .join("WindowsPowerShell")
        .join("Microsoft.PowerShell_profile.ps1");

    let profile_path = if profile_path_ps7.exists() {
        profile_path_ps7
    } else if profile_path_ps5.exists() {
        profile_path_ps5
    } else {
        profile_path_ps7
    };

    if let Some(parent) = profile_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }

    let source_cmd = format!(". '{}'", file_path.display());

    if profile_path.exists() {
        let profile_content = fs::read_to_string(&profile_path)?;
        if !profile_content.contains(&source_cmd) {
            use std::io::Write;
            let mut file = fs::OpenOptions::new().append(true).open(&profile_path)?;
            writeln!(file, "\n# try-rs integration")?;
            writeln!(file, "{}", source_cmd)?;
            eprintln!("Added configuration to {}", profile_path.display());
        } else {
            eprintln!(
                "Configuration already present in {}",
                profile_path.display()
            );
        }
    } else {
        let mut file = fs::File::create(&profile_path)?;
        writeln!(file, "# try-rs integration")?;
        writeln!(file, "{}", source_cmd)?;
        eprintln!(
            "PowerShell profile created and configured at: {}",
            profile_path.display()
        );
    }

    eprintln!(
        "You may need to restart your shell or run '. {}' to apply changes.",
        profile_path.display()
    );
    eprintln!(
        "If you get an error about running scripts, you may need to run: Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned"
    );

    Ok(())
}

pub fn setup_nushell() -> Result<()> {
    let config_dir = dirs::config_dir()
        .expect("Could not find config directory")
        .join("nushell");

    let app_config_dir = dirs::config_dir()
        .expect("Could not find config directory")
        .join("try-rs");

    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let file_path = app_config_dir.join("try-rs.nu");
    let content = r#"def --wrapped try-rs [...args] {
    # Capture output. Stderr (TUI) goes directly to terminal.
    let output = (try-rs.exe ...$args)

    if ($output | is-not-empty) {

        # Grabs the path out of stdout returned by the binary and removes the single quotes
        let $path = ($output | split row ' ').1 | str replace --all "\'" ''
        cd $path
    }
}
"#;

    fs::write(&file_path, content)?;
    eprintln!("Nushell function created at: {}", file_path.display());

    let nu_config_path = config_dir.join("config.nu");
    let source_cmd = format!("source '{}'", file_path.display());

    if nu_config_path.exists() {
        let nu_content = fs::read_to_string(&nu_config_path)?;
        if !nu_content.contains(&source_cmd) {
            use std::io::Write;
            let mut file = fs::OpenOptions::new().append(true).open(&nu_config_path)?;
            writeln!(file, "\n# try-rs integration")?;
            writeln!(file, "{}", source_cmd)?;
            eprintln!("Added configuration to {}", nu_config_path.display());
        } else {
            eprintln!(
                "Configuration already present in {}",
                nu_config_path.display()
            );
        }
    } else {
        eprintln!("Could not find config.nu at {}", nu_config_path.display());
        eprintln!("Please add the following line manually:");
        eprintln!("{}", source_cmd);
    }

    Ok(())
}
