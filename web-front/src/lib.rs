use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct App {
    context: CanvasRenderingContext2d,
    mouse_pressed: bool,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum MouseEventType {
    Down,
    Enter,
    Leave,
    Move,
    Out,
    Over,
    Up,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: CanvasRenderingContext2d) -> App {
        App {
            context,
            mouse_pressed: false,
        }
    }

    #[wasm_bindgen]
    pub fn mouse_event(&mut self, event_type: MouseEventType, x: f64, y: f64) {
        use MouseEventType::*;
        match event_type {
            Down => {
                self.mouse_pressed = true;
                self.context.begin_path();
                self.context.move_to(x, y);
            }
            Enter => {
                if self.mouse_pressed {
                    self.context.begin_path();
                    self.context.move_to(x, y);
                }
            }
            Leave | Out => {
                if self.mouse_pressed {
                    self.context.line_to(x, y);
                    self.context.stroke();
                }
            }
            Move => {
                if self.mouse_pressed {
                    self.context.line_to(x, y);
                    self.context.stroke();
                    self.context.begin_path();
                    self.context.move_to(x, y);
                }
            }
            Up => {
                if self.mouse_pressed {
                    self.context.line_to(x, y);
                    self.context.stroke();
                }
                self.mouse_pressed = false;
            }
            Over => (),
        }
    }
}
