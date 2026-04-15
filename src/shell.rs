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
                r#"def --env --wrapped try-rs [
    name_or_url?: string@__try_rs_complete
    ...args
] {{
    let all_args = (if $name_or_url == null {{ [] }} else {{ [$name_or_url] }} | append $args)

    # Pass flags/options directly to stdout without capturing
    if ($all_args | any {{ |arg| $arg | str starts-with '-' }}) {{
        ^try-rs ...$all_args
        return
    }}

    # Capture output. Stderr (TUI) goes directly to terminal.
    let output = (^try-rs ...$all_args | str trim)

    if ($output | is-not-empty) {{
        if ($output | str starts-with "cd ") {{
            # Grabs the path out of stdout returned by the binary and removes the single quotes
            let path = ($output | str replace --regex '^cd ' '' | str replace --all "'" "" | str replace --all '"' "")
            if ($path | path exists) {{
                cd $path
            }}
        }} else {{
            # If it's not a cd command, it's likely an editor command
            nu -c $output
        }}
    }}
}}

{completions}"#,
                completions = get_completions_script(shell),
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
        # Check if contains comma
        if echo "$TRY_PATH" | command grep -q ","
            for path in (string split "," $TRY_PATH)
                printf '%s\n' (string trim $path)
            end
        else
            printf '%s\n' $TRY_PATH
        end
        return
    end
    
    # Try to read from config file
    set -l config_paths "$HOME/.config/try-rs/config.toml" "$HOME/.try-rs/config.toml"
    for config_path in $config_paths
        if test -f $config_path
            # Try tries_path (supports single or multiple paths with comma)
            set -l tries_path (command grep -E '^\s*tries_path\s*=' $config_path 2>/dev/null | command sed -E 's/.*=[[:space:]]*"?([^"]*)"?.*/\1/' | command sed "s|~|$HOME|" | string trim)
            if test -n "$tries_path"
                # Check if it contains comma (multiple paths)
                if echo "$tries_path" | command grep -q ","
                    for path in (string split "," $tries_path)
                        printf '%s\n' (string trim $path)
                    end
                else
                    printf '%s\n' $tries_path
                end
                return
            end
        end
    end
    
    # Default path
    printf '%s\n' "$HOME/work/tries"
end

function __try_rs_complete_directories
    for tries_path in (__try_rs_get_tries_path)
        if test -d $tries_path
            command ls -1 $tries_path 2>/dev/null | while read -l dir
                if test -d "$tries_path/$dir"
                    echo $dir
                end
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
        if [[ "${TRY_PATH}" == *","* ]]; then
            echo "${TRY_PATH}" | tr ',' '\n'
        else
            echo "${TRY_PATH}"
        fi
        return
    fi
    
    # Try to read from config file
    local config_paths=("$HOME/.config/try-rs/config.toml" "$HOME/.try-rs/config.toml")
    for config_path in "${config_paths[@]}"; do
        if [[ -f "$config_path" ]]; then
            # Try tries_path (supports single or multiple paths with comma)
            local tries_path=$(grep -E '^[[:space:]]*tries_path[[:space:]]*=' "$config_path" 2>/dev/null | sed -E 's/.*=[[:space:]]*"?([^"]*)"?.*/\1/' | sed "s|~|$HOME|" | tr -d '[:space:]')
            if [[ -n "$tries_path" ]]; then
                if [[ "$tries_path" == *","* ]]; then
                    echo "$tries_path" | tr ',' '\n'
                else
                    echo "$tries_path"
                fi
                return
            fi
        fi
    done
    
    # Default path
    echo "$HOME/work/tries"
}

_try_rs_complete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local tries_paths=$(_try_rs_get_tries_path)
    local -a dirs=()
    
    # Split by comma if multiple paths
    IFS=',' read -ra PATH_ARRAY <<< "$tries_paths"
    
    for tries_path in "${PATH_ARRAY[@]}"; do
        # Trim whitespace
        tries_path=$(echo "$tries_path" | xargs)
        
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
    done
    
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
        if [[ "${TRY_PATH}" == *","* ]]; then
            echo "${TRY_PATH}" | tr ',' '\n'
        else
            echo "${TRY_PATH}"
        fi
        return
    fi
    
    # Try to read from config file
    local config_paths=("$HOME/.config/try-rs/config.toml" "$HOME/.try-rs/config.toml")
    for config_path in "${config_paths[@]}"; do
        if [[ -f "$config_path" ]]; then
            # Try tries_path (supports single or multiple paths with comma)
            local tries_path=$(grep -E '^[[:space:]]*tries_path[[:space:]]*=' "$config_path" 2>/dev/null | sed -E 's/.*=[[:space:]]*"?([^"]*)"?.*/\1/' | sed "s|~|$HOME|" | tr -d '[:space:]')
            if [[ -n "$tries_path" ]]; then
                if [[ "$tries_path" == *","* ]]; then
                    echo "$tries_path" | tr ',' '\n'
                else
                    echo "$tries_path"
                fi
                return
            fi
        fi
    done
    
    # Default path
    echo "$HOME/work/tries"
}

_try_rs_complete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local tries_paths=$(_try_rs_get_tries_path)
    local dirs=""
    
    # Split by comma if multiple paths
    IFS=',' read -ra PATH_ARRAY <<< "$tries_paths"
    
    for tries_path in "${PATH_ARRAY[@]}"; do
        # Trim whitespace
        tries_path=$(echo "$tries_path" | xargs)
        
        if [[ -d "$tries_path" ]]; then
            # Get list of directories
            while IFS= read -r dir; do
                if [[ -d "$tries_path/$dir" ]]; then
                    dirs="$dirs $dir"
                fi
            done < <(ls -1 "$tries_path" 2>/dev/null)
        fi
    done
    
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
    $triesPaths = $env:TRY_PATH
    if (-not $triesPaths) {
        # Try to read from config file
        $configPaths = @(
            "$env:USERPROFILE/.config/try-rs/config.toml",
            "$env:USERPROFILE/.try-rs/config.toml"
        )
        foreach ($configPath in $configPaths) {
            if (Test-Path $configPath) {
                $content = Get-Content $configPath -Raw
                # Try tries_path (supports single or multiple paths with comma)
                if ($content -match 'tries_path\s*=\s*["'']?([^"'']+)["'']?') {
                    $triesPaths = $matches[1].Replace('~', $env:USERPROFILE).Trim()
                    break
                }
            }
        }
    }
    
    # Default path
    if (-not $triesPaths) {
        $triesPaths = "$env:USERPROFILE/work/tries"
    }
    
    # Split by comma if multiple paths
    $pathArray = $triesPaths -split ','
    
    # Get directories from all paths
    foreach ($triesPath in $pathArray) {
        $triesPath = $triesPath.Trim()
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
}
"#.to_string()
        }
        Shell::NuShell => {
            r#"# try-rs tab completion for directory names
# Add this to your Nushell config or env file

export def __try_rs_get_tries_paths [] {
    # Check TRY_PATH environment variable first
    if ($env.TRY_PATH? | is-not-empty) {
        return ($env.TRY_PATH | split row "," | each { |s| $s | str trim })
    }
    
    # Try to read from config file
    let config_paths = [
        ($env.HOME | path join ".config" "try-rs" "config.toml"),
        ($env.HOME | path join ".try-rs" "config.toml")
    ]
    
    for config_path in $config_paths {
        if ($config_path | path exists) {
            let content = (open $config_path | str trim)
            # Try tries_path (supports single or multiple paths with comma)
            if ($content =~ 'tries_path\\s*=\\s*"?([^"]+)"?') {
                let path = ($content | parse -r 'tries_path\\s*=\\s*"?([^"]+)"?' | get capture0.0? | default "")
                if ($path | is-not-empty) {
                    # Check if contains comma (multiple paths)
                    if ($path | str contains ",") {
                        return ($path | split row "," | each { |s| ($s | str trim | str replace "~" $env.HOME) })
                    else
                        return ([($path | str replace "~" $env.HOME)])
                    }
                }
            }
        }
    }
    
    # Default path
    [($env.HOME | path join "work" "tries")]
}

export def __try_rs_complete [context: string] {
    let tries_paths = (__try_rs_get_tries_paths)
    
    mut all_dirs = []
    for tries_path in $tries_paths {
        if ($tries_path | path exists) {
            let dirs = (ls $tries_path | where type == "dir" | get name | path basename)
            $all_dirs = ($all_dirs | append $dirs)
        }
    }
    $all_dirs
}
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
        Shell::Fish => get_fish_functions_dir().join("try-rs.fish"),
        Shell::Zsh => config_dir.join("try-rs.zsh"),
        Shell::Bash => config_dir.join("try-rs.bash"),
        Shell::PowerShell => config_dir.join("try-rs.ps1"),
        Shell::NuShell => config_dir.join("try-rs.nu"),
    }
}

fn get_fish_functions_dir() -> PathBuf {
    if let Ok(output) = std::process::Command::new("fish")
        .args(["-c", "echo $__fish_config_dir"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let path = PathBuf::from(output_str.trim()).join("functions");
            if path.exists() || path.parent().map(|p| p.exists()).unwrap_or(false) {
                return path;
            }
        }
    }
    get_base_config_dir().join("fish").join("functions")
}

fn write_fish_picker_function() -> Result<PathBuf> {
    let file_path = get_fish_functions_dir().join("try-rs-picker.fish");
    if let Some(parent) = file_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(&file_path, FISH_PICKER_FUNCTION)?;
    eprintln!(
        "Fish picker function file created at: {}",
        file_path.display()
    );
    Ok(file_path)
}

pub fn is_shell_integration_configured(shell: &Shell) -> bool {
    get_shell_integration_path(shell).exists()
}

/// Appends a source command to an RC file if not already present.
fn append_source_to_rc(rc_path: &std::path::Path, source_cmd: &str) -> Result<()> {
    if rc_path.exists() {
        let content = fs::read_to_string(rc_path)?;
        // Check for either the exact source command or our marker comment
        if !content.contains(source_cmd) && !content.contains("# try-rs integration") {
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
            let fish_config_path = home_dir.join(".config").join("fish").join("config.fish");
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

pub fn get_installed_shells() -> Vec<Shell> {
    let mut shells = Vec::new();
    for shell in [
        Shell::Fish,
        Shell::Zsh,
        Shell::Bash,
        Shell::PowerShell,
        Shell::NuShell,
    ] {
        if is_shell_installed(&shell) {
            shells.push(shell);
        }
    }
    shells
}

fn is_shell_installed(shell: &Shell) -> bool {
    let shell_name = match shell {
        Shell::Fish => "fish",
        Shell::Zsh => "zsh",
        Shell::Bash => "bash",
        Shell::PowerShell => "pwsh",
        Shell::NuShell => "nu",
    };

    let output = std::process::Command::new("whereis")
        .arg(shell_name)
        .output();

    match output {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            let trimmed = result.trim();
            !trimmed.is_empty()
                && !trimmed.ends_with(':')
                && trimmed.starts_with(&format!("{}: ", shell_name))
        }
        Err(_) => false,
    }
}

pub fn clear_shell_setup() -> Result<()> {
    let installed_shells = get_installed_shells();

    if installed_shells.is_empty() {
        eprintln!("No supported shells found on this system.");
        return Ok(());
    }

    eprintln!("Detected shells: {:?}\n", installed_shells);
    
    eprintln!("Files to be removed:");

    for shell in &installed_shells {
        let paths = get_shell_config_paths(shell);

        for path in &paths {
            eprintln!("  - {}", path.display());
        }

        match shell {
            Shell::Fish => {
                let fish_functions = get_fish_functions_dir();
                eprintln!(
                    "  - {}",
                    fish_functions.join("try-rs-picker.fish").display()
                );
            }
            _ => {}
        }
    }

    eprintln!("\nRemoving files...");

    for shell in &installed_shells {
        clear_shell_config(shell)?;
    }

    eprintln!("\nDone! Shell integration removed.");
    Ok(())
}

fn clear_shell_config(shell: &Shell) -> Result<()> {
    let integration_file = get_shell_integration_path(shell);
    if integration_file.exists() {
        fs::remove_file(&integration_file)?;
        eprintln!("Removed integration file: {}", integration_file.display());
    }

    if let Shell::Fish = shell {
        let picker_path = get_fish_functions_dir().join("try-rs-picker.fish");
        if picker_path.exists() {
            fs::remove_file(&picker_path)?;
            eprintln!("Removed picker file: {}", picker_path.display());
        }
    }

    // Clean up RC files instead of deleting them
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let rc_files = match shell {
        Shell::Zsh => vec![home_dir.join(".zshrc")],
        Shell::Bash => vec![home_dir.join(".bashrc")],
        Shell::NuShell => vec![dirs::config_dir()
            .expect("Could not find config directory")
            .join("nushell")
            .join("config.nu")],
        Shell::PowerShell => {
             let profile_path_ps7 = home_dir
                .join("Documents")
                .join("PowerShell")
                .join("Microsoft.PowerShell_profile.ps1");
            let profile_path_ps5 = home_dir
                .join("Documents")
                .join("WindowsPowerShell")
                .join("Microsoft.PowerShell_profile.ps1");
            vec![profile_path_ps7, profile_path_ps5]
        },
        _ => vec![],
    };

    for rc_path in rc_files {
        if rc_path.exists() {
            remove_source_from_rc(&rc_path)?;
        }
    }

    Ok(())
}

fn remove_source_from_rc(rc_path: &std::path::Path) -> Result<()> {
    let content = fs::read_to_string(rc_path)?;
    if content.contains("try-rs") {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let initial_count = lines.len();
        
        // Remove lines containing try-rs integration marker or typical source commands
        lines.retain(|line| {
            !line.contains("# try-rs integration") && 
            !(line.contains("source") && line.contains("try-rs")) &&
            !(line.contains(".") && line.contains("try-rs") && rc_path.extension().map_or(false, |ext| ext == "ps1"))
        });

        if lines.len() < initial_count {
            let mut new_content = lines.join("\n");
            if !new_content.is_empty() && !new_content.ends_with('\n') {
                new_content.push('\n');
            }
            fs::write(rc_path, new_content)?;
            eprintln!("Cleaned up integration lines from {}", rc_path.display());
        }
    }
    Ok(())
}

fn get_shell_config_paths(shell: &Shell) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let config_dir = get_base_config_dir();

    match shell {
        Shell::Fish => {
            let fish_functions = get_fish_functions_dir();
            paths.push(fish_functions.join("try-rs.fish"));
        }
        Shell::Zsh => {
            paths.push(config_dir.join("try-rs.zsh"));
        }
        Shell::Bash => {
            paths.push(config_dir.join("try-rs.bash"));
        }
        Shell::PowerShell => {
            paths.push(config_dir.join("try-rs.ps1"));
        }
        Shell::NuShell => {
            paths.push(config_dir.join("try-rs.nu"));
        }
    }

    paths
}

