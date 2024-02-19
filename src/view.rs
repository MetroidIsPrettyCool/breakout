use std::error::Error;
use std::fs::File;
use std::io::Read;

use alto::{Alto, Buffer, Context, OutputDevice, Source, Stereo, StreamingSource};
use glium::backend::glutin::Display;
use glium::{
    implement_vertex, index::IndicesSource, uniform, DrawParameters, Frame, Program, Surface,
    VertexBuffer,
};
use glutin::surface::WindowSurface;
use std::time::Instant;
use winit::window::Window;

use crate::logic::LogicState;

#[cfg(test)]
mod tests;

/// Information relevant to the renderer
pub struct ViewState {
    pub frame_count: u64,
    pub then: Instant,

    pub last_frame_was: Option<Instant>,

    pub window_width: f32,
    pub window_height: f32,

    pub flat_shader: Program,
    pub window: Window,
    pub display: Display<WindowSurface>,

    pub al_context: Context,
    pub al_source: StreamingSource,
    pub bleep: Vec<u8>,
}
impl ViewState {
    pub fn new(window: Window, display: Display<WindowSurface>) -> ViewState {
        // set up shaders
        let flat_shader = glium::Program::from_source(
            &display,
            include_str!("vert.glsl"),
            include_str!("frag.glsl"),
            None,
        )
        .expect("unable to compile shaders, exiting");

        // set up openal
        let alto =
            Alto::load_default().expect("unable to load default openal implementation, exiting");
        let al_device = alto.open(None).expect("unable to open openal output device, exiting");

        let mut bleep_f = File::open("src/synth.raw").expect("unable to open synth.wav");
        let mut bleep: Vec<u8> = Vec::new();
        bleep_f.read_to_end(&mut bleep);

        let al_context = al_device.new_context(None).expect("unable to create openal context, exiting");
        let al_source = al_context.new_streaming_source().expect("unable to create openal source, exiting");

        let window_size = window.inner_size();
        ViewState {
            frame_count: 0,
            then: Instant::now(),
            last_frame_was: None,

            window_width: (window_size.width as f32),
            window_height: (window_size.height as f32),

            flat_shader,
            window,
            display,

            bleep,
            al_context,
            al_source,
        }
    }

    /// Draw a frame
    pub fn render_frame(&mut self, logic_state: &LogicState, now: Instant) {
        // TMP
        if self.frame_count % 180 == 0 {
            if self.al_source.buffers_processed() == 1 {
                self.al_source.unqueue_buffer().expect("unable to unqueue al buffer, exiting");
            }

            let buffer = self.al_context.new_buffer::<Stereo<u8>, Vec<u8>>(self.bleep.clone(), 354_000).expect("unable to create openal buffer, exiting");

            self.al_source.queue_buffer(buffer).expect("unable to queue openal buffer, exiting");

            self.al_source.play();
        }

        let mut frame = self.display.draw();

        frame.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, None, None);

        let mut vertices = logic_state.playfield.get_vertices();
        vertices.extend(logic_state.ball.get_vertices());
        vertices.extend(logic_state.paddle.get_vertices());
        for brick in logic_state.bricks.iter() {
            vertices.extend(brick.get_vertices());
        }

        draw_flat_vertices(&vertices, &mut frame, &self)
            .expect("unable to complete draw call, exiting");

        // wrap up
        frame.finish().expect("unable to finish frame, exiting");
        self.frame_count += 1;
        self.last_frame_was = Some(now);
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
    window_state: &ViewState,
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
        &window_state.flat_shader,
        &uniforms,
        &DrawParameters::default(),
    )?;
    Ok(())
}
