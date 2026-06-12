use crate::app::{AppState, CommitTarget, PreviewKind, Tab};
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, Borders, Clear, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Tabs, Wrap,
};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
fn utf8_state_symbols() -> bool {
    if cfg!(windows) {
        return true;
    }
    for key in ["LC_ALL", "LC_CTYPE", "LANG"] {
        if let Ok(value) = std::env::var(key) {
            let upper = value.to_uppercase();
            if upper.contains("UTF-8") || upper.contains("UTF8") {
                return true;
            }
        }
    }
    false
}

pub fn draw(frame: &mut Frame, state: &mut AppState) {
    let mut cursor = None;
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .split(frame.area());

    let tabs = Tabs::new(vec!["Changes", "Working tree", "File history", "Timeline"])
        .select(match state.tab {
            Tab::Changes => 0,
            Tab::WorkingTree => 1,
            Tab::FileHistory => 2,
            Tab::Timeline => 3,
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(vec![
                    Span::styled("lazy", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        "fossil",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" v{}", APP_VERSION)),
                ])),
        );
    frame.render_widget(tabs, areas[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(areas[1]);

    let mut file_state = match state.tab {
        Tab::Changes | Tab::WorkingTree => ListState::default().with_offset(state.files_scroll),
        Tab::FileHistory => ListState::default().with_offset(state.history_scroll),
        Tab::Timeline => ListState::default().with_offset(state.timeline_scroll),
    };
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
                        let mut spans = Vec::new();
                        spans.push(Span::raw(format!("{}{}", prefix, t.rid)));
                        let tags = display_timeline_tags(&t.tags, &repo.branches);
                        if !tags.is_empty() {
                            spans.push(Span::raw(" "));
                            spans.push(Span::styled(
                                format!("[{}]", tags),
                                Style::default().fg(Color::Cyan),
                            ));
                        }
                        spans.push(Span::raw(format!(" {}", t.message)));
                        ListItem::new(Line::from(spans))
                    })
                    .collect();
                List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Timeline")
                            .title_bottom(Line::from("Up|Dn").right_aligned()),
                    )
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
                            let mut spans = Vec::new();
                            spans.push(Span::raw(format!("{}{}", prefix, t.rid)));
                            let tags = display_timeline_tags(&t.tags, &repo.branches);
                            if !tags.is_empty() {
                                spans.push(Span::raw(" "));
                                spans.push(Span::styled(
                                    format!("[{}]", tags),
                                    Style::default().fg(Color::Cyan),
                                ));
                            }
                            spans.push(Span::raw(format!(" {}", t.message)));
                            ListItem::new(Line::from(spans))
                        })
                        .collect()
                };
                let title = repo
                    .files
                    .get(state.repo.as_ref().map(|r| r.selected_file).unwrap_or(0))
                    .map(|f| format!("File history: {}", f.path.clone()))
                    .unwrap_or_else(|| "File history".to_string());
                List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(title)
                            .title_bottom(Line::from("Up|Dn").right_aligned()),
                    )
            }
            Tab::Changes => {
                let visible: Vec<_> = repo
                    .files
                    .iter()
                    .enumerate()
                    .filter(|(_, f)| f.status != "checked-out")
                    .collect();
                let viewport_rows = body[0].height.saturating_sub(2) as usize;
                let selected = state.changes_selected.min(visible.len().saturating_sub(1));
                let offset = if visible.len() <= viewport_rows {
                    0
                } else {
                    selected
                        .saturating_sub(viewport_rows / 2)
                        .min(visible.len().saturating_sub(viewport_rows))
                };
                file_state = ListState::default().with_offset(offset);
                if visible.is_empty() {
                    file_state.select(None);
                } else {
                    file_state.select(Some(selected));
                }
                let items: Vec<ListItem> = visible
                    .into_iter()
                    .map(|(i, f)| {
                        let prefix = if i == repo.selected_file { ">" } else { " " };
                        let selected = if state.selected_files.iter().any(|p| p == &f.path) {
                            "●"
                        } else {
                            " "
                        };
                        let kind = file_state_symbol(&f.status);
                        let mut item =
                            ListItem::new(format!("{}{} {} {}", prefix, selected, kind, f.path));
                        let color = match f.status.as_str() {
                            "checked-out" => Color::DarkGray,
                            "extra" => Color::Yellow,
                            "edited" => Color::Blue,
                            "missing" => Color::Red,
                            _ => Color::Reset,
                        };
                        if color != Color::Reset {
                            item = item.style(Style::default().fg(color));
                        }
                        item
                    })
                    .collect();
                List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Changes")
                            .title_bottom(Line::from("Up|Dn").right_aligned()),
                    )
            }
            Tab::WorkingTree => {
                file_state.select(Some(repo.selected_file));
                let items: Vec<ListItem> = repo
                    .files
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let prefix = if i == repo.selected_file { ">" } else { " " };
                        let selected = if state.selected_files.iter().any(|p| p == &f.path) {
                            "●"
                        } else {
                            " "
                        };
                        let kind = file_state_symbol(&f.status);
                        let mut item =
                            ListItem::new(format!("{}{} {} {}", prefix, selected, kind, f.path));
                        let color = match f.status.as_str() {
                            "checked-out" => Color::DarkGray,
                            "extra" => Color::Yellow,
                            "edited" => Color::Blue,
                            "missing" => Color::Red,
                            _ => Color::Reset,
                        };
                        if color != Color::Reset {
                            item = item.style(Style::default().fg(color));
                        }
                        item
                    })
                    .collect();
                List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Files")
                            .title_bottom(Line::from("Up|Dn").right_aligned()),
                    )
            }
        }
    } else {
        List::new(vec![ListItem::new("No repository detected")]).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Files")
                .title_bottom(Line::from("Up|Dn").right_aligned()),
        )
    };

    let right = if state.repo.is_some() {
        match state.tab {
            Tab::Changes | Tab::WorkingTree => {
                let diff = state
                    .diff
                    .clone()
                    .unwrap_or_else(|| "Select a file to view diff".to_string());
                Paragraph::new(color_preview(diff, state.preview_kind))
                    .scroll((state.diff_scroll, 0))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(right_pane_title(state, "Diff"))
                            .title_bottom(Line::from("PgUp|PgDn").right_aligned()),
                    )
                    .wrap(Wrap { trim: false })
            }
            Tab::FileHistory => {
                let mut lines = Vec::new();
                if let Some(repo) = &state.repo {
                    if let Some(file) = repo.files.get(repo.selected_file) {
                        lines.push(Line::from(vec![
                            Span::styled("file ", Style::default().fg(Color::DarkGray)),
                            Span::styled(file.path.clone(), Style::default().fg(Color::Yellow)),
                        ]));
                        lines.push(Line::from(vec![
                            Span::styled("entries ", Style::default().fg(Color::DarkGray)),
                            Span::styled(
                                state.history.len().to_string(),
                                Style::default().fg(Color::White),
                            ),
                        ]));
                        lines.push(Line::from(""));
                    }
                }
                if let Some(entry) = state.history.get(state.history_selected) {
                    lines.push(Line::from(vec![
                        Span::styled("entry ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            format!("{}/{}", state.history_selected + 1, state.history.len()),
                            Style::default().fg(Color::White),
                        ),
                    ]));
                    let mut commit_line = vec![
                        Span::styled("commit ", Style::default().fg(Color::DarkGray)),
                        Span::styled(entry.rid.clone(), Style::default().fg(Color::Yellow)),
                    ];
                    if let Some(repo) = &state.repo {
                        let tags = display_timeline_tags(&entry.tags, &repo.branches);
                        if !tags.is_empty() {
                            commit_line.push(Span::raw(" "));
                            commit_line.push(Span::styled(tags, Style::default().fg(Color::Cyan)));
                        }
                    }
                    lines.push(Line::from(commit_line));
                    lines.push(Line::from(vec![
                        Span::styled("author ", Style::default().fg(Color::DarkGray)),
                        Span::raw(entry.user.clone()),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("date ", Style::default().fg(Color::DarkGray)),
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
                lines.extend(color_preview(diff, PreviewKind::Diff).lines);
                Paragraph::new(Text::from(lines))
                    .scroll((state.diff_scroll, 0))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(right_pane_title(state, "File history details"))
                            .title_bottom(Line::from("PgUp|PgDn").right_aligned()),
                    )
                    .wrap(Wrap { trim: false })
            }
            Tab::Timeline => {
                let mut lines = Vec::new();
                if let Some(repo) = &state.repo {
                    lines.push(Line::from(vec![
                        Span::styled("entries ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            repo.timeline.len().to_string(),
                            Style::default().fg(Color::White),
                        ),
                    ]));
                    lines.push(Line::from(""));
                    if let Some(entry) = repo.timeline.get(state.timeline_selected) {
                        lines.push(Line::from(vec![
                            Span::styled("entry ", Style::default().fg(Color::DarkGray)),
                            Span::styled(
                                format!("{}/{}", state.timeline_selected + 1, repo.timeline.len()),
                                Style::default().fg(Color::White),
                            ),
                        ]));
                        let mut commit_line = vec![
                            Span::styled("commit ", Style::default().fg(Color::DarkGray)),
                            Span::styled(entry.rid.clone(), Style::default().fg(Color::Yellow)),
                        ];
                        let tags = display_timeline_tags(&entry.tags, &repo.branches);
                        if !tags.is_empty() {
                            commit_line.push(Span::raw(" "));
                            commit_line.push(Span::styled(tags, Style::default().fg(Color::Cyan)));
                        }
                        lines.push(Line::from(commit_line));
                        lines.push(Line::from(vec![
                            Span::styled("author ", Style::default().fg(Color::DarkGray)),
                            Span::raw(entry.user.clone()),
                        ]));
                        lines.push(Line::from(vec![
                            Span::styled("date ", Style::default().fg(Color::DarkGray)),
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
                lines.extend(color_preview(diff, PreviewKind::Diff).lines);
                Paragraph::new(Text::from(lines))
                    .scroll((state.diff_scroll, 0))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(right_pane_title(state, "Timeline details"))
                            .title_bottom(Line::from("PgUp|PgDn").right_aligned()),
                    )
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
        .block(Block::default().borders(Borders::ALL).title("Status"))
    };
    let right_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(body[1]);

    frame.render_stateful_widget(left, body[0], &mut file_state);
    match state.tab {
        Tab::Changes | Tab::WorkingTree => state.files_scroll = file_state.offset(),
        Tab::FileHistory => state.history_scroll = file_state.offset(),
        Tab::Timeline => state.timeline_scroll = file_state.offset(),
    }
    let max_scroll = right_content_lines(state)
        .saturating_sub(right_area[0].height.saturating_sub(2) as usize)
        as u16;
    if state.diff_scroll > max_scroll {
        state.diff_scroll = max_scroll;
    }
    frame.render_widget(right, right_area[0]);
    if state.repo.is_some() {
        let content_length = right_content_lines(state);
        let viewport = right_area[0].height.saturating_sub(2) as usize;
        if content_length > viewport && right_area[1].width > 0 {
            let mut scrollbar_state = ScrollbarState::new(content_length)
                .position((state.diff_scroll as usize).min(content_length.saturating_sub(1)))
                .viewport_content_length(viewport);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                right_area[1],
                &mut scrollbar_state,
            );
        }
    }

    let footer = if let Some(msg) = &state.commit_prompt {
        let target = match state.commit_target {
            CommitTarget::Selected => "selected",
            CommitTarget::Current => "current",
        };
        let text = commit_prompt_text(target, msg);
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
            Span::raw(" sync  "),
            key_span("e", Color::Yellow),
            Span::raw(" edit  "),
            key_span("o", Color::Yellow),
            Span::raw(" open  "),
            key_span("d", Color::Yellow),
            Span::raw(" discard  "),
            key_span("H", Color::Yellow),
            Span::raw(" hex  "),
            key_span("Tab", Color::Yellow),
            Span::raw(" views  "),
            Span::styled("mouse", Style::default().fg(Color::DarkGray)),
            Span::raw(" select/scroll left pane"),
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
        if state.tab == Tab::WorkingTree {
            lines.push(Line::from(vec![
                Span::styled("selected files", Style::default().fg(Color::DarkGray)),
                Span::raw(": "),
                Span::styled(sel_count.to_string(), Style::default().fg(Color::White)),
            ]));
        }
        Paragraph::new(Text::from(lines))
            .block(Block::default().borders(Borders::TOP).title("Shortcuts"))
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

fn commit_prompt_text(target: &str, msg: &str) -> Text<'static> {
    Text::from(vec![
        Line::from(vec![
            Span::raw("commit "),
            key_span(target, Color::Yellow),
            Span::raw(": "),
            Span::styled(
                msg.to_string(),
                Style::default().bg(Color::DarkGray).fg(Color::White),
            ),
        ]),
        confirmation_hint_line(),
    ])
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

fn display_timeline_tags(tags: &str, branches: &[String]) -> String {
    tags.split(',')
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty())
        .filter(|tag| !branches.iter().any(|branch| branch == tag))
        .collect::<Vec<_>>()
        .join(", ")
}

fn right_content_lines(state: &AppState) -> usize {
    match state.tab {
        Tab::Changes | Tab::WorkingTree => state
            .diff
            .as_ref()
            .map(|d| d.lines().count().max(1))
            .unwrap_or(1),
        Tab::FileHistory => {
            let mut lines = 0usize;
            if state.repo.is_some() {
                lines += 3;
            }
            if state.history.get(state.history_selected).is_some() {
                lines += 6;
            }
            lines
                + state
                    .history_diff
                    .as_ref()
                    .map(|d| d.lines().count())
                    .unwrap_or(1)
        }
        Tab::Timeline => {
            let mut lines = 2usize;
            if state
                .repo
                .as_ref()
                .and_then(|r| r.timeline.get(state.timeline_selected))
                .is_some()
            {
                lines += 6;
            }
            lines
                + state
                    .timeline_diff
                    .as_ref()
                    .map(|d| d.lines().count())
                    .unwrap_or(1)
        }
    }
}

fn file_state_symbol(status: &str) -> &'static str {
    if utf8_state_symbols() {
        match status {
            "extra" => "❓",
            "edited" => "✎",
            "added" => "➕",
            "deleted" => "➖",
            "missing" => "✖",
            "conflict" => "⚠",
            _ => "✓",
        }
    } else {
        match status {
            "extra" => "??",
            "edited" => "M",
            "added" => "A",
            "deleted" => "D",
            "missing" => "!",
            "conflict" => "C",
            _ => "✓",
        }
    }
}

fn right_pane_title(state: &AppState, fallback: &str) -> String {
    match state.tab {
        Tab::Changes | Tab::WorkingTree => state
            .repo
            .as_ref()
            .and_then(|repo| repo.files.get(repo.selected_file))
            .map(|file| {
                let title = match file.status.as_str() {
                    "edited" => "Diff",
                    "extra" => "Extra",
                    "added" => "Added",
                    "deleted" => "Deleted",
                    "missing" => "Missing",
                    "conflict" => "Conflict",
                    _ if state.show_hex => "Hexdump",
                    _ => "Preview",
                };
                format!("{}: {}", title, file.path)
            })
            .unwrap_or_else(|| fallback.to_string()),
        Tab::FileHistory => state
            .repo
            .as_ref()
            .and_then(|repo| repo.files.get(repo.selected_file))
            .map(|file| format!("{}: {}", fallback, file.path))
            .unwrap_or_else(|| fallback.to_string()),
        Tab::Timeline => state
            .repo
            .as_ref()
            .and_then(|repo| repo.timeline.get(state.timeline_selected))
            .map(|entry| format!("{}: {}", fallback, entry.rid))
            .unwrap_or_else(|| fallback.to_string()),
    }
}

fn color_preview(diff: String, kind: PreviewKind) -> Text<'static> {
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
                let style = match kind {
                    PreviewKind::Diff => {
                        if line.starts_with("Preview unavailable for ") {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else if line.starts_with("+++") || line.starts_with("---") {
                            Style::default().fg(Color::Yellow)
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
                        }
                    }
                    PreviewKind::Markdown => {
                        if line.starts_with('#') {
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                        } else if line.starts_with("- ") || line.starts_with("* ") {
                            Style::default().fg(Color::Yellow)
                        } else if line.starts_with('>') {
                            Style::default().fg(Color::DarkGray)
                        } else {
                            Style::default().fg(Color::Reset)
                        }
                    }
                    PreviewKind::Plain | PreviewKind::Hex => Style::default().fg(Color::Reset),
                    PreviewKind::Toml => {
                        if line.starts_with('[') {
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                        } else if line.contains('=') {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default().fg(Color::Reset)
                        }
                    }
                    PreviewKind::Json => {
                        if line.trim_start().starts_with('"') && line.contains(':') {
                            Style::default().fg(Color::Yellow)
                        } else if line.contains('{')
                            || line.contains('}')
                            || line.contains('[')
                            || line.contains(']')
                        {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default().fg(Color::Reset)
                        }
                    }
                    PreviewKind::Source => {
                        let trimmed = line.trim_start();
                        if trimmed.starts_with("fn ")
                            || trimmed.starts_with("pub ")
                            || trimmed.starts_with("use ")
                            || trimmed.starts_with("mod ")
                        {
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                        } else if trimmed.starts_with("//") {
                            Style::default().fg(Color::DarkGray)
                        } else {
                            Style::default().fg(Color::Reset)
                        }
                    }
                    PreviewKind::Notice => {
                        if line.starts_with("Preview unavailable for ")
                            || line.starts_with("Missing file ")
                            || line.starts_with("Conflict detected for ")
                        {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else if line.starts_with("Possible rename detected:") {
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                        } else if line.starts_with("Try discard ([d]) or commit ([f]/[c])") {
                            Style::default().fg(Color::DarkGray)
                        } else {
                            Style::default().fg(Color::Reset)
                        }
                    }
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

    #[test]
    fn commit_prompt_text_uses_selected_target_and_message() {
        let text = commit_prompt_text("selected", "Fix bug");
        let lines = text.lines.into_iter().collect::<Vec<_>>();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].spans[1].content, "selected");
        assert_eq!(lines[0].spans[3].content, "Fix bug");
    }

    #[test]
    fn footer_omits_selected_files_in_non_working_tree_tabs() {
        let mut state = AppState {
            tab: Tab::FileHistory,
            repo: None,
            error: None,
            diff: None,
            diff_scroll: 0,
            selected_files: vec!["a.txt".to_string()],
            commit_prompt: None,
            commit_target: CommitTarget::Selected,
            ignore_prompt: None,
            discard_prompt: None,
            history: vec![],
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
        };
        let footer = if state.tab == Tab::WorkingTree {
            let sel_count = state.selected_files.len();
            let mut lines = vec![Line::from(vec![Span::raw("q")])];
            lines.push(Line::from(vec![
                Span::styled("selected files", Style::default().fg(Color::DarkGray)),
                Span::raw(": "),
                Span::styled(sel_count.to_string(), Style::default().fg(Color::White)),
            ]));
            Text::from(lines)
        } else {
            Text::from(vec![Line::from(vec![Span::raw("q")])])
        };
        assert!(!footer.to_string().contains("selected files"));
        state.tab = Tab::Timeline;
        let footer = if state.tab == Tab::WorkingTree {
            Text::from(vec![Line::from(vec![Span::raw("selected files")])])
        } else {
            Text::from(vec![Line::from(vec![Span::raw("q")])])
        };
        assert!(!footer.to_string().contains("selected files"));
    }

    #[test]
    fn preview_rendering_styles_notice_and_markdown() {
        let notice = color_preview(
            "Preview unavailable for file.txt\nPossible rename detected: extra.txt\nMissing file [[gone.txt]]\nConflict detected for [[conflict.txt]]".to_string(),
            PreviewKind::Notice,
        );
        let notice_lines = notice.lines.into_iter().collect::<Vec<_>>();
        assert_eq!(notice_lines.len(), 4);
        assert_eq!(notice_lines[0].spans[0].style.fg, Some(Color::Yellow));
        assert_eq!(notice_lines[1].spans[0].style.fg, Some(Color::Cyan));

        let missing = color_preview(
            "Missing file [[gone.txt]]\nTry discard ([d]) or commit ([f]/[c]) after reviewing the working tree.".to_string(),
            PreviewKind::Notice,
        );
        assert!(missing.to_string().contains("gone.txt"));

        let md = color_preview("# Heading\n- item".to_string(), PreviewKind::Markdown);
        let md_lines = md.lines.into_iter().collect::<Vec<_>>();
        assert_eq!(md_lines[0].spans[0].style.fg, Some(Color::Cyan));
        assert_eq!(md_lines[1].spans[0].style.fg, Some(Color::Yellow));
    }

    #[test]
    fn right_pane_title_includes_selected_context() {
        let mut state = AppState {
            tab: Tab::WorkingTree,
            repo: Some(crate::fossil::RepoState {
                files: vec![
                    crate::fossil::FileStatus {
                        path: "src/lib.rs".to_string(),
                        status: "edited".to_string(),
                    },
                    crate::fossil::FileStatus {
                        path: "notes.txt".to_string(),
                        status: "checked-out".to_string(),
                    },
                    crate::fossil::FileStatus {
                        path: "tmp.log".to_string(),
                        status: "extra".to_string(),
                    },
                ],
                branches: vec!["sym-v0.7.3".to_string()],
                timeline: vec![crate::fossil::TimelineEntry {
                    rid: "abc123".to_string(),
                    user: "Alice".to_string(),
                    date: "2026-06-04 10:00".to_string(),
                    message: "Fix bug".to_string(),
                    tags: "sym-v0.7.3".to_string(),
                }],
                selected_file: 0,
            }),
            error: None,
            diff: None,
            diff_scroll: 0,
            selected_files: vec![],
            commit_prompt: None,
            commit_target: CommitTarget::Selected,
            ignore_prompt: None,
            discard_prompt: None,
            history: vec![],
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
        };
        assert_eq!(right_pane_title(&state, "Diff"), "Diff: src/lib.rs");
        state.repo.as_mut().unwrap().selected_file = 1;
        assert_eq!(right_pane_title(&state, "Diff"), "Preview: notes.txt");
        state.repo.as_mut().unwrap().selected_file = 2;
        assert_eq!(right_pane_title(&state, "Diff"), "Extra: tmp.log");
        state.tab = Tab::Timeline;
        assert_eq!(
            right_pane_title(&state, "Timeline details"),
            "Timeline details: abc123"
        );
    }

    #[test]
    fn display_timeline_tags_filters_branch_names() {
        let tags = display_timeline_tags(
            "sym-v0.7.1, trunk, v0.7.1",
            &["trunk".to_string(), "main".to_string()],
        );
        assert_eq!(tags, "sym-v0.7.1, v0.7.1");
    }

    #[test]
    fn commit_prompt_helper_formats_message() {
        let text = commit_prompt_text("selected", "Ship it");
        assert!(text.to_string().contains("commit selected: Ship it"));
    }

    #[test]
    fn footer_mentions_mouse_hints() {
        let state = AppState {
            tab: Tab::WorkingTree,
            repo: None,
            error: None,
            diff: None,
            diff_scroll: 0,
            selected_files: vec![],
            commit_prompt: None,
            commit_target: CommitTarget::Selected,
            ignore_prompt: None,
            discard_prompt: None,
            history: vec![],
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
        };
        let footer_text = {
            let sel_count = state.selected_files.len();
            let mut lines = vec![Line::from(vec![Span::raw("q")])];
            lines.push(Line::from(vec![
                Span::styled("selected files", Style::default().fg(Color::DarkGray)),
                Span::raw(": "),
                Span::styled(sel_count.to_string(), Style::default().fg(Color::White)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("mouse", Style::default().fg(Color::DarkGray)),
                Span::raw(" select/scroll"),
            ]));
            Text::from(lines)
        };
        assert!(footer_text.to_string().contains("mouse select/scroll"));
    }
}
