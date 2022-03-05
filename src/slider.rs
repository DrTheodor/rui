use crate::*;

const SLIDER_WIDTH: f32 = 4.0;
const SLIDER_THUMB_RADIUS: f32 = 10.0;

/// Horizontal slider built from other Views.
fn hslider_f(value: impl Binding<f32>, thumb_color: Color) -> impl View {
    state(0.0, move |width| {
        let w = width.get();
        let x = value.get() * w;
        let value = value.clone();

        zstack((
            rectangle().color(CLEAR_COLOR).drag(move |off, _state| {
                value.set((value.get() + off.x / w).clamp(0.0, 1.0));
            }),
            canvas(move |sz, vger| {
                let c = sz.center();
                let paint = vger.color_paint(BUTTON_BACKGROUND_COLOR);
                vger.fill_rect(
                    [0.0, c.y - SLIDER_WIDTH / 2.0].into(),
                    [sz.width(), c.y + SLIDER_WIDTH / 2.0].into(),
                    0.0,
                    paint,
                );
                let paint = vger.color_paint(thumb_color);
                vger.fill_circle([x, c.y], SLIDER_THUMB_RADIUS, paint);
            }),
        ))
        .geom(move |sz| {
            if sz.width != w {
                width.set(sz.width)
            }
        })
    })
}

pub struct HSlider<B: Binding<f32>> {
    value: B,
    thumb: Color,
}

impl<B> View for HSlider<B>
where
    B: Binding<f32>,
{
    body_view!();
}

impl<B> HSlider<B>
where
    B: Binding<f32>,
{
    fn body(&self) -> impl View {
        hslider_f(self.value.clone(), self.thumb)
    }

    pub fn thumb_color(self, thumb_color: Color) -> Self {
        Self {
            value: self.value,
            thumb: thumb_color,
        }
    }
}

/// Horizontal slider built from other Views.
pub fn hslider(value: impl Binding<f32>) -> HSlider<impl Binding<f32>> {
    HSlider {
        value,
        thumb: AZURE_HIGHLIGHT,
    }
}

/// Vertical slider built from other Views.
pub fn vslider(value: impl Binding<f32>) -> impl View {
    state(0.0, move |height| {
        let h = height.get();
        let y = value.get() * h;
        let value = value.clone();

        zstack((
            rectangle().color(CLEAR_COLOR).drag(move |off, _state| {
                value.set((value.get() + off.y / h).clamp(0.0, 1.0));
            }),
            canvas(move |sz, vger| {
                let c = sz.center();
                let paint = vger.color_paint(BUTTON_BACKGROUND_COLOR);
                vger.fill_rect(
                    [c.x - SLIDER_WIDTH / 2.0, 0.0].into(),
                    [c.x + SLIDER_WIDTH / 2.0, sz.height()].into(),
                    0.0,
                    paint,
                );
                let paint = vger.color_paint(AZURE_HIGHLIGHT);
                vger.fill_circle([c.x, y], SLIDER_THUMB_RADIUS, paint);
            }),
        ))
        .geom(move |sz| {
            if sz.height != h {
                height.set(sz.width)
            }
        })
    })
}
