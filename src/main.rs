use glium::backend::glutin::SimpleWindowBuilder;
use glium::index::IndicesSource;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use glium::{implement_vertex, uniform, DrawParameters, Surface, VertexBuffer};
use std::time::{Duration, Instant};
use winit::window::Window;
use winit::dpi::{PhysicalSize, PhysicalPosition};

/// Paddle
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Paddle {
    /// Location of the center of the paddle
    pub x: f32,
}
impl Paddle {
    pub const WIDTH: f32 = 0.25;
    pub const HEIGHT: f32 = 0.025;
    // pub const COLOR: (f32, f32, f32, f32) = (0.0, 1.0, 0.5, 1.0);
    pub const COLOR: [f32; 3] = [0.0, 1.0, 0.5];
    pub const VERTICAL_OFFSET: f32 = -0.950;

    pub fn get_model(&self) -> Vec<Vertex>{
        vec![
            Vertex {position: [-Paddle::WIDTH / 2.0, Paddle::HEIGHT / 2.0, 0.0], color: Paddle::COLOR},
            Vertex {position: [0.0, -Paddle::HEIGHT / 2.0, 0.0], color: Paddle::COLOR},
            Vertex {position: [Paddle::WIDTH / 2.0, Paddle::HEIGHT / 2.0, 0.0], color: Paddle::COLOR},
            ]
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
    ).expect("unable to compile shaders, exiting");

    // TMP
    let mut frame_count: u32 = 0;
    let then = Instant::now();
    let mut paddle = Paddle { x: 0.0 };
    let mut mouse_position = PhysicalPosition{x: 0.0, y: 0.0};

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let time_elapsed = Instant::now().duration_since(then).as_secs_f32();
                println!("frame_count: {}", frame_count);
                println!("time_elapsed: {} secs", time_elapsed);
                println!("frames per second: {}", frame_count as f32 / time_elapsed);

                window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved {position: p, ..},
                ..
            } => {
                mouse_position = p;
            }
            Event::AboutToWait => {
                // move paddle to mouse
                let mouse_relative_x_pos = (mouse_position.x / (window.inner_size().width / 2) as f64) - 1.0;

                paddle.x = (mouse_relative_x_pos as f32).clamp(-1.0 + (Paddle::WIDTH / 2.0), 1.0 - (Paddle::WIDTH / 2.0));

                // draw a frame
                let mut frame = display.draw();

                let colors = frame_count.to_ne_bytes();
                frame.clear(
                    None,
                    Some((
                        colors[0] as f32 / 255_f32,
                        colors[1] as f32 / 255_f32,
                        colors[2] as f32 / 255_f32,
                        1.0,
                    )),
                    false,
                    None,
                    None,
                );

                // draw paddle
                let uniforms = uniform! {
                    offset: [paddle.x, Paddle::VERTICAL_OFFSET, 0.0],
                };
                let vertices = VertexBuffer::new(&display, &paddle.get_model()).expect("unable to create vertex buffer, exiting");

                frame.draw(
                    &vertices,
                    IndicesSource::NoIndices { primitives: glium::index::PrimitiveType::TrianglesList },
                    &program,
                    &uniforms,
                    &DrawParameters::default(),
                ).expect("unable to draw paddle to frame, exiting");

                // wrap up
                frame.finish().expect("unable to finish frame, exiting");

                frame_count += 1;
            }
            _ => (),
        })
        .expect("unable to run event loop, exiting");
}
