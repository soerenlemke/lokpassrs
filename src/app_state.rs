use crate::password::Password;
use arboard::Clipboard;
use std::time::Instant;

pub struct AppState {
    pub selected_row: usize,
    pub passwords: Vec<Password>,
    pub clipboard: Clipboard,
    pub notification: Option<(String, Instant)>,
    pub mode: Mode,
}

pub enum Mode {
    Normal,
    Editing {
        row: usize,
        active_field: EditField,
        title: String,
        username: String,
        password: String,
    },
}

#[derive(PartialEq)]
pub enum EditField {
    Title,
    Username,
    Password,
}
