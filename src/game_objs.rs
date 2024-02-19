use crate::view::quad;
use crate::view::Drawable;
use crate::view::Vertex;

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

    pub fn new(x: f32) -> Paddle {
        Paddle { x }
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

    pub const MODEL: [Vertex; 6] = quad(Ball::WIDTH, Ball::HEIGHT, Ball::COLOR);

    pub fn new(x: f32, y: f32, x_v: f32, y_v: f32) -> Ball {
        Ball { x, y, x_v, y_v }
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
pub struct Playfield {}
impl Playfield {
    pub const COLOR: [f32; 3] = [1.0, 1.0, 1.0];

    pub const MODEL: [Vertex; 6] = quad(2.0, 2.0, Playfield::COLOR);

    pub fn new() -> Playfield {
        Playfield {}
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

    pub const MODEL: [Vertex; 6] = quad(Brick::WIDTH, Brick::HEIGHT, Brick::COLOR);

    pub fn new(x: f32, y: f32) -> Brick {
        Brick { x, y }
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
