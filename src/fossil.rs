use anyhow::Result;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
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

pub struct FossilClient;

impl FossilClient {
    pub fn new() -> Self {
        Self
    }

    pub fn repo_state(&self) -> std::result::Result<RepoState, FossilError> {
        self.ensure_repo()?;
        let status = self.run(&["status"])?;
        let extras = self.run(&["extra"]).unwrap_or_default();
        let timeline = self
            .run(&[
                "timeline",
                "-n",
                "20",
                "-t",
                "ci",
                "-F",
                "%h|%a|%d|%c",
            ])
            .unwrap_or_default();
        Ok(RepoState {
            files: merge_files(parse_status(&status), parse_extra(&extras)),
            timeline: parse_timeline(&timeline),
            selected_file: 0,
        })
    }

    pub fn diff_for(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["diff", "--", path])
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

    pub fn commit_all(&self, message: &str) -> std::result::Result<String, FossilError> {
        self.run(&["commit", "-m", message])
    }

    pub fn ignore_glob(&self, pattern: &str) -> std::result::Result<String, FossilError> {
        update_ignore_file(pattern).map_err(|e| FossilError::CommandFailed(e.to_string()))?;
        Ok(format!("ignored {}", pattern))
    }

    pub fn cat_file(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["cat", path])
    }

    pub fn ensure_repo(&self) -> std::result::Result<(), FossilError> {
        self.run(&["info"]).map(|_| ())
    }

    fn run(&self, args: &[&str]) -> std::result::Result<String, FossilError> {
        let cmdline = format!("fossil {}", args.join(" "));
        let output = Command::new("fossil")
            .args(args)
            .output()
            .map_err(|e| FossilError::CommandFailed(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let _ = log_command(&cmdline, output.status.success(), &stdout, &stderr);

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

fn build_add_args<'a>(paths: &'a [String]) -> Vec<&'a str> {
    let mut args = vec!["add"];
    for path in paths {
        args.push(path.as_str());
    }
    args
}

fn build_commit_args<'a>(paths: &'a [String], message: &'a str) -> Vec<&'a str> {
    let mut args = vec!["commit"];
    for path in paths {
        args.push(path.as_str());
    }
    args.push("-m");
    args.push(message);
    args
}

fn update_ignore_file(pattern: &str) -> std::io::Result<()> {
    let dir = ".fossil-settings";
    let path = ".fossil-settings/ignore-glob";
    fs::create_dir_all(dir)?;
    let mut contents = fs::read_to_string(path).unwrap_or_default();
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
    let mut file = OpenOptions::new().create(true).append(true).open("fossil-debug.log")?;
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
                .map(|path| FileStatus { path: path.trim().to_string(), status: "edited".to_string() })
                .or_else(|| line.strip_prefix("ADDED   ").map(|path| FileStatus { path: path.trim().to_string(), status: "added".to_string() }))
                .or_else(|| line.strip_prefix("DELETED ").map(|path| FileStatus { path: path.trim().to_string(), status: "deleted".to_string() }))
                .or_else(|| line.strip_prefix("CHECKED-OUT ").map(|path| FileStatus { path: path.trim().to_string(), status: "checked-out".to_string() }))
        })
        .collect()
}

fn parse_extra(out: &str) -> Vec<FileStatus> {
    out.lines()
        .filter_map(|line| {
            let path = line.trim();
            (!path.is_empty()).then(|| FileStatus { path: path.to_string(), status: "extra".to_string() })
        })
        .collect()
}

fn merge_files(mut status: Vec<FileStatus>, extras: Vec<FileStatus>) -> Vec<FileStatus> {
    for extra in extras {
        if !status.iter().any(|file| file.path == extra.path) {
            status.push(extra);
        }
    }
    status
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
        let status = parse_status("EDITED src/lib.rs\nADDED   README.md\nDELETED old.txt\nCHECKED-OUT tracked.txt\nIGNORED nope\n");
        let extras = parse_extra("tmp.log\n  \nnotes.txt\n");
        let merged = merge_files(status, extras);

        assert_eq!(merged.len(), 6);
        assert!(merged.iter().any(|f| f.path == "src/lib.rs" && f.status == "edited"));
        assert!(merged.iter().any(|f| f.path == "README.md" && f.status == "added"));
        assert!(merged.iter().any(|f| f.path == "old.txt" && f.status == "deleted"));
        assert!(merged.iter().any(|f| f.path == "tracked.txt" && f.status == "checked-out"));
        assert!(merged.iter().any(|f| f.path == "tmp.log" && f.status == "extra"));
        assert!(merged.iter().any(|f| f.path == "notes.txt" && f.status == "extra"));
    }

    #[test]
    fn parses_timeline_format() {
        let entries = parse_timeline("abc123|Alice|2026-06-04 10:00|Fix bug\nzzz999|Bob|2026-06-04 11:00|Refactor\n");
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
        assert_eq!(args, vec!["commit", "a.txt", "b.txt", "-m", "hello"]);
    }

    #[test]
    fn builds_add_arguments_for_selected_paths() {
        let paths = vec!["extra.txt".to_string()];
        let args = build_add_args(&paths);
        assert_eq!(args, vec!["add", "extra.txt"]);
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
        update_ignore_file("notes.txt").unwrap();
        std::env::set_current_dir(old).unwrap();

        let contents = std::fs::read_to_string(ignore).unwrap();
        assert!(contents.contains("*.swp"));
        assert!(contents.contains("notes.txt"));
    }
}
