use ratatui::{layout::Rect, Frame};

use super::*;

pub trait ComponentList {
    const LEN: usize;
    fn render_each(&self, frame: &mut Frame, rect: impl FnMut(usize) -> Rect);
}

impl<const N: usize, T: Component> ComponentList for [T; N] {
    const LEN: usize = N;

    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        for (i, component) in self.iter().enumerate() {
            component.render(frame, rect(i));
        }
    }
}

impl ComponentList for () {
    const LEN: usize = 0;
    fn render_each(&self, _frame: &mut Frame, _rect: impl FnMut(usize) -> Rect) {}
}

impl<T: Component> ComponentList for (T,) {
    const LEN: usize = 1;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
    }
}

impl<T0: Component, T1: Component> ComponentList for (T0, T1) {
    const LEN: usize = 2;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
    }
}

impl<T0: Component, T1: Component, T2: Component> ComponentList for (T0, T1, T2) {
    const LEN: usize = 3;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
    }
}

impl<T0: Component, T1: Component, T2: Component, T3: Component> ComponentList
    for (T0, T1, T2, T3)
{
    const LEN: usize = 4;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
    }
}

impl<T0: Component, T1: Component, T2: Component, T3: Component, T4: Component> ComponentList
    for (T0, T1, T2, T3, T4)
{
    const LEN: usize = 5;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
    }
}

impl<T0: Component, T1: Component, T2: Component, T3: Component, T4: Component, T5: Component>
    ComponentList for (T0, T1, T2, T3, T4, T5)
{
    const LEN: usize = 6;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
        self.5.render(frame, rect(5));
    }
}

impl<
        T0: Component,
        T1: Component,
        T2: Component,
        T3: Component,
        T4: Component,
        T5: Component,
        T6: Component,
    > ComponentList for (T0, T1, T2, T3, T4, T5, T6)
{
    const LEN: usize = 7;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
        self.5.render(frame, rect(5));
        self.6.render(frame, rect(6));
    }
}

impl<
        T0: Component,
        T1: Component,
        T2: Component,
        T3: Component,
        T4: Component,
        T5: Component,
        T6: Component,
        T7: Component,
    > ComponentList for (T0, T1, T2, T3, T4, T5, T6, T7)
{
    const LEN: usize = 8;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
        self.5.render(frame, rect(5));
        self.6.render(frame, rect(6));
        self.7.render(frame, rect(7));
    }
}

impl<
        T0: Component,
        T1: Component,
        T2: Component,
        T3: Component,
        T4: Component,
        T5: Component,
        T6: Component,
        T7: Component,
        T8: Component,
    > ComponentList for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    const LEN: usize = 9;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
        self.5.render(frame, rect(5));
        self.6.render(frame, rect(6));
        self.7.render(frame, rect(7));
        self.8.render(frame, rect(8));
    }
}

impl<
        T0: Component,
        T1: Component,
        T2: Component,
        T3: Component,
        T4: Component,
        T5: Component,
        T6: Component,
        T7: Component,
        T8: Component,
        T9: Component,
    > ComponentList for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    const LEN: usize = 10;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
        self.5.render(frame, rect(5));
        self.6.render(frame, rect(6));
        self.7.render(frame, rect(7));
        self.8.render(frame, rect(8));
        self.9.render(frame, rect(9));
    }
}

impl<
        T0: Component,
        T1: Component,
        T2: Component,
        T3: Component,
        T4: Component,
        T5: Component,
        T6: Component,
        T7: Component,
        T8: Component,
        T9: Component,
        T10: Component,
    > ComponentList for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    const LEN: usize = 11;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
        self.5.render(frame, rect(5));
        self.6.render(frame, rect(6));
        self.7.render(frame, rect(7));
        self.8.render(frame, rect(8));
        self.9.render(frame, rect(9));
        self.10.render(frame, rect(10));
    }
}

impl<
        T0: Component,
        T1: Component,
        T2: Component,
        T3: Component,
        T4: Component,
        T5: Component,
        T6: Component,
        T7: Component,
        T8: Component,
        T9: Component,
        T10: Component,
        T11: Component,
    > ComponentList for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    const LEN: usize = 12;
    fn render_each(&self, frame: &mut Frame, mut rect: impl FnMut(usize) -> Rect) {
        self.0.render(frame, rect(0));
        self.1.render(frame, rect(1));
        self.2.render(frame, rect(2));
        self.3.render(frame, rect(3));
        self.4.render(frame, rect(4));
        self.5.render(frame, rect(5));
        self.6.render(frame, rect(6));
        self.7.render(frame, rect(7));
        self.8.render(frame, rect(8));
        self.9.render(frame, rect(9));
        self.10.render(frame, rect(10));
        self.11.render(frame, rect(11));
    }
}
