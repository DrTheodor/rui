pub use crate::*;

pub struct Button {
    text: String,
    func: Box<dyn Fn()>,
}

impl Button {
    pub const DEFAULT_SIZE: u32 = 18;
    pub const PADDING: f32 = 4.0;
}

impl View for Button {
    fn print(&self, _id: ViewID, _cx: &mut Context) {
        println!("Button({:?})", self.text);
    }
    fn process(&self, event: &Event, vid: ViewID, cx: &mut Context) {
        match &event.kind {
            EventKind::PressButton(name) => {
                if *name == self.text {
                    (*self.func)();
                }
            }
            EventKind::TouchBegin { id } => {
                if cx
                    .layout
                    .entry(vid)
                    .or_default()
                    .rect
                    .contains(event.position)
                {
                    (*self.func)();
                }
            }
            _ => (),
        }
    }

    fn draw(&self, _id: ViewID, _cx: &mut Context, vger: &mut VGER) {
        let bounds = vger.text_bounds(self.text.as_str(), Button::DEFAULT_SIZE, None);
        let padding = LocalSize::new(Button::PADDING, Button::PADDING);

        let paint = vger.color_paint(Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        });
        vger.fill_rect(
            LocalPoint::zero(),
            LocalPoint::zero() + bounds.size + padding * 2.0,
            4.0,
            paint,
        );

        vger.save();
        vger.translate(
            -LocalOffset::new(bounds.origin.x, bounds.origin.y)
                + LocalOffset::new(Button::PADDING, Button::PADDING),
        );

        vger.text(
            self.text.as_str(),
            Button::DEFAULT_SIZE,
            Color::MAGENTA,
            None,
        );

        vger.restore();
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        let padding = LocalSize::new(Button::PADDING, Button::PADDING);
        let size = vger
            .text_bounds(self.text.as_str(), Button::DEFAULT_SIZE, None)
            .size
            + padding * 2.0;

        cx.layout.insert(
            id,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), size),
                offset: LocalOffset::zero(),
            },
        );
        size
    }
}

pub fn button<F: Fn() + 'static>(name: &str, f: F) -> Button {
    Button {
        text: String::from(name),
        func: Box::new(f),
    }
}
