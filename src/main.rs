#![feature(never_type)]

use domtui::{Paragraph, Screen, Stack};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::Direction,
    text::Text, widgets::{Block, Borders, Wrap},
};

pub mod domtui;

/// A simple text block with square borders.
fn text_block<'a>(text: impl Into<Text<'a>>) -> impl domtui::Component + 'a {
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().borders(Borders::ALL));
    paragraph
}

fn main() {
    let screen = Screen::new(Stack::equal_split(
        Direction::Horizontal,
        (
            text_block("hello\n你好"),
            text_block("world\n世界"),
            Stack::equal_split(
                Direction::Vertical,
                (
                    text_block("I'm Leslie,"),
                    text_block("@leslie255 on Github."),
                    text_block("This is the thing I made."),
                    text_block("Which is a DOM-based TUI framework."),
                    text_block(
                        "Wrapped on top of ratatui, a none-DOM-based, barebone TUI framework.",
                    ),
                ),
            ),
        ),
    ));
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
