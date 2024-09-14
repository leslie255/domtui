#![allow(dead_code)]

use std::{io, rc::Rc};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    text::Text,
    widgets::Paragraph,
    CompletedFrame, Frame, Terminal,
};

#[derive(Debug, Clone)]
pub struct Screen<'a> {
    pub root: Node<'a>,
}

impl<'a> Screen<'a> {
    pub fn render(
        &self,
        terminal: &'a mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> io::Result<CompletedFrame> {
        terminal.draw(|frame| {
            let area = frame.area();
            self.root.draw(frame, area);
        })
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub inner: NodeInner<'a>,
}

impl<'a> From<Paragraph<'a>> for Node<'a> {
    fn from(paragraph: Paragraph<'a>) -> Self {
        Self {
            inner: NodeInner::Paragraph(paragraph),
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum NodeInner<'a> {
    Split(Layout, Rc<[Node<'a>]>),
    Paragraph(Paragraph<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EqualSplit {
    pub direction: Direction,
    pub count: u16,
}

impl EqualSplit {
    pub fn new(direction: Direction, count: u16) -> Self {
        Self { direction, count }
    }
}

impl From<EqualSplit> for Layout {
    fn from(self_: EqualSplit) -> Self {
        let constraint = Constraint::Ratio(1, self_.count as u32);
        let constraints = vec![constraint; self_.count as usize];
        Layout::default()
            .direction(self_.direction)
            .constraints(constraints)
    }
}

impl<'a> Node<'a> {
    pub fn split(layout: impl Into<Layout>, nodes: impl Into<Rc<[Self]>>) -> Self {
        Self {
            inner: NodeInner::Split(layout.into(), nodes.into()),
        }
    }

    pub fn equal_split(direction: Direction, nodes: impl Into<Rc<[Self]>>) -> Self {
        let nodes = nodes.into();
        Self::split(EqualSplit::new(direction, nodes.len() as u16), nodes)
    }

    pub fn equal_vertical_split(nodes: impl Into<Rc<[Self]>>) -> Self {
        Self::equal_split(Direction::Vertical, nodes)
    }

    pub fn equal_horizontal_split(nodes: impl Into<Rc<[Self]>>) -> Self {
        Self::equal_split(Direction::Horizontal, nodes)
    }

    pub fn paragraph(paragraph: impl Into<Text<'a>>) -> Self {
        Self {
            inner: NodeInner::Paragraph(Paragraph::new(paragraph)),
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        match &self.inner {
            NodeInner::Split(layout, nodes) => {
                let chunks = layout.split(area);
                assert_eq!(chunks.len(), nodes.len(), "{layout:?}");
                for (&chunk, node) in chunks.iter().zip(nodes.iter()) {
                    node.draw(frame, chunk);
                }
            }
            NodeInner::Paragraph(paragraph) => frame.render_widget(paragraph, area),
        }
    }
}
