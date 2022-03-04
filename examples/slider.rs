use rui::*;

#[derive(Clone)]
struct MyState {
    value: f32,
}

fn main() {
    rui(state(MyState { value: 0.0 }, |state: State<MyState>| {
        vstack((
            text(&format!("value: {:?}", state.get().value)).padding(Auto),
            slider(bind!(state, value)).padding(Auto),
        ))
    }));
}
