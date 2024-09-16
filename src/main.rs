#![feature(never_type, new_range_api)]

pub mod domtui;

use domtui::views::{Empty, InputField, Paragraph, ScreenBuilder, Stack};
use ratatui::style::{Color, Style};

fn main() {
    let mut builder = ScreenBuilder::new();

    let root_view = Stack::horizontal((
        Paragraph::new("hello\n你好").style(Style::new().bg(Color::LightBlue).fg(Color::Black)),
        Paragraph::new("world\n世界").style(Style::new().bg(Color::LightCyan).fg(Color::Black)),
        Stack::vertical((
            Stack::vertical((
                builder.interactive(InputField::default().placeholder("Type something here...")),
                builder.interactive(
                    InputField::default()
                        .placeholder("Type something here...")
                        .text("This is an input field with pre-filled text")
                        .cursor_at_end(),
                ),
            )),
            Empty,
        )),
    ));

    let mut screen = builder.finish(root_view);

    let mut terminal = domtui::setup_terminal();
    domtui::default_event_loop(&mut terminal, &mut screen).unwrap();
    domtui::restore_terminal(terminal);
}
