use crate::cli::Shell;
use crate::config::{get_base_config_dir, get_config_dir};
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const FISH_PICKER_FUNCTION: &str = r#"function try-rs-picker
    set -l picker_args --inline-picker

    if set -q TRY_RS_PICKER_HEIGHT
        if string match -qr '^[0-9]+$' -- "$TRY_RS_PICKER_HEIGHT"
            set picker_args $picker_args --inline-height $TRY_RS_PICKER_HEIGHT
        end
    end

    if status --is-interactive
        printf "\n"
    end

    set command (command try-rs $picker_args | string collect)
    set command_status $status

    if test $command_status -eq 0; and test -n "$command"
        eval $command
    end

    if status --is-interactive
        printf "\033[A"
        commandline -f repaint
    end
end
"#;

/// Returns the shell integration script content for the given shell type.
/// This is used by --setup-stdout to print the content to stdout.
pub fn get_shell_content(shell: &Shell) -> String {
    let completions = get_completions_script(shell);
    match shell {
        Shell::Fish => {
            format!(
                r#"function try-rs
    # Pass flags/options directly to stdout without capturing
    for arg in $argv
        if string match -q -- '-*' $arg
            command try-rs $argv
            return
        end
    end

    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    set command (command try-rs $argv | string collect)
    set command_status $status

    if test $command_status -eq 0; and test -n "$command"
        eval $command
    end
end

{picker_function}

{completions}"#,
                picker_function = FISH_PICKER_FUNCTION,
            )
        }
        Shell::Zsh => {
            format!(
                r#"try-rs() {{
    # Pass flags/options directly to stdout without capturing
    for arg in "$@"; do
        case "$arg" in
            -*) command try-rs "$@"; return ;;
        esac
    done

    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    local output
    output=$(command try-rs "$@")

    if [ -n "$output" ]; then
        eval "$output"
    fi
}}

{completions}"#
            )
        }
        Shell::Bash => {
            format!(
                r#"try-rs() {{
    # Pass flags/options directly to stdout without capturing
    for arg in "$@"; do
        case "$arg" in
            -*) command try-rs "$@"; return ;;
        esac
    done

    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    local output
    output=$(command try-rs "$@")

    if [ -n "$output" ]; then
        eval "$output"
    fi
}}

{completions}"#
            )
        }
        Shell::PowerShell => {
            format!(
                r#"# try-rs integration for PowerShell
function try-rs {{
    # Pass flags/options directly to stdout without capturing
    foreach ($a in $args) {{
        if ($a -like '-*') {{
            & try-rs.exe @args
            return
        }}
    }}

    # Captures the output of the binary (stdout) which is the "cd" or editor command
    # The TUI is rendered on stderr, so it doesn't interfere.
    $command = (try-rs.exe @args)

    if ($command) {{
        Invoke-Expression $command
    }}
}}

{completions}"#
            )
        }
        Shell::NuShell => {
            format!(
                r#"def --wrapped try-rs [...args] {{
    # Pass flags/options directly to stdout without capturing
    for arg in $args {{
        if ($arg | str starts-with '-') {{
            ^try-rs.exe ...$args
            return
        }}
    }}

    # Capture output. Stderr (TUI) goes directly to terminal.
    let output = (try-rs.exe ...$args)

    if ($output | is-not-empty) {{

        # Grabs the path out of stdout returned by the binary and removes the single quotes
        let $path = ($output | split row ' ').1 | str replace --all "'" ''
        cd $path
    }}
}}

{completions}"#
            )
        }
    }
}

/// Returns the tab completion script for the given shell.
/// This provides dynamic completion of directory names from the tries_path.
pub fn get_completions_script(shell: &Shell) -> String {
    match shell {
        Shell::Fish => {
            r#"# try-rs tab completion for directory names
function __try_rs_get_tries_path
    # Check TRY_PATH environment variable first
    if set -q TRY_PATH
        echo $TRY_PATH
        return
    end
    
    # Try to read from config file
    set -l config_paths "$HOME/.config/try-rs/config.toml" "$HOME/.try-rs/config.toml"
    for config_path in $config_paths
        if test -f $config_path
            set -l tries_path (command grep -E '^\s*tries_path\s*=' $config_path 2>/dev/null | command sed 's/.*=\s*"\?\([^"]*\)"\?.*/\1/' | command sed "s|~|$HOME|" | string trim)
            if test -n "$tries_path"
                echo $tries_path
                return
            end
        end
    end
    
    # Default path
    echo "$HOME/work/tries"
end

function __try_rs_complete_directories
    set -l tries_path (__try_rs_get_tries_path)
    
    if test -d $tries_path
        # List directories in tries_path, filtering by current token
        command ls -1 $tries_path 2>/dev/null | while read -l dir
            if test -d "$tries_path/$dir"
                echo $dir
            end
        end
    end
end

complete -f -c try-rs -n '__fish_use_subcommand' -a '(__try_rs_complete_directories)' -d 'Try directory'
"#.to_string()
        }
        Shell::Zsh => {
            r#"# try-rs tab completion for directory names
_try_rs_get_tries_path() {
    # Check TRY_PATH environment variable first
    if [[ -n "${TRY_PATH}" ]]; then
        echo "${TRY_PATH}"
        return
    fi
    
    # Try to read from config file
    local config_paths=("$HOME/.config/try-rs/config.toml" "$HOME/.try-rs/config.toml")
    for config_path in "${config_paths[@]}"; do
        if [[ -f "$config_path" ]]; then
            local tries_path=$(grep -E '^\s*tries_path\s*=' "$config_path" 2>/dev/null | sed 's/.*=\s*"\?\([^"]*\)"\?.*/\1/' | sed "s|~|$HOME|" | tr -d '[:space:]')
            if [[ -n "$tries_path" ]]; then
                echo "$tries_path"
                return
            fi
        fi
    done
    
    # Default path
    echo "$HOME/work/tries"
}

_try_rs_complete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local tries_path=$(_try_rs_get_tries_path)
    local -a dirs=()
    
    if [[ -d "$tries_path" ]]; then
        # Get list of directories
        while IFS= read -r dir; do
            dirs+=("$dir")
        done < <(ls -1 "$tries_path" 2>/dev/null | while read -r dir; do
            if [[ -d "$tries_path/$dir" ]]; then
                echo "$dir"
            fi
        done)
    fi
    
    COMPREPLY=($(compgen -W "${dirs[*]}" -- "$cur"))
}

complete -o default -F _try_rs_complete try-rs
"#.to_string()
        }
        Shell::Bash => {
            r#"# try-rs tab completion for directory names
_try_rs_get_tries_path() {
    # Check TRY_PATH environment variable first
    if [[ -n "${TRY_PATH}" ]]; then
        echo "${TRY_PATH}"
        return
    fi
    
    # Try to read from config file
    local config_paths=("$HOME/.config/try-rs/config.toml" "$HOME/.try-rs/config.toml")
    for config_path in "${config_paths[@]}"; do
        if [[ -f "$config_path" ]]; then
            local tries_path=$(grep -E '^[[:space:]]*tries_path[[:space:]]*=' "$config_path" 2>/dev/null | sed 's/.*=[[:space:]]*"\?\([^"]*\)"\?.*/\1/' | sed "s|~|$HOME|" | tr -d '[:space:]')
            if [[ -n "$tries_path" ]]; then
                echo "$tries_path"
                return
            fi
        fi
    done
    
    # Default path
    echo "$HOME/work/tries"
}

_try_rs_complete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local tries_path=$(_try_rs_get_tries_path)
    local dirs=""
    
    if [[ -d "$tries_path" ]]; then
        # Get list of directories
        while IFS= read -r dir; do
            if [[ -d "$tries_path/$dir" ]]; then
                dirs="$dirs $dir"
            fi
        done < <(ls -1 "$tries_path" 2>/dev/null)
    fi
    
    COMPREPLY=($(compgen -W "$dirs" -- "$cur"))
}

complete -o default -F _try_rs_complete try-rs
"#.to_string()
        }
        Shell::PowerShell => {
            r#"# try-rs tab completion for directory names
Register-ArgumentCompleter -CommandName try-rs -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    
    # Get tries path from environment variable or default
    $triesPath = $env:TRY_PATH
    if (-not $triesPath) {
        # Try to read from config file
        $configPaths = @(
            "$env:USERPROFILE/.config/try-rs/config.toml",
            "$env:USERPROFILE/.try-rs/config.toml"
        )
        foreach ($configPath in $configPaths) {
            if (Test-Path $configPath) {
                $content = Get-Content $configPath -Raw
                if ($content -match 'tries_path\s*=\s*["'']?([^"'']+)["'']?') {
                    $triesPath = $matches[1].Replace('~', $env:USERPROFILE).Trim()
                    break
                }
            }
        }
    }
    
    # Default path
    if (-not $triesPath) {
        $triesPath = "$env:USERPROFILE/work/tries"
    }
    
    # Get directories
    if (Test-Path $triesPath) {
        Get-ChildItem -Path $triesPath -Directory | 
            Where-Object { $_.Name -like "$wordToComplete*" } |
            ForEach-Object { 
                [System.Management.Automation.CompletionResult]::new(
                    $_.Name, 
                    $_.Name, 
                    'ParameterValue', 
                    $_.Name
                )
            }
    }
}
"#.to_string()
        }
        Shell::NuShell => {
            r#"# try-rs tab completion for directory names
# Add this to your Nushell config or env file

export def __try_rs_get_tries_path [] {
    # Check TRY_PATH environment variable first
    if ($env.TRY_PATH? | is-not-empty) {
        return $env.TRY_PATH
    }
    
    # Try to read from config file
    let config_paths = [
        ($env.HOME | path join ".config" "try-rs" "config.toml"),
        ($env.HOME | path join ".try-rs" "config.toml")
    ]
    
    for config_path in $config_paths {
        if ($config_path | path exists) {
            let content = (open $config_path | str trim)
            if ($content =~ 'tries_path\\s*=\\s*"?([^"]+)"?') {
                let path = ($content | parse -r 'tries_path\\s*=\\s*"?([^"]+)"?' | get capture0.0? | default "")
                if ($path | is-not-empty) {
                    return ($path | str replace "~" $env.HOME)
                }
            }
        }
    }
    
    # Default path
    ($env.HOME | path join "work" "tries")
}

export def __try_rs_complete [context: string] {
    let tries_path = (__try_rs_get_tries_path)
    
    if ($tries_path | path exists) {
        ls $tries_path | where type == "dir" | get name | path basename
    } else {
        []
    }
}

# Add completion to the try-rs command
export extern try-rs [
    name_or_url?: string@__try_rs_complete
    destination?: string
    --setup: string
    --setup-stdout: string
    --completions: string
    --shallow-clone(-s)
    --worktree(-w): string
]
"#.to_string()
        }
    }
}

/// Returns only the completion script (for --completions flag)
pub fn get_completion_script_only(shell: &Shell) -> String {
    let completions = get_completions_script(shell);
    match shell {
        Shell::NuShell => {
            // For NuShell, we need to provide a different format when used standalone
            r#"# try-rs tab completion for directory names
# Add this to your Nushell config

def __try_rs_get_tries_path [] {
    if ($env.TRY_PATH? | is-not-empty) {
        return $env.TRY_PATH
    }
    
    let config_paths = [
        ($env.HOME | path join ".config" "try-rs" "config.toml"),
        ($env.HOME | path join ".try-rs" "config.toml")
    ]
    
    for config_path in $config_paths {
        if ($config_path | path exists) {
            let content = (open $config_path | str trim)
            if ($content =~ 'tries_path\\s*=\\s*"?([^"]+)"?') {
                let path = ($content | parse -r 'tries_path\\s*=\\s*"?([^"]+)"?' | get capture0.0? | default "")
                if ($path | is-not-empty) {
                    return ($path | str replace "~" $env.HOME)
                }
            }
        }
    }
    
    ($env.HOME | path join "work" "tries")
}

def __try_rs_complete [context: string] {
    let tries_path = (__try_rs_get_tries_path)
    
    if ($tries_path | path exists) {
        ls $tries_path | where type == "dir" | get name | path basename
    } else {
        []
    }
}

# Register completion
export extern try-rs [
    name_or_url?: string@__try_rs_complete
    destination?: string
    --setup: string
    --setup-stdout: string
    --completions: string
    --shallow-clone(-s)
    --worktree(-w): string
]
"#.to_string()
        }
        _ => completions,
    }
}

pub fn get_shell_integration_path(shell: &Shell) -> PathBuf {
    let config_dir = match shell {
        Shell::Fish => get_base_config_dir(),
        _ => get_config_dir(),
    };

    match shell {
        Shell::Fish => config_dir
            .join("fish")
            .join("functions")
            .join("try-rs.fish"),
        Shell::Zsh => config_dir.join("try-rs.zsh"),
        Shell::Bash => config_dir.join("try-rs.bash"),
        Shell::PowerShell => config_dir.join("try-rs.ps1"),
        Shell::NuShell => config_dir.join("try-rs.nu"),
    }
}

fn write_fish_picker_function() -> Result<PathBuf> {
    let file_path = get_base_config_dir()
        .join("fish")
        .join("functions")
        .join("try-rs-picker.fish");
    if let Some(parent) = file_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(&file_path, FISH_PICKER_FUNCTION)?;
    eprintln!("Fish picker function file created at: {}", file_path.display());
    Ok(file_path)
}

pub fn is_shell_integration_configured(shell: &Shell) -> bool {
    get_shell_integration_path(shell).exists()
}

/// Appends a source command to an RC file if not already present.
fn append_source_to_rc(rc_path: &std::path::Path, source_cmd: &str) -> Result<()> {
    if rc_path.exists() {
        let content = fs::read_to_string(rc_path)?;
        if !content.contains(source_cmd) {
            let mut file = fs::OpenOptions::new().append(true).open(rc_path)?;
            writeln!(file, "\n# try-rs integration")?;
            writeln!(file, "{}", source_cmd)?;
            eprintln!("Added configuration to {}", rc_path.display());
        } else {
            eprintln!("Configuration already present in {}", rc_path.display());
        }
    } else {
        eprintln!(
            "You need to add the following line to {}:",
            rc_path.display()
        );
        eprintln!("{}", source_cmd);
    }
    Ok(())
}

/// Writes the shell integration file and returns its path.
fn write_shell_integration(shell: &Shell) -> Result<std::path::PathBuf> {
    let file_path = get_shell_integration_path(shell);
    if let Some(parent) = file_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(&file_path, get_shell_content(shell))?;
    eprintln!(
        "{:?} function file created at: {}",
        shell,
        file_path.display()
    );
    Ok(file_path)
}

/// Sets up shell integration for the given shell.
pub fn setup_shell(shell: &Shell) -> Result<()> {
    let file_path = write_shell_integration(shell)?;
    let home_dir = dirs::home_dir().expect("Could not find home directory");

    match shell {
        Shell::Fish => {
            let _picker_path = write_fish_picker_function()?;
            let fish_config_path = get_base_config_dir().join("fish").join("config.fish");
            eprintln!(
                "You may need to restart your shell or run 'source {}' to apply changes.",
                file_path.display()
            );
            eprintln!(
                "Optional: append the following to {} to bind Ctrl+T:",
                fish_config_path.display()
            );
            eprintln!("bind \\ct try-rs-picker");
            eprintln!("bind -M insert \\ct try-rs-picker");
        }
        Shell::Zsh => {
            let source_cmd = format!("source '{}'", file_path.display());
            append_source_to_rc(&home_dir.join(".zshrc"), &source_cmd)?;
        }
        Shell::Bash => {
            let source_cmd = format!("source '{}'", file_path.display());
            append_source_to_rc(&home_dir.join(".bashrc"), &source_cmd)?;
        }
        Shell::PowerShell => {
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
                append_source_to_rc(&profile_path, &source_cmd)?;
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
        }
        Shell::NuShell => {
            let nu_config_path = dirs::config_dir()
                .expect("Could not find config directory")
                .join("nushell")
                .join("config.nu");
            let source_cmd = format!("source '{}'", file_path.display());
            if nu_config_path.exists() {
                append_source_to_rc(&nu_config_path, &source_cmd)?;
            } else {
                eprintln!("Could not find config.nu at {}", nu_config_path.display());
                eprintln!("Please add the following line manually:");
                eprintln!("{}", source_cmd);
            }
        }
    }

    Ok(())
}

/// Generates a standalone completion script for the given shell.
pub fn generate_completions(shell: &Shell) -> Result<()> {
    let script = get_completion_script_only(shell);
    print!("{}", script);
    Ok(())
}
