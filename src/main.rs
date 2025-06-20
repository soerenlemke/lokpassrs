use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint::{Length, Min, Percentage};
use ratatui::layout::Layout;
use ratatui::widgets::Block;
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
    let vertical = Layout::vertical([Length(1), Min(0)]);
    let [title_area, main_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Percentage(20), Percentage(40), Percentage(40)]);
    let [title, username, password] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("passwords"), title_area);
    frame.render_widget(Block::bordered().title("title"), title);
    frame.render_widget(Block::bordered().title("username"), username);
    frame.render_widget(Block::bordered().title("password"), password);
}
