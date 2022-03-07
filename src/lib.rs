// #![feature(type_alias_impl_trait)]

mod view;
pub use view::*;

mod state;
pub use state::*;

mod text;
pub use text::*;

mod button;
pub use button::*;

mod stack;
pub use stack::*;

mod context;
pub use context::*;

mod padding;
pub use padding::*;

mod shapes;
pub use shapes::*;

mod paint;
pub use paint::*;

mod gestures;
pub use gestures::*;

mod background;
pub use background::*;

mod modifiers;
pub use modifiers::*;

mod geom;
pub use geom::*;

mod offset;
pub use offset::*;

mod canvas;
pub use canvas::*;

mod slider;
pub use slider::*;

mod list;
pub use list::*;

mod size;
pub use size::*;

mod body;
pub use body::*;

mod knob;
pub use knob::*;

mod command;
pub use command::*;

use futures::executor::block_on;
use vger::color::*;
use vger::*;

use tao::{
    event,
    event::{ElementState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
    dpi::PhysicalSize,
    menu::{MenuBar as Menu, MenuItem, MenuItemAttributes, MenuType},
};

struct Setup {
    window: Window,
    event_loop: EventLoop<()>,
    // instance: wgpu::Instance,
    size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub const TEXT_COLOR: Color = Color {
    r: 0.839,
    g: 0.839,
    b: 0.839,
    a: 1.0,
};
pub const RED_HIGHLIGHT: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.384,
    a: 1.0,
};
pub const RED_HIGHLIGHT_DARK: Color = Color {
    r: 0.369,
    g: 0.145,
    b: 0.227,
    a: 1.0,
};
pub const RED_HIGHLIGHT_BACKGROUND: Color = Color {
    r: 0.110,
    g: 0.0,
    b: 0.043,
    a: 1.0,
};
pub const AZURE_HIGHLIGHT: Color = Color {
    r: 0.0,
    g: 0.831,
    b: 1.0,
    a: 1.0,
};
pub const GREEN_HIGHLIGHT: Color = Color {
    r: 0.231,
    g: 0.769,
    b: 0.333,
    a: 1.0,
};

pub const BUTTON_BACKGROUND_COLOR: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

pub const CLEAR_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub const CONTROL_BACKGROUND: Color = Color {
    r: 0.138,
    g: 0.138,
    b: 0.148,
    a: 1.0,
};

async fn setup(title: &str) -> Setup {
    let event_loop = EventLoop::new();
    let mut builder = WindowBuilder::new();
    builder = builder.with_title(title);
    let window = builder.build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        let query_string = web_sys::window().unwrap().location().search().unwrap();
        let level: log::Level = parse_url_query_string(&query_string, "RUST_LOG")
            .map(|x| x.parse().ok())
            .flatten()
            .unwrap_or(log::Level::Error);
        console_log::init_with_level(level).expect("could not initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    // log::info!("Initializing the surface...");

    let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);

    let instance = wgpu::Instance::new(backend);
    let (size, surface) = unsafe {
        let size = window.inner_size();
        let surface = instance.create_surface(&window);
        (size, surface)
    };
    let adapter =
        wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface))
            .await
            .expect("No suitable GPU adapters found on the system!");

    #[cfg(not(target_arch = "wasm32"))]
    {
        let adapter_info = adapter.get_info();
        println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);
    }

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
            },
            trace_dir.ok().as_ref().map(std::path::Path::new),
        )
        .await
        .expect("Unable to find a suitable GPU adapter!");

    Setup {
        window,
        event_loop,
        // instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }
}

/// Call this function to describe your UI.
pub fn rui(view: impl View + 'static) {
    let setup = block_on(setup("rui"));
    let window = setup.window;
    let surface = setup.surface;
    let device = setup.device;
    let size = setup.size;
    let adapter = setup.adapter;
    let queue = setup.queue;

    // create main menubar menu
    let mut menu_bar_menu = Menu::new();
    let mut first_menu = Menu::new();
    first_menu.add_native_item(MenuItem::Quit);
    menu_bar_menu.add_submenu("My app", true, first_menu);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    surface.configure(&device, &config);

    let mut vger = VGER::new(&device, wgpu::TextureFormat::Bgra8UnormSrgb);
    let mut cx = Context::new();
    let mut mouse_position = LocalPoint::zero();

    setup.event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        // *control_flow = ControlFlow::Poll;

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;

        match event {
            event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            event::Event::WindowEvent {
                event:
                    WindowEvent::Resized(size)
                    | WindowEvent::ScaleFactorChanged {
                        new_inner_size: &mut size,
                        ..
                    },
                ..
            } => {
                println!("Resizing to {:?}", size);
                config.width = size.width.max(1);
                config.height = size.height.max(1);
                surface.configure(&device, &config);
                window.request_redraw();
            }
            event::Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                if view.needs_redraw(cx.root_id, &mut cx) {
                    window.request_redraw();
                }
            }
            event::Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // println!("RedrawRequested");

                let frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(_) => {
                        surface.configure(&device, &config);
                        surface
                            .get_current_texture()
                            .expect("Failed to acquire next surface texture!")
                    }
                };

                let window_size = window.inner_size();
                let scale = window.scale_factor() as f32;
                // println!("window_size: {:?}", window_size);
                let width = window_size.width as f32 / scale;
                let height = window_size.height as f32 / scale;

                vger.begin(width, height, scale);

                view.layout(cx.root_id, [width, height].into(), &mut cx, &mut vger);
                view.draw(cx.root_id, &mut cx, &mut vger);

                let texture_view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let desc = wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                };

                vger.encode(&device, &desc, &queue);

                frame.present();
            }
            event::Event::WindowEvent {
                event: WindowEvent::MouseInput { state, .. },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        let event = view::Event {
                            kind: EventKind::TouchBegin { id: 0 },
                            position: mouse_position,
                        };
                        view.process(&event, cx.root_id, &mut cx, &mut vger)
                    },
                    ElementState::Released => { 
                        let event = view::Event {
                            kind: EventKind::TouchEnd { id: 0 },
                            position: mouse_position,
                        };
                        view.process(&event, cx.root_id, &mut cx, &mut vger)
                    },
                    _ => {}
                };
            }
            event::Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let scale = window.scale_factor() as f32;
                mouse_position = [
                    position.x as f32 / scale,
                    (config.height as f32 - position.y as f32) / scale,
                ]
                .into();
                let event = view::Event {
                    kind: EventKind::TouchMove { id: 0 },
                    position: mouse_position,
                };
                view.process(&event, cx.root_id, &mut cx, &mut vger)
            }
            _ => (),
        }
    });
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_state_clone() {
        let s = State::new(0);
        let s2 = s.clone();
        s.set(42);
        assert_eq!(s2.get(), 42);
    }

    #[test]
    fn test_button() {
        let _ = button(text("click me"), || {
            println!("clicked!");
        });
    }

    #[test]
    fn test_state() {
        let _ = state(0, |_s: State<usize>| EmptyView {});
    }

    fn counter0(start: usize) -> impl View {
        state(start, |count: State<usize>| {
            button(text(&format!("{:?}", count.get())), move || {
                let value = count.get();
                count.set(value + 1);
            })
        })
    }

    #[test]
    fn test_state2() {
        let mut cx = Context::new();
        let v = counter(42);
        v.print(ViewID::default(), &mut cx);
    }

    fn counter(start: usize) -> impl View {
        state(start, |count: State<usize>| {
            let count2 = count.clone();
            let value_string = format!("value: {:?}", count.get());
            vstack((
                text(value_string.as_str()),
                button(text("increment"), move || {
                    let value = count.get();
                    count.set(value + 1);
                }),
                button(text("decrement"), move || {
                    let value = count2.get();
                    count2.set(value - 1);
                }),
            ))
        })
    }

    /*
    #[test]
    fn test_state3() {
        let mut cx = Context::new();
        let v = counter(42);
        println!("\"drawing\" the UI");
        v.print(ViewID::default(), &mut cx);
        println!("ok, now pressing increment button");
        v.process(
            &Event {
                kind: EventKind::PressButton(String::from("increment")),
                position: [0.0, 0.0].into(),
            },
            ViewID::default(),
            &mut cx,
        );
        println!("\"drawing\" the UI again");
        v.print(ViewID::default(), &mut cx);
    }

    */

    fn counter3<B>(count: B) -> impl View
    where
        B: Binding<usize> + Clone + 'static,
    {
        let count2 = count.clone();
        vstack((
            button(text("increment"), move || {
                let value = count.get();
                count.set(value + 1);
            }),
            button(text("decrement"), move || {
                let value = count2.get();
                count2.set(value - 1);
            }),
        ))
    }

    #[test]
    fn test_binding() {
        let _ = state(42, |count: State<usize>| counter3(count));
    }

    fn ok_button<F: Fn() + 'static>(f: F) -> impl View {
        button(text("ok"), f)
    }

    #[derive(Clone)]
    struct BindingTestData {
        x: usize,
    }

    #[test]
    fn test_bind() {
        let s = State::new(BindingTestData { x: 0 });
        let b = bind!(s, x);
        b.set(42);
        assert_eq!(s.get().x, 42);
    }
}
