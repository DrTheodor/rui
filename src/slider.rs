use crate::*;

pub fn slider(value: impl Binding<f32> + 'static) -> impl View {
    let x = value.get();
    state(0.0, move |width| {
        let value2 = value.clone();
        circle().offset([x, 0.0].into()).drag(move |off, _state| {
            value2.set(value2.get() + off.x);
        })
    })
}
