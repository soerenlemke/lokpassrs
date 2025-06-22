use crate::app_state::EditField;
use crate::{AppState, Mode};
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Span, Style};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table};
use ratatui::Frame;
use std::time::Duration;

pub fn draw(frame: &mut Frame, app_state: &mut AppState) {
    let area = frame.area();

    if let Mode::Editing { field, buffer, .. } = &app_state.mode {
        let (fieldname, color) = match field {
            EditField::Title => ("Title", Color::Magenta),
            EditField::Username => ("Username", Color::Cyan),
            EditField::Password => ("Password", Color::Yellow),
        };
        let edit_area = Rect {
            x: (area.width / 2) - 20,
            y: (area.height / 2) - 2,
            width: 40,
            height: 3,
        };
        let para = Paragraph::new(Span::styled(
            format!("Edit {}:\n{}", fieldname, buffer),
            Style::default().fg(color).bg(Color::Black),
        ))
        .block(Block::default().borders(Borders::ALL).title("Edit Mode"));
        frame.render_widget(para, edit_area);
        return;
    }

    draw_table(frame, app_state, area);
    draw_notification(frame, app_state, area);
}

fn draw_table(frame: &mut Frame, app_state: &mut AppState, area: Rect) {
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

fn draw_notification(frame: &mut Frame, app_state: &mut AppState, area: Rect) {
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
