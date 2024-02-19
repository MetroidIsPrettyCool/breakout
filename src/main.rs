use glium::backend::glutin::Display;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::index::IndicesSource;
use glium::{implement_vertex, uniform, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glutin::surface::WindowSurface;
use std::error::Error;
use std::time::{Duration, Instant};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use winit::window::Window;

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
    /// vbo
    pub vbo: VertexBuffer<Vertex>,
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

    fn new(x: f32, display: &Display<WindowSurface>) -> Paddle {
        Paddle {
            x,
            vbo: VertexBuffer::immutable(display, &Self::MODEL)
                .expect("unable to create paddle VBO, exiting"),
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

    /// vbo
    pub vbo: VertexBuffer<Vertex>,
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

    fn new(x: f32, y: f32, x_v: f32, y_v: f32, display: &Display<WindowSurface>) -> Ball {
        Ball {
            x,
            y,
            x_v,
            y_v,
            vbo: VertexBuffer::immutable(display, &Self::MODEL)
                .expect("unable to create ball VBO, exiting"),
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
    /// vbo
    pub vbo: VertexBuffer<Vertex>,
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

    fn new(display: &Display<WindowSurface>) -> Playfield {
        Playfield {
            vbo: VertexBuffer::immutable(display, &Self::MODEL)
                .expect("unable to create playfield VBO, exiting"),
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

    /// vbo
    pub vbo: VertexBuffer<Vertex>,
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

    pub fn new(x: f32, y: f32, display: &Display<WindowSurface>) -> Brick {
        Brick {
            x,
            y,
            vbo: VertexBuffer::immutable(display, &Self::MODEL)
                .expect("unable to create brick VBO, exiting"),
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

/// Flat-Shaded Vertex
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn main() {
    // create event loop
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("unable to create window, exiting");
    event_loop.set_control_flow(ControlFlow::Poll);

    // set up window
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    // TMP
    let mut paddle = Paddle::new(0.0, &display);
    let playfield = Playfield::new(&display);
    let mut ball = Ball::new(0.0, 0.0, 0.5, -0.86602540378, &display);
    let mut bricks: Vec<Vec<Brick>> = Vec::new();
    for i in 0..Brick::ROWS {
        bricks.push(Vec::new());
        let x = (i as f32 * 2.0 / Brick::ROWS as f32) - 1.0;
        for j in 0..Brick::COLUMNS {
            // y coords are top half only
            let y = j as f32 * 1.0 / Brick::COLUMNS as f32;
            bricks[i].push(Brick::new(x, y, &display));
        }
    }

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
                // timey-wimey
                let now = Instant::now();
                let delta_t = match window_state.last_frame_was {
                    Some(then) => now.duration_since(then),
                    None => Duration::ZERO,
                };

                // move paddle to mouse
                paddle.x = (window_state.mouse_x_relative)
                    .clamp(-1.0 + (Paddle::WIDTH / 2.0), 1.0 - (Paddle::WIDTH / 2.0));

                // move ball
                ball.x += ball.x_v * delta_t.as_secs_f32();
                ball.y += ball.y_v * delta_t.as_secs_f32();

                // check ball collisions with...

                // ...playfield
                if ball.x + Ball::WIDTH / 2.0 > 1.0 {
                    ball.x_v *= -1.0;
                    ball.x -= ball.x + Ball::WIDTH / 2.0 - 1.0;
                }
                if ball.x - Ball::WIDTH / 2.0 < -1.0 {
                    ball.x_v *= -1.0;
                    ball.x -= ball.x - Ball::WIDTH / 2.0 + 1.0;
                }
                if ball.y + Ball::HEIGHT / 2.0 > 1.0 {
                    ball.y_v *= -1.0;
                    ball.y -= ball.y + Ball::HEIGHT / 2.0 - 1.0;
                }
                if ball.y - Ball::HEIGHT / 2.0 < -1.0 {
                    ball.y_v *= -1.0;
                    ball.y -= ball.y - Ball::HEIGHT / 2.0 + 1.0;
                }

                // ...paddle
                if ball.x - (Ball::WIDTH / 2.0) < paddle.x + (Paddle::WIDTH / 2.0)
                    && ball.x + (Ball::WIDTH / 2.0) > paddle.x - (Paddle::WIDTH / 2.0)
                    && ball.y - (Ball::HEIGHT / 2.0)
                        < Paddle::VERTICAL_OFFSET + (Paddle::HEIGHT / 2.0)
                    && ball.y + (Ball::HEIGHT / 2.0)
                        > Paddle::VERTICAL_OFFSET - (Paddle::HEIGHT / 2.0)
                {
                    ball.y_v *= -1.0;
                    ball.y -= ball.y - Ball::HEIGHT / 2.0 - Paddle::VERTICAL_OFFSET;
                }

                // draw a frame
                let mut frame = window_state.display.draw();

                frame.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, None, None);

                let mut vertices = playfield.get_vertices();
                vertices.extend(ball.get_vertices());
                vertices.extend(paddle.get_vertices());
                for column in bricks.iter() {
                    for brick in column {
                        vertices.extend(brick.get_vertices());
                    }
                }

                draw(&vertices, &mut frame, &window_state)
                    .expect("unable to complete draw call, exiting");

                // wrap up
                frame.finish().expect("unable to finish frame, exiting");

                window_state.frame_count += 1;
                window_state.last_frame_was = Some(now);
            }
            _ => (),
        })
        .expect("unable to run event loop, exiting");
}
