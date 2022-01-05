mod mnist;

use mnist::NN;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlElement};

const MNIST_SIZE: i32 = 28;

#[wasm_bindgen]
pub struct App {
    letter_canvas: CanvasRenderingContext2d,
    canvas_size: i32,
    prob_view: HtmlElement,
    mouse_pressed: bool,
    data: [f32; (MNIST_SIZE * MNIST_SIZE) as usize],
    model: NN,
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
    pub fn new(
        canvas_size: i32,
        letter_canvas: CanvasRenderingContext2d,
        prob_view: HtmlElement,
    ) -> App {
        App {
            letter_canvas,
            canvas_size,
            prob_view,
            mouse_pressed: false,
            data: [0.0; (MNIST_SIZE * MNIST_SIZE) as usize],
            model: NN::new(),
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
    pub fn clear(&mut self) {
        self.letter_canvas.set_fill_style(&"ghostwhite".into());
        self.letter_canvas
            .fill_rect(0.0, 0.0, self.canvas_size as f64, self.canvas_size as f64);
        self.data = [0.0; (MNIST_SIZE * MNIST_SIZE) as usize];
        self.prob_view.set_inner_text("");
    }
}

impl App {
    fn draw_rect(&mut self, x: i32, y: i32) {
        let cell_size = self.canvas_size / MNIST_SIZE;

        let dist2 = |ix, iy| {
            let dx = ix * cell_size + cell_size / 2 - x;
            let dy = iy * cell_size + cell_size / 2 - y;
            dx * dx + dy * dy
        };

        let ix = x / cell_size;
        let iy = y / cell_size;
        for ix in ix - 1..=ix + 1 {
            for iy in iy - 1..=iy + 1 {
                let i = iy * MNIST_SIZE + ix;
                if i < 0 || self.data.len() as i32 <= i {
                    continue;
                }
                let d = dist2(ix, iy);
                let ox = ix * cell_size;
                let oy = iy * cell_size;
                if d > cell_size * cell_size * 2 {
                    continue;
                }
                self.data[i as usize] = 1.0;
                self.letter_canvas.set_fill_style(&"black".into());
                self.letter_canvas.fill_rect(
                    ox as f64,
                    oy as f64,
                    cell_size as f64,
                    cell_size as f64,
                );
            }
        }
        self.run_nn_and_show();
    }

    fn run_nn_and_show(&self) {
        let result = self.model.run(&self.data);
        let s = match result {
            Ok(prob) => {
                let mut s = String::new();
                for (i, &p) in prob.iter().enumerate() {
                    let label = if i < 10 {
                        i.to_string()
                    } else {
                        " ".to_string()
                    };
                    s.push_str(&format!("{}: {} {:.2}%\n", label, bar(25.0, p), p * 100.0));
                }
                s
            }
            Err(e) => format!("{}", e),
        };
        self.prob_view.set_inner_text(&s);
    }
}

fn bar(max_length: f32, p: f32) -> String {
    let chunks = (max_length * p).floor();
    let remainder = ((max_length - chunks) * 8.0 / max_length).floor();
    let mut bar = String::new();
    for _ in 0..chunks as usize {
        bar.push('█');
    }
    match remainder as usize {
        0 => (),
        1 => bar.push('▏'),
        2 => bar.push('▎'),
        3 => bar.push('▍'),
        4 => bar.push('▌'),
        5 => bar.push('▋'),
        6 => bar.push('▊'),
        7 => bar.push('▉'),
        _ => (),
    }
    bar
}
