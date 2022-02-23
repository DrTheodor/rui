use crate::*;

pub struct Canvas {
    func: Box<dyn Fn(LocalRect, &mut VGER)>,
}

impl View for Canvas {
    fn print(&self, _id: ViewID, _cx: &mut Context) {
        println!("canvas");
    }

    fn process(&self, _event: &Event, _id: ViewID, _cx: &mut Context, _vger: &mut VGER) {
        // do nothing
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        let rect = cx.layout.entry(id).or_insert(LayoutBox::default()).rect;

        vger.save();
        (*self.func)(rect, vger);
        vger.restore();
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, _vger: &mut VGER) -> LocalSize {
        cx.layout.insert(
            id,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), sz),
                offset: LocalOffset::zero(),
            },
        );
        sz
    }

    fn hittest(
        &self,
        _id: ViewID,
        _pt: LocalPoint,
        _cx: &mut Context,
        _vger: &mut VGER,
    ) -> Option<ViewID> {
        None
    }
}

pub fn canvas<F: Fn(LocalRect, &mut VGER) + 'static>(f: F) -> Canvas {
    Canvas {
        func: Box::new(f)
    }
}