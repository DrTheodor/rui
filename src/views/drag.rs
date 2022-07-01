use crate::*;
use std::any::Any;

#[derive(Clone, Copy, Eq, PartialEq)]
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

impl<V, F, A> Drag<V, F>
where
    V: View,
    F: Fn(&mut Context, LocalOffset, GestureState, Option<MouseButton>) -> A + 'static,
{
    pub fn new(v: V, f: F) -> Self {
        Self { child: v, func: f }
    }
}

impl<V, F, A> View for Drag<V, F>
where
    V: View,
    F: Fn(&mut Context, LocalOffset, GestureState, Option<MouseButton>) -> A + 'static,
    A: 'static,
{
    fn process(
        &self,
        event: &Event,
        vid: ViewId,
        cx: &mut Context,
        vger: &mut Vger,
        actions: &mut Vec<Box<dyn Any>>,
    ) {
        match &event {
            Event::TouchBegin { id, position } => {
                if self.hittest(vid, *position, cx, vger).is_some() {
                    cx.touches[*id] = vid;
                    cx.starts[*id] = *position;
                    cx.previous_position[*id] = *position;
                }
            }
            Event::TouchMove { id, position } => {
                if cx.touches[*id] == vid {
                    let delta = *position - cx.previous_position[*id];
                    actions.push(Box::new((self.func)(
                        cx,
                        delta,
                        GestureState::Changed,
                        cx.mouse_button,
                    )));
                    cx.previous_position[*id] = *position;
                }
            }
            Event::TouchEnd { id, position } => {
                if cx.touches[*id] == vid {
                    cx.touches[*id] = ViewId::default();
                    actions.push(Box::new((self.func)(
                        cx,
                        *position - cx.previous_position[*id],
                        GestureState::Ended,
                        cx.mouse_button,
                    )));
                }
            }
            _ => (),
        }
    }

    fn draw(&self, id: ViewId, cx: &mut Context, vger: &mut Vger) {
        self.child.draw(id.child(&0), cx, vger)
    }

    fn layout(&self, id: ViewId, sz: LocalSize, cx: &mut Context, vger: &mut Vger) -> LocalSize {
        self.child.layout(id.child(&0), sz, cx, vger)
    }

    fn dirty(&self, id: ViewId, xform: LocalToWorld, cx: &mut Context) {
        self.child.dirty(id.child(&0), xform, cx);
    }

    fn hittest(
        &self,
        id: ViewId,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut Vger,
    ) -> Option<ViewId> {
        self.child.hittest(id.child(&0), pt, cx, vger)
    }

    fn commands(&self, id: ViewId, cx: &mut Context, cmds: &mut Vec<CommandInfo>) {
        self.child.commands(id.child(&0), cx, cmds)
    }

    fn gc(&self, id: ViewId, cx: &mut Context, map: &mut Vec<ViewId>) {
        self.child.gc(id.child(&0), cx, map)
    }

    fn access(
        &self,
        id: ViewId,
        cx: &mut Context,
        nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        self.child.access(id.child(&0), cx, nodes)
    }
}

impl<V, F> private::Sealed for Drag<V, F> {}
