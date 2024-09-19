#![feature(never_type, new_range_api)]

pub mod domtui;

use domtui::views::{InputField, Paragraph, ScreenBuilder, Stack, ViewExt};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Wrap},
};

fn borders(fg: Color) -> Block<'static> {
    Block::new()
        .borders(Borders::ALL)
        .style(Style::new().fg(fg))
}

fn main() {
    let mut builder = ScreenBuilder::new();

    let root_view = Stack::horizontal((
        Paragraph::new("HELLO\n(This view has a preferred size of 16*16)")
            .bg(Color::LightYellow)
            .fg(Color::Black)
            .wrap(Wrap::default())
            .prefers_size((16, 16)),
        Paragraph::new("WORLD\n(This view doesn't have a preferred size, it just spreads out equally with other views)")
            .bg(Color::LightCyan)
            .fg(Color::Black)
            .wrap(Wrap::default()),
        Stack::vertical((
            builder.tagged_view_cell(
                "input_field0",
                InputField::default()
                    .placeholder("Type something here...")
                    .block_focused(borders(Color::LightYellow))
                    .block_unfocused(borders(Color::DarkGray)),
            ),
            builder.tagged_view_cell(
                "input_field1",
                InputField::default()
                    .placeholder("Type something here...")
                    .text("UTF-8 文本编辑!")
                    .cursor_at_end()
                    .block_focused(borders(Color::LightYellow))
                    .block_unfocused(borders(Color::DarkGray)),
            ).prefers_size((u16::MAX, 4)),
        )),
    ));

    let mut screen = builder.finish(root_view);

    let mut terminal = domtui::setup_terminal();
    domtui::default_event_loop(&mut terminal, &mut screen).unwrap();
    domtui::restore_terminal(terminal);

    unsafe {
        screen.inspect_view_with_tag_unchecked::<(), InputField>("input_field0", |input_field| {
            let text = input_field.content().text();
            println!("input_field0: {text:?}")
        });
        screen.inspect_view_with_tag_unchecked::<(), InputField>("input_field1", |input_field| {
            let text = input_field.content().text();
            println!("input_field1: {text:?}")
        });
    }
}
