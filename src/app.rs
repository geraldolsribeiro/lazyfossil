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
                            if content.trim().is_empty() {
                                format!("Empty file: {}", file.path)
                            } else {
                                content
                            }
                        }
                        Err(err) => format!("content error for {}: {}", file.path, err),
                    },
                    _ => match self.client.diff_for(&file.path) {
                        Ok(diff) => {
                            if diff.trim().is_empty() {
                                format!("No diff for {}", file.path)
                            } else {
                                diff
                            }
                        }
                        Err(err) => format!("diff error for {}: {}", file.path, err),
                    },
                });
            } else {
                self.state.diff = Some("No file selected".to_string());
            }
        }
    }

    fn select_prev(&mut self) {
        if let Some(repo) = &mut self.state.repo {
            if repo.selected_file > 0 {
                repo.selected_file -= 1;
            }
        }
        self.refresh_diff();
    }

    fn scroll_diff_up(&mut self) {
        self.state.diff_scroll = self.state.diff_scroll.saturating_sub(1);
    }

    fn scroll_diff_down(&mut self) {
        self.state.diff_scroll = self.state.diff_scroll.saturating_add(1);
    }

    fn select_next(&mut self) {
        if let Some(repo) = &mut self.state.repo {
            if repo.selected_file + 1 < repo.files.len() {
                repo.selected_file += 1;
            }
        }
        self.refresh_diff();
    }

    fn click_file(&mut self, _column: u16, row: u16) {
        let index = row.saturating_sub(4) as usize;
        if let Some(repo) = &mut self.state.repo {
            if index < repo.files.len() {
                repo.selected_file = index;
                self.refresh_diff();
            }
        }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        self.refresh();
        loop {
            terminal.draw(|frame| ui::draw(frame, &self.state))?;

            if event::poll(Duration::from_millis(150))? {
                match event::read()? {
                    Event::Key(KeyEvent { code, .. }) => match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => self.refresh(),
                        KeyCode::Up => self.select_prev(),
                        KeyCode::Down => self.select_next(),
                        KeyCode::PageUp => self.scroll_diff_up(),
                        KeyCode::PageDown => self.scroll_diff_down(),
                        KeyCode::Tab => {
                            self.state.tab = match self.state.tab {
                                Tab::WorkingTree => Tab::History,
                                Tab::History => Tab::WorkingTree,
                            }
                        }
                        _ => {}
                    },
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
