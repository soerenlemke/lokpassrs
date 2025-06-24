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
    if let Mode::Editing {
        row,
        active_field,
        title,
        username,
        password,
    } = &mut app_state.mode
    {
        match key.code {
            KeyCode::Esc => {
                app_state.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                let pw = &mut app_state.passwords[*row];
                pw.title = title.parse().unwrap();
                pw.username = username.parse().unwrap();
                pw.password = password.parse().unwrap();
                app_state.mode = Mode::Normal;
                app_state.notification = Some(("Passwort geändert!".to_string(), Instant::now()));
            }
            KeyCode::Tab | KeyCode::Down => {
                *active_field = match active_field {
                    EditField::Title => EditField::Username,
                    EditField::Username => EditField::Password,
                    EditField::Password => EditField::Title,
                };
            }
            KeyCode::BackTab | KeyCode::Up => {
                *active_field = match active_field {
                    EditField::Title => EditField::Password,
                    EditField::Username => EditField::Title,
                    EditField::Password => EditField::Username,
                };
            }
            KeyCode::Char(c) => match active_field {
                EditField::Title => title.push(c),
                EditField::Username => username.push(c),
                EditField::Password => password.push(c),
            },
            KeyCode::Backspace => match active_field {
                EditField::Title => {
                    title.pop();
                }
                EditField::Username => {
                    username.pop();
                }
                EditField::Password => {
                    password.pop();
                }
            },
            _ => {}
        }
        return Ok(false);
    }

    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Up | KeyCode::BackTab => {
            if app_state.selected_row > 1 {
                app_state.selected_row -= 1;
            }
        }
        KeyCode::Down | KeyCode::Tab => {
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
            let pw = &app_state.passwords[row];
            app_state.mode = Mode::Editing {
                row,
                active_field: EditField::Title,
                title: pw.title.to_string(),
                username: pw.username.to_string(),
                password: pw.password.to_string(),
            };
        }
        _ => {}
    }
    Ok(false)
}
