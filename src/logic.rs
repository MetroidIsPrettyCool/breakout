use std::time::Duration;

use crate::view::WindowState;

pub mod game_objs;
use game_objs::GameObject;

/// Game state
#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub paddle: GameObject,
    pub playfield: GameObject,
    pub ball: GameObject,
    pub bricks: Vec<GameObject>,
    pub balls_remaining: u32,
    pub score: u32,
    pub too_late: bool,
}
impl GameState {
    pub fn new() -> GameState {
        GameState {
            paddle: GameObject::paddle(),
            playfield: GameObject::playfield(),
            ball: GameObject::ball(),
            bricks: GameObject::bricks(),
            balls_remaining: 2,
            score: 0,
            too_late: false,
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
        // right border
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x + game_state.ball.width / 2.0 - 1.0;
    }
    if game_state.ball.x - game_state.ball.width / 2.0 < -1.0 {
        // left border
        game_state.ball.x_v *= -1.0;
        game_state.ball.x -= game_state.ball.x - game_state.ball.width / 2.0 + 1.0;
    }
    if game_state.ball.y + game_state.ball.height / 2.0 > 1.0 {
        // top border
        game_state.ball.y_v *= -1.0;
        game_state.ball.y -= game_state.ball.y + game_state.ball.height / 2.0 - 1.0;
    }
    if game_state.ball.y - game_state.ball.height / 2.0 < -1.0 {
        // bottom border
        if game_state.balls_remaining > 0 {
            game_state.balls_remaining -= 1;
            game_state.ball = GameObject::ball();
            game_state.too_late = false;
            println!(
                "balls remaining: {}, score: {}",
                game_state.balls_remaining, game_state.score
            );
        } else {
            println!("game over! score: {}", game_state.score);
            game_state.ball.x = 0.0;
            game_state.ball.y = -0.5;
            game_state.ball.x_v = 0.0;
            game_state.ball.y_v = 0.0;
        }
    }

    // paddle
    if !game_state.too_late && objs_overlap(&game_state.paddle, &game_state.ball) {
        game_state.ball.y_v = game_state.ball.y_v.abs();
        game_state.ball.y -= game_state.ball.y
            - game_state.ball.height / 2.0
            - game_state.paddle.y
            - game_state.paddle.height / 2.0;
    }

    if game_state.ball.y - game_state.ball.height / 2.0
        < game_state.paddle.y + game_state.paddle.height / 2.0
    {
        game_state.too_late = true;
    }

    // bricks
    for (index, brick) in game_state.bricks.iter().enumerate() {
        if objs_overlap(&brick, &game_state.ball) {
            // bounce the ball
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

            // update score
            game_state.score += 1;

            // destroy the brick
            game_state.bricks.remove(index);
            break;
        }
    }
}
