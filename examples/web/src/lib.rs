use log::info;
use wasm_bindgen::prelude::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::web::WindowExtWebSys,
    window::WindowBuilder,
};
use winit_blit::{NativeFormat, PixelBufferTyped};

fn add_canvas_to_doc(canvas: &web_sys::Node) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    body.append_child(&canvas)
        .expect("append canvas to HTML body");
}

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Software rendering example")
        .build(&event_loop)
        .unwrap();

    add_canvas_to_doc(&window.canvas());

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        Event::RedrawRequested(window_id) => {
            if window_id == window.id() {
                let (width, height): (u32, u32) = window.inner_size().into();
                let mut buffer =
                    PixelBufferTyped::<NativeFormat>::new_supported(width, height, &window);

                for (i, row) in buffer.rows_mut().enumerate() {
                    let value = (i % 256) as u16;
                    for (j, pixel) in row.into_iter().enumerate() {
                        let value = value * (j % 256) as u16 / 256;
                        *pixel = NativeFormat::from_rgb(
                            (256 * value / 256) as u8,
                            (256 * value / 256) as u8,
                            (256 * value / 256) as u8,
                        );
                    }
                }

                buffer.blit(&window).unwrap();
            }
        }
        _ => *control_flow = ControlFlow::Wait,
    });
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Better panic handling in the browser's console.
    console_error_panic_hook::set_once();
    // Send rust logs to the browser's console.
    console_log::init_with_level(log::Level::Debug).expect("Failed to init logging");
    // Make sure it worked.
    info!("blit loaded and running from rust");
    main();
    Ok(())
}