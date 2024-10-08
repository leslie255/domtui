#![allow(dead_code)]

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Debug},
    io,
    sync::{Arc, Mutex, Weak},
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
pub struct Screen<'a, V: View + 'a> {
    root_view: V,
    dynamic_sites: Vec<ViewCellWeakRef<'a>>,
    dynamic_site_tags: HashMap<Cow<'a, str>, ViewCellWeakRef<'a>>,
}

/// `'a` for allowing to borrow from a data source.
#[derive(Debug, Clone, Default)]
pub struct ScreenBuilder<'a> {
    dynamic_sites: Vec<ViewCellWeakRef<'a>>,
    dynamic_site_tags: HashMap<Cow<'a, str>, ViewCellWeakRef<'a>>,
}

impl<'a> ScreenBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finish<V: View>(self, root_view: V) -> Screen<'a, V> {
        Screen {
            root_view,
            dynamic_sites: self.dynamic_sites,
            dynamic_site_tags: self.dynamic_site_tags,
        }
    }

    /// Wrap a `MutView` into a `ViewCell`, which implements non-mut `View`.
    pub fn view_cell(&mut self, view: impl MutView + Send + 'a) -> ViewCell<'a> {
        let view = ViewCell::new(false, view);
        self.dynamic_sites.push(view.downgrade());
        view
    }

    /// Wrap a `MutView` into a `ViewCell`, which implements non-mut `View`, and tag it.
    pub fn tagged_view_cell(
        &mut self,
        tag: impl Into<Cow<'a, str>>,
        view: impl MutView + Send + 'a,
    ) -> ViewCell<'a> {
        let view = ViewCell::new(false, view);
        self.dynamic_site_tags.insert(tag.into(), view.downgrade());
        self.dynamic_sites.push(view.downgrade());
        view
    }
}

impl<'a, V: View + 'a> Screen<'a, V> {
    /// Create a screen with just non-mut views.
    /// For creating a screen with mutable views, use `ScreenBuilder` and
    /// `ScreenBuilder::view_cell`, `ScreenBuilder::tagged_view_cell`.
    pub fn new(root_view: V) -> Self {
        ScreenBuilder::default().finish(root_view)
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

    /// Wrap a `MutView` into a `ViewCell`, which implements non-mut `View`.
    /// This function is for mutating views in a screen after it was built, for creating a
    /// `ViewCell` during building of the screen, use `ScreenBuilder`.
    pub fn view_cell(&mut self, view: impl MutView + Send + 'a) -> ViewCell<'a> {
        let dynamic_site = ViewCell::new(false, view);
        self.dynamic_sites.push(dynamic_site.downgrade());
        dynamic_site
    }

    /// Wrap a `MutView` into a `ViewCell`, which implements non-mut `View`, and tag it.
    /// This function is for mutating views in a screen after it was built, for creating a
    /// `ViewCell` during building of the screen, use `ScreenBuilder`.
    pub fn tagged_view_cell(&mut self, view: impl MutView + Send + 'a) -> ViewCell<'a> {
        let dynamic_site = ViewCell::new(false, view);
        self.dynamic_sites.push(dynamic_site.downgrade());
        dynamic_site
    }

    /// If multiple views were tagged the same, only one of them is inspected, randomly.
    /// Returns `None` if no view of such tag exists.
    /// If more than one view of such tag exist, one of the views would be provided at random.
    ///
    /// # Safety
    /// `V` must be of the correct type that the value was initialized with.
    pub unsafe fn inspect_view_with_tag_unchecked<T, V2: MutView + 'a>(
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
        // FIXME: optimize this by keeping track the of index of the focused view.
        if self.dynamic_sites.is_empty() {
            return;
        }

        // Unfocus the focused view.
        let (idx, focused_view) = self
            .dynamic_sites
            .iter()
            .enumerate()
            .find(|(_, weak_mv)| {
                weak_mv
                    .upgrade()
                    .is_some_and(|vc| vc.inner.lock().unwrap().is_focused)
            })
            .map(|(i, weak_mv)| (i, weak_mv.clone()))
            .unwrap_or_default();
        let start_idx = match focused_view.upgrade() {
            Some(focused_view) => {
                let mut focused_view = focused_view.inner.lock().unwrap();
                focused_view.is_focused = false;
                focused_view.view.on_unfocus();
                idx + 1
            }
            None => 0,
        };

        // Focus the next focusable.
        let next_focusable = self.dynamic_sites[start_idx..]
            .iter()
            .chain(
                self.dynamic_sites
                    .get(..start_idx.wrapping_sub(1))
                    .unwrap_or(&[])
                    .iter(),
            )
            .find(|&weak_mv| {
                weak_mv
                    .upgrade()
                    .is_some_and(|vc| vc.inner.lock().unwrap().view.is_focusable())
            })
            .map(|weak_mv| weak_mv.upgrade().unwrap());
        if let Some(next_focusable) = next_focusable {
            let mut next_focusable = next_focusable.inner.lock().unwrap();
            next_focusable.is_focused = true;
            next_focusable.view.on_focus();
        }
    }

    /// Returns the view currently in focus in the form of a `ViewCell`.
    /// Returns `None` if no view is in focus (including the situation where a view was focused but
    /// was since deleted).
    pub fn focused(&self) -> Option<ViewCell<'a>> {
        for mv in &self.dynamic_sites {
            let mv = mv.upgrade()?;
            if mv.inner.lock().unwrap().is_focused {
                return Some(mv);
            }
        }
        None
    }

    /// Pass an event into the screen.
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                // TODO: self.focus_prev();
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
                    focused_view
                        .inner
                        .lock()
                        .unwrap()
                        .view
                        .on_key_event(key_event);
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Self::new(width, height)
    }
}

/// A `View` is an immtuable view.
/// For mutable views, use `MutView` and wrap it in a `ViewCell`.
pub trait View {
    fn render(&self, frame: &mut Frame, area: Rect);

    /// Views like `Stack` will try to satisfy the wanted view size as much as possible.
    /// Note that there are two functions of the name `preferred_size` in `View` and `MutView`.
    /// `ViewCell` would delegate calls to `View::preferred_size` to `MutView::preferred_size` of
    /// the inner mutable view.
    fn preferred_size(&self) -> Option<Size> {
        None
    }
}

/// A mutable view.
/// To be able to render a mutable view, wrap it in a `ViewCell`.
#[allow(unused_variables)]
pub trait MutView {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool);

    /// Views like `Stack` will try to satisfy the wanted view size as much as possible.
    /// Note that there are two functions of the name `preferred_size` in `View` and `MutView`.
    /// `ViewCell` would delegate calls to `View::preferred_size` to `MutView::preferred_size` of
    /// the inner mutable view.
    fn preferred_size(&self) -> Option<Size> {
        None
    }

    fn is_focusable(&self) -> bool {
        false
    }

    fn on_focus(&mut self) {}

    fn on_unfocus(&mut self) {}

    fn on_key_event(&mut self, key_event: KeyEvent) {}
}

/// Wrap a `MutView` into a `View` through internal mutability.
/// Also erases its type.
/// Can be created by calling `view_cell` on `Screen` or `ScreenBuilder`.
#[derive(Debug, Clone, From)]
pub struct ViewCell<'a> {
    inner: Arc<Mutex<ViewCellInner<'a>>>,
}

impl<'a> ViewCell<'a> {
    /// Internal function for creating a new `ViewCell`.
    fn new(is_focused: bool, view: impl MutView + Send + 'a) -> Self {
        let inner = ViewCellInner {
            is_focused,
            view: Box::new(view),
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    /// Downgrade to a weak reference.
    fn downgrade(&self) -> ViewCellWeakRef<'a> {
        Arc::downgrade(&self.inner).into()
    }

    /// Downcast the wrapped `MutView` into a value of concrete type.
    /// Because `ViewCell` erases the type of the wrapped view, such downcasting is `unsafe`.
    ///
    /// FIXME: make it safe.
    ///
    /// # Safety
    /// `V` must be of the correct type that the value was initialized with.
    pub unsafe fn inspect<T, MV: MutView + 'a>(&self, f: impl FnOnce(&mut MV) -> T) -> T {
        trait RawPtr {
            fn raw_ptr(&mut self) -> *mut ();
        }
        impl<V: MutView + ?Sized> RawPtr for V {
            fn raw_ptr(&mut self) -> *mut () {
                self as *mut V as *mut ()
            }
        }
        let mut borrow_mut = self.inner.lock().unwrap();
        let view: &mut MV = unsafe { &mut *(borrow_mut.view.as_mut().raw_ptr() as *mut _) };
        f(view)
    }
}

impl View for ViewCell<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let inner = self.inner.lock().unwrap();
        inner.view.render(frame, area, inner.is_focused);
    }

    fn preferred_size(&self) -> Option<Size> {
        self.inner.lock().unwrap().view.preferred_size()
    }
}

struct ViewCellInner<'a> {
    is_focused: bool,
    /// FIXME: Remove this `Box` for one less indirection.
    view: Box<dyn MutView + Send + 'a>,
}

impl Debug for ViewCellInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ViewCellInner")
            .field("is_focused", &self.is_focused)
            .finish_non_exhaustive()
    }
}

/// A weak reference to a `MutView`.
/// FIXME: Maybe expose this in the future as an API.
#[derive(Debug, Clone, From)]
pub(crate) struct ViewCellWeakRef<'a> {
    weak: Weak<Mutex<ViewCellInner<'a>>>,
}

impl<'a> ViewCellWeakRef<'a> {
    /// Creates a dangling reference.
    pub(crate) const fn new() -> Self {
        Self { weak: Weak::new() }
    }

    /// Like `Weak::upgrade`, it may fail when either the original `Arc` is dropped or if the inner
    /// weak pointer is dangling (such pointer can be created by `ViewCellWeakRef::default()`).
    pub(crate) fn upgrade(&self) -> Option<ViewCell<'a>> {
        self.weak.upgrade().map(Into::into)
    }

    /// If two `ViewCellWeakRef` points to the same view.
    /// Returns `true` if both are invalid.
    pub(crate) fn is(&self, other: Self) -> bool {
        self.weak.ptr_eq(&other.weak)
    }
}

impl<'a> Default for ViewCellWeakRef<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// An empty view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Empty;

impl View for Empty {
    fn render(&self, _frame: &mut Frame, _area: Rect) {}
}

/// Wraps an inner view and override's its size preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizedView<V: View> {
    preferred_size: Size,
    inner: V,
}

impl<V: View> View for SizedView<V> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        self.inner.render(frame, area);
    }

    fn preferred_size(&self) -> Option<Size> {
        Some(self.preferred_size)
    }
}

impl<V: View> SizedView<V> {
    pub const fn new(preferred_size: Size, inner: V) -> Self {
        Self {
            inner,
            preferred_size,
        }
    }
}

pub trait ViewExt: View + Sized {
    fn prefers_size(self, preferred_size: impl Into<Size>) -> SizedView<Self> {
        SizedView::new(preferred_size.into(), self)
    }
}

impl<V: View + Sized> ViewExt for V {}

/// An immutable view that displays some text.
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

impl<'a> View for Paragraph<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(&self.widget, area);
    }
}

/// A stack of views that is either horizontal or vertical.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack<Vs: ViewTuple> {
    children: Vs,
    layout: Layout,
    direction: Direction,
}

impl<Vs: ViewTuple> Stack<Vs> {
    fn with_layout(direction: Direction, constraints: Vec<Constraint>, children: Vs) -> Self {
        Self {
            children,
            layout: Layout::new(direction, constraints),
            direction,
        }
    }

    /// Automatically derive layout, given a direction.
    /// For views with `preferred_size`, `preferred_size` is used.
    /// Views without `preferred_size`s are equally split.
    pub fn with_direction(direction: Direction, children: Vs) -> Self {
        let mut constraints: Vec<Constraint> = Vec::with_capacity(Vs::LEN);
        children.for_each_preferred_size(|preferred_size| match preferred_size {
            Some(Size { width, height }) => {
                let length = match direction {
                    Direction::Horizontal => width,
                    Direction::Vertical => height,
                };
                constraints.push(Constraint::Length(length));
            }
            None => constraints.push(Constraint::Fill(1)),
        });
        Self::with_layout(direction, constraints, children)
    }

    pub fn horizontal(children: Vs) -> Self {
        Self::with_direction(Direction::Horizontal, children)
    }

    pub fn vertical(children: Vs) -> Self {
        Self::with_direction(Direction::Vertical, children)
    }
}

impl<Vs: ViewTuple> View for Stack<Vs> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = self.layout.split(area);
        self.children.for_each_render(frame, |i, preferred_size| {
            let mut area = chunks[i];
            match (self.direction, preferred_size) {
                (Direction::Horizontal, Some(preferrde_size)) => {
                    if preferrde_size.height != 0 {
                        area.height = preferrde_size.height.min(area.height);
                    }
                }
                (Direction::Vertical, Some(preferred_size)) => {
                    if preferred_size.width != 0 {
                        area.width = preferred_size.width.min(area.width);
                    }
                }
                _ => (),
            };
            area
        })
    }
}

/// FIXME: make it multi-line.
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
                let caret_next = input_field::next_index_in_str(text, caret);
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

impl<'a> MutView for InputField<'a> {
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
        paragraph.render(frame, area);
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
