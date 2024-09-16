use std::convert::Infallible;

use ratatui::{layout::Rect, Frame};

use super::*;

/// A tuple `(V0, V1, V2, ...)` where all its members are `View`s.
/// For convenience sake it's also implemented for all `V: View` and `!`.
/// For practicality sake it's only implemented for tuples up to (inclusive) 12 members.
pub trait ViewTuple {
    const LEN: usize;
    fn render_each(&self, frame: &mut Frame, rect: impl FnMut(usize) -> Rect);
}

impl ViewTuple for ! {
    const LEN: usize = 0;
    fn render_each(&self, _frame: &mut Frame, _rect: impl FnMut(usize) -> Rect) {}
}

impl ViewTuple for Infallible {
    const LEN: usize = 0;
    fn render_each(&self, _frame: &mut Frame, _rect: impl FnMut(usize) -> Rect) {}
}

impl<V: StaticView> ViewTuple for V {
    const LEN: usize = 1;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.render_static(frame, rect(0));
    }
}

impl ViewTuple for () {
    const LEN: usize = 0;
    fn render_each(&self, _frame: &mut Frame, _rect: impl FnMut(usize) -> Rect) {}
}

impl<V: StaticView> ViewTuple for (V,) {
    const LEN: usize = 1;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
    }
}

impl<V0: StaticView, V1: StaticView> ViewTuple for (V0, V1) {
    const LEN: usize = 2;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
    }
}

impl<V0: StaticView, V1: StaticView, V2: StaticView> ViewTuple for (V0, V1, V2) {
    const LEN: usize = 3;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
    }
}

impl<V0: StaticView, V1: StaticView, V2: StaticView, V3: StaticView> ViewTuple for (V0, V1, V2, V3) {
    const LEN: usize = 4;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
    }
}

impl<V0: StaticView, V1: StaticView, V2: StaticView, V3: StaticView, V4: StaticView> ViewTuple for (V0, V1, V2, V3, V4) {
    const LEN: usize = 5;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
    }
}

impl<V0: StaticView, V1: StaticView, V2: StaticView, V3: StaticView, V4: StaticView, V5: StaticView> ViewTuple
    for (V0, V1, V2, V3, V4, V5)
{
    const LEN: usize = 6;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
        self.5.render_static(frame, rect(5));
    }
}

impl<V0: StaticView, V1: StaticView, V2: StaticView, V3: StaticView, V4: StaticView, V5: StaticView, V6: StaticView> ViewTuple
    for (V0, V1, V2, V3, V4, V5, V6)
{
    const LEN: usize = 7;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
        self.5.render_static(frame, rect(5));
        self.6.render_static(frame, rect(6));
    }
}

impl<V0: StaticView, V1: StaticView, V2: StaticView, V3: StaticView, V4: StaticView, V5: StaticView, V6: StaticView, V7: StaticView> ViewTuple
    for (V0, V1, V2, V3, V4, V5, V6, V7)
{
    const LEN: usize = 8;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
        self.5.render_static(frame, rect(5));
        self.6.render_static(frame, rect(6));
        self.7.render_static(frame, rect(7));
    }
}

impl<V0: StaticView, V1: StaticView, V2: StaticView, V3: StaticView, V4: StaticView, V5: StaticView, V6: StaticView, V7: StaticView, V8: StaticView>
    ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8)
{
    const LEN: usize = 9;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
        self.5.render_static(frame, rect(5));
        self.6.render_static(frame, rect(6));
        self.7.render_static(frame, rect(7));
        self.8.render_static(frame, rect(8));
    }
}

impl<
        V0: StaticView,
        V1: StaticView,
        V2: StaticView,
        V3: StaticView,
        V4: StaticView,
        V5: StaticView,
        V6: StaticView,
        V7: StaticView,
        V8: StaticView,
        V9: StaticView,
    > ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9)
{
    const LEN: usize = 10;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
        self.5.render_static(frame, rect(5));
        self.6.render_static(frame, rect(6));
        self.7.render_static(frame, rect(7));
        self.8.render_static(frame, rect(8));
        self.9.render_static(frame, rect(9));
    }
}

impl<
        V0: StaticView,
        V1: StaticView,
        V2: StaticView,
        V3: StaticView,
        V4: StaticView,
        V5: StaticView,
        V6: StaticView,
        V7: StaticView,
        V8: StaticView,
        V9: StaticView,
        V10: StaticView,
    > ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10)
{
    const LEN: usize = 11;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
        self.5.render_static(frame, rect(5));
        self.6.render_static(frame, rect(6));
        self.7.render_static(frame, rect(7));
        self.8.render_static(frame, rect(8));
        self.9.render_static(frame, rect(9));
        self.10.render_static(frame, rect(10));
    }
}

impl<
        V0: StaticView,
        V1: StaticView,
        V2: StaticView,
        V3: StaticView,
        V4: StaticView,
        V5: StaticView,
        V6: StaticView,
        V7: StaticView,
        V8: StaticView,
        V9: StaticView,
        V10: StaticView,
        V11: StaticView,
    > ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11)
{
    const LEN: usize = 12;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render_static(frame, rect(0));
        self.1.render_static(frame, rect(1));
        self.2.render_static(frame, rect(2));
        self.3.render_static(frame, rect(3));
        self.4.render_static(frame, rect(4));
        self.5.render_static(frame, rect(5));
        self.6.render_static(frame, rect(6));
        self.7.render_static(frame, rect(7));
        self.8.render_static(frame, rect(8));
        self.9.render_static(frame, rect(9));
        self.10.render_static(frame, rect(10));
        self.11.render_static(frame, rect(11));
    }
}
