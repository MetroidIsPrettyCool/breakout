use glium::backend::glutin::Display;
use glium::index::IndicesSource;
use glium::{implement_vertex, uniform, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glutin::surface::WindowSurface;
use std::error::Error;
use std::time::{Duration, Instant};
use winit::window::Window;

use winit::event_loop::EventLoopWindowTarget;

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

/// Paddle
#[derive(Debug)]
pub struct Paddle {
    /// Position of the center of the paddle
    pub x: f32,
}
impl Paddle {
    pub const WIDTH: f32 = 0.25;
    pub const HEIGHT: f32 = 0.025;
    pub const COLOR: [f32; 3] = [0.0, 1.0, 0.5];
    pub const VERTICAL_OFFSET: f32 = -0.950;

    pub const MODEL: [Vertex; 3] = [
        Vertex {
            position: [-Paddle::WIDTH / 2.0, Paddle::HEIGHT / 2.0, 0.0],
            color: Paddle::COLOR,
        },
        Vertex {
            position: [0.0, -Paddle::HEIGHT / 2.0, 0.0],
            color: Paddle::COLOR,
        },
        Vertex {
            position: [Paddle::WIDTH / 2.0, Paddle::HEIGHT / 2.0, 0.0],
            color: Paddle::COLOR,
        },
    ];

    fn new(x: f32) -> Paddle {
        Paddle {
            x
        }
    }
}
impl Drawable for Paddle {
    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Self::MODEL;
        for vertex in vertices.iter_mut() {
            vertex.position[0] += self.x;
            vertex.position[1] += Self::VERTICAL_OFFSET;
        }
        vertices.to_vec()
    }
}

/// Ball
#[derive(Debug)]
pub struct Ball {
    /// Position of the center of the ball
    pub x: f32,
    pub y: f32,

    /// Velocity
    pub x_v: f32,
    pub y_v: f32,
}
impl Ball {
    pub const WIDTH: f32 = 0.025;
    pub const HEIGHT: f32 = 0.025;
    pub const COLOR: [f32; 3] = [0.259, 0.051, 0.671];

    pub const MODEL: [Vertex; 6] = [
        Vertex {
            position: [(Ball::WIDTH / 2.0), (Ball::HEIGHT / 2.0), 0.0],
            color: Ball::COLOR,
        },
        Vertex {
            position: [-(Ball::WIDTH / 2.0), (Ball::HEIGHT / 2.0), 0.0],
            color: Ball::COLOR,
        },
        Vertex {
            position: [-(Ball::WIDTH / 2.0), -(Ball::HEIGHT / 2.0), 0.0],
            color: Ball::COLOR,
        },
        Vertex {
            position: [(Ball::WIDTH / 2.0), (Ball::HEIGHT / 2.0), 0.0],
            color: Ball::COLOR,
        },
        Vertex {
            position: [-(Ball::WIDTH / 2.0), -(Ball::HEIGHT / 2.0), 0.0],
            color: Ball::COLOR,
        },
        Vertex {
            position: [(Ball::WIDTH / 2.0), -(Ball::HEIGHT / 2.0), 0.0],
            color: Ball::COLOR,
        },
    ];

    fn new(x: f32, y: f32, x_v: f32, y_v: f32) -> Ball {
        Ball {
            x,
            y,
            x_v,
            y_v,
        }
    }
}
impl Drawable for Ball {
    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Self::MODEL;
        for vertex in vertices.iter_mut() {
            vertex.position[0] += self.x;
            vertex.position[1] += self.y;
        }
        vertices.to_vec()
    }
}

// Game playfield
#[derive(Debug)]
pub struct Playfield {
}
impl Playfield {
    pub const COLOR: [f32; 3] = [1.0, 1.0, 1.0];

    pub const MODEL: [Vertex; 6] = [
        Vertex {
            position: [1.0, 1.0, 0.0],
            color: Playfield::COLOR,
        },
        Vertex {
            position: [-1.0, 1.0, 0.0],
            color: Playfield::COLOR,
        },
        Vertex {
            position: [-1.0, -1.0, 0.0],
            color: Playfield::COLOR,
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            color: Playfield::COLOR,
        },
        Vertex {
            position: [-1.0, -1.0, 0.0],
            color: Playfield::COLOR,
        },
        Vertex {
            position: [1.0, -1.0, 0.0],
            color: Playfield::COLOR,
        },
    ];

    fn new() -> Playfield {
        Playfield {
        }
    }
}
impl Drawable for Playfield {
    fn get_vertices(&self) -> Vec<Vertex> {
        Self::MODEL.to_vec()
    }
}

/// Brick
#[derive(Debug)]
pub struct Brick {
    /// Position of the center of the brick
    pub x: f32,
    pub y: f32,
}
impl Brick {
    pub const COLOR: [f32; 3] = [0.5, 0.0, 0.1];
    pub const WIDTH: f32 = 0.02;
    pub const HEIGHT: f32 = 0.015;

    pub const ROWS: usize = 20;
    pub const COLUMNS: usize = 40;

    pub const MODEL: [Vertex; 6] = [
        Vertex {
            position: [(Brick::WIDTH / 2.0), (Brick::HEIGHT / 2.0), 0.0],
            color: Brick::COLOR,
        },
        Vertex {
            position: [-(Brick::WIDTH / 2.0), (Brick::HEIGHT / 2.0), 0.0],
            color: Brick::COLOR,
        },
        Vertex {
            position: [-(Brick::WIDTH / 2.0), -(Brick::HEIGHT / 2.0), 0.0],
            color: Brick::COLOR,
        },
        Vertex {
            position: [(Brick::WIDTH / 2.0), (Brick::HEIGHT / 2.0), 0.0],
            color: Brick::COLOR,
        },
        Vertex {
            position: [-(Brick::WIDTH / 2.0), -(Brick::HEIGHT / 2.0), 0.0],
            color: Brick::COLOR,
        },
        Vertex {
            position: [(Brick::WIDTH / 2.0), -(Brick::HEIGHT / 2.0), 0.0],
            color: Brick::COLOR,
        },
    ];

    pub fn new(x: f32, y: f32) -> Brick {
        Brick {
            x,
            y,
        }
    }
}
impl Drawable for Brick {
    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Self::MODEL;
        for vertex in vertices.iter_mut() {
            vertex.position[0] += self.x;
            vertex.position[1] += self.y;
        }
        vertices.to_vec()
    }
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
