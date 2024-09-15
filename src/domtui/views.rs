#![allow(dead_code)]

use std::{
    cell::RefCell,
    fmt::{self, Debug},
    io,
    rc::{Rc, Weak},
};

use ratatui::{
    backend::Backend,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{self, Block, Wrap},
    Frame, Terminal,
};

use super::view_tuple::ViewTuple;

pub trait View {
    fn render(&self, frame: &mut Frame, area: Rect);
}

impl<'a, V: View> View for &'a V {
    fn render(&self, frame: &mut Frame, area: Rect) {
        (*self).render(frame, area)
    }
}

#[derive(Debug, Clone)]
pub struct Screen<'a, V: View + 'a> {
    root_view: V,
    interactive_views: Vec<Weak<RefCell<InteractiveViewInner<'a>>>>,
}

#[derive(Debug, Clone, Default)]
pub struct ScreenBuilder<'a> {
    interactive_views: Vec<Weak<RefCell<InteractiveViewInner<'a>>>>,
}

impl<'a> ScreenBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finish<V: View>(self, root_view: V) -> Screen<'a, V> {
        if let Some(first_iv) = self.interactive_views.first() {
            first_iv.upgrade().unwrap().borrow_mut().is_focused = true;
        }
        Screen {
            root_view,
            interactive_views: self.interactive_views,
        }
    }

    /// Wrap an `InteractiveView` into a `View`.
    pub fn add_interactive(
        &mut self,
        interactive_view: impl InteractiveViewTrait + 'a,
    ) -> InteractiveView<'a> {
        let interactive_view = InteractiveView::new(false, interactive_view);
        self.interactive_views.push(interactive_view.downgrade());
        interactive_view
    }
}

impl<'a, V: View + 'a> Screen<'a, V> {
    /// Create a screen with no dynamic views.
    pub fn new(root_view: V) -> Self {
        Self {
            root_view,
            interactive_views: Vec::new(),
        }
    }

    pub fn render<B: Backend>(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.autoresize()?;
        let mut frame = terminal.get_frame();
        let area = frame.area();
        self.root_view.render(&mut frame, area);
        terminal.hide_cursor()?;
        terminal.flush()?;
        terminal.swap_buffers();
        terminal.backend_mut().flush()?;
        Ok(())
    }

    pub fn focus_next(&mut self) {
        // Unfocus the currnet one.
        // FIXME: make this more efficient by keeping track the of index of the focused view.
        if self.interactive_views.is_empty() {
            return;
        }
        let (idx, focused_view) = self
            .interactive_views
            .iter()
            .enumerate()
            .find(|(_, weak_iv)| weak_iv.upgrade().unwrap().borrow().is_focused)
            .map(|(i, weak_iv)| (i, weak_iv.clone()))
            .unwrap_or_default();
        if let Some(focused_view) = focused_view.upgrade() {
            let mut focused_view = focused_view.borrow_mut();
            focused_view.is_focused = false;
            focused_view.view.on_unfocus();
        }

        // Focus the next focusable.
        let next_focusable = self.interactive_views[(idx + 1)..]
            .iter()
            .chain(self.interactive_views[..idx].iter())
            .find(|&weak_iv| weak_iv.upgrade().unwrap().borrow().view.is_focusable())
            .map(|weak_iv| weak_iv.upgrade().unwrap());
        if let Some(next_focusable) = next_focusable {
            let mut next_focusable = next_focusable.borrow_mut();
            next_focusable.is_focused = true;
            next_focusable.view.on_unfocus();
        }
    }

    pub fn focused_view<'b>(&'b self) -> Option<InteractiveView<'a>> {
        let iv_weak = self
            .interactive_views
            .iter()
            .find(|iv_weak| iv_weak.upgrade().unwrap().borrow().is_focused)?;
        Some(InteractiveView {
            inner: iv_weak.upgrade().unwrap(),
        })
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                self.focus_next();
            }
            Event::Key(key_event) => {
                if let Some(focused_view) = self.focused_view() {
                    focused_view.inner.borrow_mut().view.on_key_event(key_event);
                }
            }
            _ => (),
        }
    }
}

/// A `DynamicView` can be wrapped into a `View` by `DynamicSite`, which can only be dispatched by
/// a Screen.
#[allow(unused_variables)]
pub trait InteractiveViewTrait {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool);
    fn is_focusable(&self) -> bool {
        true
    }
    fn on_focus(&mut self) {}
    fn on_unfocus(&mut self) {}
    fn on_key_event(&mut self, key_event: KeyEvent) {}
}

#[derive(Debug, Clone)]
pub struct InteractiveView<'a> {
    inner: Rc<RefCell<InteractiveViewInner<'a>>>,
}

impl<'a> InteractiveView<'a> {
    fn new(is_focused: bool, interactive_view: impl InteractiveViewTrait + 'a) -> Self {
        let inner = InteractiveViewInner {
            is_focused,
            view: Box::new(interactive_view),
        };
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    /// Downgrade to a `Weak` reference.
    fn downgrade(&self) -> Weak<RefCell<InteractiveViewInner<'a>>> {
        Rc::downgrade(&self.inner)
    }
}

impl View for InteractiveView<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let inner = self.inner.borrow();
        inner.view.render(frame, area, inner.is_focused);
    }
}

struct InteractiveViewInner<'a> {
    is_focused: bool,
    /// FIXME: Remove this `Box` for one less indirection.
    view: Box<dyn InteractiveViewTrait + 'a>,
}

impl Debug for InteractiveViewInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InteractiveViewWrapperInner")
            .field("is_focused", &self.is_focused)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Empty;

impl View for Empty {
    fn render(&self, _frame: &mut Frame, _area: Rect) {}
}

#[derive(Debug, Clone)]
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
        Self {
            widget: self.widget.style(style),
        }
    }

    pub fn wrap(self, wrap: Wrap) -> Self {
        Self {
            widget: self.widget.wrap(wrap),
        }
    }

    pub fn block(self, block: Block<'a>) -> Self {
        Self {
            widget: self.widget.block(block),
        }
    }
}

impl<'a> View for Paragraph<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(&self.widget, area);
    }
}

#[derive(Debug, Clone)]
pub struct Stack<Ts: ViewTuple> {
    children: Ts,
    layout: Layout,
}

impl<Children: ViewTuple> Stack<Children> {
    pub fn new(children: Children, layout: Layout) -> Self {
        Self { children, layout }
    }

    pub fn equal_split(direction: Direction, children: Children) -> Self {
        let constraints = vec![Constraint::Ratio(1, Children::LEN as u32); Children::LEN];
        let layout = Layout::new(direction, constraints);
        Self::new(children, layout)
    }

    pub fn equal_split_horizontal(children: Children) -> Self {
        Self::equal_split(Direction::Horizontal, children)
    }

    pub fn equal_split_vertical(children: Children) -> Self {
        Self::equal_split(Direction::Vertical, children)
    }
}

impl<Children: ViewTuple> View for Stack<Children> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = self.layout.split(area);
        self.children.render_each(frame, |i| chunks[i])
    }
}
