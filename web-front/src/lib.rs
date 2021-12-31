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
#[derive(Debug, Copy, Clone)]
pub struct MouseEvent {
    event_type: MouseEventType,
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl MouseEvent {
    #[wasm_bindgen(constructor)]
    pub fn new(event_type: MouseEventType, x: f64, y: f64) -> MouseEvent {
        MouseEvent { event_type, x, y }
    }
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
    pub fn mouse_event(&mut self, event: MouseEvent) {
        use MouseEventType::*;
        match event.event_type {
            Down => {
                self.mouse_pressed = true;
                self.context.begin_path();
                self.context.move_to(event.x, event.y);
            }
            Enter => {
                if self.mouse_pressed {
                    self.context.begin_path();
                    self.context.move_to(event.x, event.y);
                }
            }
            Leave | Out => {
                if self.mouse_pressed {
                    self.context.line_to(event.x, event.y);
                    self.context.stroke();
                }
            }
            Move => {
                if self.mouse_pressed {
                    self.context.line_to(event.x, event.y);
                    self.context.stroke();
                    self.context.begin_path();
                    self.context.move_to(event.x, event.y);
                }
            }
            Up => {
                if self.mouse_pressed {
                    self.context.line_to(event.x, event.y);
                    self.context.stroke();
                }
                self.mouse_pressed = false;
            }
            Over => (),
        }
    }
}
