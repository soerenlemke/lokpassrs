mod password;

use crate::password::Password;
use arboard::Clipboard;
use crossterm::event;
use crossterm::event::{Event, KeyEvent};
use ratatui::layout::Constraint;
use ratatui::prelude::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table};
use ratatui::Frame;
use std::time::{Duration, Instant};

struct AppState {
    selected_row: usize,
    passwords: Vec<Password>,
    clipboard: Clipboard,
    notification: Option<(String, Instant)>,
}

fn main() -> std::io::Result<()> {
    let password_one = Password::new(
        "test title one".parse().unwrap(),
        "test username one".parse().unwrap(),
        "test password one".parse().unwrap(),
    );
    let password_two = Password::new(
        "test title two".parse().unwrap(),
        "test username two".parse().unwrap(),
        "test password two".parse().unwrap(),
    );

    let mut app_state = AppState {
        selected_row: 1,
        passwords: vec![password_one, password_two],
        clipboard: Clipboard::new().unwrap(),
        notification: None,
    };

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut app_state);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app_state: &mut AppState) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, app_state))?;

        if event::poll(Duration::from_millis(100))? {
            // Es gibt ein Event
            if let Event::Key(key) = event::read()? {
                let num_rows = 1 + app_state.passwords.len();
                if handle_events_with_key(app_state, num_rows, key)? {
                    break Ok(());
                }
            }
        }
    }
}

fn handle_events_with_key(
    app_state: &mut AppState,
    num_rows: usize,
    key: KeyEvent,
) -> std::io::Result<bool> {
    use crossterm::event::KeyCode;
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Up => {
            if app_state.selected_row > 1 {
                app_state.selected_row -= 1;
            }
        }
        KeyCode::Down => {
            if app_state.selected_row + 1 < num_rows {
                app_state.selected_row += 1;
            }
        }
        KeyCode::Char('u') => {
            let username = &app_state.passwords[app_state.selected_row - 1].username;
            app_state.clipboard.set_text(username).unwrap();
            app_state.notification = Some(("username copied!".to_string(), Instant::now()));
        }
        KeyCode::Char('p') => {
            let password = &app_state.passwords[app_state.selected_row - 1].password;
            app_state.clipboard.set_text(password).unwrap();
            app_state.notification = Some(("password copied!".to_string(), Instant::now()));
        }
        _ => {}
    }
    Ok(false)
}

fn draw(frame: &mut Frame, app_state: &mut AppState) {
    use ratatui::style::{Color, Style};
    let area = frame.area();

    let mut rows = vec![Row::new(vec!["Title", "Username", "Password"])];
    for (i, pw) in app_state.passwords.iter().enumerate() {
        let mut row = Row::new(vec![
            pw.title.to_string(),
            pw.username.to_string(),
            pw.password.to_string(),
        ]);

        if i + 1 == app_state.selected_row {
            row = row.style(Style::default().fg(Color::White).bg(Color::Blue));
        }
        rows.push(row);
    }

    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(35),
        Constraint::Percentage(35),
    ];

    let table =
        Table::new(rows, widths).block(Block::default().borders(Borders::ALL).title("Passwords"));

    frame.render_widget(table, area);

    if let Some((msg, time)) = &app_state.notification {
        if time.elapsed() < Duration::from_secs(2) {
            let notif_area = ratatui::layout::Rect {
                x: area.x,
                y: area.y + area.height - 2,
                width: area.width,
                height: 1,
            };
            let notif = Paragraph::new(Span::styled(
                msg,
                Style::default().fg(Color::Yellow).bg(Color::Black),
            ));
            frame.render_widget(notif, notif_area);
        } else {
            app_state.notification = None;
        }
    }
}
