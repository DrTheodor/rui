use std::any::Any;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use tao::event_loop::EventLoopProxy;
use std::cell::RefCell;
use crate::*;

static STATE_DIRTY: AtomicBool = AtomicBool::new(false);

thread_local! {
    pub static ENABLE_DIRTY: RefCell<bool> = RefCell::new(true);
}

pub(crate) fn is_state_dirty() -> bool {
    STATE_DIRTY.load(Ordering::Relaxed)
}

pub(crate) fn set_state_dirty() {
    ENABLE_DIRTY.with(|enable| {
        if *enable.borrow() {
            STATE_DIRTY.store(true, Ordering::Relaxed);
        }
    })
}

pub(crate) fn clear_state_dirty() {
    STATE_DIRTY.store(false, Ordering::Relaxed);
}

struct Holder<S> {
    value: S,
}

/// Contains application state. Application state is created using `state`.
#[derive(Clone)]
pub struct State<S> {
    value: Arc<Mutex<Holder<S>>>,
    event_loop_proxy: Option<EventLoopProxy<()>>,
}

impl<S> State<S> {
    pub fn new(value: S, event_loop_proxy: Option<EventLoopProxy<()>>) -> Self {
        Self {
            value: Arc::new(Mutex::new(Holder { value })),
            event_loop_proxy,
        }
    }
}

impl<S> AnyState for State<S>
where
    S: 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<S> Binding<S> for State<S>
where
    S: Clone + 'static,
{
    fn with<T, F: FnOnce(&S) -> T>(&self, f: F) -> T {
        f(&self.value.lock().unwrap().value)
    }
    fn with_mut<T, F: FnOnce(&mut S) -> T>(&self, f: F) -> T {
        let mut holder = self.value.lock().unwrap();
        // Set dirty so the view tree will be redrawn.
        set_state_dirty();
        let t = f(&mut holder.value);

        // Wake up the event loop.
        if let Some(proxy) = &self.event_loop_proxy {
            if let Err(err) = proxy.send_event(()) {
                println!("error waking up event loop: {:?}", err);
            }
        }

        t
    }
}

struct StateView<D, F> {
    default: D,
    func: F,
}

impl<S, V, D, F> View for StateView<D, F>
where
    V: View,
    S: Clone + 'static,
    D: Fn() -> S,
    F: Fn(State<S>) -> V,
{
    fn print(&self, id: ViewID, cx: &mut Context) {
        let s = cx.get_state(id, &self.default);
        (self.func)(s).print(id.child(&0), cx);
    }

    fn process(&self, event: &Event, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        let s = cx.get_state(id, &self.default);
        (self.func)(s).process(event, id.child(&0), cx, vger);
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        let s = cx.get_state(id, &self.default);
        (self.func)(s).draw(id.child(&0), cx, vger);
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        let s = cx.get_state(id, &self.default);
        (self.func)(s).layout(id.child(&0), sz, cx, vger)
    }

    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID> {
        let s = cx.get_state(id, &self.default);
        (self.func)(s).hittest(id.child(&0), pt, cx, vger)
    }

    fn commands(&self, id: ViewID, cx: &mut Context, cmds: &mut Vec<CommandInfo>) {
        let s = cx.get_state(id, &self.default);
        (self.func)(s).commands(id.child(&0), cx, cmds);
    }

    fn gc(&self, id: ViewID, cx: &mut Context, map: &mut StateMap) {
        let s = cx.get_state(id, &self.default);
        map.insert(id, Box::new(s.clone()));
        (self.func)(s).gc(id.child(&0), cx, map);
    }

    fn access(
        &self,
        id: ViewID,
        cx: &mut Context,
        nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        let s = cx.get_state(id, &self.default);
        (self.func)(s).access(id.child(&0), cx, nodes)
    }
}

impl<S, F> private::Sealed for StateView<S, F> {}

/// State allows you to associate some state with a view.
/// This is what you'll use for a data model, as well as per-view state.
/// Your state should be efficiently clonable. Use Rc as necessary.
///
/// `initial` is the initial value for your state.
///
/// `f` callback which is passed a `State<S>`
pub fn state<
    S: Clone + 'static,
    V: View + 'static,
    D: Fn() -> S + 'static,
    F: Fn(State<S>) -> V + 'static,
>(
    initial: D,
    f: F,
) -> impl View + 'static {
    StateView {
        default: initial,
        func: f,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let _ = state(|| 0, |_s| EmptyView {});
    }

    #[test]
    fn test_state_clone() {
        let s = State::new(0, None);
        let s2 = s.clone();
        s.set(42);
        assert_eq!(s2.get(), 42);
    }
}
