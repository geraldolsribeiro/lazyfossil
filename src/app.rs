use crate::fossil::{FossilClient, FossilError, RepoState};
use crate::ui;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = App::new().run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
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
}

impl App {
    fn new() -> Self {
        Self {
            client: FossilClient::new(),
            state: AppState {
                tab: Tab::WorkingTree,
                repo: None,
                error: None,
            },
        }
    }

    fn refresh(&mut self) {
        match self.client.repo_state() {
            Ok(repo) => {
                self.state.repo = Some(repo);
                self.state.error = None;
            }
            Err(FossilError::NotRepository) => {
                self.state.repo = None;
                self.state.error = Some("Not inside a Fossil checkout".to_string());
            }
            Err(err) => self.state.error = Some(err.to_string()),
        }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        self.refresh();
        loop {
            terminal.draw(|frame| ui::draw(frame, &self.state))?;

            if event::poll(Duration::from_millis(150))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => self.refresh(),
                        KeyCode::Tab => {
                            self.state.tab = match self.state.tab {
                                Tab::WorkingTree => Tab::History,
                                Tab::History => Tab::WorkingTree,
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
