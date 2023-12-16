use raw_window_handle::HasDisplayHandle;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_blit::{NativeFormat, PixelBufferTyped};

fn main() {
    let event_loop = EventLoop::new().expect("failed to build new event loop");

    let window = WindowBuilder::new()
        .with_title("Software rendering example")
        .build(&event_loop)
        .expect("failed to build window");
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
                ..
            } => {
                if window_id == window.id() {
                    let (width, height): (u32, u32) = window.inner_size().into();
                    let mut buffer = PixelBufferTyped::<NativeFormat>::new_supported(
                        width,
                        height,
                        &window,
                        &window
                            .display_handle()
                            .expect("couldn't get display for window"),
                    );

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
            _ => (),
        })
        .expect("main event loop failed");
}
