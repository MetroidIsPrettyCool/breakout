use glium::backend::glutin::SimpleWindowBuilder;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use breakout::logic::GameState;
use breakout::view::WindowState;

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
                breakout::on_close_requested(window_target, &mut window_state, &mut game_state);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position: p, .. },
                ..
            } => {
                breakout::on_cursor_moved(&mut window_state, p);
            }
            Event::AboutToWait => {
                breakout::run(&mut window_state, &mut game_state);
            }
            _ => (),
        })
        .expect("unable to run event loop, exiting");
}
