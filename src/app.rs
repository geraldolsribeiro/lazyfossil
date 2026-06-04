use crate::fossil::{FossilClient, FossilError, RepoState};
use crate::ui;
use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs;
use std::io;
use std::time::Duration;

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = App::new().run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    res
}

struct App {
    client: FossilClient,
    state: AppState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    WorkingTree,
    History,
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommitTarget {
    Selected,
    Current,
    All,
}

impl App {
    fn new() -> Self {
        Self {
            client: FossilClient::new(),
            state: AppState {
                tab: Tab::WorkingTree,
                repo: None,
                error: None,
                diff: None,
                diff_scroll: 0,
                selected_files: Vec::new(),
                commit_prompt: None,
                commit_target: CommitTarget::Selected,
                ignore_prompt: None,
            },
        }
    }

    fn refresh(&mut self) {
        match self.client.repo_state() {
            Ok(repo) => {
                self.state.repo = Some(repo);
                self.state.error = None;
                self.state.diff_scroll = 0;
                self.refresh_diff();
            }
            Err(FossilError::NotRepository) => {
                self.state.repo = None;
                self.state.diff = None;
                self.state.diff_scroll = 0;
                self.state.selected_files.clear();
                self.state.error = Some("Not inside a Fossil checkout".to_string());
            }
            Err(err) => self.state.error = Some(err.to_string()),
        }
    }

    fn refresh_diff(&mut self) {
        if let Some(repo) = &self.state.repo {
            if let Some(file) = repo.files.get(repo.selected_file) {
                self.state.diff_scroll = 0;
                self.state.diff = Some(match file.status.as_str() {
                    "extra" => match fs::read_to_string(&file.path) {
                        Ok(content) => {
                            if content.trim().is_empty() { format!("Empty file: {}", file.path) } else { content }
                        }
                        Err(err) => format!("content error for {}: {}", file.path, err),
                    },
                    _ => match self.client.diff_for(&file.path) {
                        Ok(diff) => if diff.trim().is_empty() { format!("No diff for {}", file.path) } else { diff },
                        Err(err) => format!("diff error for {}: {}", file.path, err),
                    },
                });
            } else {
                self.state.diff = Some("No file selected".to_string());
            }
        }
    }

    fn current_file_path(&self) -> Option<String> {
        self.state.repo.as_ref()?.files.get(self.state.repo.as_ref()?.selected_file).map(|f| f.path.clone())
    }

    fn toggle_selected_file(&mut self) {
        let Some(path) = self.current_file_path() else { return; };
        if let Some(pos) = self.state.selected_files.iter().position(|p| p == &path) {
            self.state.selected_files.remove(pos);
        } else {
            self.state.selected_files.push(path);
        }
    }

    fn start_ignore(&mut self) {
        self.state.ignore_prompt = self.current_file_path();
    }

    fn confirm_ignore(&mut self) {
        let Some(path) = self.state.ignore_prompt.take() else { return; };
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
        let Some(message) = self.state.commit_prompt.take() else { return; };
        let message = message.trim().to_string();
        if message.is_empty() {
            self.state.error = Some("Commit message cannot be empty".to_string());
            return;
        }
        let Some(repo) = &self.state.repo else { return; };
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
            CommitTarget::All => repo.files.iter().map(|f| f.path.clone()).collect(),
        };
        if paths.is_empty() {
            self.state.error = Some("No file selected".to_string());
            return;
        }

        let extras: Vec<String> = paths
            .iter()
            .filter_map(|path| repo.files.iter().find(|f| &f.path == path && f.status == "extra").map(|f| f.path.clone()))
            .collect();

        let result = (|| {
            if !extras.is_empty() {
                self.client.add_files(&extras)?;
            }
            self.client.commit_paths(&paths, &message)
        })();

        match result {
            Ok(_) => {
                self.state.selected_files.clear();
                self.refresh();
            }
            Err(err) => self.state.error = Some(err.to_string()),
        }
    }

    fn cancel_commit(&mut self) {
        self.state.commit_prompt = None;
    }

    fn handle_commit_input(&mut self, code: KeyCode) {
        let Some(buf) = self.state.commit_prompt.as_mut() else { return; };
        match code {
            KeyCode::Esc => self.cancel_commit(),
            KeyCode::Enter => self.submit_commit(),
            KeyCode::Backspace => { buf.pop(); }
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

    fn select_prev(&mut self) {
        if let Some(repo) = &mut self.state.repo { if repo.selected_file > 0 { repo.selected_file -= 1; } }
        self.refresh_diff();
    }

    fn scroll_diff_up(&mut self) { self.state.diff_scroll = self.state.diff_scroll.saturating_sub(1); }
    fn scroll_diff_down(&mut self) { self.state.diff_scroll = self.state.diff_scroll.saturating_add(1); }

    fn select_next(&mut self) {
        if let Some(repo) = &mut self.state.repo { if repo.selected_file + 1 < repo.files.len() { repo.selected_file += 1; } }
        self.refresh_diff();
    }

    fn click_file(&mut self, _column: u16, row: u16) {
        let index = row.saturating_sub(4) as usize;
        if let Some(repo) = &mut self.state.repo { if index < repo.files.len() { repo.selected_file = index; self.refresh_diff(); } }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        self.refresh();
        loop {
            terminal.draw(|frame| ui::draw(frame, &self.state))?;
            if event::poll(Duration::from_millis(150))? {
                match event::read()? {
                    Event::Key(KeyEvent { code, .. }) => {
                        if self.state.commit_prompt.is_some() { self.handle_commit_input(code); continue; }
                        if self.state.ignore_prompt.is_some() { self.handle_ignore_input(code); continue; }
                        match code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('r') => self.refresh(),
                            KeyCode::Up => self.select_prev(),
                            KeyCode::Down => self.select_next(),
                            KeyCode::PageUp => self.scroll_diff_up(),
                            KeyCode::PageDown => self.scroll_diff_down(),
                            KeyCode::Char(' ') => self.toggle_selected_file(),
                            KeyCode::Char('c') => self.start_commit(CommitTarget::Selected),
                            KeyCode::Char('f') => self.start_commit(CommitTarget::Current),
                            KeyCode::Char('a') => self.start_commit(CommitTarget::All),
                            KeyCode::Char('i') => self.start_ignore(),
                            KeyCode::Tab => self.state.tab = match self.state.tab { Tab::WorkingTree => Tab::History, Tab::History => Tab::WorkingTree },
                            _ => {}
                        }
                    }
                    Event::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::ScrollUp => self.scroll_diff_up(),
                        MouseEventKind::ScrollDown => self.scroll_diff_down(),
                        MouseEventKind::Down(_) => self.click_file(mouse.column, mouse.row),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fossil::FileStatus;

    fn repo() -> RepoState {
        RepoState {
            files: vec![
                FileStatus { path: "tracked.txt".into(), status: "edited".into() },
                FileStatus { path: "extra.txt".into(), status: "extra".into() },
            ],
            timeline: vec![],
            selected_file: 0,
        }
    }

    #[test]
    fn toggles_selection_in_memory() {
        let mut app = App::new();
        app.state.repo = Some(repo());

        app.toggle_selected_file();
        assert_eq!(app.state.selected_files, vec!["tracked.txt"]);

        app.toggle_selected_file();
        assert!(app.state.selected_files.is_empty());
    }

    #[test]
    fn current_file_path_tracks_selection() {
        let mut app = App::new();
        app.state.repo = Some(repo());
        assert_eq!(app.current_file_path().as_deref(), Some("tracked.txt"));
        app.state.repo.as_mut().unwrap().selected_file = 1;
        assert_eq!(app.current_file_path().as_deref(), Some("extra.txt"));
    }

    #[test]
    fn start_commit_initializes_prompt() {
        let mut app = App::new();
        app.start_commit(CommitTarget::Selected);
        assert_eq!(app.state.commit_target, CommitTarget::Selected);
        assert_eq!(app.state.commit_prompt.as_deref(), Some(""));
    }

    #[test]
    fn handles_commit_input_buffer() {
        let mut app = App::new();
        app.state.commit_prompt = Some(String::new());
        app.handle_commit_input(KeyCode::Char('a'));
        app.handle_commit_input(KeyCode::Char('b'));
        app.handle_commit_input(KeyCode::Backspace);
        assert_eq!(app.state.commit_prompt.as_deref(), Some("a"));
    }

    #[test]
    fn start_ignore_and_cancel() {
        let mut app = App::new();
        app.state.repo = Some(repo());
        app.start_ignore();
        assert_eq!(app.state.ignore_prompt.as_deref(), Some("tracked.txt"));
        app.handle_ignore_input(KeyCode::Char('n'));
        assert!(app.state.ignore_prompt.is_none());
    }
}
