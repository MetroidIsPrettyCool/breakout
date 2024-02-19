#![feature(const_fn_floating_point_arithmetic)]

use std::time::{Duration, Instant};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoopWindowTarget,
};

pub mod logic;
use logic::GameState;

pub mod game_objs;

pub mod view;
use view::WindowState;

pub fn on_close_requested(
    window_target: &EventLoopWindowTarget<()>,
    window_state: &mut WindowState,
    _game_state: &mut GameState,
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

pub fn on_cursor_moved(window_state: &mut WindowState, p: PhysicalPosition<f64>) {
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

pub fn run(window_state: &mut WindowState, game_state: &mut GameState) {
    // timey-wimey
    let now = Instant::now();
    let delta_t = match window_state.last_frame_was {
        Some(then) => now.duration_since(then),
        None => Duration::ZERO,
    };

    logic::tick(window_state, game_state, delta_t);

    view::render_frame(window_state, game_state);

    // state management
    window_state.frame_count += 1;
    window_state.last_frame_was = Some(now);
}
