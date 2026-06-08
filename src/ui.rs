use crate::app::{AppState, CommitTarget, Tab};
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap};

pub fn draw(frame: &mut Frame, state: &AppState) {
    let mut cursor = None;
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(4)])
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
            let kind = match f.status.as_str() { "extra" => "??", "edited" => "M", "added" => "A", "deleted" => "D", "conflict" => "C", _ => "✓" };
            let mut item = ListItem::new(format!("{}{} {}", prefix, selected, format!("{} {}", kind, f.path)));
            if f.status == "checked-out" {
                item = item.style(Style::default().fg(Color::Green));
            } else if f.status == "edited" {
                item = item.style(Style::default().fg(Color::LightRed));
            }
            item
        }).collect();
        List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(Block::default().borders(Borders::ALL).title("Files"))
    } else {
        List::new(vec![ListItem::new("No repository detected")])
            .block(Block::default().borders(Borders::ALL).title("Files"))
    };

    let right = if state.repo.is_some() {
        match state.tab {
            Tab::WorkingTree => {
                let diff = state.diff.clone().unwrap_or_else(|| "Select a file to view diff".to_string());
                Paragraph::new(color_diff(diff))
                    .scroll((state.diff_scroll, 0))
                    .block(Block::default().borders(Borders::ALL).title("Details"))
                    .wrap(Wrap { trim: false })
            }
            Tab::History => {
                let lines = if state.history.is_empty() {
                    vec![Line::from("No history entries found")]
                } else {
                    state.history.iter().take(12).flat_map(|t| [Line::from(format!("{} {}", t.rid, t.message)), Line::from(format!("  {}  {}", t.user, t.date)), Line::from("")]).collect::<Vec<_>>()
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
        let text = Text::from(vec![
            Line::from(vec![
                Span::raw("commit "),
                Span::styled(target, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(": "),
                Span::styled(msg.clone(), Style::default().bg(Color::DarkGray).fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" cancel · "),
                Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" confirm"),
            ]),
        ]);
        cursor = Some((areas[2].x + 1 + 7 + target.len() as u16 + 2 + msg.chars().count() as u16, areas[2].y + 1));
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Commit"))
    } else if let Some(path) = &state.ignore_prompt {
        let text = Text::from(vec![
            Line::from(vec![
                Span::raw("ignore "),
                Span::styled(path.clone(), Style::default().bg(Color::DarkGray).fg(Color::White)),
                Span::raw("?"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" cancel"),
            ]),
        ]);
        cursor = Some((areas[2].x + 1 + 7 + path.chars().count() as u16, areas[2].y + 1));
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Ignore"))
    } else if let Some(path) = &state.discard_prompt {
        let text = Text::from(vec![
            Line::from(vec![
                Span::raw("discard changes in "),
                Span::styled(path.clone(), Style::default().bg(Color::DarkGray).fg(Color::White)),
                Span::raw("?"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" cancel · "),
                Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" confirm"),
            ]),
        ]);
        cursor = Some((areas[2].x + 1 + 19 + path.chars().count() as u16, areas[2].y + 1));
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Discard"))
    } else {
        let sel_count = state.selected_files.len();
        let mut lines = vec![Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" quit  "),
            Span::styled("r", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" refresh  "),
            Span::styled("Space", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" select  "),
            Span::styled("i", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" ignore  "),
            Span::styled("c", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" commit  "),
            Span::styled("p", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" pull  "),
            Span::styled("e", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" edit  "),
            Span::styled("o", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" open  "),
            Span::styled("d", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" discard"),
        ])];
        if let Some(repo) = &state.repo {
            if let Some(f) = repo.files.get(repo.selected_file) {
                lines.push(Line::from(vec![
                    Span::styled("selected", Style::default().fg(Color::DarkGray)),
                    Span::raw(": "),
                    Span::styled(f.path.clone(), Style::default().fg(Color::White)),
                    Span::raw(" ["),
                    Span::styled(f.status.clone(), Style::default().fg(Color::DarkGray)),
                    Span::raw("]"),
                ]));
            }
        }
        lines.push(Line::from(vec![
            Span::styled("selected files", Style::default().fg(Color::DarkGray)),
            Span::raw(": "),
            Span::styled(sel_count.to_string(), Style::default().fg(Color::White)),
        ]));
        Paragraph::new(Text::from(lines)).block(Block::default().borders(Borders::TOP))
    };

    frame.render_widget(footer, areas[2]);
    if let Some(error) = &state.error {
        let popup_area = centered_rect(60, 20, frame.area());
        let popup = Paragraph::new(Text::from(vec![
            Line::from(error.clone()),
            Line::from(""),
            Line::from(Span::styled("Press Esc to dismiss", Style::default().fg(Color::DarkGray))),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Warning"))
        .wrap(Wrap { trim: true });
        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup, popup_area);
    }
    if let Some((x, y)) = cursor {
        frame.set_cursor_position((x, y));
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn color_diff(diff: String) -> Text<'static> {
    Text::from(diff.lines().map(|line| {
        if line.starts_with("Press [o] to open externally or [H] for a future hex view") {
            return Line::from(vec![
                Span::raw("Press "),
                Span::styled("o", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" to open externally or "),
                Span::styled("H", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" for a future hex view"),
            ]);
        }
        let style = if line.starts_with("Preview unavailable for ") {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if line.starts_with("+++") || line.starts_with("---") { Style::default().fg(Color::Blue) }
        else if line.starts_with("@@") { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) }
        else if line.starts_with('+') { Style::default().fg(Color::Green) }
        else if line.starts_with('-') { Style::default().fg(Color::Red) }
        else { Style::default().fg(Color::Reset) };
        Line::from(Span::styled(line.to_string(), style))
    }).collect::<Vec<_>>())
}
