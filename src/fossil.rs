use anyhow::Result;
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

    pub fn add_file(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["add", "--", path])
    }

    pub fn forget_file(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["forget", "--", path])
    }

    pub fn cat_file(&self, path: &str) -> std::result::Result<String, FossilError> {
        self.run(&["cat", path])
    }

    pub fn ensure_repo(&self) -> std::result::Result<(), FossilError> {
        self.run(&["info"]).map(|_| ())
    }

    fn run(&self, args: &[&str]) -> std::result::Result<String, FossilError> {
        let output = Command::new("fossil")
            .args(args)
            .output()
            .map_err(|e| FossilError::CommandFailed(e.to_string()))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
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
