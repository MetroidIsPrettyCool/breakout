use crate::view::Drawable;
use crate::view::Vertex;

pub const PADDLE_WIDTH: f32 = 0.25;
pub const PADDLE_HEIGHT: f32 = 0.025;
pub const PADDLE_COLOR: [f32; 3] = [0.0, 1.0, 0.5];
pub const PADDLE_VERTICAL_OFFSET: f32 = -0.950;

pub const BALL_WIDTH: f32 = 0.025;
pub const BALL_HEIGHT: f32 = 0.025;
pub const BALL_COLOR: [f32; 3] = [0.259, 0.051, 0.671];

pub const PLAYFIELD_COLOR: [f32; 3] = [1.0, 1.0, 1.0];

pub const BRICK_COLOR: [f32; 3] = [0.5, 0.0, 0.1];
pub const BRICK_WIDTH: f32 = 0.02;
pub const BRICK_HEIGHT: f32 = 0.015;
pub const BRICK_ROWS: usize = 20;
pub const BRICK_COLUMNS: usize = 40;

/// Generic game object
#[derive(Clone, Debug, PartialEq)]
pub struct GameObject {
    /// Position relative to one's center
    pub x: f32,
    pub y: f32,
    /// Dimensions
    pub width: f32,
    pub height: f32,
    /// Velocity
    pub x_v: f32,
    pub y_v: f32,
    /// Model
    pub model: Vec<Vertex>,
}
impl GameObject {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        x_v: f32,
        y_v: f32,
        model: Vec<Vertex>,
    ) -> GameObject {
        GameObject {
            x,
            y,
            width,
            height,
            x_v,
            y_v,
            model,
        }
    }
}
impl Drawable for GameObject {
    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = self.model.clone();
        for vertex in vertices.iter_mut() {
            vertex.position[0] += self.x;
            vertex.position[1] += self.y;
        }
        vertices
    }
}
