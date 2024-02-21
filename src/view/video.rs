use glium::backend::glutin::{Display, SimpleWindowBuilder};
use glium::{
    implement_vertex, index::IndicesSource, uniform, DrawParameters, Frame, Program, Surface,
    VertexBuffer,
};
use glutin::surface::WindowSurface;
use std::error::Error;
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::logic::LogicState;

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

pub struct VideoState {
    window_width: f32,
    window_height: f32,

    flat_shader: Program,
    window: Window,
    display: Display<WindowSurface>,
}
impl VideoState {
    pub fn new(event_loop: &EventLoop<()>) -> VideoState {
        // set up opengl and winit
        let (window, display) = SimpleWindowBuilder::new().build(event_loop);

        let flat_shader = glium::Program::from_source(
            &display,
            include_str!("video/vert.glsl"),
            include_str!("video/frag.glsl"),
            None,
        )
        .expect("unable to compile shaders, exiting");

        let window_size = window.inner_size();
        VideoState {
            window_width: (window_size.width as f32),
            window_height: (window_size.height as f32),

            flat_shader,
            window,
            display,
        }
    }

    pub fn update(&mut self, logic_state: &LogicState) {
        // video
        let mut frame = self.display.draw();

        frame.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, None, None);

        let mut vertices = Vec::new();
        for game_obj in logic_state.game_objs() {
            vertices.extend(game_obj.get_vertices());
        }

        self.draw_flat_vertices(&vertices, &mut frame)
            .expect("unable to complete draw call, exiting");

        frame.finish().expect("unable to finish frame, exiting");
    }

    pub fn window_size(&self) -> (f32, f32) {
        (self.window_width, self.window_height)
    }

    fn draw_flat_vertices(
        &self,
        vertices: &Vec<Vertex>,
        frame: &mut Frame,
    ) -> Result<(), Box<dyn Error>> {
        let uniforms = uniform! {
            window_aspect: self.window.inner_size().width as f32 / self.window.inner_size().height as f32,
        };
        frame.draw(
            &VertexBuffer::new(&self.display, vertices).expect("unable to construct vbo, exiting"),
            IndicesSource::NoIndices {
                primitives: glium::index::PrimitiveType::TrianglesList,
            },
            &self.flat_shader,
            &uniforms,
            &DrawParameters::default(),
        )?;
        Ok(())
    }
}

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
