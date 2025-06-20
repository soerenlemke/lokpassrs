use crossterm::event;
use crossterm::event::Event;
use ratatui::text::Text;
use ratatui::Frame;

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("Failed to draw terminal");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello world");
    frame.render_widget(text, frame.area());
}
