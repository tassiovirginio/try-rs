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

    /// Use shallow clone (--depth 1) when cloning repositories
    #[arg(short, long)]
    pub shallow_clone: bool,

    /// Create a git worktree from current repository (must be inside a git repo)
    #[arg(short = 'w', long = "worktree", value_name = "WORKTREE_NAME")]
    pub worktree: Option<String>,
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shell {
    Fish,
    Zsh,
    Bash,
    #[allow(clippy::enum_variant_names)]
    NuShell,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
}
