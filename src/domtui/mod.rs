mod component_tuple;

use std::io;

use component_tuple::ComponentList;
use ratatui::{
    backend::Backend, layout::{Constraint, Direction, Layout, Rect}, style::Style, text::Text, widgets::{self, Block, Wrap}, CompletedFrame, Frame, Terminal
};

#[derive(Debug, Clone)]
pub struct Screen<RootComponent: Component> {
    root_component: RootComponent,
}

impl<RootComponent: Component> Screen<RootComponent> {
    pub fn new(root_component: RootComponent) -> Self {
        Self { root_component }
    }

    pub fn render<'a, B: Backend>(
        &self,
        terminal: &'a mut Terminal<B>,
    ) -> io::Result<CompletedFrame<'a>> {
        terminal.draw(move |frame| {
            self.root_component.render(frame, frame.area());
        })
    }
}

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
}

#[derive(Debug, Clone, derive_more::From)]
pub struct Paragraph<'a> {
    widget: widgets::Paragraph<'a>,
}

impl<'a> Paragraph<'a> {
    pub fn new(text: impl Into<Text<'a>>) -> Self {
        Self {
            widget: widgets::Paragraph::new(text),
        }
    }
    
    pub fn style(self, style: Style) -> Self {
        self.widget.style(style).into()
    }
    
    pub fn wrap(self, wrap: Wrap) -> Self {
        self.widget.wrap(wrap).into()
    }
    
    pub fn block(self, block: Block<'a>) -> Self {
        self.widget.block(block).into()
    }
}

impl<'a> Component for Paragraph<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(&self.widget, area);
    }
}

#[derive(Debug, Clone)]
pub struct Stack<Ts: ComponentList> {
    children: Ts,
    layout: Layout,
}

impl<Children: ComponentList> Stack<Children> {
    pub fn new(children: Children, layout: Layout) -> Self {
        Self { children, layout }
    }

    pub fn equal_split(direction: Direction, children: Children) -> Self {
        let constraints = vec![Constraint::Ratio(1, Children::LEN as u32); Children::LEN];
        let layout = Layout::new(direction, constraints);
        Self { children, layout }
    }
}

impl<Children: ComponentList> Component for Stack<Children> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = self.layout.split(area);
        self.children.render_each(frame, |i| chunks[i])
    }
}
