pub mod input_field;
pub mod view_tuple;
pub mod views;

use std::io::{stdout, Stdout};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm, Terminal,
};

use views::{Screen, View};

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

/// Shorthand for rendering a view with no dynamic parts.
/// For rendering views with dynamic parts, use `Screen`.
pub fn render<V: View, B: Backend>(terminal: &mut Terminal<B>, view: V) -> std::io::Result<()> {
    let screen = Screen::new(view);
    screen.render(terminal)
}
