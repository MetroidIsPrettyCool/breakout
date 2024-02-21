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

    let mut logic_state = LogicState::new();
    let mut view_state = ViewState::new(&event_loop);
    let mut control_state = ControlState::new();

    let init_time = Instant::now();
    let mut last_frame_was: Option<Instant> = None;

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let time_elapsed = Instant::now().duration_since(init_time).as_secs_f32();
                println!("frame_count: {}", view_state.frame_count());
                println!("time_elapsed: {} secs", time_elapsed);
                println!("frames per second: {}", view_state.calculate_fps());

                window_target.exit();
            }
            Event::AboutToWait => {
                // timey-wimey
                let now = Instant::now();
                let delta_t = match last_frame_was {
                    Some(then) => now.duration_since(then),
                    None => Duration::ZERO,
                };

                logic_state.update(&control_state, now, delta_t);

                view_state.update(&logic_state);

                // more timey-wimey
                last_frame_was = Some(now);
            }
            _ => control_state.update(&view_state, event),
        })
        .expect("unable to run event loop, exiting");
}
