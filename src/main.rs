use glium::backend::glutin::SimpleWindowBuilder;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use glium::Surface;

use std::time::{Duration, Instant};

fn main() {
    // create event loop
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("unable to create window, exiting");
    event_loop.set_control_flow(ControlFlow::Poll);

    // set up window
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    // TMP
    let mut frame_count: u32 = 0;
    let then = Instant::now();

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let time_elapsed = Instant::now().duration_since(then).as_secs_f32();
                println!("frame_count: {}", frame_count);
                println!("time_elapsed: {} secs", time_elapsed);
                println!("frames per second: {}", frame_count as f32 / time_elapsed);

                window_target.exit();
            }
            Event::AboutToWait => {
                // draw a frame
                let mut frame = display.draw();

                let colors = frame_count.to_ne_bytes();
                frame.clear(
                    None,
                    Some((
                        colors[0] as f32 / 255_f32,
                        colors[1] as f32 / 255_f32,
                        colors[2] as f32 / 255_f32,
                        1.0,
                    )),
                    false,
                    None,
                    None,
                );

                frame.finish().expect("unable to finish frame, exiting");

                frame_count += 1;
            }
            _ => (),
        })
        .expect("unable to run event loop, exiting");
}
