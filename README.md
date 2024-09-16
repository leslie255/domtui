# DOMTUI

**Make TUI (text user interface) with DOM, based on [ratatui](https://ratatui.rs)**

(WIP)

## Example

<img width="872" alt="screenshot" src="https://github.com/user-attachments/assets/5aacb9a9-f824-4223-8ee7-4eced673bb90">

```rs
use std::borrow::Cow;

use domtui::views::{InputField, Paragraph, ScreenBuilder, Stack};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

fn main() {
    let mut builder = ScreenBuilder::new();

    let root_view = Stack::horizontal((
        // Stacks can have variable number of children (allows 0~12).
        Paragraph::new("HELLO\n你好").style(Style::new().bg(Color::LightYellow).fg(Color::Black)),
        Paragraph::new("WORLD\n世界").style(Style::new().bg(Color::LightCyan).fg(Color::Black)),
        Stack::vertical((Stack::vertical((
            builder.interactive(input_field("Type something here...", "")),
            builder.interactive(input_field("Type something here...", "UTF-8 文本编辑!")),
        )),)),
    ));

    let mut screen = builder.finish(root_view);

    let mut terminal = domtui::setup_terminal();
    domtui::default_event_loop(&mut terminal, &mut screen).unwrap();
    domtui::restore_terminal(terminal);
}

fn input_field<'a>(
    placeholder: impl Into<Cow<'a, str>>,
    text: impl Into<String>,
) -> InputField<'a> {
    InputField::default()
        .placeholder(placeholder.into())
        .text(text.into())
        .cursor_at_end()
        .block_focused(borders(Color::LightYellow))
        .block_unfocused(borders(Color::DarkGray))
}

fn borders(fg: Color) -> Block<'static> {
    Block::new()
        .borders(Borders::ALL)
        .style(Style::new().fg(fg))
}
```

## LICENSE

This project is licensed under BSD 2-clause.
