use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use chrono::{Local, NaiveDate, NaiveDateTime};

const DATE_PREFIX_FORMAT: &str = "%Y-%m-%d";

/// Default separator placed between the date prefix and the name, and between
/// the words of a multi-word name.
pub const DEFAULT_NAME_SEPARATOR: &str = " ";

/// Characters accepted between a date prefix and the rest of a folder name.
const DATE_PREFIX_SEPARATORS: &[char] = &[' ', '-', '_', '.'];

/// Checks if current directory is inside a git repository
pub fn is_inside_git_repo<P: AsRef<Path>>(path: P) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path.as_ref())
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check whether a git worktree at the given path is locked.
///
/// A worktree is considered locked when its `.git` file (not directory)
/// points to a parent repository that contains a `locked` file alongside
/// the worktree's administrative data.
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

/// Parse a `.git` file (worktree pointer) and return the path it points to.
fn parse_dot_git(dot_git: &Path) -> std::io::Result<PathBuf> {
    Ok(first_line(&std::fs::read(dot_git)?).into())
}

/// Extract the first path component from a `.git` worktree pointer file.
///
/// The file format is `gitdir: /path/to/worktree\n`. This function skips
/// the `gitdir: ` prefix and returns everything up to the first newline.
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

/// Build a Git command without keeping the worktree directory open.
/// Windows cannot move or remove a directory used as a process working directory.
fn worktree_command(path: &Path) -> std::io::Result<Command> {
    let source = path.canonicalize()?;
    let dot_git = source.join(".git");
    let git_dir = if dot_git.is_file() {
        let administrative_path = parse_dot_git(&dot_git)?;
        if administrative_path.is_absolute() {
            administrative_path
        } else {
            source.join(administrative_path)
        }
    } else {
        dot_git
    };
    let mut command = Command::new("git");
    command
        .arg("--git-dir")
        .arg(git_dir)
        .current_dir(source.parent().unwrap_or(&source));
    Ok(command)
}

/// Remove a Git worktree without locking its directory on Windows.
pub fn remove_git_worktree(path_to_remove: &Path) -> std::io::Result<std::process::Output> {
    let source = path_to_remove.canonicalize()?;
    worktree_command(&source)?
        .args(["worktree", "remove"])
        .arg(source)
        .output()
}

/// Expand a path string, resolving a leading `~/` to the user's home directory.
pub fn expand_path(path_str: &str) -> PathBuf {
    if (path_str.starts_with("~/") || (cfg!(windows) && path_str.starts_with("~\\")))
        && let Some(home) = dirs::home_dir()
    {
        return home.join(&path_str[2..]);
    }
    PathBuf::from(path_str)
}

/// Check whether a string looks like a git URL.
///
/// Returns `true` if the string starts with `http://`, `https://`, `git@`,
/// `ssh://`, or ends with `.git`.
pub fn is_git_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("git@")
        || s.starts_with("ssh://")
        || s.ends_with(".git")
}

/// Extract the repository name from a git URL.
///
/// Strips the `.git` suffix and trailing slashes, then takes the last
/// path/colon-delimited component. Falls back to `"cloned-repo"` if
/// the URL is empty or has no identifiable name.
pub fn extract_repo_name(url: &str) -> String {
    let clean_url = url.trim_end_matches('/').trim_end_matches(".git");
    if let Some(last_part) = clean_url.rsplit(['/', '\\', ':']).next()
        && !last_part.is_empty()
    {
        return last_part.to_string();
    }
    "cloned-repo".to_string()
}

/// Get the free disk space on the filesystem containing `path`, in megabytes.
///
/// Uses `statvfs` on Unix. Returns `None` when the path is invalid or when
/// querying the filesystem statistics fails. Always returns `None` on
/// non-Unix platforms (the default stub).
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

/// Parse a date prefix using `fmt` and return the parsed date plus the name.
fn extract_prefix_date_with_fmt(name: &str, fmt: &str) -> Option<(SystemTime, String)> {
    let (naive_date, rest) = NaiveDate::parse_and_remainder(name, fmt).ok()?;
    let rhs = rest.strip_prefix(DATE_PREFIX_SEPARATORS)?;
    if rhs.is_empty() {
        return None;
    }
    let dt: NaiveDateTime = naive_date.into();
    let dt_local = dt.and_local_timezone(Local).single()?;
    Some((dt_local.into(), rhs.into()))
}

/// Extract a `YYYY-MM-DD` date prefix from a directory name.
///
/// Returns the parsed `SystemTime` and the remainder of the name when the
/// name starts with a valid date followed by a separator (space, `-`, `_` or
/// `.`) and a non-empty name. Returns `None` otherwise.
pub fn extract_prefix_date(name: &str) -> Option<(SystemTime, String)> {
    extract_prefix_date_with_fmt(name, DATE_PREFIX_FORMAT)
}

/// Extract a date prefix honouring the configured `date_prefix_format`.
///
/// The configured format is tried first, then the default `%Y-%m-%d`, so
/// folders created before the format was changed are still recognised.
/// A format that does not describe a parseable date (for example `%Y`) simply
/// fails to parse and falls back to the default format.
pub fn extract_prefix_date_with_format(
    name: &str,
    format: Option<&str>,
) -> Option<(SystemTime, String)> {
    if let Some(fmt) = format
        && fmt != DATE_PREFIX_FORMAT
        && let Some(result) = extract_prefix_date_with_fmt(name, fmt)
    {
        return Some(result);
    }
    extract_prefix_date(name)
}

/// Generate a date string suitable as a directory name prefix.
///
/// Uses `%Y-%m-%d` by default. An optional custom format string can be
/// provided via the `format` parameter. An invalid format (for example `%Q`)
/// falls back to the default instead of panicking: `chrono`'s `DelayedFormat`
/// returns `fmt::Error` while being displayed, which `to_string()` would
/// otherwise turn into a panic.
pub fn generate_prefix_date(format: Option<&str>) -> String {
    let now = Local::now();
    let fmt = format.unwrap_or(DATE_PREFIX_FORMAT);
    let mut out = String::new();
    if write!(out, "{}", now.format(fmt)).is_ok() && !out.is_empty() {
        return out;
    }
    now.format(DATE_PREFIX_FORMAT).to_string()
}

/// Join the words of a folder name with the configured separator.
///
/// With the default separator (a space) the name is returned unchanged.
/// With any other separator, each run of whitespace is replaced by it, so
/// `"aaa bbb  ccc"` becomes `"aaa-bbb-ccc"` with `"-"`.
pub fn apply_name_separator(name: &str, separator: &str) -> String {
    if separator == DEFAULT_NAME_SEPARATOR {
        return name.to_string();
    }
    name.split_whitespace().collect::<Vec<_>>().join(separator)
}

/// Prepend today's date prefix to `name`, joined by `separator`.
pub fn prefix_with_date(name: &str, format: Option<&str>, separator: &str) -> String {
    format!("{}{}{}", generate_prefix_date(format), separator, name)
}

/// Calculate the total size of a directory tree in megabytes.
///
/// Walks all nested directories and sums the sizes of regular files.
/// Symlinks (both file and directory) are intentionally skipped.
pub fn get_folder_size_mb(path: &Path) -> u64 {
    fn dir_size(path: &Path) -> u64 {
        let mut stack = vec![path.to_path_buf()];
        let mut size = 0u64;
        while let Some(dir) = stack.pop() {
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                // Use symlink_metadata to avoid following symlinks
                let Ok(meta) = entry.metadata() else {
                    continue;
                };
                if meta.is_dir() {
                    stack.push(entry.path());
                } else if meta.is_file() {
                    size += meta.len();
                }
                // Symlinks and other special files are intentionally skipped
            }
        }
        size
    }
    dir_size(path) / (1024 * 1024)
}

/// Find folders inside `path` whose name (possibly date-prefixed) matches `name`.
///
/// Returns a list of `(parent_path, folder_name)` tuples for every immediate
/// subdirectory whose filename equals `name` or whose date-stripped display
/// name equals `name`. Recognises the default `%Y-%m-%d` date prefix.
///
/// This convenience wrapper is part of the library API used by the test suite.
/// The binary crate re-declares `mod utils` and calls
/// [`matching_folders_with_format`] instead, hence the `dead_code` allowance.
#[allow(dead_code)]
pub fn matching_folders(name: &str, path: &PathBuf) -> Vec<(PathBuf, String)> {
    matching_folders_with_format(name, path, None)
}

/// Like [`matching_folders`], but strips date prefixes using the configured
/// `date_prefix_format`, so folders created with a custom format can be found.
pub fn matching_folders_with_format(
    name: &str,
    path: &PathBuf,
    date_prefix_format: Option<&str>,
) -> Vec<(PathBuf, String)> {
    let mut result = vec![];
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            if let Ok(metadata) = entry.metadata()
                && metadata.is_dir()
            {
                let filename = entry.file_name().to_string_lossy().to_string();
                if filename == name {
                    result.push((path.clone(), filename));
                } else if let Some((_, stripped_name)) =
                    extract_prefix_date_with_format(&filename, date_prefix_format)
                    && name == stripped_name
                {
                    result.push((path.clone(), filename));
                }
            }
        }
    }
    result
}

/// The outcome of the folder selection process.
pub enum SelectionResult {
    /// A explicit folder that is guaranteed to exist already
    Folder(String),
    /// No existing match, a new folder should be created
    New(String),
    /// Nothing was selected in the UI, quit
    None,
}

/// Move a workspace while keeping Git worktree administration consistent.
pub fn move_workspace(source: &Path, destination: &Path) -> std::io::Result<()> {
    if is_git_worktree(source) {
        let source = source.canonicalize()?;
        let destination = if destination.is_absolute() {
            destination.to_path_buf()
        } else {
            std::env::current_dir()?.join(destination)
        };
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let output = worktree_command(&source)?
            .args(["worktree", "move"])
            .arg(&source)
            .arg(&destination)
            .output()?;
        if !output.status.success() {
            return Err(std::io::Error::other(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ));
        }
        Ok(())
    } else {
        fs::rename(source, destination)
    }
}
