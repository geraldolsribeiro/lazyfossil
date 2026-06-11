use crate::config::Config;
use crate::fossil::{FossilClient, FossilError, RepoState};
use crate::ui;
use anyhow::Result;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::process::Command;

const ASCII_LOGO: &str = include_str!("../doc/images/lazyfossil_logo_01.txt");
use std::time::Duration;

pub fn run(debug_enabled: bool) -> Result<()> {
    if debug_enabled {
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open("fossil-debug.log");
    }
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = App::new(debug_enabled).run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    res
}

struct App {
    client: FossilClient,
    config: Config,
    state: AppState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Changes,
    WorkingTree,
    FileHistory,
    Timeline,
}

pub struct AppState {
    pub tab: Tab,
    pub repo: Option<RepoState>,
    pub error: Option<String>,
    pub diff: Option<String>,
    pub diff_scroll: u16,
    pub selected_files: Vec<String>,
    pub commit_prompt: Option<String>,
    pub commit_target: CommitTarget,
    pub ignore_prompt: Option<String>,
    pub discard_prompt: Option<String>,
    pub history: Vec<crate::fossil::TimelineEntry>,
    pub history_diff: Option<String>,
    pub history_path: Option<String>,
    pub timeline_diff: Option<String>,
    pub redraw: bool,
    pub show_hex: bool,
    pub history_selected: usize,
    pub timeline_selected: usize,
    pub changes_selected: usize,
    pub files_scroll: usize,
    pub history_scroll: usize,
    pub timeline_scroll: usize,
    pub preview_kind: PreviewKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PreviewKind {
    Diff,
    Plain,
    Markdown,
    Toml,
    Json,
    Source,
    Hex,
    Notice,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommitTarget {
    Selected,
    Current,
}

impl App {
    fn new(debug_enabled: bool) -> Self {
        Self {
            client: FossilClient::new(debug_enabled),
            config: Config::load(),
            state: AppState {
                tab: Tab::Changes,
                repo: None,
                error: None,
                diff: None,
                diff_scroll: 0,
                selected_files: Vec::new(),
                commit_prompt: None,
                commit_target: CommitTarget::Selected,
                ignore_prompt: None,
                discard_prompt: None,
                history: Vec::new(),
                history_diff: None,
                history_path: None,
                timeline_diff: None,
                redraw: false,
                show_hex: false,
                history_selected: 0,
                timeline_selected: 0,
                changes_selected: 0,
                files_scroll: 0,
                history_scroll: 0,
                timeline_scroll: 0,
                preview_kind: PreviewKind::Diff,
            },
        }
    }

    fn refresh(&mut self) {
        match self.client.repo_state() {
            Ok(repo) => {
                self.state.repo = Some(repo);
                self.state.error = None;
                self.state.diff_scroll = 0;
                self.state.history_selected = 0;
                self.state.timeline_selected = 0;
                self.state.changes_selected = 0;
                self.refresh_views();
                self.refresh_timeline_details();
            }
            Err(FossilError::NotRepository) => {
                self.state.repo = None;
                self.state.diff = None;
                self.state.diff_scroll = 0;
                self.state.history_selected = 0;
                self.state.timeline_selected = 0;
                self.state.changes_selected = 0;
                self.state.history_diff = None;
                self.state.history_path = None;
                self.state.timeline_diff = None;
                self.state.selected_files.clear();
                self.state.error = Some("Not inside a Fossil checkout".to_string());
            }
            Err(err) => self.state.error = Some(err.to_string()),
        }
    }

    fn refresh_history(&mut self) {
        if let Some(path) = self.current_file_path() {
            let file_changed = self.state.history_path.as_deref() != Some(path.as_str());
            self.state.history = self
                .client
                .history_timeline(Some(&path))
                .unwrap_or_default();
            if file_changed {
                self.state.history_selected = 0;
            } else if self.state.history_selected >= self.state.history.len() {
                self.state.history_selected = 0;
            }
            self.state.history_path = Some(path);
            self.refresh_history_details();
        }
    }

    fn refresh_history_details(&mut self) {
        let Some(entry) = self.state.history.get(self.state.history_selected) else {
            self.state.history_diff = None;
            return;
        };
        let Some(path) = self.current_file_path() else {
            self.state.history_diff = self.client.checkin_diff(&entry.rid).ok();
            return;
        };
        self.state.history_diff = self
            .client
            .checkin_file_diff(&entry.rid, &path)
            .ok()
            .filter(|diff| !diff.trim().is_empty())
            .or_else(|| self.client.checkin_diff(&entry.rid).ok())
            .filter(|diff| !diff.trim().is_empty());
        if self.state.history_diff.is_none() {
            self.state.history_diff = Some(format!(
                "No diff available for [[{}]] at commit [[{}]]",
                path, entry.rid
            ));
            self.state.preview_kind = PreviewKind::Notice;
        }
    }

    fn refresh_timeline(&mut self) {
        if let Some(repo) = &self.state.repo {
            self.state.history = repo.timeline.clone();
        }
    }

    fn refresh_timeline_details(&mut self) {
        if let Some(repo) = &self.state.repo {
            if let Some(entry) = repo.timeline.get(self.state.timeline_selected) {
                self.state.timeline_diff = self.client.checkin_diff(&entry.rid).ok();
            }
        }
    }

    fn refresh_views(&mut self) {
        match self.state.tab {
            Tab::Timeline => {
                self.refresh_timeline();
                self.refresh_timeline_details();
            }
            Tab::FileHistory => {
                self.refresh_history();
                self.refresh_diff();
            }
            _ => {
                self.refresh_diff();
            }
        }
    }

    fn sync_with_remote(&mut self) {
        match self.client.sync() {
            Ok(_) => self.refresh(),
            Err(err) => self.state.error = Some(err.to_string()),
        }
    }

    fn refresh_diff(&mut self) {
        if let Some(repo) = &self.state.repo {
            if let Some(file) = repo.files.get(repo.selected_file) {
                self.state.diff_scroll = 0;
                let path = self.display_path(&file.path);
                self.state.diff = Some(match file.status.as_str() {
                    "missing" => {
                        self.state.preview_kind = PreviewKind::Notice;
                        self.missing_file_message(&file.path)
                    }
                    "conflict" => {
                        self.state.preview_kind = PreviewKind::Notice;
                        self.conflict_message(&file.path)
                    }
                    _ if self.state.show_hex => {
                        self.state.preview_kind = PreviewKind::Hex;
                        match fs::read(&path) {
                            Ok(bytes) => Self::hexdump(&bytes),
                            Err(err) => format!("content error for {}: {}", file.path, err),
                        }
                    }
                    "extra" | "checked-out" => match fs::read(&path) {
                        Ok(bytes) => match String::from_utf8(bytes) {
                            Ok(content) => {
                                self.state.preview_kind = preview_kind_for_path(&file.path);
                                if content.trim().is_empty() {
                                    format!("Empty file: {}", file.path)
                                } else {
                                    Self::expand_tabs(&content)
                                }
                            }
                            Err(_) => {
                                self.state.preview_kind = PreviewKind::Notice;
                                Self::binary_preview_notice(&file.path)
                            }
                        },
                        Err(err) => {
                            self.state.preview_kind = PreviewKind::Notice;
                            format!("content error for {}: {}", file.path, err)
                        }
                    },
                    _ => match self.client.diff_for(&file.path) {
                        Ok(diff) => {
                            if diff.trim().is_empty() {
                                match fs::read(&path) {
                                    Ok(bytes) => match String::from_utf8(bytes) {
                                        Ok(content) => {
                                            self.state.preview_kind =
                                                preview_kind_for_path(&file.path);
                                            Self::expand_tabs(&content)
                                        }
                                        Err(_) => {
                                            self.state.preview_kind = PreviewKind::Notice;
                                            Self::binary_preview_notice(&file.path)
                                        }
                                    },
                                    Err(err) => {
                                        self.state.preview_kind = PreviewKind::Notice;
                                        format!("diff/content error for {}: {}", file.path, err)
                                    }
                                }
                            } else {
                                self.state.preview_kind = PreviewKind::Diff;
                                diff
                            }
                        }
                        Err(err) => {
                            self.state.preview_kind = PreviewKind::Notice;
                            format!("diff error for {}: {}", file.path, err)
                        }
                    },
                });
            } else {
                self.state.diff = Some("No file selected".to_string());
            }
        }
    }

    fn expand_tabs(input: &str) -> String {
        const TABSTOP: usize = 8;
        let mut out = String::with_capacity(input.len());
        let mut col = 0usize;
        for ch in input.chars() {
            match ch {
                '\n' => {
                    out.push('\n');
                    col = 0;
                }
                '\t' => {
                    let spaces = TABSTOP - (col % TABSTOP);
                    out.extend(std::iter::repeat_n(' ', spaces));
                    col += spaces;
                }
                _ => {
                    out.push(ch);
                    col += 1;
                }
            }
        }
        out
    }

    fn binary_preview_notice(path: &str) -> String {
        let name = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path);
        let logo = ASCII_LOGO.trim_end();
        format!(
            "Preview unavailable for {}\n\nPress [o] to open externally or [H] for hex view\n\n{}",
            name, logo
        )
    }

    fn display_path(&self, path: &str) -> std::path::PathBuf {
        if let Some(root) = self.client.checkout_root_path() {
            root.join(path)
        } else {
            Path::new(path).to_path_buf()
        }
    }

    fn missing_file_message(&self, path: &str) -> String {
        if let Some(renamed) = self.find_renamed_extra(path) {
            format!(
                "Missing file [[{}]]\nPossible rename detected: [[{}]]\nTry discard ([d]) or commit ([f]/[c]) after reviewing the replacement.",
                path, renamed
            )
        } else {
            format!(
                "Missing file [[{}]]\nTry discard ([d]) or commit ([f]/[c]) after reviewing the working tree.",
                path
            )
        }
    }

    fn conflict_message(&self, path: &str) -> String {
        format!(
            "Conflict detected for [[{}]]\nResolve the conflict, then commit when ready.",
            path
        )
    }

    fn find_renamed_extra(&self, missing_path: &str) -> Option<String> {
        let repo = self.state.repo.as_ref()?;
        let original = self.client.cat_file(missing_path).ok()?.into_bytes();
        for file in repo.files.iter().filter(|f| f.status == "extra") {
            let path = self.display_path(&file.path);
            if let Ok(bytes) = fs::read(&path) {
                if bytes == original {
                    return Some(file.path.clone());
                }
            }
        }
        None
    }

    fn hexdump(bytes: &[u8]) -> String {
        const WIDTH: usize = 16;
        let mut out = String::new();
        for (i, chunk) in bytes.chunks(WIDTH).enumerate() {
            let offset = i * WIDTH;
            out.push_str(&format!("{:08x}  ", offset));
            for b in chunk {
                out.push_str(&format!("{:02x} ", b));
            }
            for _ in chunk.len()..WIDTH {
                out.push_str("   ");
            }
            out.push(' ');
            for b in chunk {
                let c = if b.is_ascii_graphic() || *b == b' ' {
                    *b as char
                } else {
                    '.'
                };
                out.push(c);
            }
            out.push('\n');
        }
        out
    }
    fn current_file_path(&self) -> Option<String> {
        self.state
            .repo
            .as_ref()?
            .files
            .get(self.state.repo.as_ref()?.selected_file)
            .map(|f| f.path.clone())
    }

    fn open_in_editor(&mut self) {
        let Some(path) = self.current_file_path() else {
            return;
        };
        let editor = env::var("EDITOR")
            .ok()
            .or_else(|| self.config.editor.clone())
            .unwrap_or_else(|| "vi".to_string());
        let full_path = self.display_path(&path);
        let path_arg = full_path.to_string_lossy().to_string();
        match self.spawn_external(&editor, &[path_arg.as_str()]) {
            Ok(_) => {
                self.refresh();
                self.state.redraw = true;
            }
            Err(err) => self.state.error = Some(err),
        }
    }

    fn start_discard(&mut self) {
        self.state.discard_prompt = self.current_file_path();
    }

    fn confirm_discard(&mut self) {
        let Some(path) = self.state.discard_prompt.take() else {
            return;
        };
        let is_extra = self
            .state
            .repo
            .as_ref()
            .and_then(|repo| repo.files.iter().find(|f| f.path == path))
            .is_some_and(|file| file.status == "extra");

        let result = if is_extra {
            let full_path = self.display_path(&path);
            fs::remove_file(&full_path).map_err(|e| e.to_string())
        } else {
            self.client
                .discard_file(&path)
                .map(|_| ())
                .map_err(|e| e.to_string())
        };

        match result {
            Ok(_) => {
                self.refresh();
                self.state.redraw = true;
            }
            Err(err) => self.state.error = Some(err),
        }
    }

    fn cancel_discard(&mut self) {
        self.state.discard_prompt = None;
    }

    fn open_current_file(&mut self) {
        let Some(path) = self.current_file_path() else {
            return;
        };
        let Some(cmd) = open_command_for(&path, self.config.editor.as_deref()) else {
            self.state.error = Some("No app configured for this file type".to_string());
            return;
        };
        match self.spawn_external(&cmd, &[path.as_str()]) {
            Ok(_) => {
                self.refresh();
                self.state.redraw = true;
            }
            Err(err) => self.state.error = Some(err),
        }
    }

    fn spawn_external(&self, program: &str, args: &[&str]) -> std::result::Result<(), String> {
        disable_raw_mode().map_err(|e| e.to_string())?;
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let mut command = Command::new(program);
        command.args(args);
        let status = command.status().map_err(|e| e.to_string())?;
        let _ = execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture);
        enable_raw_mode().map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{} exited with {}", program, status))
        }
    }

    fn toggle_selected_file(&mut self) {
        let Some(path) = self.current_file_path() else {
            return;
        };
        if let Some(pos) = self.state.selected_files.iter().position(|p| p == &path) {
            self.state.selected_files.remove(pos);
        } else {
            self.state.selected_files.push(path);
        }
    }

    fn toggle_select_all(&mut self) {
        let Some(repo) = &self.state.repo else {
            return;
        };
        let all_selected = !repo.files.is_empty()
            && repo
                .files
                .iter()
                .all(|f| self.state.selected_files.iter().any(|p| p == &f.path));
        if all_selected {
            self.state.selected_files.clear();
        } else {
            self.state.selected_files = repo.files.iter().map(|f| f.path.clone()).collect();
        }
    }

    fn start_ignore(&mut self) {
        self.state.ignore_prompt = self.current_file_path();
    }

    fn confirm_ignore(&mut self) {
        let Some(path) = self.state.ignore_prompt.take() else {
            return;
        };
        match self.client.ignore_glob(&path) {
            Ok(_) => self.refresh(),
            Err(err) => self.state.error = Some(err.to_string()),
        }
    }

    fn cancel_ignore(&mut self) {
        self.state.ignore_prompt = None;
    }

    fn start_commit(&mut self, target: CommitTarget) {
        self.state.commit_target = target;
        self.state.commit_prompt = Some(String::new());
    }

    fn submit_commit(&mut self) {
        let Some(message) = self.state.commit_prompt.take() else {
            return;
        };
        let message = message.trim().to_string();
        if message.is_empty() {
            self.state.error = Some("Commit message cannot be empty".to_string());
            return;
        }
        let Some(repo) = &self.state.repo else {
            return;
        };
        let current_path = self.current_file_path();
        let paths = match self.state.commit_target {
            CommitTarget::Selected => {
                if self.state.selected_files.is_empty() {
                    current_path.into_iter().collect::<Vec<_>>()
                } else {
                    self.state.selected_files.clone()
                }
            }
            CommitTarget::Current => current_path.into_iter().collect::<Vec<_>>(),
        };
        if paths.is_empty() {
            self.state.error = Some("No file selected".to_string());
            return;
        }

        let extras: Vec<String> = paths
            .iter()
            .filter_map(|path| {
                repo.files
                    .iter()
                    .find(|f| &f.path == path && f.status == "extra")
                    .map(|f| f.path.clone())
            })
            .collect();
        let missing_files: Vec<String> = paths
            .iter()
            .filter_map(|path| {
                repo.files
                    .iter()
                    .find(|f| &f.path == path && f.status == "missing")
                    .map(|f| f.path.clone())
            })
            .collect();
        let binary_files: Vec<String> = paths
            .iter()
            .filter(|path| is_binary_path(path))
            .cloned()
            .collect();

        let result = (|| {
            if !binary_files.is_empty() {
                let binary_glob = self
                    .config
                    .binary_glob
                    .as_deref()
                    .unwrap_or("*.png,*.jpg,*.jpeg,*.gif,*.ico");
                self.client.set_binary_glob(binary_glob)?;
            }
            if !extras.is_empty() {
                self.client.add_files(&extras)?;
            }
            if !missing_files.is_empty() {
                self.client.remove_files(&missing_files)?;
            }
            self.client.commit_paths(&paths, &message)
        })();

        match result {
            Ok(_) => {
                self.state.selected_files.clear();
                self.refresh();
                self.state.redraw = true;
            }
            Err(err) => self.state.error = Some(err.to_string()),
        }
    }

    fn cancel_commit(&mut self) {
        self.state.commit_prompt = None;
    }

    fn handle_commit_input(&mut self, code: KeyCode) {
        let Some(buf) = self.state.commit_prompt.as_mut() else {
            return;
        };
        match code {
            KeyCode::Esc => self.cancel_commit(),
            KeyCode::Enter => self.submit_commit(),
            KeyCode::Backspace => {
                buf.pop();
            }
            KeyCode::Char(c) => buf.push(c),
            _ => {}
        }
    }

    fn handle_ignore_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => self.cancel_ignore(),
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => self.confirm_ignore(),
            _ => {}
        }
    }

    fn handle_discard_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => self.cancel_discard(),
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => self.confirm_discard(),
            _ => {}
        }
    }

    fn select_prev(&mut self) {
        match self.state.tab {
            Tab::Timeline => {
                if self.state.timeline_selected > 0 {
                    self.state.timeline_selected -= 1;
                }
                self.sync_timeline_scroll();
                self.refresh_timeline();
                self.refresh_timeline_details();
            }
            Tab::FileHistory => {
                if self.state.history_selected > 0 {
                    self.state.history_selected -= 1;
                }
                self.sync_history_scroll();
                self.refresh_history();
                self.refresh_history_details();
            }
            Tab::Changes => {
                let visible = self.change_file_indices();
                if self.state.changes_selected > 0 {
                    self.state.changes_selected -= 1;
                }
                if let Some(repo) = &mut self.state.repo {
                    if let Some(idx) = visible.get(self.state.changes_selected).copied() {
                        repo.selected_file = idx;
                    }
                }
                self.center_changes_scroll();
                self.refresh_views();
            }
            _ => {
                if let Some(repo) = &mut self.state.repo {
                    if repo.selected_file > 0 {
                        repo.selected_file -= 1;
                    }
                }
                self.sync_files_scroll();
                self.refresh_views();
            }
        }
    }

    fn scroll_diff_up(&mut self) {
        self.state.diff_scroll = self.state.diff_scroll.saturating_sub(3);
    }
    fn scroll_diff_down(&mut self) {
        self.state.diff_scroll = self.state.diff_scroll.saturating_add(3);
    }

    fn select_next(&mut self) {
        match self.state.tab {
            Tab::Timeline => {
                if self.state.timeline_selected + 1 < self.state.repo.as_ref().map(|r| r.timeline.len()).unwrap_or(0) {
                    self.state.timeline_selected += 1;
                }
                self.sync_timeline_scroll();
                self.refresh_timeline();
                self.refresh_timeline_details();
            }
            Tab::FileHistory => {
                if self.state.history_selected + 1 < self.state.history.len() {
                    self.state.history_selected += 1;
                }
                self.sync_history_scroll();
                self.refresh_history();
                self.refresh_history_details();
            }
            Tab::Changes => {
                let visible = self.change_file_indices();
                if self.state.changes_selected + 1 < visible.len() {
                    self.state.changes_selected += 1;
                }
                if let Some(repo) = &mut self.state.repo {
                    if let Some(idx) = visible.get(self.state.changes_selected).copied() {
                        repo.selected_file = idx;
                    }
                }
                self.center_changes_scroll();
                self.refresh_views();
            }
            _ => {
                if let Some(repo) = &mut self.state.repo {
                    if repo.selected_file + 1 < repo.files.len() {
                        repo.selected_file += 1;
                    }
                }
                self.sync_files_scroll();
                self.refresh_views();
            }
        }
    }

    fn sync_files_scroll(&mut self) {
        if let Some(repo) = &self.state.repo {
            self.state.files_scroll = self
                .state
                .files_scroll
                .min(repo.files.len().saturating_sub(1));
        }
    }

    fn center_changes_scroll(&mut self) {
        let visible_len = self.change_file_indices().len();
        if visible_len == 0 {
            self.state.files_scroll = 0;
            return;
        }
        let selected = self.state.changes_selected.min(visible_len - 1);
        let half = 5usize;
        self.state.files_scroll = selected.saturating_sub(half);
    }

    fn change_file_indices(&self) -> Vec<usize> {
        self.state
            .repo
            .as_ref()
            .map(|repo| {
                repo.files
                    .iter()
                    .enumerate()
                    .filter(|(_, f)| f.status != "checked-out")
                    .map(|(i, _)| i)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn selected_file_path(&self) -> Option<String> {
        self.state
            .repo
            .as_ref()?
            .files
            .get(self.state.repo.as_ref()?.selected_file)
            .map(|f| f.path.clone())
    }

    fn select_file_by_path(&mut self, path: &str) {
        if let Some(repo) = &mut self.state.repo {
            if let Some(idx) = repo.files.iter().position(|f| f.path == path) {
                repo.selected_file = idx;
                self.sync_files_scroll();
            }
        }
    }

    fn sync_history_scroll(&mut self) {
        self.state.history_scroll = self
            .state
            .history_scroll
            .min(self.state.history.len().saturating_sub(1));
    }

    fn sync_timeline_scroll(&mut self) {
        if let Some(repo) = &self.state.repo {
            self.state.timeline_scroll = self
                .state
                .timeline_scroll
                .min(repo.timeline.len().saturating_sub(1));
        }
    }

    fn click_file(&mut self, _column: u16, row: u16) {
        let index = row.saturating_sub(4) as usize;
        if let Some(repo) = &mut self.state.repo {
            match self.state.tab {
                Tab::Timeline => {
                    let index = self.state.timeline_scroll.saturating_add(index);
                    if index < repo.timeline.len() {
                        self.state.timeline_selected = index;
                        self.refresh_timeline();
                        self.refresh_timeline_details();
                    }
                }
                Tab::FileHistory => {
                    let index = self.state.history_scroll.saturating_add(index);
                    if index < self.state.history.len() {
                        self.state.history_selected = index;
                        self.refresh_history();
                        self.refresh_history_details();
                    }
                }
                _ => {
                    let index = self.state.files_scroll.saturating_add(index);
                    if index < repo.files.len() {
                        repo.selected_file = index;
                        self.refresh_views();
                    }
                }
            }
        }
    }

    fn mouse_in_left_pane(&self, column: u16, terminal_width: u16) -> bool {
        column < terminal_width.saturating_mul(38) / 100
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        self.refresh();
        loop {
            if self.state.redraw {
                terminal.clear()?;
                self.state.redraw = false;
            }
            terminal.draw(|frame| ui::draw(frame, &mut self.state))?;
            if event::poll(Duration::from_millis(150))? {
                match event::read()? {
                    Event::Key(KeyEvent { code, .. }) => {
                        if self.state.commit_prompt.is_some() {
                            self.handle_commit_input(code);
                            continue;
                        }
                        if self.state.ignore_prompt.is_some() {
                            self.handle_ignore_input(code);
                            continue;
                        }
                        if self.state.discard_prompt.is_some() {
                            self.handle_discard_input(code);
                            continue;
                        }
                        if self.state.repo.is_none() {
                            if let KeyCode::Char('q') = code {
                                break;
                            }
                            continue;
                        }
                        match code {
                            KeyCode::Esc => {
                                self.state.error = None;
                            }
                            KeyCode::Char('q') => break,
                            KeyCode::Char('r') => self.refresh(),
                            KeyCode::Char('p') | KeyCode::Char('P') => self.sync_with_remote(),
                            KeyCode::Up => self.select_prev(),
                            KeyCode::Down => self.select_next(),
                            KeyCode::PageUp => self.scroll_diff_up(),
                            KeyCode::PageDown => self.scroll_diff_down(),
                            KeyCode::Char(' ') => self.toggle_selected_file(),
                            KeyCode::Char('c') => self.start_commit(CommitTarget::Selected),
                            KeyCode::Char('f') => self.start_commit(CommitTarget::Current),
                            KeyCode::Char('a') => self.toggle_select_all(),
                            KeyCode::Char('i') => self.start_ignore(),
                            KeyCode::Char('e') => self.open_in_editor(),
                            KeyCode::Char('d') => self.start_discard(),
                            KeyCode::Char('o') => self.open_current_file(),
                            KeyCode::Char('H') => {
                                self.state.show_hex = !self.state.show_hex;
                                self.refresh_views();
                            }
                            KeyCode::Tab => {
                                let prev_path = self.selected_file_path();
                                self.state.tab = match self.state.tab {
                                    Tab::Changes => Tab::WorkingTree,
                                    Tab::WorkingTree => Tab::FileHistory,
                                    Tab::FileHistory => Tab::Timeline,
                                    Tab::Timeline => Tab::Changes,
                                };
                                if matches!(self.state.tab, Tab::WorkingTree) {
                                    if let Some(path) = prev_path.as_deref() {
                                        self.select_file_by_path(path);
                                    }
                                } else if matches!(self.state.tab, Tab::Changes) {
                                    self.center_changes_scroll();
                                }
                                match self.state.tab {
                                    Tab::Timeline => self.refresh_timeline(),
                                    Tab::FileHistory => self.refresh_history(),
                                    Tab::WorkingTree | Tab::Changes => self.refresh_diff(),
                                }
                                if matches!(self.state.tab, Tab::FileHistory) {
                                    self.refresh_history_details();
                                }
                            }
                            _ => {}
                        }
                    }
                    Event::Mouse(mouse) => {
                        let terminal_width = terminal.size().map(|s| s.width).unwrap_or(100);
                        match mouse.kind {
                            MouseEventKind::ScrollUp => {
                                if self.mouse_in_left_pane(mouse.column, terminal_width) {
                                    self.select_prev();
                                } else {
                                    self.scroll_diff_up();
                                }
                            }
                            MouseEventKind::ScrollDown => {
                                if self.mouse_in_left_pane(mouse.column, terminal_width) {
                                    self.select_next();
                                } else {
                                    self.scroll_diff_down();
                                }
                            }
                            MouseEventKind::Down(_)
                                if self.mouse_in_left_pane(mouse.column, terminal_width) =>
                            {
                                self.click_file(mouse.column, mouse.row);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn is_binary_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".png", ".jpg", ".jpeg", ".gif", ".ico"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

fn preview_kind_for_path(path: &str) -> PreviewKind {
    if path.ends_with(".md") {
        PreviewKind::Markdown
    } else if path.ends_with(".toml") {
        PreviewKind::Toml
    } else if path.ends_with(".json") || path.ends_with(".json5") {
        PreviewKind::Json
    } else if is_source_path(path) {
        PreviewKind::Source
    } else {
        PreviewKind::Plain
    }
}

fn is_source_path(path: &str) -> bool {
    path == "Makefile"
        || path == "makefile"
        || path == "GNUmakefile"
        || path == "Dockerfile"
        || path.ends_with(".mk")
        || path.ends_with(".make")
        || path.ends_with(".cmake")
        || path.ends_with(".gradle")
        || path.ends_with(".gradle.kts")
        || path.ends_with(".nix")
        || path.ends_with(".tf")
        || path.ends_with(".tfvars")
        || path.ends_with(".yaml")
        || path.ends_with(".yml")
        || path.ends_with(".rs")
        || path.ends_with(".c")
        || path.ends_with(".cc")
        || path.ends_with(".cpp")
        || path.ends_with(".cxx")
        || path.ends_with(".h")
        || path.ends_with(".hpp")
        || path.ends_with(".hh")
        || path.ends_with(".m")
        || path.ends_with(".mm")
        || path.ends_with(".s")
        || path.ends_with(".S")
        || path.ends_with(".asm")
        || path.ends_with(".java")
        || path.ends_with(".js")
        || path.ends_with(".jsx")
        || path.ends_with(".ts")
        || path.ends_with(".tsx")
        || path.ends_with(".go")
        || path.ends_with(".py")
        || path.ends_with(".rb")
        || path.ends_with(".pl")
        || path.ends_with(".pm")
        || path.ends_with(".php")
        || path.ends_with(".swift")
        || path.ends_with(".scala")
        || path.ends_with(".kt")
        || path.ends_with(".kts")
        || path.ends_with(".lua")
        || path.ends_with(".sh")
        || path.ends_with(".bash")
        || path.ends_with(".zsh")
}

fn open_command_for(path: &str, editor: Option<&str>) -> Option<String> {
    let ext = Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
    let cmd = match ext.as_str() {
        "txt" | "md" | "rs" | "toml" | "log" => editor.unwrap_or("vi").to_string(),
        "png" | "jpg" | "jpeg" | "gif" | "ico" => "xdg-open".to_string(),
        "pdf" => "xdg-open".to_string(),
        _ => "xdg-open".to_string(),
    };
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fossil::FileStatus;

    fn repo() -> RepoState {
        RepoState {
            files: vec![
                FileStatus {
                    path: "tracked.txt".into(),
                    status: "edited".into(),
                },
                FileStatus {
                    path: "extra.txt".into(),
                    status: "extra".into(),
                },
            ],
            timeline: vec![],
            selected_file: 0,
        }
    }

    #[test]
    fn toggles_selection_in_memory() {
        let mut app = App::new(false);
        app.state.repo = Some(repo());

        app.toggle_selected_file();
        assert_eq!(app.state.selected_files, vec!["tracked.txt"]);

        app.toggle_selected_file();
        assert!(app.state.selected_files.is_empty());
    }

    #[test]
    fn toggles_select_all_and_none() {
        let mut app = App::new(false);
        app.state.repo = Some(repo());

        app.toggle_select_all();
        assert_eq!(app.state.selected_files, vec!["tracked.txt", "extra.txt"]);

        app.toggle_select_all();
        assert!(app.state.selected_files.is_empty());
    }

    #[test]
    fn current_file_path_tracks_selection() {
        let mut app = App::new(false);
        app.state.repo = Some(repo());
        assert_eq!(app.current_file_path().as_deref(), Some("tracked.txt"));
        app.state.repo.as_mut().unwrap().selected_file = 1;
        assert_eq!(app.current_file_path().as_deref(), Some("extra.txt"));
    }

    #[test]
    fn start_commit_initializes_prompt() {
        let mut app = App::new(false);
        app.start_commit(CommitTarget::Selected);
        assert_eq!(app.state.commit_target, CommitTarget::Selected);
        assert_eq!(app.state.commit_prompt.as_deref(), Some(""));
    }

    #[test]
    fn handles_commit_input_buffer() {
        let mut app = App::new(false);
        app.state.commit_prompt = Some(String::new());
        app.handle_commit_input(KeyCode::Char('a'));
        app.handle_commit_input(KeyCode::Char('b'));
        app.handle_commit_input(KeyCode::Backspace);
        assert_eq!(app.state.commit_prompt.as_deref(), Some("a"));
    }

    #[test]
    fn start_ignore_and_cancel() {
        let mut app = App::new(false);
        app.state.repo = Some(repo());
        app.start_ignore();
        assert_eq!(app.state.ignore_prompt.as_deref(), Some("tracked.txt"));
        app.handle_ignore_input(KeyCode::Char('n'));
        assert!(app.state.ignore_prompt.is_none());
    }

    #[test]
    fn mouse_left_pane_detection_uses_terminal_split() {
        let app = App::new(false);
        assert!(app.mouse_in_left_pane(20, 100));
        assert!(app.mouse_in_left_pane(37, 100));
        assert!(!app.mouse_in_left_pane(40, 100));
    }

    #[test]
    fn mouse_right_pane_is_not_treated_as_list_click_area() {
        let app = App::new(false);
        assert!(!app.mouse_in_left_pane(80, 100));
    }

    #[test]
    fn click_file_uses_scrolled_offsets() {
        let mut app = App::new(false);
        app.state.repo = Some(repo());
        app.state.files_scroll = 1;
        app.click_file(1, 4);
        assert_eq!(app.state.repo.as_ref().unwrap().selected_file, 1);
    }

    #[test]
    fn missing_and_conflict_messages_are_clear() {
        let app = App::new(false);
        assert!(app
            .missing_file_message("gone.txt")
            .contains("Missing file [[gone.txt]]"));
        assert!(app
            .missing_file_message("gone.txt")
            .contains("Try discard ([d]) or commit ([f]/[c])"));
        assert!(app
            .conflict_message("conflict.txt")
            .contains("Resolve the conflict"));
    }

    #[test]
    fn notice_messages_use_marker_highlights() {
        let app = App::new(false);
        let missing = app.missing_file_message("gone.txt");
        assert!(missing.contains("[[gone.txt]]"));
        let conflict = app.conflict_message("conflict.txt");
        assert!(conflict.contains("[[conflict.txt]]"));
    }


    #[test]
    fn preview_kind_matches_file_type() {
        assert!(matches!(
            preview_kind_for_path("README.md"),
            PreviewKind::Markdown
        ));
        assert!(matches!(
            preview_kind_for_path("Cargo.toml"),
            PreviewKind::Toml
        ));
        assert!(matches!(
            preview_kind_for_path("data.json"),
            PreviewKind::Json
        ));
        assert!(matches!(
            preview_kind_for_path("src/main.rs"),
            PreviewKind::Source
        ));
        assert!(matches!(
            preview_kind_for_path("Makefile"),
            PreviewKind::Source
        ));
        assert!(matches!(
            preview_kind_for_path("Dockerfile"),
            PreviewKind::Source
        ));
        assert!(matches!(
            preview_kind_for_path("notes.txt"),
            PreviewKind::Plain
        ));
    }
}
