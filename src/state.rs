use glium::backend::glutin::Display;
use glium::Program;
use glutin::surface::WindowSurface;
use std::time::Instant;
use winit::window::Window;

use crate::game_objs::{Paddle, Ball, Playfield, Brick};

/// Information relevant to the renderer
#[derive(Debug)]
pub struct WindowState {
    pub frame_count: u64,
    pub then: Instant,

    pub last_frame_was: Option<Instant>,

    pub mouse_x_relative: f32,
    pub mouse_y_relative: f32,

    pub window_width: f32,
    pub window_height: f32,

    pub program: Program,
    pub window: Window,
    pub display: Display<WindowSurface>,
}
impl WindowState {
    pub fn new(window: Window, display: Display<WindowSurface>) -> WindowState {
        // set up shaders
        let program = glium::Program::from_source(
            &display,
            include_str!("vert.glsl"),
            include_str!("frag.glsl"),
            None,
        )
        .expect("unable to compile shaders, exiting");

        let window_size = window.inner_size();
        WindowState {
            frame_count: 0,
            then: Instant::now(),
            last_frame_was: None,

            mouse_x_relative: 0.0,
            mouse_y_relative: 0.0,

            window_width: (window_size.width as f32),
            window_height: (window_size.height as f32),

            program,
            window,
            display,
        }
    }
}

/// Game state
// #[derive(Copy, Clone, Debug, PartialEq)]
#[derive(Debug)]
pub struct GameState {
    pub paddle: Paddle,
    pub playfield: Playfield,
    pub ball: Ball,
    pub bricks: Vec<Vec<Brick>>,
}
impl GameState {
    pub fn new() -> GameState {
        let paddle = Paddle::new(0.0);
        let playfield = Playfield::new();
        let ball = Ball::new(0.0, 0.0, 0.5, -0.86602540378);
        let mut bricks: Vec<Vec<Brick>> = Vec::new();
        for i in 0..Brick::ROWS {
            bricks.push(Vec::new());
            let x = (i as f32 * 2.0 / Brick::ROWS as f32) - 1.0;
            for j in 0..Brick::COLUMNS {
                // y coords are top half only
                let y = j as f32 * 1.0 / Brick::COLUMNS as f32;
                bricks[i].push(Brick::new(x, y));
            }
        }
        GameState {
            paddle,
            playfield,
            ball,
            bricks,
        }
    }
}
