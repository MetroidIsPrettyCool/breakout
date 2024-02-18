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

/// Things that can be drawn to the screen
pub trait Drawable {
    fn draw(
        &self,
        frame: &mut Frame,
        display: &Display<WindowSurface>,
        program: &Program,
        game_state: &GameState,
    ) -> Result<(), Box<dyn Error>>;
}

/// Paddle
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Paddle {
    /// Position of the center of the paddle
    pub x: f32,
}
impl Paddle {
    pub const WIDTH: f32 = 0.25;
    pub const HEIGHT: f32 = 0.025;
    pub const COLOR: [f32; 3] = [0.0, 1.0, 0.5];
    pub const VERTICAL_OFFSET: f32 = -0.950;

    pub fn get_model(&self) -> Vec<Vertex> {
        vec![
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
        ]
    }
}
impl Drawable for Paddle {
    fn draw(
        &self,
        frame: &mut Frame,
        display: &Display<WindowSurface>,
        program: &Program,
        game_state: &GameState,
    ) -> Result<(), Box<dyn Error>> {
        let uniforms = uniform! {
            offset: [self.x, Self::VERTICAL_OFFSET, 0.0],
            window_aspect: [game_state.window_width, game_state.window_height],
        };
        let vertices = VertexBuffer::new(display, &self.get_model())?;
        frame.draw(
            &vertices,
            IndicesSource::NoIndices {
                primitives: glium::index::PrimitiveType::TrianglesList,
            },
            program,
            &uniforms,
            &DrawParameters::default(),
        )?;
        Ok(())
    }
}

// Game playfield
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Playfield {}
impl Playfield {
    pub const COLOR: [f32; 3] = [1.0, 1.0, 1.0];

    pub fn get_model(&self) -> Vec<Vertex> {
        vec![
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
        ]
    }
}
impl Drawable for Playfield {
    fn draw(
        &self,
        frame: &mut Frame,
        display: &Display<WindowSurface>,
        program: &Program,
        game_state: &GameState,
    ) -> Result<(), Box<dyn Error>> {
        let uniforms = uniform! {
            offset: [0.0_f32, 0.0_f32, 0.0_f32],
            window_aspect: [game_state.window_width, game_state.window_height],
        };
        let vertices = VertexBuffer::new(display, &self.get_model())?;
        frame.draw(
            &vertices,
            IndicesSource::NoIndices {
                primitives: glium::index::PrimitiveType::TrianglesList,
            },
            program,
            &uniforms,
            &DrawParameters::default(),
        )?;
        Ok(())
    }
}

/// Information relevant to the renderer
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GameState {
    pub frame_count: u64,
    pub then: Instant,

    pub mouse_x_relative: f32,
    pub mouse_y_relative: f32,

    pub window_width: f32,
    pub window_height: f32,
}
impl GameState {
    pub fn new(window: &Window) -> GameState {
        let window_size = window.inner_size();
        GameState {
            frame_count: 0,
            then: Instant::now(),

            mouse_x_relative: 0.0,
            mouse_y_relative: 0.0,

            window_width: (window_size.width as f32),
            window_height: (window_size.height as f32),
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

    // set up shaders
    let program = glium::Program::from_source(
        &display,
        include_str!("vert.glsl"),
        include_str!("frag.glsl"),
        None,
    )
    .expect("unable to compile shaders, exiting");

    // TMP
    let mut paddle = Paddle { x: 0.0 };
    let playfield = Playfield {};

    let mut game_state = GameState::new(&window);

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let time_elapsed = Instant::now().duration_since(game_state.then).as_secs_f32();
                println!("frame_count: {}", game_state.frame_count);
                println!("time_elapsed: {} secs", time_elapsed);
                println!(
                    "frames per second: {}",
                    game_state.frame_count as f32 / time_elapsed
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
                } = window.inner_size();

                game_state.mouse_x_relative =
                    (p.x as f32 / (window.inner_size().width / 2) as f32) - 1.0;
                game_state.mouse_y_relative =
                    (p.y as f32 / (window.inner_size().height / 2) as f32) - 1.0;

                // correct for aspect ratio
                if window_width > window_height {
                    game_state.mouse_x_relative *= window_width as f32 / window_height as f32;
                } else {
                    game_state.mouse_y_relative *= window_height as f32 / window_width as f32;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(s),
                ..
            } => {
                game_state.window_width = s.width as f32;
                game_state.window_height = s.height as f32;
            }
            Event::AboutToWait => {
                // move paddle to mouse
                paddle.x = (game_state.mouse_x_relative)
                    .clamp(-1.0 + (Paddle::WIDTH / 2.0), 1.0 - (Paddle::WIDTH / 2.0));

                // draw a frame
                let mut frame = display.draw();

                frame.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, None, None);

                // draw playfield
                playfield
                    .draw(&mut frame, &display, &program, &game_state)
                    .expect("unable to draw playfield to frame, exiting");

                // draw paddle
                paddle
                    .draw(&mut frame, &display, &program, &game_state)
                    .expect("unable to draw paddle to frame, exiting");

                // wrap up
                frame.finish().expect("unable to finish frame, exiting");

                game_state.frame_count += 1;
            }
            _ => (),
        })
        .expect("unable to run event loop, exiting");
}
