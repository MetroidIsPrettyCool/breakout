#![feature(const_fn_floating_point_arithmetic)]

use std::time::{Duration, Instant};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoopWindowTarget,
};

pub mod logic;
use logic::LogicState;

pub mod view;
use view::ViewState;

pub fn on_close_requested(
    window_target: &EventLoopWindowTarget<()>,
    window_state: &mut ViewState,
    _game_state: &mut LogicState,
) {
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

pub fn on_cursor_moved(window_state: &mut ViewState, p: PhysicalPosition<f64>) {
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

pub fn run(view_state: &mut ViewState, logic_state: &mut LogicState) {
    // timey-wimey
    let now = Instant::now();
    let delta_t = match view_state.last_frame_was {
        Some(then) => now.duration_since(then),
        None => Duration::ZERO,
    };

    logic_state.tick(view_state, delta_t);

    view_state.render_frame(logic_state);

    // state management
    view_state.frame_count += 1;
    view_state.last_frame_was = Some(now);
}
