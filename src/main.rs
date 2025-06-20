mod password;

use crate::password::Password;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint;
use ratatui::widgets::{Block, Borders, Row, Table};
use ratatui::Frame;

struct AppState {
    selected_row: usize,
    passwords: Vec<Password>,
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
    };

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut app_state);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app_state: &mut AppState) -> std::io::Result<()> {
    loop {
        terminal
            .draw(|f| draw(f, app_state))
            .expect("Failed to draw terminal");
        let num_rows = 1 + app_state.passwords.len();
        if handle_events(app_state, num_rows)? {
            break Ok(());
        }
    }
}

fn handle_events(app_state: &mut AppState, num_rows: usize) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
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
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

fn draw(frame: &mut Frame, app_state: &AppState) {
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
}
