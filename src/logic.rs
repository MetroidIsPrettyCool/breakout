use std::time::Duration;

use crate::view::WindowState;

pub mod game_objs;
use game_objs::GameObject;

use self::game_objs::{
    BALL_COLOR, BALL_HEIGHT, BALL_WIDTH, BRICK_COLOR, BRICK_COLUMNS, BRICK_HEIGHT, BRICK_ROWS,
    BRICK_WIDTH, PADDLE_COLOR, PADDLE_HEIGHT, PADDLE_VERTICAL_OFFSET, PADDLE_WIDTH,
    PLAYFIELD_COLOR,
};

/// Game state
#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub paddle: GameObject,
    pub playfield: GameObject,
    pub ball: GameObject,
    pub bricks: Vec<Vec<GameObject>>,
}
impl GameState {
    pub fn new() -> GameState {
        let paddle = GameObject::new(
            0.0,
            PADDLE_VERTICAL_OFFSET,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            0.0,
            0.0,
            crate::view::iso_tri_down(PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_COLOR).to_vec(),
        );

        let playfield = GameObject::new(
            0.0,
            0.0,
            2.0,
            2.0,
            0.0,
            0.0,
            crate::view::quad(2.0, 2.0, PLAYFIELD_COLOR).to_vec(),
        );

        let ball = GameObject::new(
            0.0,
            0.0,
            BALL_WIDTH,
            BALL_HEIGHT,
            0.5,
            -0.86602540378,
            crate::view::quad(0.025, BALL_HEIGHT, BALL_COLOR).to_vec(),
        );

        let mut bricks: Vec<Vec<GameObject>> = Vec::new();
        for i in 0..BRICK_ROWS {
            bricks.push(Vec::new());
            let x = (i as f32 * 2.0 / BRICK_ROWS as f32) - 1.0;
            for j in 0..BRICK_COLUMNS {
                // y coords are top half only
                let y = j as f32 * 1.0 / BRICK_COLUMNS as f32;
                bricks[i].push(GameObject::new(
                    x,
                    y,
                    BRICK_WIDTH,
                    BRICK_HEIGHT,
                    0.0,
                    0.0,
                    crate::view::quad(BRICK_WIDTH, BRICK_HEIGHT, BRICK_COLOR).to_vec(),
                ));
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
        .clamp(-1.0 + (PADDLE_WIDTH / 2.0), 1.0 - (PADDLE_WIDTH / 2.0));

    // move ball
    game_state.ball.x += game_state.ball.x_v * delta_t.as_secs_f32();
    game_state.ball.y += game_state.ball.y_v * delta_t.as_secs_f32();

    // check ball collisions with...

    // ...playfield
    if game_state.ball.x + BALL_WIDTH / 2.0 > 1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x + BALL_WIDTH / 2.0 - 1.0;
    }
    if game_state.ball.x - BALL_WIDTH / 2.0 < -1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x - BALL_WIDTH / 2.0 + 1.0;
    }
    if game_state.ball.y + BALL_HEIGHT / 2.0 > 1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y + BALL_HEIGHT / 2.0 - 1.0;
    }
    if game_state.ball.y - BALL_HEIGHT / 2.0 < -1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y - BALL_HEIGHT / 2.0 + 1.0;
    }

    // ...paddle
    if game_state.ball.x - (BALL_WIDTH / 2.0) < game_state.paddle.x + (PADDLE_WIDTH / 2.0)
        && game_state.ball.x + (BALL_WIDTH / 2.0) > game_state.paddle.x - (PADDLE_WIDTH / 2.0)
        && game_state.ball.y - (BALL_HEIGHT / 2.0) < PADDLE_VERTICAL_OFFSET + (PADDLE_HEIGHT / 2.0)
        && game_state.ball.y + (BALL_HEIGHT / 2.0) > PADDLE_VERTICAL_OFFSET - (PADDLE_HEIGHT / 2.0)
    {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y - BALL_HEIGHT / 2.0 - PADDLE_VERTICAL_OFFSET;
    }
}
