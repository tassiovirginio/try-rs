use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "try-rs")]
#[command(about = format!("🦀 try-rs {} 🦀\nA blazing fast, Rust-based workspace manager for your temporary experiments.", env!("CARGO_PKG_VERSION")), long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Create or jump to an experiment / Clone a git URL. Starts TUI if omitted
    #[arg(value_name = "NAME_OR_URL")]
    pub name_or_url: Option<String>,

    /// Destination folder name when cloning a repository
    #[arg(value_name = "DESTINATION")]
    pub destination: Option<String>,

    /// Generate shell integration code for the specified shell
    #[arg(long)]
    pub setup: Option<Shell>,

    /// Print shell integration code to stdout (for use with tools like Nix home-manager)
    #[arg(long)]
    pub setup_stdout: Option<Shell>,

    /// Generate shell completion script for tab completion of directory names
    #[arg(long)]
    pub completions: Option<Shell>,

    /// Use shallow clone (--depth 1) when cloning repositories
    #[arg(short, long)]
    pub shallow_clone: bool,

    /// Create a git worktree from current repository (must be inside a git repo)
    #[arg(short = 'w', long = "worktree", value_name = "WORKTREE_NAME")]
    pub worktree: Option<String>,

    /// Render the picker inline (non-fullscreen), useful for shell key bindings
    #[arg(long)]
    pub inline_picker: bool,

    /// Inline picker height in terminal rows (default: 18)
    #[arg(long, value_name = "LINES", requires = "inline_picker")]
    pub inline_height: Option<u16>,
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Shell {
    Fish,
    Zsh,
    Bash,
    #[allow(clippy::enum_variant_names)]
    NuShell,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
}
