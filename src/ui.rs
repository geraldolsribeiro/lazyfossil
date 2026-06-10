use crate::app::{AppState, CommitTarget, Tab};
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap};

pub fn draw(frame: &mut Frame, state: &AppState) {
    let mut cursor = None;
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .split(frame.area());

    let tabs = Tabs::new(vec!["Working tree", "File history", "Timeline"])
        .select(match state.tab {
            Tab::WorkingTree => 0,
            Tab::FileHistory => 1,
            Tab::Timeline => 2,
        })
        .block(Block::default().borders(Borders::ALL).title("lazyfossil"));
    frame.render_widget(tabs, areas[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(areas[1]);

    let mut file_state = ListState::default();
    let left = if let Some(repo) = &state.repo {
        match state.tab {
            Tab::Timeline => {
                file_state.select(Some(state.timeline_selected));
                let items: Vec<ListItem> = repo
                    .timeline
                    .iter()
                    .enumerate()
                    .map(|(i, t)| {
                        let prefix = if i == state.timeline_selected {
                            ">"
                        } else {
                            " "
                        };
                        ListItem::new(format!("{}{} {}", prefix, t.rid, t.message))
                    })
                    .collect();
                List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .block(Block::default().borders(Borders::ALL).title("Timeline"))
            }
            Tab::FileHistory => {
                file_state.select(Some(state.history_selected));
                let items: Vec<ListItem> = if state.history.is_empty() {
                    vec![ListItem::new("No history entries found")]
                } else {
                    state
                        .history
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            let prefix = if i == state.history_selected {
                                ">"
                            } else {
                                " "
                            };
                            ListItem::new(format!("{}{} {}", prefix, t.rid, t.message))
                        })
                        .collect()
                };
                List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .block(Block::default().borders(Borders::ALL).title("History"))
            }
            _ => {
                file_state.select(Some(repo.selected_file));
                let items: Vec<ListItem> = repo
                    .files
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let prefix = if i == repo.selected_file { ">" } else { " " };
                        let selected = if state.selected_files.iter().any(|p| p == &f.path) {
                            "*"
                        } else {
                            " "
                        };
                        let kind = match f.status.as_str() {
                            "extra" => "??",
                            "edited" => "M",
                            "added" => "A",
                            "deleted" => "D",
                            "missing" => "!",
                            "conflict" => "C",
                            _ => "✓",
                        };
                        let mut item =
                            ListItem::new(format!("{}{} {} {}", prefix, selected, kind, f.path));
                        if f.status == "checked-out" {
                            item = item.style(Style::default().fg(Color::Green));
                        } else if f.status == "edited" {
                            item = item.style(Style::default().fg(Color::LightRed));
                        } else if f.status == "missing" {
                            item = item.style(Style::default().fg(Color::Red));
                        }
                        item
                    })
                    .collect();
                List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .block(Block::default().borders(Borders::ALL).title("Files"))
            }
        }
    } else {
        List::new(vec![ListItem::new("No repository detected")])
            .block(Block::default().borders(Borders::ALL).title("Files"))
    };

    let right = if state.repo.is_some() {
        match state.tab {
            Tab::WorkingTree => {
                let diff = state
                    .diff
                    .clone()
                    .unwrap_or_else(|| "Select a file to view diff".to_string());
                Paragraph::new(color_diff(diff))
                    .scroll((state.diff_scroll, 0))
                    .block(Block::default().borders(Borders::ALL).title("Details"))
                    .wrap(Wrap { trim: false })
            }
            Tab::FileHistory => {
                let mut lines = Vec::new();
                if let Some(entry) = state.history.get(state.history_selected) {
                    lines.push(Line::from(vec![
                        Span::styled("commit ", Style::default().fg(Color::DarkGray)),
                        Span::styled(entry.rid.clone(), Style::default().fg(Color::Yellow)),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("author ", Style::default().fg(Color::DarkGray)),
                        Span::raw(entry.user.clone()),
                        Span::raw("  "),
                        Span::styled(entry.date.clone(), Style::default().fg(Color::DarkGray)),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("message ", Style::default().fg(Color::DarkGray)),
                        Span::raw(entry.message.clone()),
                    ]));
                    lines.push(Line::from(""));
                }
                let diff = state
                    .history_diff
                    .clone()
                    .unwrap_or_else(|| "No history diff available".to_string());
                lines.extend(color_diff(diff).lines);
                Paragraph::new(Text::from(lines))
                    .scroll((state.diff_scroll, 0))
                    .block(Block::default().borders(Borders::ALL).title("Details"))
                    .wrap(Wrap { trim: false })
            }
            Tab::Timeline => {
                let mut lines = Vec::new();
                if let Some(repo) = &state.repo {
                    if let Some(entry) = repo.timeline.get(state.timeline_selected) {
                        lines.push(Line::from(vec![
                            Span::styled("commit ", Style::default().fg(Color::DarkGray)),
                            Span::styled(entry.rid.clone(), Style::default().fg(Color::Yellow)),
                        ]));
                        lines.push(Line::from(vec![
                            Span::styled("author ", Style::default().fg(Color::DarkGray)),
                            Span::raw(entry.user.clone()),
                            Span::raw("  "),
                            Span::styled(entry.date.clone(), Style::default().fg(Color::DarkGray)),
                        ]));
                        lines.push(Line::from(vec![
                            Span::styled("message ", Style::default().fg(Color::DarkGray)),
                            Span::raw(entry.message.clone()),
                        ]));
                        lines.push(Line::from(""));
                    }
                }
                let diff = state
                    .timeline_diff
                    .clone()
                    .unwrap_or_else(|| "No timeline diff available".to_string());
                lines.extend(color_diff(diff).lines);
                Paragraph::new(Text::from(lines))
                    .scroll((state.diff_scroll, 0))
                    .block(Block::default().borders(Borders::ALL).title("Details"))
                    .wrap(Wrap { trim: false })
            }
        }
    } else {
        Paragraph::new(
            state
                .error
                .clone()
                .unwrap_or_else(|| "Open a Fossil checkout to begin".to_string()),
        )
        .block(Block::default().borders(Borders::ALL).title("Details"))
    };
    frame.render_stateful_widget(left, body[0], &mut file_state);
    frame.render_widget(right, body[1]);

    let footer = if let Some(msg) = &state.commit_prompt {
        let target = match state.commit_target {
            CommitTarget::Selected => "selected",
            CommitTarget::Current => "current",
        };
        let text = Text::from(vec![
            Line::from(vec![
                Span::raw("commit "),
                key_span(target, Color::Yellow),
                Span::raw(": "),
                Span::styled(
                    msg.clone(),
                    Style::default().bg(Color::DarkGray).fg(Color::White),
                ),
            ]),
            confirmation_hint_line(),
        ]);
        cursor = Some((
            areas[2].x + 1 + 7 + target.len() as u16 + 2 + msg.chars().count() as u16,
            areas[2].y + 1,
        ));
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Commit"))
    } else if let Some(path) = &state.ignore_prompt {
        let text = confirmation_prompt("ignore ", path, "?");
        cursor = Some((
            areas[2].x + 1 + 7 + path.chars().count() as u16,
            areas[2].y + 1,
        ));
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Ignore"))
    } else if let Some(path) = &state.discard_prompt {
        let text = confirmation_prompt("discard changes in ", path, "?");
        cursor = Some((
            areas[2].x + 1 + 19 + path.chars().count() as u16,
            areas[2].y + 1,
        ));
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Discard"))
    } else if state.repo.is_none() {
        let lines = vec![Line::from(vec![
            key_span("q", Color::Yellow),
            Span::raw(" quit"),
        ])];
        Paragraph::new(Text::from(lines)).block(Block::default().borders(Borders::TOP))
    } else {
        let sel_count = state.selected_files.len();
        let mut lines = vec![Line::from(vec![
            key_span("q", Color::Yellow),
            Span::raw(" quit  "),
            key_span("r", Color::Yellow),
            Span::raw(" refresh  "),
            key_span("Space", Color::Yellow),
            Span::raw(" select  "),
            key_span("i", Color::Yellow),
            Span::raw(" ignore  "),
            key_span("c", Color::Yellow),
            Span::raw(" commit  "),
            key_span("a", Color::Yellow),
            Span::raw(" all/none  "),
            key_span("p", Color::Yellow),
            Span::raw(" pull  "),
            key_span("e", Color::Yellow),
            Span::raw(" edit  "),
            key_span("o", Color::Yellow),
            Span::raw(" open  "),
            key_span("d", Color::Yellow),
            Span::raw(" discard  "),
            key_span("H", Color::Yellow),
            Span::raw(" hex"),
        ])];
        if state.tab != Tab::Timeline {
            if let Some(repo) = &state.repo {
                if let Some(f) = repo.files.get(repo.selected_file) {
                    lines.push(Line::from(vec![
                        Span::styled("selected", Style::default().fg(Color::DarkGray)),
                        Span::raw(": "),
                        Span::styled(f.path.clone(), Style::default().fg(Color::Yellow)),
                        Span::raw(" ["),
                        Span::styled(f.status.clone(), Style::default().fg(Color::Cyan)),
                        Span::raw("]"),
                    ]));
                }
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
        if state.repo.is_some() {
            let popup_area = centered_rect(60, 20, frame.area());
            let mut lines = error
                .lines()
                .map(|line| Line::from(styled_message_line(line)))
                .collect::<Vec<_>>();
            lines.push(Line::from(Span::styled(
                "Press Esc to dismiss",
                Style::default().fg(Color::DarkGray),
            )));
            let popup = Paragraph::new(Text::from(lines))
                .block(Block::default().borders(Borders::ALL).title("Warning"))
                .wrap(Wrap { trim: true });
            frame.render_widget(Clear, popup_area);
            frame.render_widget(popup, popup_area);
        }
    }
    if state.repo.is_none() {
        let popup_area = centered_rect(72, 42, frame.area());
        let popup = Paragraph::new(Text::from(info_box_lines()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Not a Fossil checkout"),
            )
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

fn confirmation_prompt(prefix: &str, value: &str, suffix: &str) -> Text<'static> {
    Text::from(vec![
        Line::from(vec![
            Span::raw(prefix.to_string()),
            Span::styled(
                value.to_string(),
                Style::default().bg(Color::DarkGray).fg(Color::White),
            ),
            Span::raw(suffix.to_string()),
        ]),
        confirmation_hint_line(),
    ])
}

fn confirmation_hint_line() -> Line<'static> {
    Line::from(vec![
        key_span("Esc", Color::Red),
        Span::raw(" cancel · "),
        key_span("Enter", Color::Green),
        Span::raw(" confirm"),
    ])
}

fn key_span(label: &str, color: Color) -> Span<'static> {
    Span::styled(
        label.to_string(),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

fn styled_message_line(line: &str) -> Vec<Span<'static>> {
    // let mut s = line.to_string();
    // vec![Span::raw(s)]
    let mut spans = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("[[") {
        let (before, after_start) = rest.split_at(start);
        if !before.is_empty() {
            spans.push(Span::raw(before.to_string()));
        }
        let inner = &after_start[2..];
        if let Some(end) = inner.find("]]") {
            let value = &inner[..end];
            spans.push(Span::styled(
                value.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
            rest = &inner[end + 2..];
        } else {
            spans.push(Span::raw(after_start.to_string()));
            rest = "";
            break;
        }
    }
    if !rest.is_empty() {
        spans.push(Span::raw(rest.to_string()));
    }
    if spans.is_empty() {
        vec![Span::raw(line.to_string())]
    } else {
        spans
    }
}

fn info_box_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "lazyfossil could not find a Fossil checkout in this directory.",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "What you can do:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • move into a Fossil checkout and restart"),
        Line::from("  • run `fossil open <repo>` or `fossil checkout <uuid>`"),
        Line::from("  • press q to quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Repository actions are disabled until a checkout is detected.",
            Style::default().fg(Color::DarkGray),
        )]),
    ]
}

fn color_diff(diff: String) -> Text<'static> {
    Text::from(
        diff.lines()
            .map(|line| {
                if line.starts_with("Press [o] to open externally or [H] for hex view") {
                    return Line::from(vec![
                        Span::raw("Press "),
                        key_span("o", Color::Yellow),
                        Span::raw(" to open externally or "),
                        key_span("H", Color::Yellow),
                        Span::raw(" for hex view"),
                    ]);
                }
                let style = if line.starts_with("Preview unavailable for ") {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if line.starts_with("+++") || line.starts_with("---") {
                    Style::default().fg(Color::Blue)
                } else if line.starts_with("@@") {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else if line.starts_with('+') {
                    Style::default().fg(Color::Green)
                } else if line.starts_with('-') {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Reset)
                };
                Line::from(Span::styled(line.to_string(), style))
            })
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirmation_prompt_contains_hint_line() {
        let text = confirmation_prompt("ignore ", "tracked.txt", "?");
        let lines = text.lines.into_iter().collect::<Vec<_>>();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].spans[1].content, "tracked.txt");
        assert_eq!(lines[1].spans[0].content, "Esc");
        assert_eq!(lines[1].spans[2].content, "Enter");
    }

    #[test]
    fn styled_message_line_highlights_markers() {
        let spans = styled_message_line("File [[path/to/file]] renamed");
        assert!(spans
            .iter()
            .any(|span| span.content.as_ref() == "path/to/file"));
        assert!(spans.len() >= 3);
    }

    #[test]
    fn key_span_uses_expected_label() {
        let span = key_span("Esc", Color::Red);
        assert_eq!(span.content, "Esc");
    }

    #[test]
    fn info_box_mentions_checkout_actions() {
        let lines = info_box_lines();
        assert!(lines
            .iter()
            .any(|line| line.to_string().contains("not find a Fossil checkout")));
        assert!(lines
            .iter()
            .any(|line| line.to_string().contains("press q to quit")));
    }
}
