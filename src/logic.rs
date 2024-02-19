use crate::game_objs::{Ball, Brick, Paddle, Playfield};

/// Game state
// #[derive(Copy, Clone, Debug, PartialEq)]
#[derive(Debug)]
pub struct GameState {
    pub paddle: Paddle,
    pub playfield: Playfield,
    pub ball: Ball,
    pub bricks: Vec<Vec<Brick>>,
}
impl GameState {
    pub fn new() -> GameState {
        let paddle = Paddle::new(0.0);
        let playfield = Playfield::new();
        let ball = Ball::new(0.0, 0.0, 0.5, -0.86602540378);
        let mut bricks: Vec<Vec<Brick>> = Vec::new();
        for i in 0..Brick::ROWS {
            bricks.push(Vec::new());
            let x = (i as f32 * 2.0 / Brick::ROWS as f32) - 1.0;
            for j in 0..Brick::COLUMNS {
                // y coords are top half only
                let y = j as f32 * 1.0 / Brick::COLUMNS as f32;
                bricks[i].push(Brick::new(x, y));
            }
        }
        GameState {
            paddle,
            playfield,
            ball,
            bricks,
        }
    }
}
