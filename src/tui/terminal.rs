use std::{
    io,
    time::{Duration, Instant},
};

use anyhow::Result;
use chrono::DateTime;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use crate::cache::{
    notifications::Notification,
    read::{mark_as_read, read_all_notifications},
};
use crate::tui::app::TerminalApp as App;

pub fn open() -> Result<()> {
    terminal()
}

fn terminal() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);

    let mut list = read_all_notifications();
    list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let app = App::create_list(list);
    let res = start_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn start_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App<Notification>,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c') | KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    KeyCode::Enter => open_url_in_browser(&app),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn open_url_in_browser(app: &App<Notification>) {
    if let Some(current) = app.items.current() {
        if open::that(current.url.clone()).is_err() {
            println!("Could not open url: {} in a browser", current.url)
        };
        mark_as_read(&current.id);
    }
}

fn help_block() -> Block<'static> {
    Block::default().title(Span::styled(
        "    (↑) scroll up    (↓) scroll down    (q/esc) quit    (enter) open in browser    ".to_string(),
        Style::default().add_modifier(Modifier::BOLD).fg(Color::Green),
    ))
}

fn parse_into_duration(updated_at: &str) -> String {
    let current_timestamp = DateTime::parse_from_rfc3339(updated_at)
        .unwrap()
        .with_timezone(&chrono::Utc);
    let duration = chrono::offset::Utc::now() - current_timestamp;

    if duration.num_hours() <= 48 {
        return format!("{} hours", duration.num_hours());
    }

    format!("{} days", duration.num_days())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App<Notification>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .vertical_margin(5)
        .horizontal_margin(10)
        .split(f.size());

    let items = &app.items.items;
    let total_notifications = items.len();

    let items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(index, n)| {
            let lines = vec![
                Spans::from(Span::styled(
                    format!("({}) [{}] {}", index + 1, n.kind.to_lowercase(), n.subject),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Rgb(34, 139, 34)), // Dark Green,
                )),
                Spans::from(format!("{} ⏰ {} ago", n.name, parse_into_duration(&n.updated_at))),
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

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
    f.render_widget(help_block(), chunks[1]);
}
