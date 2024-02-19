use glium::backend::glutin::SimpleWindowBuilder;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use breakout::state::{GameState, WindowState};

fn main() {
    // create event loop
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("unable to create window, exiting");
    event_loop.set_control_flow(ControlFlow::Poll);

    // set up window
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let mut game_state = GameState::new();

    let mut window_state = WindowState::new(window, display);

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let time_elapsed = Instant::now()
                    .duration_since(window_state.then)
                    .as_secs_f32();
                println!("frame_count: {}", window_state.frame_count);
                println!("time_elapsed: {} secs", time_elapsed);
                println!(
                    "frames per second: {}",
                    window_state.frame_count as f32 / time_elapsed
                );

                window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position: p, .. },
                ..
            } => {
                let PhysicalSize {
                    width: window_width,
                    height: window_height,
                } = window_state.window.inner_size();

                window_state.mouse_x_relative =
                    (p.x as f32 / (window_state.window.inner_size().width / 2) as f32) - 1.0;
                window_state.mouse_y_relative =
                    (p.y as f32 / (window_state.window.inner_size().height / 2) as f32) - 1.0;

                // correct for aspect ratio
                if window_width > window_height {
                    window_state.mouse_x_relative *= window_width as f32 / window_height as f32;
                } else {
                    window_state.mouse_y_relative *= window_height as f32 / window_width as f32;
                }
            }
            Event::AboutToWait => {
                breakout::run(window_target, &mut window_state, &mut game_state);
            }
            _ => (),
        })
        .expect("unable to run event loop, exiting");
}
