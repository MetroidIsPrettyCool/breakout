use std::error::Error;

use alto::{Alto, Context, Source, Stereo, StreamingSource};
use glium::backend::glutin::{Display, SimpleWindowBuilder};
use glium::{
    implement_vertex, index::IndicesSource, uniform, DrawParameters, Frame, Program, Surface,
    VertexBuffer,
};
use glutin::surface::WindowSurface;
use std::time::Instant;
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::logic::interaction::Bounce;
use crate::logic::LogicState;

#[cfg(test)]
mod tests;

const SAMPLE_RATE: i32 = 44_000;

const BYTES_BOUNCE_PADDLE: &[u8] = include_bytes!("bounce-paddle.raw");
const BYTES_BOUNCE_PLAYFIELD_BORDER: &[u8] = include_bytes!("bounce-playfieldborder.raw");
const BYTES_BOUNCE_BRICK: &[u8] = include_bytes!("bounce-brick.raw");

/// Information relevant to the renderer
pub struct ViewState {
    frame_count: u64,

    init_time: Instant,

    window_width: f32,
    window_height: f32,

    flat_shader: Program,
    window: Window,
    display: Display<WindowSurface>,

    al_context: Context,
    al_source: StreamingSource,
}
impl ViewState {
    pub fn new(event_loop: &EventLoop<()>) -> ViewState {
        // set up opengl and winit
        let (window, display) = SimpleWindowBuilder::new().build(event_loop);

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

        let al_device = alto
            .open(None)
            .expect("unable to open openal output device, exiting");

        let al_context = al_device
            .new_context(None)
            .expect("unable to create openal context, exiting");
        al_context
            .set_gain(0.1)
            .expect("unable to set openal context gain, exiting");

        let al_source = al_context
            .new_streaming_source()
            .expect("unable to create openal source, exiting");

        let window_size = window.inner_size();
        ViewState {
            frame_count: 0,
            init_time: Instant::now(),

            window_width: (window_size.width as f32),
            window_height: (window_size.height as f32),

            flat_shader,
            window,
            display,

            al_context,
            al_source,
        }
    }

    /// Draw a frame
    pub fn update(&mut self, logic_state: &LogicState) {
        // audio
        if let Some(bounce) = logic_state.bounce() {
            if self.al_source.buffers_queued() == 1 {
                self.al_source.stop();
                self.al_source
                    .unqueue_buffer()
                    .expect("unable to unqueue al buffer, exiting");
            }

            let buffer = self
                .al_context
                .new_buffer::<Stereo<u8>, &[u8]>(
                    match bounce {
                        Bounce::Brick => BYTES_BOUNCE_BRICK,
                        Bounce::Paddle => BYTES_BOUNCE_PADDLE,
                        Bounce::PlayfieldBorder => BYTES_BOUNCE_PLAYFIELD_BORDER,
                    },
                    SAMPLE_RATE,
                )
                .expect("unable to create openal buffer, exiting");

            self.al_source
                .queue_buffer(buffer)
                .expect("unable to queue openal buffer, exiting");

            self.al_source.play();
        }

        // video
        let mut frame = self.display.draw();

        frame.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, None, None);

        let mut vertices = Vec::new();
        for game_obj in logic_state.game_objs() {
            vertices.extend(game_obj.get_vertices());
        }

        draw_flat_vertices(&vertices, &mut frame, &self)
            .expect("unable to complete draw call, exiting");

        // wrap up
        frame.finish().expect("unable to finish frame, exiting");
        self.frame_count += 1;
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn window_size(&self) -> (f32, f32) {
        (self.window_width, self.window_height)
    }

    pub fn calculate_fps(&self) -> f32 {
        self.frame_count as f32 / Instant::now().duration_since(self.init_time).as_secs_f32()
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
