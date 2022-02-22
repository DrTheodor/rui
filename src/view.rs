use crate::*;

#[derive(Clone, Debug)]
pub enum EventKind {
    PressButton(String),
    TouchBegin { id: usize },
    TouchEnd { id: usize },
}

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: EventKind,
    pub position: LocalPoint,
}

pub trait View {
    fn print(&self, id: ViewID, cx: &mut Context);
    fn process(&self, event: &Event, id: ViewID, cx: &mut Context, vger: &mut VGER);
    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER);
    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize;
    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID>;
}

pub struct EmptyView {}

impl View for EmptyView {
    fn print(&self, _id: ViewID, _cx: &mut Context) {
        println!("EmptyView");
    }
    fn process(&self, _event: &Event, _id: ViewID, _cx: &mut Context, vger: &mut VGER) {}
    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {}
    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        [0.0, 0.0].into()
    }
    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID> {
        None
    }
}

pub trait Modifiers: View + Sized {
    fn padding(self, param: impl Into<PaddingParam>) -> Padding<Self>;
    fn tap<F: Fn() + 'static>(self, f: F) -> Tap<Self>;
}

impl <V: View + 'static> Modifiers for V {
    fn padding(self, param: impl Into<PaddingParam>) -> Padding<Self> {
        Padding::new(self, param.into())
    }
    fn tap<F: Fn() + 'static>(self, f: F) -> Tap<Self> {
        Tap::new(self, f)
    }
}