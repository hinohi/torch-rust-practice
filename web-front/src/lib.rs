use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const MNIST_SIZE: i32 = 28;

#[wasm_bindgen]
pub struct App {
    context: CanvasRenderingContext2d,
    canvas_size: i32,
    mouse_pressed: bool,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum MouseEventType {
    Down,
    Enter,
    Leave,
    Move,
    Up,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_size: i32, context: CanvasRenderingContext2d) -> App {
        App {
            context,
            canvas_size,
            mouse_pressed: false,
        }
    }

    #[wasm_bindgen]
    pub fn mouse_event(&mut self, event_type: MouseEventType, x: i32, y: i32, buttons: u32) {
        use MouseEventType::*;
        match event_type {
            Down => {
                self.mouse_pressed = true;
                self.draw_rect(x, y);
            }
            Enter => {
                if buttons & 1 == 1 {
                    self.mouse_pressed = true;
                }
                if self.mouse_pressed {
                    self.draw_rect(x, y);
                }
            }
            Leave => {
                self.mouse_pressed = false;
            }
            Move => {
                if self.mouse_pressed {
                    self.draw_rect(x, y);
                }
            }
            Up => {
                self.mouse_pressed = false;
            }
        }
    }

    #[wasm_bindgen]
    pub fn clear(&self) {
        self.context.set_fill_style(&"ghostwhite".into());
        self.context
            .fill_rect(0.0, 0.0, self.canvas_size as f64, self.canvas_size as f64);
    }
}

impl App {
    fn draw_rect(&self, x: i32, y: i32) {
        let cell_size = self.canvas_size / MNIST_SIZE;
        let ox = x / cell_size * cell_size;
        let oy = y / cell_size * cell_size;
        self.context.set_fill_style(&"black".into());
        self.context
            .fill_rect(ox as f64, oy as f64, cell_size as f64, cell_size as f64);
    }
}
