use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, MouseButton, WindowEvent},
};

use crate::view::ViewState;

pub struct ControlState {
    pub mouse_x_relative: f32,
    pub mouse_y_relative: f32,

    lmb_up_frame: Option<u64>,
    lmb_down_frame: Option<u64>,
}
impl ControlState {
    pub fn new() -> ControlState {
        ControlState {
            mouse_x_relative: 0.0,
            mouse_y_relative: 0.0,

            lmb_up_frame: None,
            lmb_down_frame: None,
        }
    }

    pub fn update(&mut self, view_state: &ViewState, event: Event<()>) {
        match event {
            Event::WindowEvent {
                event: win_event, ..
            } => match win_event {
                WindowEvent::CursorMoved { position: p, .. } => {
                    self.on_cursor_moved(view_state, p);
                }
                WindowEvent::MouseInput { state: s, button: b, .. } => {
                    self.on_mouse_input(view_state, s, b);
                }
                _ => (),
            },
            _ => (),
        }
    }

    /// Has the user clicked this frame?
    pub fn clicked(&self) -> bool {
        if let Some(up_frame) = self.lmb_up_frame && let Some(down_frame) = self.lmb_down_frame {
            up_frame >= down_frame
        } else {
            false
        }
    }

    fn on_mouse_input(&mut self, view_state: &ViewState, button_state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => {
                match button_state {
                    ElementState::Pressed => self.lmb_down_frame = Some(view_state.frame_count),
                    ElementState::Released => self.lmb_up_frame = Some(view_state.frame_count),
                }
            }
            _ => (),
        }
    }

    fn on_cursor_moved(&mut self, view_state: &ViewState, p: PhysicalPosition<f64>) {
        let PhysicalSize {
            width: window_width,
            height: window_height,
        } = view_state.window.inner_size();

        self.mouse_x_relative = (p.x as f32 / (window_width / 2) as f32) - 1.0;
        self.mouse_y_relative = (p.y as f32 / (window_height / 2) as f32) - 1.0;

        // correct for aspect ratio
        if window_width > window_height {
            self.mouse_x_relative *= window_width as f32 / window_height as f32;
        } else {
            self.mouse_y_relative *= window_height as f32 / window_width as f32;
        }
    }
}
