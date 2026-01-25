use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Checks if current directory is inside a git repository
pub fn is_inside_git_repo<P: AsRef<Path>>(path: P) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path.as_ref())
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn is_git_worktree_locked(path: &Path) -> bool {
    let dot_git = path.join(".git");
    if dot_git.is_file() {
        let parent = parse_dot_git(&dot_git);
        match parent {
            Ok(parent_path) => {
                return parent_path.join("locked").exists();
            }
            Err(_) => {
                return false;
            }
        }
    }
    false
}

/// Checks if a path is a git worktree (not the main working tree)
/// A worktree has a .git file (not directory) that points to the main repo
pub fn is_git_worktree(path: &Path) -> bool {
    let dot_git = path.join(".git");
    // If .git is a file (not a directory), it's a worktree
    dot_git.is_file()
}

fn parse_dot_git(dot_git: &Path) -> std::io::Result<PathBuf> {
    Ok(first_line(&std::fs::read(dot_git)?).into())
}

#[cfg(unix)]
pub fn first_line(bytes: &[u8]) -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(
        bytes
            .iter()
            .copied()
            .skip_while(|&b| b != b' ')
            .skip(1)
            .take_while(|&b| b != b'\n')
            .collect::<Vec<_>>(),
    )
}

#[cfg(not(unix))]
pub fn first_line(bytes: &[u8]) -> OsString {
    let vec: Vec<u8> = bytes
        .iter()
        .copied()
        .skip_while(|&b| b != b' ')
        .skip(1)
        .take_while(|&b| b != b'\n')
        .collect();
    OsString::from(String::from_utf8_lossy(&vec).to_string())
}

pub fn remove_git_worktree(path_to_remove: &Path) -> std::io::Result<std::process::Output> {
    Command::new("git")
        .args(["worktree", "remove", "."])
        .current_dir(path_to_remove)
        .output()
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

#[cfg(unix)]
pub fn get_free_disk_space_mb(path: &Path) -> Option<u64> {
    use std::ffi::CString;
    use std::mem::MaybeUninit;
    use std::os::unix::ffi::OsStrExt;

    let c_path = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat: MaybeUninit<libc::statvfs> = MaybeUninit::uninit();

    unsafe {
        if libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) == 0 {
            let stat = stat.assume_init();
            let free_bytes = (stat.f_bavail as u64) * (stat.f_frsize as u64);
            return Some(free_bytes / (1024 * 1024));
        }
    }
    None
}

#[cfg(not(unix))]
pub fn get_free_disk_space_mb(_path: &Path) -> Option<u64> {
    None
}
