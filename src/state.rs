use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::*;

pub trait Binding<S> {
    fn get(&self) -> RefMut<'_, S>;
}

pub trait AnyState {}

#[derive(Clone)]
pub struct State<S> {
    value: Rc<RefCell<S>>,
}

impl<S> State<S> {
    pub fn new(value: S) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn set(&self, value: S) {
        *self.value.borrow_mut() = value;
    }
}

impl<S> AnyState for State<S> {}

impl<S> Binding<S> for State<S> {
    fn get(&self) -> RefMut<'_, S> {
        // Here we can indicate that a state change has
        // been made.
        self.value.borrow_mut()
    }
}

pub struct StateView<S: 'static, V: View> {
    default: S,
    func: Box<dyn Fn(State<S>) -> V>,
}

impl<S, V> View for StateView<S, V>
where
    V: View,
    S: Clone,
{
    fn print(&self, id: ViewID, cx: &mut Context) {
        cx.with_state(self.default.clone(), id, |state: State<S>, cx| {
            (*self.func)(state.clone()).print(id.child(0), cx);
        });
    }

    fn process(&self, event: &Event, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        cx.with_state_vger(vger, self.default.clone(), id, |state: State<S>, cx, vger| {
            (*self.func)(state.clone()).process(event, id.child(0), cx, vger);
        })
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        cx.with_state_vger(
            vger,
            self.default.clone(),
            id,
            |state: State<S>, cx, vger| {
                (*self.func)(state.clone()).draw(id.child(0), cx, vger);
            },
        );
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        cx.with_state_vger(
            vger,
            self.default.clone(),
            id,
            |state: State<S>, cx, vger| {
                (*self.func)(state.clone()).layout(id.child(0), sz, cx, vger)
            },
        )
    }

    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID> {
        cx.with_state_vger(
            vger,
            self.default.clone(),
            id,
            |state: State<S>, cx, vger| {
                (*self.func)(state.clone()).hittest(id.child(0), pt, cx, vger)
            },
        )
    }
}

pub fn state<S: Clone, V: View, F: Fn(State<S>) -> V + 'static>(
    initial: S,
    f: F,
) -> StateView<S, V> {
    StateView {
        default: initial,
        func: Box::new(f),
    }
}

pub struct ValueBinding<'a, S> {
    func: Box<dyn Fn() -> RefMut<'a, S> >,
}

impl<S> Binding<S> for ValueBinding<'_, S> {
    fn get(&self) -> RefMut<'_, S> {
        (*self.func)()
    }
} 