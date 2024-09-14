use std::borrow::Cow;

use dom::{Node, Screen};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    widgets::{Block, Borders, Paragraph, Wrap},
};

mod dom;

/// A simple text block with square borders.
fn text_block<'a>(text: impl Into<Cow<'a, str>>) -> Node<'a> {
    let string: Cow<str> = text.into();
    let paragraph = Paragraph::new(string)
        .wrap(Wrap { trim: true }) // line wrap
        .block(Block::new().borders(Borders::ALL));
    paragraph.into()
}

fn main() {
    let screen = Screen {
        root: Node::equal_horizontal_split([
            text_block("hello\n你好"),
            text_block("world\n世界"),
            Node::equal_vertical_split([
                text_block("I'm Leslie."),
                text_block("I'm Leslie."),
                text_block("This is the thing I made."),
                text_block("Which is a DOM-based TUI framework."),
                text_block("Wrapped on top of ratatui, a none-DOM-based, barebone TUI framework."),
            ]),
        ]),
    };
    let mut terminal = ratatui::init();
    'event_loop: loop {
        screen.render(&mut terminal).unwrap();
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
                break 'event_loop;
            }
            _ => continue 'event_loop,
        }
    }
    ratatui::restore();
}
