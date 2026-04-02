//! Layout and widgets for the Ratatui interface.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use super::app::{App, ConfigFocus, GenFocus, InsertFocus, Screen, StatusKind};

/// Renders the current screen into `frame`.
pub fn draw(frame: &mut Frame<'_>, app: &mut App) {
    let main = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(main);

    let body = chunks[0];
    let footer = chunks[1];

    match &app.screen {
        Screen::Main => draw_main(frame, app, body),
        Screen::Generate { .. } => draw_generate(frame, app, body),
        Screen::Insert { .. } => draw_insert(frame, app, body),
        Screen::Get { .. } => draw_get(frame, app, body),
        Screen::DeleteConfirm { .. } => draw_delete(frame, app, body),
        Screen::Config { .. } => draw_config(frame, app, body),
    }

    draw_status_line(frame, app, footer);
}

fn draw_main(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    let help = Paragraph::new(
        "passgen TUI — g generate  i insert  Enter get  d delete  s settings  q quit  j/k move",
    )
    .style(Style::default().fg(Color::Cyan))
    .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(help, chunks[0]);

    let items: Vec<ListItem> = app
        .entries
        .iter()
        .map(|s| ListItem::new(Line::from(s.as_str())))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Stored passwords "),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
}

fn draw_generate(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let Screen::Generate {
        name,
        length,
        focus,
    } = &app.screen
    else {
        return;
    };

    let name_label = if *focus == GenFocus::Name {
        "> Name: "
    } else {
        "  Name: "
    };
    let len_label = if *focus == GenFocus::Length {
        "> Length: "
    } else {
        "  Length: "
    };

    let text = vec![
        Line::from(vec![Span::styled(
            "Generate password — Tab switch field  Enter submit  Esc cancel",
            Style::default().fg(Color::Cyan),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw(name_label),
            Span::styled(name.as_str(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw(len_label),
            Span::styled(length.as_str(), Style::default().fg(Color::Yellow)),
        ]),
    ];

    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Generate "))
        .wrap(Wrap { trim: true });
    frame.render_widget(p, area);
}

fn draw_insert(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let Screen::Insert {
        name,
        password,
        focus,
    } = &app.screen
    else {
        return;
    };

    let name_label = if *focus == InsertFocus::Name {
        "> Name: "
    } else {
        "  Name: "
    };
    let pass_display: String = "*".repeat(password.len());
    let pass_label = if *focus == InsertFocus::Password {
        "> Password: "
    } else {
        "  Password: "
    };

    let text = vec![
        Line::from(vec![Span::styled(
            "Insert — Tab switch  Enter save  Esc cancel (password masked)",
            Style::default().fg(Color::Cyan),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw(name_label),
            Span::styled(name.as_str(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw(pass_label),
            Span::styled(pass_display, Style::default().fg(Color::Yellow)),
        ]),
    ];

    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Insert "))
        .wrap(Wrap { trim: true });
    frame.render_widget(p, area);
}

fn draw_get(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let Screen::Get { name, plain, err } = &app.screen else {
        return;
    };

    let body = match (plain, err) {
        (Some(p), _) => {
            vec![
                Line::from(vec![
                    Span::raw("Entry: "),
                    Span::styled(name, Style::default().bold()),
                ]),
                Line::from(""),
                Line::from(Span::styled(p, Style::default().fg(Color::Green))),
            ]
        }
        (None, Some(e)) => vec![
            Line::from(vec![
                Span::raw("Entry: "),
                Span::styled(name, Style::default().bold()),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                format!("Error: {e}"),
                Style::default().fg(Color::Red),
            )),
        ],
        (None, None) => vec![Line::from("Unknown state.")],
    };

    let mut lines = vec![Line::from(vec![Span::styled(
        "Get — c copy to clipboard  Esc back",
        Style::default().fg(Color::Cyan),
    )])];
    lines.push(Line::from(""));
    lines.extend(body);

    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Retrieve "))
        .wrap(Wrap { trim: true });
    frame.render_widget(p, area);
}

fn draw_delete(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let Screen::DeleteConfirm { name } = &app.screen else {
        return;
    };

    let text = vec![
        Line::from(vec![Span::styled(
            "Delete this entry?",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(Span::styled(name.as_str(), Style::default().fg(Color::Red))),
        Line::from(""),
        Line::from("y confirm   n / Esc cancel"),
    ];

    let p = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Confirm delete "),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(p, area);
}

fn draw_config(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let Screen::Config {
        show_pass,
        passgen_key,
        focus,
        is_new,
    } = &app.screen
    else {
        return;
    };

    let show_line = if *focus == ConfigFocus::ShowPass {
        "> Show password in CLI/TUI: "
    } else {
        "  Show password in CLI/TUI: "
    };
    let key_label = if *focus == ConfigFocus::Key {
        "> Encryption key: "
    } else {
        "  Encryption key: "
    };

    let mut lines = vec![Line::from(vec![Span::styled(
        "Settings — Tab switch  Space toggle show  Ctrl+S save  Esc back (if config exists)",
        Style::default().fg(Color::Cyan),
    )])];
    if *is_new {
        lines.push(Line::from(Span::styled(
            "First-time setup: set key and press Ctrl+S.",
            Style::default().fg(Color::Yellow),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw(show_line),
        Span::styled(
            if *show_pass { "yes" } else { "no" },
            Style::default().fg(Color::Yellow),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::raw(key_label),
        Span::styled(passgen_key.as_str(), Style::default().fg(Color::Yellow)),
    ]));

    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Configuration "),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(p, area);
}

fn draw_status_line(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let line = match &app.status {
        Some((StatusKind::Ok, m)) => {
            Line::from(Span::styled(m.as_str(), Style::default().fg(Color::Green)))
        }
        Some((StatusKind::Err, m)) => {
            Line::from(Span::styled(m.as_str(), Style::default().fg(Color::Red)))
        }
        None => Line::from(""),
    };
    frame.render_widget(Paragraph::new(line), area);
}
