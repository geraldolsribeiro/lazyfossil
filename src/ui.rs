use crate::app::{AppState, Tab};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap};

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

    let left = if let Some(repo) = &state.repo {
        let items: Vec<ListItem> = repo
            .files
            .iter()
            .map(|f| ListItem::new(format!("{}  {}", f.status, f.path)))
            .collect();
        List::new(items).block(Block::default().borders(Borders::ALL).title("Files"))
    } else {
        List::new(vec![ListItem::new("No repository detected")])
            .block(Block::default().borders(Borders::ALL).title("Files"))
    };

    let right_text = if let Some(repo) = &state.repo {
        match state.tab {
            Tab::WorkingTree => "Diff preview goes here".to_string(),
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

    frame.render_widget(left, body[0]);
    frame.render_widget(right, body[1]);

    let footer = Paragraph::new("q quit  r refresh  tab switch view")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, areas[2]);
}
