use std::time::Instant;
use winit::event_loop::EventLoop;

use crate::logic::LogicState;

pub mod audio;
use audio::AudioState;

use self::video::VideoState;

pub mod video;

#[cfg(test)]
mod tests;

/// Information relevant to the renderer
pub struct ViewState {
    frame_count: u64,
    init_time: Instant,

    video_state: VideoState,
    audio_state: AudioState,
}
impl ViewState {
    pub fn new(event_loop: &EventLoop<()>) -> ViewState {
        ViewState {
            frame_count: 0,
            init_time: Instant::now(),

            video_state: VideoState::new(event_loop),
            audio_state: AudioState::new(),
        }
    }

    /// Draw a frame
    pub fn update(&mut self, logic_state: &LogicState) {
        self.audio_state.update(logic_state);
        self.video_state.update(logic_state);

        self.frame_count += 1;
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn window_size(&self) -> (f32, f32) {
        self.video_state.window_size()
    }

    pub fn calculate_fps(&self) -> f32 {
        self.frame_count as f32 / Instant::now().duration_since(self.init_time).as_secs_f32()
    }
}
