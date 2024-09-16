#![allow(dead_code)]

use std::{
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    io,
    rc::{Rc, Weak},
};

use derive_more::From;

use ratatui::{
    backend::Backend,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span, Text},
    widgets::{self, Block, Wrap},
    Frame, Terminal,
};

use super::{
    input_field::{self, Cursor, InputFieldContent},
    view_tuple::ViewTuple,
};

/// `'a` for allowing to borrow from a data source.
#[derive(Debug, Clone)]
pub struct Screen<'a, V: StaticView + 'a> {
    root_view: V,
    dynamic_sites: Vec<DynamicViewWrapperWeakRef<'a>>,
    dynamic_site_tags: HashMap<Cow<'a, str>, DynamicViewWrapperWeakRef<'a>>,
}

/// `'a` for allowing to borrow from a data source.
#[derive(Debug, Clone, Default)]
pub struct ScreenBuilder<'a> {
    dynamic_sites: Vec<DynamicViewWrapperWeakRef<'a>>,
    dynamic_site_tags: HashMap<Cow<'a, str>, DynamicViewWrapperWeakRef<'a>>,
}

impl<'a> ScreenBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finish<V: StaticView>(self, root_view: V) -> Screen<'a, V> {
        Screen {
            root_view,
            dynamic_sites: self.dynamic_sites,
            dynamic_site_tags: self.dynamic_site_tags,
        }
    }

    /// Wrap a `View` into a `DynamicViewWrapper`.
    pub fn dynamic_site(&mut self, view: impl View + 'a) -> DynamicViewWrapper<'a> {
        let view = DynamicViewWrapper::new(false, view);
        self.dynamic_sites.push(view.downgrade());
        view
    }

    /// Wrap a `View` into a `DynamicViewWrapper`.
    pub fn tagged_dynamic_site(
        &mut self,
        tag: impl Into<Cow<'a, str>>,
        view: impl View + 'a,
    ) -> DynamicViewWrapper<'a> {
        let view = DynamicViewWrapper::new(false, view);
        self.dynamic_site_tags.insert(tag.into(), view.downgrade());
        self.dynamic_sites.push(view.downgrade());
        view
    }
}

impl<'a, V: StaticView + 'a> Screen<'a, V> {
    /// Create a screen with no dynamic views.
    pub fn new(root_view: V) -> Self {
        ScreenBuilder::default().finish(root_view)
    }

    pub fn render<B: Backend>(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.autoresize()?;
        let mut frame = terminal.get_frame();
        let area = frame.area();
        self.root_view.render_static(&mut frame, area);
        terminal.hide_cursor()?;
        terminal.flush()?;
        terminal.swap_buffers();
        terminal.backend_mut().flush()?;
        Ok(())
    }

    /// This has the same effect as calling `ScreenBuilder::dynamic_site` before the screen was
    /// built.
    pub fn dynamic_site(&mut self, view: impl View + 'a) -> DynamicViewWrapper<'a> {
        let dynamic_site = DynamicViewWrapper::new(false, view);
        self.dynamic_sites.push(dynamic_site.downgrade());
        dynamic_site
    }

    /// If multiple views were tagged the same, only one of them is inspected, randomly.
    ///
    /// # Safety
    /// `V` must be of the correct type that the value was initialized with.
    pub unsafe fn inspect_view_with_tag_unchecked<T, V2: View + 'a>(
        &self,
        tag: &str,
        f: impl FnOnce(&mut V2) -> T,
    ) -> Option<T> {
        let view = self.dynamic_site_tags.get(tag)?;
        Some(view.upgrade().unwrap().inspect::<_, V2>(f))
    }

    /// Switch focus to the next focusable view.
    /// A focusable view is an `View` with its `is_focusable` returning `true`.
    pub fn focus_next(&mut self) {
        // Unfocus the currnet one.
        // FIXME: make this more efficient by keeping track the of index of the focused view.
        if self.dynamic_sites.is_empty() {
            return;
        }
        let (idx, focused_view) = self
            .dynamic_sites
            .iter()
            .enumerate()
            .find(|(_, weak_iv)| weak_iv.upgrade().unwrap().inner.borrow().is_focused)
            .map(|(i, weak_iv)| (i, weak_iv.clone()))
            .unwrap_or_default();
        if let Some(focused_view) = focused_view.upgrade() {
            let mut focused_view = focused_view.inner.borrow_mut();
            focused_view.is_focused = false;
            focused_view.view.on_unfocus();
        }

        // Focus the next focusable.
        let next_focusable = self.dynamic_sites[(idx + 1)..]
            .iter()
            .chain(self.dynamic_sites[..idx].iter())
            .find(|&weak_iv| {
                weak_iv
                    .upgrade()
                    .unwrap()
                    .inner
                    .borrow()
                    .view
                    .is_focusable()
            })
            .map(|weak_iv| weak_iv.upgrade().unwrap());
        if let Some(next_focusable) = next_focusable {
            let mut next_focusable = next_focusable.inner.borrow_mut();
            next_focusable.is_focused = true;
            next_focusable.view.on_unfocus();
        }
    }

    /// Switch focus to the previous focusable view.
    /// A focusable view is an `View` with its `is_focusable` returning `true`.
    pub fn focus_prev(&mut self) {
        // Unfocus the currnet one.
        // FIXME: make this more efficient by keeping track the of index of the focused view.
        if self.dynamic_sites.is_empty() {
            return;
        }
        let (idx, focused_view) = self
            .dynamic_sites
            .iter()
            .rev()
            .enumerate()
            .find(|(_, weak_iv)| weak_iv.upgrade().unwrap().inner.borrow().is_focused)
            .map(|(i, weak_iv)| (i, weak_iv.clone()))
            .unwrap_or_default();
        if let Some(focused_view) = focused_view.upgrade() {
            let mut focused_view = focused_view.inner.borrow_mut();
            focused_view.is_focused = false;
            focused_view.view.on_unfocus();
        }

        // Focus the next focusable.
        let next_focusable = self.dynamic_sites[..idx]
            .iter()
            .chain(self.dynamic_sites[(idx + 1)..].iter())
            .find(|&weak_iv| {
                weak_iv
                    .upgrade()
                    .unwrap()
                    .inner
                    .borrow()
                    .view
                    .is_focusable()
            })
            .map(|weak_iv| weak_iv.upgrade().unwrap());
        if let Some(next_focusable) = next_focusable {
            let mut next_focusable = next_focusable.inner.borrow_mut();
            next_focusable.is_focused = true;
            next_focusable.view.on_unfocus();
        }
    }

    /// Returns the view currently in focus in the form of a `DynamicViewWrapper`.
    /// Returns `None` if no view is in focus (including the situation where a view was focused but
    /// was since deleted).
    pub fn focused<'b>(&'b self) -> Option<DynamicViewWrapper<'a>> {
        let iv_weak = self
            .dynamic_sites
            .iter()
            .find(|iv_weak| iv_weak.upgrade().unwrap().inner.borrow().is_focused)?;
        iv_weak.upgrade()
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                self.focus_prev();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                self.focus_next();
            }
            Event::Key(key_event) => {
                if let Some(focused_view) = self.focused() {
                    focused_view.inner.borrow_mut().view.on_key_event(key_event);
                }
            }
            _ => (),
        }
    }
}

/// `StaticView`s are a subset of `View`s that are immutable.
/// A dynamic `View` can be wrapped into a `StaticView` through `DynamicViewWrapper`, which can
/// only be created by `ScreenBuilder::dynamic_site` or `Screen::dynamic_site`.
pub trait StaticView {
    fn render_static(&self, frame: &mut Frame, area: Rect);
}

/// All static views are dynamic views.
impl<V: StaticView> View for V {
    fn render(&self, frame: &mut Frame, area: Rect, _is_focused: bool) {
        self.render_static(frame, area)
    }
}

#[allow(unused_variables)]
pub trait View {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool);

    fn is_focusable(&self) -> bool {
        false
    }

    fn on_focus(&mut self) {}

    fn on_unfocus(&mut self) {}

    fn on_key_event(&mut self, key_event: KeyEvent) {}
}

/// Wrap a dynamic view into a static view through internal mutability.
/// Also erases its type.
#[derive(Debug, Clone, From)]
pub struct DynamicViewWrapper<'a> {
    inner: Rc<RefCell<DynamicViewWrapperInner<'a>>>,
}

impl<'a> DynamicViewWrapper<'a> {
    fn new(is_focused: bool, view: impl View + 'a) -> Self {
        let inner = DynamicViewWrapperInner {
            is_focused,
            view: Box::new(view),
        };
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    /// Downgrade to a `Weak` reference.
    fn downgrade(&self) -> DynamicViewWrapperWeakRef<'a> {
        Rc::downgrade(&self.inner).into()
    }

    /// Try to downcast the wrapped `View` into a value of `V: View` and do something to it.
    ///
    /// # Safety
    /// `V` must be of the correct type that the value was initialized with.
    pub unsafe fn inspect<T, V: View + 'a>(&self, f: impl FnOnce(&mut V) -> T) -> T {
        trait RawPtr {
            fn raw_ptr(&mut self) -> *mut ();
        }
        impl<V: View + ?Sized> RawPtr for V {
            fn raw_ptr(&mut self) -> *mut () {
                self as *mut V as *mut ()
            }
        }
        let mut borrow_mut = self.inner.borrow_mut();
        let view: &mut V = unsafe { &mut *(borrow_mut.view.as_mut().raw_ptr() as *mut _) };
        f(view)
    }
}

impl StaticView for DynamicViewWrapper<'_> {
    fn render_static(&self, frame: &mut Frame, area: Rect) {
        let inner = self.inner.borrow();
        inner.view.render(frame, area, inner.is_focused);
    }
}

struct DynamicViewWrapperInner<'a> {
    is_focused: bool,
    /// FIXME: Remove this `Box` for one less indirection.
    view: Box<dyn View + 'a>,
}

impl Debug for DynamicViewWrapperInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DynamicViewWrapperInner")
            .field("is_focused", &self.is_focused)
            .finish_non_exhaustive()
    }
}

/// FIXME: Maybe expose this in the future as API.
#[derive(Debug, Clone, Default, From)]
struct DynamicViewWrapperWeakRef<'a> {
    weak: Weak<RefCell<DynamicViewWrapperInner<'a>>>,
}

impl<'a> DynamicViewWrapperWeakRef<'a> {
    /// Like `Weak::upgrade`, it may fail when either the original `Rc` is dropped or the `Weak` is
    /// null.
    fn upgrade(&self) -> Option<DynamicViewWrapper<'a>> {
        self.weak.upgrade().map(Into::into)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Empty;

impl StaticView for Empty {
    fn render_static(&self, _frame: &mut Frame, _area: Rect) {}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph<'a> {
    widget: widgets::Paragraph<'a>,
}

impl<'a> Paragraph<'a> {
    pub fn new(text: impl Into<Text<'a>>) -> Self {
        Self {
            widget: widgets::Paragraph::new(text),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.widget = self.widget.style(style);
        self
    }

    pub fn get_style(&self) -> Style {
        <widgets::Paragraph as Styled>::style(&self.widget)
    }

    pub fn fg(self, color: Color) -> Self {
        let style = self.get_style().fg(color);
        self.style(style)
    }

    pub fn bg(self, color: Color) -> Self {
        let style = self.get_style().bg(color);
        self.style(style)
    }

    pub fn add_modifier(self, modifier: Modifier) -> Self {
        let style = self.get_style().add_modifier(modifier);
        self.style(style)
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.widget = self.widget.wrap(wrap);
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.widget = self.widget.block(block);
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.widget = self.widget.alignment(alignment);
        self
    }
}

impl<'a> StaticView for Paragraph<'a> {
    fn render_static(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(&self.widget, area);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn horizontal(children: Children) -> Self {
        Self::equal_split(Direction::Horizontal, children)
    }

    pub fn vertical(children: Children) -> Self {
        Self::equal_split(Direction::Vertical, children)
    }
}

impl<Children: ViewTuple> StaticView for Stack<Children> {
    fn render_static(&self, frame: &mut Frame, area: Rect) {
        let chunks = self.layout.split(area);
        self.children.render_each(frame, |i| chunks[i])
    }
}

#[derive(Debug, Clone, Hash)]
pub struct InputField<'a> {
    placeholder: Cow<'a, str>,
    content: InputFieldContent,
    style_focused: Style,
    style_unfocused: Style,
    style_placeholder: Style,
    style_selection: Style,
    block_focused: Block<'a>,
    block_unfocused: Block<'a>,
}

impl<'a> Default for InputField<'a> {
    fn default() -> Self {
        Self {
            placeholder: Cow::default(),
            content: InputFieldContent::default(),
            style_focused: Style::default().fg(Color::White),
            style_unfocused: Style::default().fg(Color::White),
            style_placeholder: Style::new().fg(Color::DarkGray),
            style_selection: Style::new().bg(Color::LightBlue).fg(Color::Black),
            block_focused: Block::default(),
            block_unfocused: Block::default(),
        }
    }
}

impl<'a> InputField<'a> {
    pub fn placeholder(mut self, placeholder: impl Into<Cow<'a, str>>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.content.set_text(text.into());
        self
    }

    pub fn cursor_at_end(mut self) -> Self {
        self.content.cursor_to_end();
        self
    }

    pub fn cursor_at_beginning(mut self) -> Self {
        self.content.cursor_to_beginning();
        self
    }

    pub fn style_focused(mut self, style_focused: Style) -> Self {
        self.style_focused = style_focused;
        self
    }

    pub fn style_unfocused(mut self, style_unfocused: Style) -> Self {
        self.style_unfocused = style_unfocused;
        self
    }

    pub fn block_focused(mut self, block_focused: Block<'a>) -> Self {
        self.block_focused = block_focused;
        self
    }

    pub fn block_unfocused(mut self, block_unfocused: Block<'a>) -> Self {
        self.block_unfocused = block_unfocused;
        self
    }

    pub fn style_placeholder(mut self, style_placeholder: Style) -> Self {
        self.style_placeholder = style_placeholder;
        self
    }

    pub fn style_selection(mut self, style_selection: Style) -> Self {
        self.style_selection = style_selection;
        self
    }

    pub fn content(&self) -> &InputFieldContent {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut InputFieldContent {
        &mut self.content
    }

    fn render_paragraph(&'a self, is_focused: bool) -> Paragraph<'a> {
        let text = self.content.text();
        if text.is_empty() {
            return self.render_placeholder_paragraph(is_focused);
        }
        if !is_focused {
            return Paragraph::new(text).style(self.style_unfocused);
        }
        match self.content.cursor() {
            Cursor::Caret(caret) => {
                if self.content.caret_is_at_end() {
                    return Paragraph::new(Line::from(vec![
                        Span::styled(text, self.style_focused),
                        Span::styled(" ", self.caret_style()),
                    ]));
                }
                let mut caret_next = caret;
                input_field::index_next(text, &mut caret_next);
                Paragraph::new(Line::from(vec![
                    Span::styled(&text[0..caret], self.style_focused),
                    Span::styled(&text[caret..caret_next], self.caret_style()),
                    Span::styled(&text[caret_next..], self.style_focused),
                ]))
            }
            Cursor::Selection(range) => Paragraph::new(Line::from(vec![
                Span::styled(&text[0..range.start], self.style_focused),
                Span::styled(&text[range], self.style_selection),
                Span::styled(&text[range.end..], self.style_focused),
            ])),
        }
    }

    fn render_placeholder_paragraph(&'a self, is_focused: bool) -> Paragraph<'a> {
        if is_focused {
            let placeholder: &'a str = match &self.placeholder {
                Cow::Borrowed(s) => s,
                Cow::Owned(s) => s.as_str(),
            };
            let placeholder_caret_style = self.caret_style().patch(self.style_placeholder);
            let head: &'a str = placeholder.get(..1).unwrap_or("");
            let tail: &'a str = placeholder.get(1..).unwrap_or("");
            Paragraph::new(Line::from(vec![
                Span::styled(head, placeholder_caret_style),
                Span::styled(tail, self.style_placeholder),
            ]))
        } else {
            Paragraph::new(Line::from(vec![Span::styled(
                &self.placeholder[..],
                self.style_placeholder,
            )]))
        }
    }

    fn caret_style(&self) -> Style {
        Style::new().bg(Color::White).fg(Color::Black)
    }
}

impl<'a> View for InputField<'a> {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let block = if is_focused {
            self.block_focused.clone()
        } else {
            self.block_unfocused.clone()
        };
        let paragraph = self
            .render_paragraph(is_focused)
            .block(block)
            .wrap(Wrap { trim: false });
        paragraph.render_static(frame, area);
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn on_key_event(&mut self, key_event: KeyEvent) {
        const CONTROL_SHIFT: KeyModifiers = match KeyModifiers::from_bits(
            KeyModifiers::CONTROL.bits() | KeyModifiers::SHIFT.bits(),
        ) {
            Some(x) => x,
            None => KeyModifiers::NONE,
        };
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::NONE, KeyCode::Left) | (KeyModifiers::CONTROL, KeyCode::Char('b')) => {
                self.content.caret_left()
            }
            (KeyModifiers::NONE, KeyCode::Right) | (KeyModifiers::CONTROL, KeyCode::Char('f')) => {
                self.content.caret_right()
            }
            (KeyModifiers::CONTROL, KeyCode::Left | KeyCode::Char('a')) => {
                self.content.caret_left_end();
            }
            (KeyModifiers::CONTROL, KeyCode::Right | KeyCode::Char('e')) => {
                self.content.caret_right_end();
            }
            (KeyModifiers::SHIFT, KeyCode::Left) => {
                self.content.select_left();
            }
            (KeyModifiers::SHIFT, KeyCode::Right) => {
                self.content.select_right();
            }
            (CONTROL_SHIFT, KeyCode::Left) => {
                self.content.select_left_end();
            }
            (CONTROL_SHIFT, KeyCode::Right) => {
                self.content.select_right_end();
            }
            (KeyModifiers::NONE, KeyCode::Backspace) => self.content.delete_backward(),
            (KeyModifiers::NONE, KeyCode::Delete) | (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                self.content.delete_forward();
            }
            (KeyModifiers::NONE, KeyCode::Char(char)) => self.content.insert(char),
            (KeyModifiers::SHIFT, KeyCode::Char(char)) => {
                // FIXME: Respect more advanced keyboard layout (such as those with AltGr).
                for char in char.to_uppercase() {
                    self.content.insert(char);
                }
            }
            _ => (),
        }
    }
}
