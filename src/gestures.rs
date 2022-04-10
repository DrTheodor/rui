use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Struct for the `tap` gesture.
pub struct Tap<V, F> {
    child: V,
    func: F,
}

impl<V, F> Tap<V, F>
where
    V: View,
    F: Fn() + 'static,
{
    pub fn new(v: V, f: F) -> Self {
        Self { child: v, func: f }
    }
}

impl<V, F> View for Tap<V, F>
where
    V: View,
    F: Fn() + 'static,
{
    fn print(&self, id: ViewID, cx: &mut Context) {
        println!("Tap {{");
        (self.child).print(id.child(&0), cx);
        println!("}}");
    }

    fn process(&self, event: &Event, vid: ViewID, cx: &mut Context, vger: &mut VGER) {
        match &event.kind {
            EventKind::TouchBegin { id } => {
                if self.hittest(vid, event.position, cx, vger).is_some() {
                    cx.touches[*id] = vid;
                }
            }
            EventKind::TouchEnd { id } => {
                if cx.touches[*id] == vid {
                    cx.touches[*id] = ViewID::default();
                    (self.func)();
                }
            }
            _ => (),
        }
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        self.child.draw(id.child(&0), cx, vger)
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        self.child.layout(id.child(&0), sz, cx, vger)
    }

    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID> {
        self.child.hittest(id.child(&0), pt, cx, vger)
    }

    fn commands(&self, id: ViewID, cx: &mut Context, cmds: &mut Vec<CommandInfo>) {
        self.child.commands(id.child(&0), cx, cmds)
    }

    fn gc(&self, id: ViewID, cx: &mut Context, map: &mut Vec<ViewID>) {
        self.child.gc(id.child(&0), cx, map)
    }

    fn access(
        &self,
        id: ViewID,
        cx: &mut Context,
        nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        self.child.access(id.child(&0), cx, nodes)
    }
}

impl<V, F> private::Sealed for Tap<V, F> {}

pub enum GestureState {
    Began,
    Changed,
    Ended,
}

/// Struct for the `drag` gesture.
pub struct Drag<V, F> {
    child: V,
    func: F,
}

impl<V, F> Drag<V, F>
where
    V: View,
    F: Fn(LocalOffset, GestureState) + 'static,
{
    pub fn new(v: V, f: F) -> Self {
        Self { child: v, func: f }
    }
}

impl<V, F> View for Drag<V, F>
where
    V: View,
    F: Fn(LocalOffset, GestureState) + 'static,
{
    fn print(&self, id: ViewID, cx: &mut Context) {
        println!("Drag {{");
        (self.child).print(id.child(&0), cx);
        println!("}}");
    }

    fn process(&self, event: &Event, vid: ViewID, cx: &mut Context, vger: &mut VGER) {
        match &event.kind {
            EventKind::TouchBegin { id } => {
                if self.hittest(vid, event.position, cx, vger).is_some() {
                    cx.touches[*id] = vid;
                    cx.starts[*id] = event.position;
                    cx.previous_position[*id] = event.position;
                }
            }
            EventKind::TouchMove { id } => {
                if cx.touches[*id] == vid {
                    let delta = event.position - cx.previous_position[*id];
                    (self.func)(delta, GestureState::Changed);
                    cx.previous_position[*id] = event.position;
                }
            }
            EventKind::TouchEnd { id } => {
                if cx.touches[*id] == vid {
                    cx.touches[*id] = ViewID::default();
                    (self.func)(
                        event.position - cx.previous_position[*id],
                        GestureState::Ended,
                    );
                }
            }
            _ => (),
        }
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        self.child.draw(id.child(&0), cx, vger)
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        self.child.layout(id.child(&0), sz, cx, vger)
    }

    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID> {
        self.child.hittest(id.child(&0), pt, cx, vger)
    }

    fn commands(&self, id: ViewID, cx: &mut Context, cmds: &mut Vec<CommandInfo>) {
        self.child.commands(id.child(&0), cx, cmds)
    }

    fn gc(&self, id: ViewID, cx: &mut Context, map: &mut Vec<ViewID>) {
        self.child.gc(id.child(&0), cx, map)
    }

    fn access(
        &self,
        id: ViewID,
        cx: &mut Context,
        nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        self.child.access(id.child(&0), cx, nodes)
    }
}

impl<V, F> private::Sealed for Drag<V, F> {}

/// Struct for the `tap` gesture.
pub struct Tap2<V, F> {
    child: V,
    func: Rc<RefCell<F>>,
}

impl<'a, V, F> Tap2<V, F>
where
    V: View,
    F: FnMut() + 'a,
{
    pub fn new(v: V, f: F) -> Self {
        Self {
            child: v,
            func: Rc::new(RefCell::new(f)),
        }
    }
}

impl<'a, V, F> View for Tap2<V, F>
where
    V: View,
    F: FnMut() + 'a,
{
    fn print(&self, id: ViewID, cx: &mut Context) {
        println!("Tap {{");
        (self.child).print(id.child(&0), cx);
        println!("}}");
    }

    fn process(&self, event: &Event, vid: ViewID, cx: &mut Context, vger: &mut VGER) {
        match &event.kind {
            EventKind::TouchBegin { id } => {
                if self.hittest(vid, event.position, cx, vger).is_some() {
                    cx.touches[*id] = vid;
                }
            }
            EventKind::TouchEnd { id } => {
                if cx.touches[*id] == vid {
                    cx.touches[*id] = ViewID::default();
                    self.func.borrow_mut()();
                }
            }
            _ => (),
        }
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        self.child.draw(id.child(&0), cx, vger)
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        self.child.layout(id.child(&0), sz, cx, vger)
    }

    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID> {
        self.child.hittest(id.child(&0), pt, cx, vger)
    }

    fn commands(&self, id: ViewID, cx: &mut Context, cmds: &mut Vec<CommandInfo>) {
        self.child.commands(id.child(&0), cx, cmds)
    }

    fn gc(&self, id: ViewID, cx: &mut Context, map: &mut Vec<ViewID>) {
        self.child.gc(id.child(&0), cx, map)
    }

    fn access(
        &self,
        id: ViewID,
        cx: &mut Context,
        nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        self.child.access(id.child(&0), cx, nodes)
    }
}

impl<V, F> private::Sealed for Tap2<V, F> {}
