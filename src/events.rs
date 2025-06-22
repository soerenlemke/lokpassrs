use crate::app_state::EditField;
use crate::password::Password;
use crate::{AppState, Mode};
use crossterm::event::{KeyCode, KeyEvent};
use std::time::Instant;

pub fn handle_events_with_key(
    app_state: &mut AppState,
    num_rows: usize,
    key: KeyEvent,
) -> std::io::Result<bool> {
    if let Mode::Editing { row, field, buffer } = &mut app_state.mode {
        match key.code {
            KeyCode::Esc => {
                app_state.mode = Mode::Normal;
            }
            KeyCode::Enter => match field {
                EditField::Title => {
                    app_state.passwords[*row].title = buffer.parse().unwrap();
                    *field = EditField::Username;
                    buffer.clear();
                    buffer.push_str(&app_state.passwords[*row].username.to_string());
                }
                EditField::Username => {
                    app_state.passwords[*row].username = buffer.parse().unwrap();
                    *field = EditField::Password;
                    buffer.clear();
                    buffer.push_str(&app_state.passwords[*row].password.to_string());
                }
                EditField::Password => {
                    app_state.passwords[*row].password = buffer.parse().unwrap();
                    app_state.mode = Mode::Normal;
                    app_state.notification =
                        Some(("Passwort geändert!".to_string(), Instant::now()));
                }
            },
            KeyCode::Char(c) => {
                buffer.push(c);
            }
            KeyCode::Backspace => {
                buffer.pop();
            }
            _ => {}
        }
        return Ok(false);
    }

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
        KeyCode::Char('a') => {
            app_state.passwords.push(Password::new(
                "new title".parse().unwrap(),
                "new username".parse().unwrap(),
                "new password".parse().unwrap(),
            ));
            app_state.selected_row = app_state.passwords.len();
            app_state.notification =
                Some(("neues Passwort hinzugefügt".to_string(), Instant::now()));
        }
        KeyCode::Char('e') => {
            let row = app_state.selected_row - 1;
            let title = app_state.passwords[row].title.to_string();
            app_state.mode = Mode::Editing {
                row,
                field: EditField::Title,
                buffer: title,
            }
        }
        _ => {}
    }
    Ok(false)
}
