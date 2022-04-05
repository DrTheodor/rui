use crate::*;

/// Struct for `text`.
pub struct Text {
    text: String,
    size: u32,
}

impl Text {
    pub const DEFAULT_SIZE: u32 = 18;
}

impl View for Text {
    fn print(&self, _id: ViewID, _cx: &mut Context) {
        println!("Text({:?})", self.text);
    }
    fn process(&self, _event: &Event, _id: ViewID, _cx: &mut Context, _vger: &mut VGER) {}
    fn draw(&self, _id: ViewID, _cx: &mut Context, vger: &mut VGER) {
        let origin = vger.text_bounds(self.text.as_str(), self.size, None).origin;

        vger.save();
        vger.translate([-origin.x, -origin.y]);
        vger.text(self.text.as_str(), self.size, TEXT_COLOR, None);
        vger.restore();
    }
    fn layout(&self, id: ViewID, _sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        let size = vger.text_bounds(self.text.as_str(), self.size, None).size;

        cx.layout.insert(
            id,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), size),
                offset: LocalOffset::zero(),
            },
        );
        size
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

    fn commands(&self, _id: ViewID, _cx: &mut Context, _cmds: &mut Vec<CommandInfo>) {}

    fn gc(&self, _id: ViewID, _cx: &mut Context, _map: &mut StateMap) {
        // do nothing
    }

    fn access(&self, id: ViewID, _cx: &mut Context, nodes: &mut Vec<accesskit::Node>) -> Option<accesskit::NodeId> {
        let aid = id.access_id();
        nodes.push(
            accesskit::Node::new(aid, accesskit::Role::LabelText)
        );
        Some(aid)
    }
}

impl Text {
    pub fn font_size(self, size: u32) -> Self {
        Self {
            text: self.text,
            size,
        }
    }
}

/// Shows a string as a label (not editable).
pub fn text(name: &str) -> Text {
    Text {
        text: String::from(name),
        size: Text::DEFAULT_SIZE,
    }
}
