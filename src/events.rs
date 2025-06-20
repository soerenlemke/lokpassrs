use crate::AppState;
use crossterm::event::KeyEvent;
use std::time::Instant;

pub fn handle_events_with_key(
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
