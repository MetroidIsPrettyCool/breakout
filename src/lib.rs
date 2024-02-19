use glium::backend::glutin::Display;
use glium::index::IndicesSource;
use glium::{implement_vertex, uniform, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glutin::surface::WindowSurface;
use std::error::Error;
use std::time::{Duration, Instant};
use winit::window::Window;
use winit::event_loop::EventLoopWindowTarget;

pub mod state;
use state::{GameState, WindowState};

pub mod game_objs;
use game_objs::{Paddle, Ball, Playfield, Brick};

fn draw(
    vertices: &Vec<Vertex>,
    frame: &mut Frame,
    window_state: &WindowState,
) -> Result<(), Box<dyn Error>> {
    let uniforms = uniform! {
        window_aspect: [window_state.window.inner_size().width as f32, window_state.window.inner_size().height as f32],
    };
    frame.draw(
        &VertexBuffer::new(&window_state.display, vertices)
            .expect("unable to construct vbo, exiting"),
        IndicesSource::NoIndices {
            primitives: glium::index::PrimitiveType::TrianglesList,
        },
        &window_state.program,
        &uniforms,
        &DrawParameters::default(),
    )?;
    Ok(())
}

/// Things that can be drawn to the screen
pub trait Drawable {
    fn get_vertices(&self) -> Vec<Vertex>;
}

/// Flat-Shaded Vertex
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

pub fn run(
    window_target: &EventLoopWindowTarget<()>,
    window_state: &mut WindowState,
    game_state: &mut GameState,
) {
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

    draw(&vertices, &mut frame, &window_state).expect("unable to complete draw call, exiting");

    // wrap up
    frame.finish().expect("unable to finish frame, exiting");

    window_state.frame_count += 1;
    window_state.last_frame_was = Some(now);
}
