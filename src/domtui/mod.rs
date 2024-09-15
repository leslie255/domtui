pub mod view_tuple;
pub mod views;
pub mod input_field;

use std::io::{stdout, Stdout};

use ratatui::{backend::CrosstermBackend, crossterm, Terminal};

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
