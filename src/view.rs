use std::error::Error;

use glium::backend::glutin::Display;
use glium::Program;
use glium::{
    implement_vertex, index::IndicesSource, uniform, DrawParameters, Frame, Surface, VertexBuffer,
};
use glutin::surface::WindowSurface;
use std::time::Instant;
use winit::window::Window;

use crate::logic::GameState;

#[cfg(test)]
mod tests;

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

/// Return a quad of specified width and height
pub const fn quad(width: f32, height: f32, color: [f32; 3]) -> [Vertex; 6] {
    [
        Vertex {
            position: [(width / 2.0), (height / 2.0), 0.0],
            color,
        },
        Vertex {
            position: [-(width / 2.0), (height / 2.0), 0.0],
            color,
        },
        Vertex {
            position: [-(width / 2.0), -(height / 2.0), 0.0],
            color,
        },
        Vertex {
            position: [(width / 2.0), (height / 2.0), 0.0],
            color,
        },
        Vertex {
            position: [-(width / 2.0), -(height / 2.0), 0.0],
            color,
        },
        Vertex {
            position: [(width / 2.0), -(height / 2.0), 0.0],
            color,
        },
    ]
}

/// Return an isosceles triangle pointing down
pub const fn iso_tri_down(width: f32, height: f32, color: [f32; 3]) -> [Vertex; 3] {
    [
        Vertex {
            position: [-width / 2.0, height / 2.0, 0.0],
            color,
        },
        Vertex {
            position: [0.0, -height / 2.0, 0.0],
            color,
        },
        Vertex {
            position: [width / 2.0, height / 2.0, 0.0],
            color,
        },
    ]
}

fn draw_flat_vertices(
    vertices: &Vec<Vertex>,
    frame: &mut Frame,
    window_state: &WindowState,
) -> Result<(), Box<dyn Error>> {
    let uniforms = uniform! {
        window_aspect: window_state.window.inner_size().width as f32 / window_state.window.inner_size().height as f32,
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

/// Draw a frame
pub fn render_frame(window_state: &mut WindowState, game_state: &GameState) {
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

    draw_flat_vertices(&vertices, &mut frame, &window_state)
        .expect("unable to complete draw call, exiting");

    // wrap up
    frame.finish().expect("unable to finish frame, exiting");
}
