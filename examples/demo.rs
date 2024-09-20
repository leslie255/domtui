use domtui::views::*;

use ratatui::style::*;
use ratatui::widgets::{Block, Borders, Wrap};

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
            .block(borders(Color::LightRed).title_top("Borders!"))
            .wrap(Wrap::default()),
        Stack::vertical((
            builder.tagged_view_cell(
                "input_field0",
                InputField::default()
                    .placeholder("Type something here...")
                    .text("")
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
            ).prefers_size((0, 4)),
        )),
    ));

    let mut screen = builder.finish(root_view);

    let mut terminal = domtui::setup_terminal();
    domtui::default_event_loop(&mut terminal, &mut screen).unwrap();
    domtui::restore_terminal(terminal);
}
