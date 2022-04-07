use rui::*;

fn button_example() -> impl View {
    hstack((
        caption("button"),
        button(text("press me"), || println!("pressed")),
    ))
}

fn slider_example() -> impl View {
    hstack((caption("slider"), state(|| 0.5, |s| hslider(s))))
}

fn caption(s: &str) -> impl View {
    text(s).font_size(12).padding(Auto)
}

fn knob_example() -> impl View {
    hstack((
        caption("knob"),
        state(|| 0.5, |s| knob(s).size([30.0, 30.0]).padding(Auto)),
    ))
}

fn toggle_example() -> impl View {
    hstack((
        caption("toggle"),
        state(|| false, |b| toggle(b).size([30.0, 30.0]).padding(Auto)),
    ))
}

fn text_editor_example() -> impl View {
    hstack((
        caption("text_editor"),
        state(|| "edit me".to_string(), |txt| text_editor(txt).padding(Auto)),
    ))
}

fn main() {
    rui(vstack((
        text("rui widget gallery"),
        button_example(),
        slider_example(),
        knob_example(),
        toggle_example(),
        text_editor_example(),
    ))
    .padding(Auto)
    .window_title("rui widget gallery"))
}
