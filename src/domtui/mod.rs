pub mod view_tuple;
pub mod views;

use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm, CompletedFrame, Terminal,
};

use views::View;

pub fn setup_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    use crossterm::event::EnableMouseCapture;
    crossterm::execute!(stdout(), EnableMouseCapture).unwrap();
    ratatui::init()
}

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) {
    use crossterm::event::DisableMouseCapture;
    terminal.show_cursor().unwrap();
    crossterm::execute!(stdout(), DisableMouseCapture).unwrap();
    ratatui::restore()
}

pub fn render_as_root_view<'a, B: Backend, V: View>(
    terminal: &'a mut Terminal<B>,
    root_view: &V,
) -> io::Result<CompletedFrame<'a>> {
    terminal.draw(|frame| {
        root_view.render(frame, frame.area());
    })
}
