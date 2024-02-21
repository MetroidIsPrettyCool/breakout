use std::time::{Duration, Instant};

use crate::control::ControlState;

pub mod game_objs;
use game_objs::GameObject;

pub mod interaction;
use interaction::Bounce;

/// Game state
#[derive(Clone, Debug, PartialEq)]
pub struct LogicState {
    pub paddle: GameObject,
    pub playfield: GameObject,
    pub ball: GameObject,
    pub bricks: Vec<GameObject>,

    pub balls_remaining: u32,
    pub score: u32,

    pub too_late: bool,
    pub bounce: Option<Bounce>,

    pub then: Instant,

    pub game_started: bool
}
impl LogicState {
    pub fn new() -> LogicState {
        LogicState {
            paddle: GameObject::paddle(),
            playfield: GameObject::playfield(),
            ball: GameObject::ball(2),
            bricks: GameObject::bricks(),

            balls_remaining: 2,
            score: 0,

            too_late: false,
            bounce: None,

            then: Instant::now(),

            game_started: false,
        }
    }

    pub fn update(&mut self, control_state: &ControlState, now: Instant, delta_t: Duration) {
        if !self.game_started {
            self.game_started = control_state.clicked();
        }

        if self.game_started {
            self.game_tick(control_state, now, delta_t);
        }
    }

    fn game_tick(&mut self, control_state: &ControlState, now: Instant, delta_t: Duration) {
        // upkeep
        self.bounce = None;

        // move paddle to mouse
        let new_paddle_x = (control_state.mouse_x_relative).clamp(
            -1.0 + (self.paddle.width / 2.0),
            1.0 - (self.paddle.width / 2.0),
        );
        self.paddle.x_v = (new_paddle_x - self.paddle.x) * delta_t.as_secs_f32();
        self.paddle.x = new_paddle_x;

        // move ball
        self.ball.x += self.ball.x_v * delta_t.as_secs_f32();
        self.ball.y += self.ball.y_v * delta_t.as_secs_f32();

        // check ball collisions with...

        // playfield
        if self.ball.x + self.ball.width / 2.0 > 1.0 {
            // right border
            self.ball.x_v *= -1.0;
            self.ball.x -= self.ball.x + self.ball.width / 2.0 - 1.0;

            self.bounce = Some(Bounce::PlayfieldBorder);
        }
        if self.ball.x - self.ball.width / 2.0 < -1.0 {
            // left border
            self.ball.x_v *= -1.0;
            self.ball.x -= self.ball.x - self.ball.width / 2.0 + 1.0;

            self.bounce = Some(Bounce::PlayfieldBorder);
        }
        if self.ball.y + self.ball.height / 2.0 > 1.0 {
            // top border
            self.ball.y_v *= -1.0;
            self.ball.y -= self.ball.y + self.ball.height / 2.0 - 1.0;

            self.bounce = Some(Bounce::PlayfieldBorder);
        }
        if self.ball.y - self.ball.height / 2.0 < -1.0 {
            // bottom border
            if self.balls_remaining > 0 {
                self.balls_remaining -= 1;
                self.ball = GameObject::ball(self.balls_remaining);
                self.too_late = false;
                println!(
                    "balls remaining: {}, score: {}",
                    self.balls_remaining, self.score
                );
            } else {
                println!("game over! score: {}", self.score);
                self.ball.x = 0.0;
                self.ball.y = -0.5;
                self.ball.x_v = 0.0;
                self.ball.y_v = 0.0;
            }
        }

        // paddle
        if !self.too_late && objs_overlap(&self.paddle, &self.ball) {
            self.ball.y_v = self.ball.y_v.abs();
            self.ball.x_v += self.paddle.x_v * game_objs::PADDLE_PUSH_SCALE;
            self.ball.y -=
                self.ball.y - self.ball.height / 2.0 - self.paddle.y - self.paddle.height / 2.0;

            self.bounce = Some(Bounce::Paddle);
        }

        if self.ball.y - self.ball.height / 2.0 < self.paddle.y + self.paddle.height / 2.0 {
            self.too_late = true;
        }

        // bricks
        for (index, brick) in self.bricks.iter().enumerate() {
            if objs_overlap(&brick, &self.ball) {
                // bounce the ball
                let overlap_width = ((brick.x + brick.width / 2.0)
                    .min(self.ball.x + self.ball.width / 2.0)
                    - (brick.x - brick.width / 2.0).max(self.ball.x - self.ball.width / 2.0))
                .abs();
                let overlap_height = ((brick.y + brick.height / 2.0)
                    .min(self.ball.y + self.ball.height / 2.0)
                    - (brick.y - brick.height / 2.0).max(self.ball.y - self.ball.height / 2.0))
                .abs();

                if overlap_width >= overlap_height {
                    self.ball.y_v *= -1.0;
                }
                if overlap_height >= overlap_width {
                    self.ball.x_v *= -1.0;
                }

                self.bounce = Some(Bounce::Brick);

                // update score
                self.score += 1 * time_elapsed_to_score_mult(now.duration_since(self.then));

                // destroy the brick
                self.bricks.remove(index);
                break;
            }
        }
    }

}

fn objs_overlap(a: &GameObject, b: &GameObject) -> bool {
    a.x - a.width / 2.0 < b.x + b.width / 2.0
        && a.x + a.width / 2.0 > b.x - b.width / 2.0
        && a.y - a.height / 2.0 < b.y + b.height / 2.0
        && a.y + a.height / 2.0 > b.y - b.height / 2.0
}

fn time_elapsed_to_score_mult(elapsed: Duration) -> u32 {
    if elapsed < Duration::from_secs(10) {
        10
    } else if elapsed < Duration::from_secs(30) {
        5
    } else if elapsed < Duration::from_secs(60) {
        2
    } else {
        1
    }
}
