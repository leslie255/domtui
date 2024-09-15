#![feature(never_type)]

use std::borrow::Cow;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Wrap},
};

pub mod domtui;

use domtui::views::{Empty, InteractiveViewTrait, Paragraph, ScreenBuilder, Stack, View};

struct FocusableTextBlock<'a> {
    string: Cow<'a, str>,
}

impl<'a> FocusableTextBlock<'a> {
    fn new(string: impl Into<Cow<'a, str>>) -> Self {
        Self {
            string: string.into(),
        }
    }
}

impl<'a> InteractiveViewTrait for FocusableTextBlock<'a> {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, is_focused: bool) {
        let border_style = if is_focused {
            Style::new().fg(Color::Yellow)
        } else {
            Style::new()
        };
        let border_title = if is_focused {
            Line::from("FOCUS HERE")
        } else {
            Line::from("")
        };
        let s: &str = &self.string;
        let paragraph = Paragraph::new(s).wrap(Wrap { trim: false }).block(
            Block::new()
                .title_bottom(border_title)
                .borders(Borders::ALL)
                .border_style(border_style),
        );
        paragraph.render(frame, area);
    }

    fn on_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        }
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::NONE, KeyCode::Char(char)) => {
                self.string.to_mut().push(char);
            }
            (KeyModifiers::NONE, KeyCode::Enter) => {
                self.string.to_mut().push('\n');
            }
            (KeyModifiers::NONE, KeyCode::Backspace) => {
                self.string.to_mut().pop();
            }
            _ => (),
        }
    }
}

fn main() {
    let mut builder = ScreenBuilder::new();
    let root_view = Stack::equal_split_horizontal((
        builder.add_interactive(FocusableTextBlock::new("hello\n你好")),
        builder.add_interactive(FocusableTextBlock::new("world\n世界")),
        Stack::equal_split_vertical((
            builder.add_interactive(FocusableTextBlock::new("I'm Leslie,")),
            Empty,
            builder.add_interactive(FocusableTextBlock::new("This is the thing I made.")),
            builder.add_interactive(FocusableTextBlock::new(
                "Which is a DOM-based TUI framework.",
            )),
            builder.add_interactive(FocusableTextBlock::new(
                "Wrapped on top of ratatui, a none-DOM-based, barebone TUI framework.",
            )),
        )),
    ));

    let mut screen = builder.finish(root_view);
    let mut terminal = domtui::setup_terminal();

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
            event => {
                screen.handle_event(event);
                continue 'event_loop;
            }
        }
    }

    domtui::restore_terminal(terminal);
}
