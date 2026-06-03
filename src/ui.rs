use crate::app::{AppState, CommitTarget, Tab};
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap};

pub fn draw(frame: &mut Frame, state: &AppState) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let tabs = Tabs::new(vec!["Working tree", "History"])
        .select(if state.tab == Tab::WorkingTree { 0 } else { 1 })
        .block(Block::default().borders(Borders::ALL).title("lazyfossil"));
    frame.render_widget(tabs, areas[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(areas[1]);

    let mut file_state = ListState::default();
    let left = if let Some(repo) = &state.repo {
        file_state.select(Some(repo.selected_file));
        let items: Vec<ListItem> = repo.files.iter().enumerate().map(|(i, f)| {
            let prefix = if i == repo.selected_file { ">" } else { " " };
            let selected = if state.selected_files.iter().any(|p| p == &f.path) { "*" } else { " " };
            let kind = match f.status.as_str() { "extra" => "E", "edited" => "M", "added" => "A", "deleted" => "D", _ => "?" };
            ListItem::new(format!("{}{} {} {}", prefix, selected, kind, f.path))
        }).collect();
        List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(Block::default().borders(Borders::ALL).title("Files"))
    } else {
        List::new(vec![ListItem::new("No repository detected")])
            .block(Block::default().borders(Borders::ALL).title("Files"))
    };

    let right = if let Some(repo) = &state.repo {
        match state.tab {
            Tab::WorkingTree => {
                let diff = state.diff.clone().unwrap_or_else(|| "Select a file to view diff".to_string());
                Paragraph::new(color_diff(diff))
                    .scroll((state.diff_scroll, 0))
                    .block(Block::default().borders(Borders::ALL).title("Details"))
                    .wrap(Wrap { trim: false })
            }
            Tab::History => {
                let lines = if repo.timeline.is_empty() {
                    vec![Line::from("No history entries found")]
                } else {
                    repo.timeline.iter().take(12).flat_map(|t| [Line::from(format!("{} {}", t.rid, t.message)), Line::from(format!("  {}  {}", t.user, t.date)), Line::from("")]).collect::<Vec<_>>()
                };
                Paragraph::new(Text::from(lines)).block(Block::default().borders(Borders::ALL).title("Details")).wrap(Wrap { trim: true })
            }
        }
    } else {
        Paragraph::new(state.error.clone().unwrap_or_else(|| "Open a Fossil checkout to begin".to_string()))
            .block(Block::default().borders(Borders::ALL).title("Details"))
    };

    frame.render_stateful_widget(left, body[0], &mut file_state);
    frame.render_widget(right, body[1]);

    let footer = if let Some(msg) = &state.commit_prompt {
        let target = match state.commit_target {
            CommitTarget::Selected => "selected",
            CommitTarget::Current => "current",
            CommitTarget::All => "all",
        };
        Paragraph::new(format!("commit {}: {}", target, msg)).block(Block::default().borders(Borders::ALL))
    } else {
        let sel_count = state.selected_files.len();
        let base = if let Some(repo) = &state.repo {
            repo.files.get(repo.selected_file).map(|f| format!("q quit  r refresh  space toggle  c commit selected  f commit file  a commit all  tab switch view | selected: {} [{}]", f.path, f.status)).unwrap_or_else(|| "q quit  r refresh  space toggle  c commit selected  f commit file  a commit all  tab switch view".to_string())
        } else {
            "q quit  r refresh  space toggle  c commit selected  f commit file  a commit all  tab switch view".to_string()
        };
        Paragraph::new(format!("{}\nselected: {}", base, sel_count)).block(Block::default().borders(Borders::TOP))
    };

    frame.render_widget(footer, areas[2]);
}

fn color_diff(diff: String) -> Text<'static> {
    Text::from(diff.lines().map(|line| {
        let style = if line.starts_with("+++") || line.starts_with("---") { Style::default().fg(Color::Blue) }
        else if line.starts_with("@@") { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) }
        else if line.starts_with('+') { Style::default().fg(Color::Green) }
        else if line.starts_with('-') { Style::default().fg(Color::Red) }
        else { Style::default().fg(Color::Reset) };
        Line::from(Span::styled(line.to_string(), style))
    }).collect::<Vec<_>>())
}
