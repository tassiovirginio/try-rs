use std::path::PathBuf;
use std::process::Command;

/// Verifica se o diretório atual está dentro de um repositório git
pub fn is_inside_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn expand_path(path_str: &str) -> PathBuf {
    if (path_str.starts_with("~/") || (cfg!(windows) && path_str.starts_with("~\\")))
        && let Some(home) = dirs::home_dir()
    {
        return home.join(&path_str[2..]);
    }
    PathBuf::from(path_str)
}

pub fn is_git_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("git@")
        || s.starts_with("ssh://")
        || s.ends_with(".git")
}

pub fn extract_repo_name(url: &str) -> String {
    let clean_url = url.trim_end_matches('/').trim_end_matches(".git");
    if let Some(last_part) = clean_url.rsplit(['/', ':']).next()
        && !last_part.is_empty()
    {
        return last_part.to_string();
    }
    "cloned-repo".to_string()
}
