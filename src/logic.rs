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
    pub bricks: Vec<GameObject>,
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
            -0.5,
            BALL_WIDTH,
            BALL_HEIGHT,
            f32::cos(300_f32.to_degrees()),
            f32::sin(300_f32.to_degrees()),
            crate::view::quad(0.025, BALL_HEIGHT, BALL_COLOR).to_vec(),
        );

        let mut bricks: Vec<GameObject> = Vec::new();
        for i in 0..BRICK_COLUMNS {
            let x = ((i as f32 + 0.5) * 2.0 / BRICK_COLUMNS as f32) - 1.0;
            for j in 0..BRICK_ROWS {
                // y coords are top half only
                let y = (j as f32 + 0.5) / BRICK_ROWS as f32;
                bricks.push(GameObject::new(
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

fn objs_overlap(a: &GameObject, b: &GameObject) -> bool {
    a.x - a.width / 2.0 < b.x + b.width / 2.0
        && a.x + a.width / 2.0 > b.x - b.width / 2.0
        && a.y - a.height / 2.0 < b.y + b.height / 2.0
        && a.y + a.height / 2.0 > b.y - b.height / 2.0
}

pub fn tick(window_state: &WindowState, game_state: &mut GameState, delta_t: Duration) {
    // move paddle to mouse
    game_state.paddle.x = (window_state.mouse_x_relative).clamp(
        -1.0 + (game_state.paddle.width / 2.0),
        1.0 - (game_state.paddle.height / 2.0),
    );

    // move ball
    game_state.ball.x += game_state.ball.x_v * delta_t.as_secs_f32();
    game_state.ball.y += game_state.ball.y_v * delta_t.as_secs_f32();

    // check ball collisions with...

    // playfield
    if game_state.ball.x + game_state.ball.width / 2.0 > 1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x + game_state.ball.width / 2.0 - 1.0;
    }
    if game_state.ball.x - game_state.ball.width / 2.0 < -1.0 {
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x - game_state.ball.width / 2.0 + 1.0;
    }
    if game_state.ball.y + game_state.ball.height / 2.0 > 1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y + game_state.ball.height / 2.0 - 1.0;
    }
    if game_state.ball.y - game_state.ball.height / 2.0 < -1.0 {
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y - game_state.ball.height / 2.0 + 1.0;
    }

    // paddle
    if objs_overlap(&game_state.paddle, &game_state.ball) {
        game_state.ball.y_v = game_state.ball.y_v.abs();
        game_state.ball.y -= game_state.ball.y
            - game_state.ball.height / 2.0
            - game_state.paddle.y
            - game_state.paddle.height / 2.0;
    }

    // bricks
    for (index, brick) in game_state.bricks.iter().enumerate() {
        if objs_overlap(&brick, &game_state.ball) {
            let overlap_width = ((brick.x + brick.width / 2.0)
                .min(game_state.ball.x + game_state.ball.width / 2.0)
                - (brick.x - brick.width / 2.0)
                    .max(game_state.ball.x - game_state.ball.width / 2.0))
            .abs();
            let overlap_height = ((brick.y + brick.height / 2.0)
                .min(game_state.ball.y + game_state.ball.height / 2.0)
                - (brick.y - brick.height / 2.0)
                    .max(game_state.ball.y - game_state.ball.height / 2.0))
            .abs();

            if overlap_width >= overlap_height {
                game_state.ball.y_v *= -1.0;
            }
            if overlap_height >= overlap_width {
                game_state.ball.x_v *= -1.0;
            }

            game_state.bricks.remove(index);
            break;
        }
    }
}
