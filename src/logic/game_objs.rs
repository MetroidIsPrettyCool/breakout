use crate::view::Drawable;
use crate::view::Vertex;

pub const PADDLE_WIDTH: f32 = 0.25;
pub const PADDLE_HEIGHT: f32 = 0.025;
pub const PADDLE_COLOR: [f32; 3] = [0.0, 1.0, 0.5];
pub const PADDLE_VERTICAL_OFFSET: f32 = -0.8;
pub const PADDLE_PUSH_SCALE: f32 = 600.0;

pub const BALL_WIDTH: f32 = 0.025;
pub const BALL_HEIGHT: f32 = 0.025;
pub const BALL_COLOR: [f32; 3] = [0.259, 0.051, 0.671];

pub const PLAYFIELD_COLOR: [f32; 3] = [1.0, 1.0, 1.0];

pub const BRICK_COLOR: [f32; 3] = [0.5, 0.0, 0.1];
pub const BRICK_WIDTH: f32 = 0.1;
pub const BRICK_HEIGHT: f32 = 0.06;
pub const BRICK_ROWS: usize = 12;
pub const BRICK_COLUMNS: usize = 15;

#[derive(Clone, Debug, PartialEq)]
pub enum GameObjectKind {
    Paddle,
    Brick(u32),
    Playfield,
    Ball,
}

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
    /// What type of game object are we?
    pub kind: GameObjectKind,
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
        kind: GameObjectKind,
    ) -> GameObject {
        GameObject {
            x,
            y,
            width,
            height,
            x_v,
            y_v,
            model,
            kind,
        }
    }

    pub fn paddle() -> GameObject {
        Self::new(
            0.0,
            PADDLE_VERTICAL_OFFSET,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            0.0,
            0.0,
            crate::view::iso_tri_down(PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_COLOR).to_vec(),
            GameObjectKind::Paddle,
        )
    }

    pub fn playfield() -> GameObject {
        Self::new(
            0.0,
            0.0,
            2.0,
            2.0,
            0.0,
            0.0,
            crate::view::quad(2.0, 2.0, PLAYFIELD_COLOR).to_vec(),
            GameObjectKind::Playfield,
        )
    }

    pub fn ball(num: u32) -> GameObject {
        let start_angle = if num % 2 == 0 { 290_f32 } else { 250_f32 };
        Self::new(
            0.0,
            -0.25,
            BALL_WIDTH,
            BALL_HEIGHT,
            f32::cos(start_angle.to_radians()),
            f32::sin(start_angle.to_radians()),
            crate::view::quad(0.025, BALL_HEIGHT, BALL_COLOR).to_vec(),
            GameObjectKind::Ball,
        )
    }

    pub fn brick(x: f32, y: f32, color: [f32; 3]) -> GameObject {
        Self::new(
            x,
            y,
            BRICK_WIDTH,
            BRICK_HEIGHT,
            0.0,
            0.0,
            crate::view::quad(BRICK_WIDTH, BRICK_HEIGHT, color).to_vec(),
            GameObjectKind::Brick(1),
        )
    }

    pub fn bricks() -> Vec<GameObject> {
        let mut bricks: Vec<GameObject> = Vec::new();
        for i in 0..BRICK_COLUMNS {
            let x = ((i as f32 + 0.5) * 2.0 / BRICK_COLUMNS as f32) - 1.0;
            for j in 0..BRICK_ROWS {
                // y coords are top half only
                let y = (j as f32 + 0.5) / BRICK_ROWS as f32;
                bricks.push(Self::brick(x, y, BRICK_COLOR));
            }
        }
        bricks
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
