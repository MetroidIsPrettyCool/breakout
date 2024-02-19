use std::time::{Duration, Instant};

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use breakout::control::ControlState;
use breakout::logic::LogicState;
use breakout::view::ViewState;

fn main() {
    // create event loop
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("unable to create window, exiting");
    event_loop.set_control_flow(ControlFlow::Poll);

    // set up window
    let mut logic_state = LogicState::new();

    let mut view_state = ViewState::new(&event_loop);

    let mut control_state = ControlState::new();

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let time_elapsed = Instant::now().duration_since(logic_state.then).as_secs_f32();
                println!("frame_count: {}", view_state.frame_count);
                println!("time_elapsed: {} secs", time_elapsed);
                println!(
                    "frames per second: {}",
                    view_state.frame_count as f32 / time_elapsed
                );

                window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position: p, .. },
                ..
            } => {
                control_state.update_cursor_position(&view_state, p);
            }
            Event::AboutToWait => {
                // timey-wimey
                let now = Instant::now();
                let delta_t = match view_state.last_frame_was {
                    Some(then) => now.duration_since(then),
                    None => Duration::ZERO,
                };

                logic_state.update(&control_state, now, delta_t);

                view_state.update(&logic_state, now);
            }
            _ => (),
        })
        .expect("unable to run event loop, exiting");
}
