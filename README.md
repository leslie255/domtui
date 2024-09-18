# DOMTUI

**Make TUI (text user interface) with DOM, based on [ratatui](https://ratatui.rs)**

(WIP)

## Example

<img width="872" alt="screenshot" src="https://github.com/user-attachments/assets/5aacb9a9-f824-4223-8ee7-4eced673bb90">

```rs

fn borders(fg: Color) -> Block<'static> {
    Block::new()
        .borders(Borders::ALL)
        .style(Style::new().fg(fg))
}

fn main() {
    let mut builder = ScreenBuilder::new();

    let root_view = Stack::horizontal((
        Paragraph::new("HELLO\n你好")
            .bg(Color::LightYellow)
            .fg(Color::Black)
            .prefers_size((20, 20)),
        Paragraph::new("WORLD\n世界")
            .bg(Color::LightCyan)
            .fg(Color::Black),
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
            ),
        )),
    ));

    let mut screen = builder.finish(root_view);

    let mut terminal = domtui::setup_terminal();
    domtui::default_event_loop(&mut terminal, &mut screen).unwrap();
    domtui::restore_terminal(terminal);
}
```

## LICENSE

This project is licensed under BSD 2-clause.
