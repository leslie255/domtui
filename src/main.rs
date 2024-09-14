#![feature(never_type)]

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    text::Text,
    widgets::{Block, Borders, Wrap},
};

pub mod domtui;

use domtui::views::{Empty, Paragraph, Stack, View};

/// A simple text block with square borders.
fn text_block<'a>(text: impl Into<Text<'a>>) -> impl View + 'a {
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().borders(Borders::ALL));
    paragraph
}

fn main() {
    let root_view = Stack::equal_split_horizontal((
        text_block("hello\n你好"),
        text_block("world\n世界"),
        Stack::equal_split_vertical((
            text_block("I'm Leslie,"),
            Empty,
            text_block("This is the thing I made."),
            text_block("Which is a DOM-based TUI framework."),
            text_block("Wrapped on top of ratatui, a none-DOM-based, barebone TUI framework."),
        )),
    ));

    let mut terminal = domtui::setup_terminal();

    'event_loop: loop {
        domtui::render_as_root_view(&mut terminal, &root_view).unwrap();

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

    domtui::restore_terminal(terminal)
}
