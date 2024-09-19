use std::convert::Infallible;

use ratatui::{layout::Rect, Frame};
use views::Size;

use super::*;

#[rustfmt::skip]
mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for ! {}
    impl Sealed for Infallible {}
    impl<V: View> Sealed for V {}
    impl Sealed for () {}
    impl<V0: View> Sealed for (V0,) {}
    impl<V0: View, V1: View> Sealed for (V0, V1) {}
    impl<V0: View, V1: View, V2: View> Sealed for (V0, V1, V2) {}
    impl<V0: View, V1: View, V2: View, V3: View> Sealed for (V0, V1, V2, V3) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View> Sealed for (V0, V1, V2, V3, V4) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View> Sealed for (V0, V1, V2, V3, V4, V5) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View> Sealed for (V0, V1, V2, V3, V4, V5, V6) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View> Sealed for (V0, V1, V2, V3, V4, V5, V6, V7) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View> Sealed for (V0, V1, V2, V3, V4, V5, V6, V7, V8) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View, V9: View> Sealed for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View, V9: View, V10: View> Sealed for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10) {}
    impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View, V9: View, V10: View, V11: View> Sealed for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11) {}
}

/// A tuple `(V0, V1, V2, ...)` where all its members are `View`s.
/// For convenience sake it's also implemented for all `V: View` and `!`.
/// For practicality sake it's only implemented for tuples up to (inclusive) 12 members.
pub trait ViewTuple: private::Sealed {
    const LEN: usize;
    /// Call `render` for each of the `View`s in the `ViewTuple`.
    /// `preferred_size` is called on each `View`.
    fn for_each_render(&self, frame: &mut Frame, rect: impl FnMut(usize, Option<Size>) -> Rect);

    /// Call `preferred_size` for each of the `View`s in the `ViewTuple`.
    fn for_each_preferred_size(&self, f: impl FnMut(Option<Size>));
}

impl ViewTuple for ! {
    const LEN: usize = 0;
    fn for_each_render(&self, _frame: &mut Frame, _rect: impl FnMut(usize, Option<Size>) -> Rect) {}
    fn for_each_preferred_size(&self, _f: impl FnMut(Option<Size>)) {}
}

impl ViewTuple for Infallible {
    const LEN: usize = 0;
    fn for_each_render(&self, _frame: &mut Frame, _rect: impl FnMut(usize, Option<Size>) -> Rect) {}
    fn for_each_preferred_size(&self, _f: impl FnMut(Option<Size>)) {}
}

impl<V: View> ViewTuple for V {
    const LEN: usize = 1;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.render(frame, rect(0, self.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.preferred_size());
    }
}

impl ViewTuple for () {
    const LEN: usize = 0;
    fn for_each_render(&self, _frame: &mut Frame, _rect: impl FnMut(usize, Option<Size>) -> Rect) {}
    fn for_each_preferred_size(&self, _f: impl FnMut(Option<Size>)) {}
}

impl<V: View> ViewTuple for (V,) {
    const LEN: usize = 1;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
    }
}

impl<V0: View, V1: View> ViewTuple for (V0, V1) {
    const LEN: usize = 2;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
    }
}

impl<V0: View, V1: View, V2: View> ViewTuple for (V0, V1, V2) {
    const LEN: usize = 3;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
    }
}

impl<V0: View, V1: View, V2: View, V3: View> ViewTuple for (V0, V1, V2, V3) {
    const LEN: usize = 4;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
    }
}

impl<V0: View, V1: View, V2: View, V3: View, V4: View> ViewTuple for (V0, V1, V2, V3, V4) {
    const LEN: usize = 5;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
    }
}

impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View> ViewTuple
    for (V0, V1, V2, V3, V4, V5)
{
    const LEN: usize = 6;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
        self.5.render(frame, rect(5, self.5.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
        f(self.4.preferred_size());
    }
}

impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View> ViewTuple
    for (V0, V1, V2, V3, V4, V5, V6)
{
    const LEN: usize = 7;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
        self.5.render(frame, rect(5, self.5.preferred_size()));
        self.6.render(frame, rect(6, self.6.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
        f(self.4.preferred_size());
        f(self.5.preferred_size());
    }
}

impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View> ViewTuple
    for (V0, V1, V2, V3, V4, V5, V6, V7)
{
    const LEN: usize = 8;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
        self.5.render(frame, rect(5, self.5.preferred_size()));
        self.6.render(frame, rect(6, self.6.preferred_size()));
        self.7.render(frame, rect(7, self.7.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
        f(self.4.preferred_size());
        f(self.5.preferred_size());
        f(self.6.preferred_size());
    }
}

impl<V0: View, V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View>
    ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8)
{
    const LEN: usize = 9;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
        self.5.render(frame, rect(5, self.5.preferred_size()));
        self.6.render(frame, rect(6, self.6.preferred_size()));
        self.7.render(frame, rect(7, self.7.preferred_size()));
        self.8.render(frame, rect(8, self.8.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
        f(self.4.preferred_size());
        f(self.5.preferred_size());
        f(self.6.preferred_size());
        f(self.7.preferred_size());
    }
}

impl<
        V0: View,
        V1: View,
        V2: View,
        V3: View,
        V4: View,
        V5: View,
        V6: View,
        V7: View,
        V8: View,
        V9: View,
    > ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9)
{
    const LEN: usize = 10;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
        self.5.render(frame, rect(5, self.5.preferred_size()));
        self.6.render(frame, rect(6, self.6.preferred_size()));
        self.7.render(frame, rect(7, self.7.preferred_size()));
        self.8.render(frame, rect(8, self.8.preferred_size()));
        self.9.render(frame, rect(9, self.9.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
        f(self.4.preferred_size());
        f(self.5.preferred_size());
        f(self.6.preferred_size());
        f(self.7.preferred_size());
        f(self.8.preferred_size());
    }
}

impl<
        V0: View,
        V1: View,
        V2: View,
        V3: View,
        V4: View,
        V5: View,
        V6: View,
        V7: View,
        V8: View,
        V9: View,
        V10: View,
    > ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10)
{
    const LEN: usize = 11;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
        self.5.render(frame, rect(5, self.5.preferred_size()));
        self.6.render(frame, rect(6, self.6.preferred_size()));
        self.7.render(frame, rect(7, self.7.preferred_size()));
        self.8.render(frame, rect(8, self.8.preferred_size()));
        self.9.render(frame, rect(9, self.9.preferred_size()));
        self.10.render(frame, rect(10, self.10.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
        f(self.4.preferred_size());
        f(self.5.preferred_size());
        f(self.6.preferred_size());
        f(self.7.preferred_size());
        f(self.8.preferred_size());
        f(self.9.preferred_size());
    }
}

impl<
        V0: View,
        V1: View,
        V2: View,
        V3: View,
        V4: View,
        V5: View,
        V6: View,
        V7: View,
        V8: View,
        V9: View,
        V10: View,
        V11: View,
    > ViewTuple for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11)
{
    const LEN: usize = 12;
    fn for_each_render(
        &self,
        frame: &mut Frame,
        mut rect: impl FnMut(usize, Option<Size>) -> Rect,
    ) {
        self.0.render(frame, rect(0, self.0.preferred_size()));
        self.1.render(frame, rect(1, self.1.preferred_size()));
        self.2.render(frame, rect(2, self.2.preferred_size()));
        self.3.render(frame, rect(3, self.3.preferred_size()));
        self.4.render(frame, rect(4, self.4.preferred_size()));
        self.5.render(frame, rect(5, self.5.preferred_size()));
        self.6.render(frame, rect(6, self.6.preferred_size()));
        self.7.render(frame, rect(7, self.7.preferred_size()));
        self.8.render(frame, rect(8, self.8.preferred_size()));
        self.9.render(frame, rect(9, self.9.preferred_size()));
        self.10.render(frame, rect(10, self.10.preferred_size()));
        self.11.render(frame, rect(11, self.11.preferred_size()));
    }
    fn for_each_preferred_size(&self, mut f: impl FnMut(Option<Size>)) {
        f(self.0.preferred_size());
        f(self.1.preferred_size());
        f(self.1.preferred_size());
        f(self.2.preferred_size());
        f(self.3.preferred_size());
        f(self.4.preferred_size());
        f(self.5.preferred_size());
        f(self.6.preferred_size());
        f(self.7.preferred_size());
        f(self.8.preferred_size());
        f(self.9.preferred_size());
        f(self.10.preferred_size());
    }
}
