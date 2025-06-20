mod events;
mod password;
mod ui;

use crate::events::handle_events_with_key;
use crate::password::Password;
use crate::ui::draw;
use arboard::Clipboard;
use crossterm::event;
use crossterm::event::Event;
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
            if let Event::Key(key) = event::read()? {
                let num_rows = 1 + app_state.passwords.len();
                if handle_events_with_key(app_state, num_rows, key)? {
                    break Ok(());
                }
            }
        }
    }
}
