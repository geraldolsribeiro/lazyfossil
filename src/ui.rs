use crate::app::{AppState, Tab};
use ratatui::prelude::*;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap};

pub fn draw(frame: &mut Frame, state: &AppState) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(frame.area());

    let tabs = Tabs::new(vec!["Working tree", "History"])
        .select(if state.tab == Tab::WorkingTree { 0 } else { 1 })
        .block(Block::default().borders(Borders::ALL).title("pi-lazyfossil"));
    frame.render_widget(tabs, areas[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(areas[1]);

    let mut file_state = ListState::default();
    let left = if let Some(repo) = &state.repo {
        file_state.select(Some(repo.selected_file));
        let items: Vec<ListItem> = repo
            .files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let prefix = if i == repo.selected_file { ">" } else { " " };
                ListItem::new(format!("{} {}  {}", prefix, f.status, f.path))
            })
            .collect();
        List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(Block::default().borders(Borders::ALL).title("Files"))
    } else {
        List::new(vec![ListItem::new("No repository detected")])
            .block(Block::default().borders(Borders::ALL).title("Files"))
    };

    let right_text = if let Some(repo) = &state.repo {
        match state.tab {
            Tab::WorkingTree => state.diff.clone().unwrap_or_else(|| "Select a file to view diff".to_string()),
            Tab::History => repo
                .timeline
                .iter()
                .take(12)
                .map(|t| format!("{} {}", t.rid, t.message))
                .collect::<Vec<_>>()
                .join("\n"),
        }
    } else {
        state.error.clone().unwrap_or_else(|| "Open a Fossil checkout to begin".to_string())
    };

    let right = Paragraph::new(right_text)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .wrap(Wrap { trim: true });

    frame.render_stateful_widget(left, body[0], &mut file_state);
    frame.render_widget(right, body[1]);

    let footer_text = if let Some(repo) = &state.repo {
        repo.files
            .get(repo.selected_file)
            .map(|f| format!("q quit  r refresh  tab switch view  | selected: {} ({})", f.path, f.status))
            .unwrap_or_else(|| "q quit  r refresh  tab switch view".to_string())
    } else {
        "q quit  r refresh  tab switch view".to_string()
    };

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, areas[2]);
}
