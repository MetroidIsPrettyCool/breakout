use alto::{Alto, Context, Source, Stereo, StreamingSource};

use crate::logic::{interaction::Bounce, LogicState};

const SAMPLE_RATE: i32 = 44_000;

pub const BYTES_BOUNCE_PADDLE: &[u8] = include_bytes!("audio/bounce-paddle.raw");
pub const BYTES_BOUNCE_PLAYFIELD_BORDER: &[u8] = include_bytes!("audio/bounce-playfieldborder.raw");
pub const BYTES_BOUNCE_BRICK: &[u8] = include_bytes!("audio/bounce-brick.raw");

pub struct AudioState {
    al_context: Context,
    al_source: StreamingSource,
}
impl AudioState {
    pub fn new() -> AudioState {
        // set up openal
        let alto =
            Alto::load_default().expect("unable to load default openal implementation, exiting");

        let al_device = alto
            .open(None)
            .expect("unable to open openal output device, exiting");

        let al_context = al_device
            .new_context(None)
            .expect("unable to create openal context, exiting");
        al_context
            .set_gain(0.1)
            .expect("unable to set openal context gain, exiting");

        let al_source = al_context
            .new_streaming_source()
            .expect("unable to create openal source, exiting");

        AudioState {
            al_context,
            al_source,
        }
    }

    pub fn update(&mut self, logic_state: &LogicState) {
        if let Some(bounce) = logic_state.bounce() {
            if self.al_source.buffers_queued() == 1 {
                self.al_source.stop();
                self.al_source
                    .unqueue_buffer()
                    .expect("unable to unqueue al buffer, exiting");
            }

            let buffer = self
                .al_context
                .new_buffer::<Stereo<u8>, &[u8]>(
                    match bounce {
                        Bounce::Brick => BYTES_BOUNCE_BRICK,
                        Bounce::Paddle => BYTES_BOUNCE_PADDLE,
                        Bounce::PlayfieldBorder => BYTES_BOUNCE_PLAYFIELD_BORDER,
                    },
                    SAMPLE_RATE,
                )
                .expect("unable to create openal buffer, exiting");

            self.al_source
                .queue_buffer(buffer)
                .expect("unable to queue openal buffer, exiting");

            self.al_source.play();
        }
    }
}
