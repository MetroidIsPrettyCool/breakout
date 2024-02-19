#![feature(const_fn_floating_point_arithmetic)]

use glium::Surface;
use std::time::{Duration, Instant};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoopWindowTarget,
};

pub mod logic;
use logic::GameState;

pub mod game_objs;
use game_objs::{Ball, Brick, Paddle, Playfield};

pub mod view;
use view::{Drawable, WindowState};

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

    // move paddle to mouse
    game_state.paddle.x = (window_state.mouse_x_relative)
        .clamp(-1.0 + (Paddle::WIDTH / 2.0), 1.0 - (Paddle::WIDTH / 2.0));

    // move ball
    game_state.ball.x += game_state.ball.x_v * delta_t.as_secs_f32();
    game_state.ball.y += game_state.ball.y_v * delta_t.as_secs_f32();

    // check ball collisions with...

    // ...playfield
    if game_state.ball.x + Ball::WIDTH / 2.0 > 1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x + Ball::WIDTH / 2.0 - 1.0;
    }
    if game_state.ball.x - Ball::WIDTH / 2.0 < -1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x - Ball::WIDTH / 2.0 + 1.0;
    }
    if game_state.ball.y + Ball::HEIGHT / 2.0 > 1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y + Ball::HEIGHT / 2.0 - 1.0;
    }
    if game_state.ball.y - Ball::HEIGHT / 2.0 < -1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y - Ball::HEIGHT / 2.0 + 1.0;
    }

    // ...paddle
    if game_state.ball.x - (Ball::WIDTH / 2.0) < game_state.paddle.x + (Paddle::WIDTH / 2.0)
        && game_state.ball.x + (Ball::WIDTH / 2.0) > game_state.paddle.x - (Paddle::WIDTH / 2.0)
        && game_state.ball.y - (Ball::HEIGHT / 2.0)
            < Paddle::VERTICAL_OFFSET + (Paddle::HEIGHT / 2.0)
        && game_state.ball.y + (Ball::HEIGHT / 2.0)
            > Paddle::VERTICAL_OFFSET - (Paddle::HEIGHT / 2.0)
    {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y - Ball::HEIGHT / 2.0 - Paddle::VERTICAL_OFFSET;
    }

    // draw a frame
    let mut frame = window_state.display.draw();

    frame.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, None, None);

    let mut vertices = game_state.playfield.get_vertices();
    vertices.extend(game_state.ball.get_vertices());
    vertices.extend(game_state.paddle.get_vertices());
    for column in game_state.bricks.iter() {
        for brick in column {
            vertices.extend(brick.get_vertices());
        }
    }

    view::draw(&vertices, &mut frame, &window_state)
        .expect("unable to complete draw call, exiting");

    // wrap up
    frame.finish().expect("unable to finish frame, exiting");

    window_state.frame_count += 1;
    window_state.last_frame_was = Some(now);
}
