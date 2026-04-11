use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};

use anyhow::Result;
use chrono::DateTime;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::cache::{
    notifications::Notification,
    read::{mark_as_read, read_all_notifications},
};
use crate::tui::list::StatefulList;

pub fn open() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);

    let mut list = read_all_notifications();
    list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let mut app = StatefulList::with_items(list);
    let res = run_event_loop(&mut terminal, &mut app, tick_rate);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut StatefulList<Notification>,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c') | KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Left => app.unselect(),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('m') => mark_notification_as_read(app),
                    KeyCode::Enter => open_url_in_browser(app),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn mark_notification_as_read(app: &mut StatefulList<Notification>) {
    if let Some(current) = app.current() {
        let id = current.id.clone();
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            crate::poll::mark_notification_as_read(&id).await;
        });
        mark_as_read(&id);
        app.mark();
    }
}

fn open_url_in_browser(app: &mut StatefulList<Notification>) {
    if let Some(current) = app.current() {
        let id = current.id.clone();
        let url = current.url.clone();
        if open::that(&url).is_err() {
            println!("Could not open url: {} in a browser", url)
        };
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            crate::poll::mark_notification_as_read(&id).await;
        });
        mark_as_read(&id);
        app.mark();
    }
}

fn help_block() -> Block<'static> {
    Block::default().title(Span::styled(
        "    (m) mark as read    (↑) scroll up    (↓) scroll down    (q/esc) quit    (enter) open in browser    "
            .to_string(),
        Style::default().add_modifier(Modifier::BOLD).fg(Color::Green),
    ))
}

fn parse_into_duration(updated_at: &str) -> String {
    let Ok(current_timestamp) = DateTime::parse_from_rfc3339(updated_at) else {
        return "unknown".to_string();
    };
    let duration = chrono::offset::Utc::now() - current_timestamp.with_timezone(&chrono::Utc);
    if duration.num_minutes() <= 1 {
        return format!("{} seconds", duration.num_seconds());
    }
    if duration.num_hours() <= 1 {
        return format!("{} minutes", duration.num_minutes());
    }
    if duration.num_days() <= 1 {
        return format!("{} hours", duration.num_hours());
    }
    format!("{} days", duration.num_days())
}

fn ui(f: &mut Frame, app: &mut StatefulList<Notification>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .vertical_margin(5)
        .horizontal_margin(10)
        .split(f.area());

    let items = &app.items;
    let total_notifications = items.len();

    let items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(index, n)| {
            let lines = vec![
                Line::from(Span::styled(
                    format!("({}) [{}] {}", index + 1, n.kind.to_lowercase(), n.subject),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Rgb(34, 139, 34)),
                )),
                Line::from(format!("{} ⏰ {} ago", n.name, parse_into_duration(&n.updated_at))),
            ];
            ListItem::new(lines).style(Style::default().fg(Color::White))
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(
            format!(" ghostie - Showing {} notifications ", total_notifications),
            Style::default().add_modifier(Modifier::BOLD),
        )))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("👉");

    f.render_stateful_widget(items, chunks[0], &mut app.state);
    f.render_widget(help_block(), chunks[1]);
}
