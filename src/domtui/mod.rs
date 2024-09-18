pub mod input_field;
pub mod view_tuple;
pub mod views;

use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{self, event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}}, Terminal,
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
pub fn render<V: View, B: Backend>(terminal: &mut Terminal<B>, view: V) -> io::Result<()> {
    let screen = Screen::new(view);
    screen.render(terminal)
}

/// Simple event loop for just rendering a `Screen` with nothing else, ends on `<C-q>`.
pub fn default_event_loop<V: View, B: Backend>(
    terminal: &mut Terminal<B>,
    screen: &mut Screen<V>,
) -> io::Result<()> {
    'event_loop: loop {
        screen.render(terminal).unwrap();
        if !event::poll(std::time::Duration::from_millis(100)).unwrap() {
            continue 'event_loop;
        }
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c' | 'q'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                break 'event_loop Ok(());
            }
            event => {
                screen.handle_event(event);
                continue 'event_loop;
            }
        }
    }
}
