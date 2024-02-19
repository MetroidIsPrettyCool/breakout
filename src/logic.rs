use std::time::Duration;

use crate::{game_objs::{Ball, Brick, Paddle, Playfield}, view::WindowState};

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

pub fn tick(window_state: &WindowState, game_state: &mut GameState, delta_t: Duration) {
    // move paddle to mouse
    game_state.paddle.x = (window_state.mouse_x_relative)
        .clamp(-1.0 + (Paddle::WIDTH / 2.0), 1.0 - (Paddle::WIDTH / 2.0));

    // move ball
    game_state.ball.x += game_state.ball.x_v * delta_t.as_secs_f32();
    game_state.ball.y += game_state.ball.y_v * delta_t.as_secs_f32();

    // check ball collisions with...

    // ...playfield
    if game_state.ball.x + Ball::WIDTH / 2.0 > 1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x + Ball::WIDTH / 2.0 - 1.0;
    }
    if game_state.ball.x - Ball::WIDTH / 2.0 < -1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x - Ball::WIDTH / 2.0 + 1.0;
    }
    if game_state.ball.y + Ball::HEIGHT / 2.0 > 1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y + Ball::HEIGHT / 2.0 - 1.0;
    }
    if game_state.ball.y - Ball::HEIGHT / 2.0 < -1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y - Ball::HEIGHT / 2.0 + 1.0;
    }

    // ...paddle
    if game_state.ball.x - (Ball::WIDTH / 2.0) < game_state.paddle.x + (Paddle::WIDTH / 2.0)
        && game_state.ball.x + (Ball::WIDTH / 2.0) > game_state.paddle.x - (Paddle::WIDTH / 2.0)
        && game_state.ball.y - (Ball::HEIGHT / 2.0)
            < Paddle::VERTICAL_OFFSET + (Paddle::HEIGHT / 2.0)
        && game_state.ball.y + (Ball::HEIGHT / 2.0)
            > Paddle::VERTICAL_OFFSET - (Paddle::HEIGHT / 2.0)
    {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y - Ball::HEIGHT / 2.0 - Paddle::VERTICAL_OFFSET;
    }
}
