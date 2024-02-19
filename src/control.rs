use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::view::ViewState;

pub struct ControlState {
    pub mouse_x_relative: f32,
    pub mouse_y_relative: f32,
}
impl ControlState {
    pub fn new() -> ControlState {
        ControlState {
            mouse_x_relative: 0.0,
            mouse_y_relative: 0.0,
        }
    }

    pub fn update_cursor_position(&mut self, window_state: &ViewState, p: PhysicalPosition<f64>) {
        let PhysicalSize {
            width: window_width,
            height: window_height,
        } = window_state.window.inner_size();

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
