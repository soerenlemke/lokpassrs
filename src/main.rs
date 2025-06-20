mod password;

use crate::password::Password;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint;
use ratatui::widgets::{Block, Borders, Row, Table};
use ratatui::Frame;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(draw).expect("Failed to draw terminal");
        if handle_events()? {
            break Ok(());
        }
    }
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            if let KeyCode::Char('q') = key.code {
                return Ok(true);
            }
        }
        _ => {}
    }
    Ok(false)
}

fn draw(frame: &mut Frame) {
    let area = frame.area();

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
    let passwords = [password_one, password_two];

    let mut rows = vec![Row::new(vec!["Title", "Username", "Password"])];
    for pw in passwords {
        rows.push(Row::new(vec![
            pw.title.to_string(),
            pw.username.to_string(),
            pw.password.to_string(),
        ]));
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
