use anyhow::Result;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct RepoState {
    pub files: Vec<FileStatus>,
    pub timeline: Vec<TimelineEntry>,
    pub selected_file: usize,
}

#[derive(Debug, Clone)]
pub struct FileStatus {
    pub path: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TimelineEntry {
    pub rid: String,
    pub user: String,
    pub message: String,
    pub date: String,
}

#[derive(Debug)]
pub enum FossilError {
    NotRepository,
    CommandFailed(String),
}

impl std::fmt::Display for FossilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FossilError::NotRepository => write!(f, "not a fossil repository"),
            FossilError::CommandFailed(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for FossilError {}

pub struct FossilClient {
    checkout_root: Option<PathBuf>,
    debug_enabled: bool,
}

impl FossilClient {
    pub fn new(debug_enabled: bool) -> Self {
        Self {
            checkout_root: None,
            debug_enabled,
        }
    }

    pub fn checkout_root_path(&self) -> Option<&Path> {
        self.checkout_root.as_deref()
    }

    pub fn repo_state(&mut self) -> std::result::Result<RepoState, FossilError> {
        self.ensure_repo()?;
        self.checkout_root = Some(self.checkout_root()?);
        let status = self.run(&["status"])?;
        let tracked = self.run(&["ls"])?;
        let extras = self.run(&["extras", "--dotfiles"]).unwrap_or_default();
        let timeline = self.history_timeline(None).unwrap_or_default();
        Ok(RepoState {
            files: merge_files(
                parse_tracked(&tracked),
                parse_status(&status),
                parse_extra(&extras),
            ),
            timeline,
            selected_file: 0,
        })
    }

    pub fn diff_for(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["diff", "--", path])
    }

    pub fn checkin_diff(&self, rid: &str) -> std::result::Result<String, FossilError> {
        self.run(&["diff", "--checkin", rid])
    }

    pub fn sync(&self) -> std::result::Result<String, FossilError> {
        self.run(&["sync"])
    }

    pub fn history_timeline(
        &self,
        path: Option<&str>,
    ) -> std::result::Result<Vec<TimelineEntry>, FossilError> {
        let mut args: Vec<String> = vec!["timeline", "-n", "50", "-t", "ci", "-F", "%h|%a|%d|%c"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        if let Some(path) = path {
            args.push("-p".to_string());
            args.push(path.to_string());
        }
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let output = self.run(&arg_refs)?;
        Ok(parse_timeline(&output))
    }

    pub fn add_files(&self, paths: &[String]) -> std::result::Result<String, FossilError> {
        self.run(&build_add_args(paths))
    }

    pub fn commit_paths(
        &self,
        paths: &[String],
        message: &str,
    ) -> std::result::Result<String, FossilError> {
        self.run(&build_commit_args(paths, message))
    }

    pub fn ignore_glob(&self, pattern: &str) -> std::result::Result<String, FossilError> {
        let root = self
            .checkout_root
            .as_deref()
            .ok_or(FossilError::NotRepository)?;
        update_ignore_file(root, pattern).map_err(|e| FossilError::CommandFailed(e.to_string()))?;
        Ok(format!("ignored {}", pattern))
    }

    pub fn discard_file(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["revert", "--", path])
    }

    pub fn remove_files(&self, paths: &[String]) -> std::result::Result<String, FossilError> {
        self.run(&build_rm_args(paths))
    }

    pub fn set_binary_glob(&self, pattern: &str) -> std::result::Result<String, FossilError> {
        self.run(&["settings", "binary-glob", pattern])
    }

    pub fn cat_file(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["cat", path])
    }

    pub fn ensure_repo(&self) -> std::result::Result<(), FossilError> {
        self.run(&["info"]).map(|_| ())
    }

    fn checkout_root(&self) -> std::result::Result<PathBuf, FossilError> {
        let mut command = Command::new("fossil");
        command.arg("info");
        let output = command
            .output()
            .map_err(|e| FossilError::CommandFailed(e.to_string()))?;
        let info = String::from_utf8_lossy(&output.stdout);
        for line in info.lines() {
            if let Some(root) = line.strip_prefix("local-root:") {
                return Ok(Path::new(root.trim()).to_path_buf());
            }
        }
        Err(FossilError::CommandFailed(
            "unable to determine checkout root".to_string(),
        ))
    }

    fn run(&self, args: &[&str]) -> std::result::Result<String, FossilError> {
        let cmdline = format!("fossil {}", args.join(" "));
        let mut command = Command::new("fossil");
        command.args(args);
        if let Some(root) = self.checkout_root.as_deref() {
            command.current_dir(root);
        }
        let output = command
            .output()
            .map_err(|e| FossilError::CommandFailed(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if self.debug_enabled {
            let _ = log_command(&cmdline, output.status.success(), &stdout, &stderr);
        }

        if output.status.success() {
            Ok(stdout)
        } else {
            let lowered = stderr.to_lowercase();
            if lowered.contains("not within an open checkout")
                || lowered.contains("not an open checkout")
                || lowered.contains("use 'fossil open'")
                || lowered.contains("repository filename")
                || lowered.contains("no such file or directory")
            {
                Err(FossilError::NotRepository)
            } else {
                Err(FossilError::CommandFailed(stderr))
            }
        }
    }
}

fn build_add_args(paths: &[String]) -> Vec<&str> {
    let mut args = vec!["add"];
    for path in paths {
        args.push(path.as_str());
    }
    args
}

fn build_commit_args<'a>(paths: &'a [String], message: &'a str) -> Vec<&'a str> {
    let mut args = vec!["commit", "-m", message];
    for path in paths {
        args.push(path.as_str());
    }
    args
}

fn build_rm_args(paths: &[String]) -> Vec<&str> {
    let mut args = vec!["rm"];
    for path in paths {
        args.push(path.as_str());
    }
    args
}

fn update_ignore_file(root: &Path, pattern: &str) -> std::io::Result<()> {
    let dir = root.join(".fossil-settings");
    let path = dir.join("ignore-glob");
    fs::create_dir_all(&dir)?;
    let mut contents = fs::read_to_string(&path).unwrap_or_default();
    let pattern = pattern.trim();
    if pattern.is_empty() {
        return Ok(());
    }
    if !contents.lines().any(|line| line.trim() == pattern) {
        if !contents.ends_with('\n') && !contents.is_empty() {
            contents.push('\n');
        }
        contents.push_str(pattern);
        contents.push('\n');
        fs::write(path, contents)?;
    }
    Ok(())
}

fn log_command(cmd: &str, success: bool, stdout: &str, stderr: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("fossil-debug.log")?;
    writeln!(file, "=== {} ===", cmd)?;
    writeln!(file, "status: {}", if success { "ok" } else { "err" })?;
    if !stdout.trim().is_empty() {
        writeln!(file, "stdout:\n{}", stdout)?;
    }
    if !stderr.trim().is_empty() {
        writeln!(file, "stderr:\n{}", stderr)?;
    }
    writeln!(file)?;
    Ok(())
}

fn parse_status(out: &str) -> Vec<FileStatus> {
    out.lines()
        .filter_map(|line| {
            line.strip_prefix("EDITED ")
                .map(|path| FileStatus {
                    path: path.trim().to_string(),
                    status: "edited".to_string(),
                })
                .or_else(|| {
                    line.strip_prefix("ADDED   ").map(|path| FileStatus {
                        path: path.trim().to_string(),
                        status: "added".to_string(),
                    })
                })
                .or_else(|| {
                    line.strip_prefix("DELETED ").map(|path| FileStatus {
                        path: path.trim().to_string(),
                        status: "deleted".to_string(),
                    })
                })
                .or_else(|| {
                    line.strip_prefix("MISSING ").map(|path| FileStatus {
                        path: path.trim().to_string(),
                        status: "missing".to_string(),
                    })
                })
                .or_else(|| {
                    line.strip_prefix("CHECKED-OUT ").map(|path| FileStatus {
                        path: path.trim().to_string(),
                        status: "checked-out".to_string(),
                    })
                })
                .or_else(|| {
                    line.strip_prefix("CONFLICT ").map(|path| FileStatus {
                        path: path.trim().to_string(),
                        status: "conflict".to_string(),
                    })
                })
                .or_else(|| {
                    line.strip_prefix("MERGE-CONFLICT ").map(|path| FileStatus {
                        path: path.trim().to_string(),
                        status: "conflict".to_string(),
                    })
                })
        })
        .collect()
}

fn parse_extra(out: &str) -> Vec<FileStatus> {
    out.lines()
        .filter_map(|line| {
            let path = line.trim();
            (!path.is_empty()).then(|| FileStatus {
                path: path.to_string(),
                status: "extra".to_string(),
            })
        })
        .collect()
}

fn parse_tracked(out: &str) -> Vec<FileStatus> {
    out.lines()
        .filter_map(|line| {
            let path = line.trim();
            (!path.is_empty()).then(|| FileStatus {
                path: path.to_string(),
                status: "checked-out".to_string(),
            })
        })
        .collect()
}

fn merge_files(
    mut tracked: Vec<FileStatus>,
    status: Vec<FileStatus>,
    extras: Vec<FileStatus>,
) -> Vec<FileStatus> {
    for file in status.into_iter().chain(extras) {
        if let Some(existing) = tracked.iter_mut().find(|entry| entry.path == file.path) {
            existing.status = file.status;
        } else {
            tracked.push(file);
        }
    }
    tracked.sort_by(
        |a, b| match (a.path.starts_with('.'), b.path.starts_with('.')) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => a.path.cmp(&b.path),
        },
    );
    tracked
}

fn parse_timeline(out: &str) -> Vec<TimelineEntry> {
    out.lines()
        .filter_map(|line| {
            let parts: Vec<_> = line.splitn(4, '|').collect();
            if parts.len() == 4 {
                Some(TimelineEntry {
                    rid: parts[0].trim().to_string(),
                    user: parts[1].trim().to_string(),
                    date: parts[2].trim().to_string(),
                    message: parts[3].trim().to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn _dummy_result() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status_and_extras_and_merges() {
        let tracked = parse_tracked("src/lib.rs\nREADME.md\nold.txt\ntracked.txt\n");
        let status = parse_status("EDITED src/lib.rs\nADDED   README.md\nDELETED old.txt\nMISSING gone.txt\nCHECKED-OUT tracked.txt\nIGNORED nope\n");
        let extras = parse_extra("tmp.log\n  \nnotes.txt\n.hidden\n");
        let merged = merge_files(tracked, status, extras);

        assert_eq!(merged.len(), 7);
        assert_eq!(merged.first().map(|f| f.path.as_str()), Some("README.md"));
        assert_eq!(merged.last().map(|f| f.path.as_str()), Some(".hidden"));
        assert!(merged
            .iter()
            .any(|f| f.path == "src/lib.rs" && f.status == "edited"));
        assert!(merged
            .iter()
            .any(|f| f.path == "README.md" && f.status == "added"));
        assert!(merged
            .iter()
            .any(|f| f.path == "old.txt" && f.status == "deleted"));
        assert!(merged
            .iter()
            .any(|f| f.path == "gone.txt" && f.status == "missing"));
        assert!(merged
            .iter()
            .any(|f| f.path == "tracked.txt" && f.status == "checked-out"));
        assert!(merged
            .iter()
            .any(|f| f.path == "tmp.log" && f.status == "extra"));
        assert!(merged
            .iter()
            .any(|f| f.path == "notes.txt" && f.status == "extra"));
    }

    #[test]
    fn parses_timeline_format() {
        let entries = parse_timeline(
            "abc123|Alice|2026-06-04 10:00|Fix bug\nzzz999|Bob|2026-06-04 11:00|Refactor\n",
        );
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].rid, "abc123");
        assert_eq!(entries[0].user, "Alice");
        assert_eq!(entries[0].date, "2026-06-04 10:00");
        assert_eq!(entries[0].message, "Fix bug");
    }

    #[test]
    fn builds_commit_arguments_for_selected_paths() {
        let paths = vec!["a.txt".to_string(), "b.txt".to_string()];
        let args = build_commit_args(&paths, "hello");
        assert_eq!(args, vec!["commit", "-m", "hello", "a.txt", "b.txt"]);
    }

    #[test]
    fn builds_add_arguments_for_selected_paths() {
        let paths = vec!["extra.txt".to_string()];
        let args = build_add_args(&paths);
        assert_eq!(args, vec!["add", "extra.txt"]);
    }

    #[test]
    fn builds_rm_arguments_for_missing_paths() {
        let paths = vec!["missing.txt".to_string()];
        let args = build_rm_args(&paths);
        assert_eq!(args, vec!["rm", "missing.txt"]);
    }

    #[test]
    fn updates_ignore_file_contents() {
        let dir = std::env::temp_dir().join(format!("lazyfossil-test-{}", std::process::id()));
        let settings = dir.join(".fossil-settings");
        std::fs::create_dir_all(&settings).unwrap();
        let ignore = settings.join("ignore-glob");
        std::fs::write(&ignore, "*.swp\n").unwrap();

        let old = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();
        update_ignore_file(&dir, "notes.txt").unwrap();
        std::env::set_current_dir(old).unwrap();

        let contents = std::fs::read_to_string(ignore).unwrap();
        assert!(contents.contains("*.swp"));
        assert!(contents.contains("notes.txt"));
    }
}
