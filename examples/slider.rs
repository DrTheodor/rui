use rui::*;

#[derive(Clone)]
struct MyState {
    value: f32,
}

fn main() {
    rui(state(MyState { value: 0.0 }, |state| {
        vstack((
            text(&format!("value: {:?}", state.get().value)).padding(Auto),
            hslider(bind!(state, value))
                .thumb_color(RED_HIGHLIGHT)
                .padding(Auto),
        ))
    }));
}
