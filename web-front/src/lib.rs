use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let canvas: HtmlCanvasElement = document()
        .get_element_by_id("canvas")
        .expect("No canvas")
        .dyn_into()
        .expect("No canvas");

    canvas.set_width(600);
    canvas.set_height(600);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .expect("This Platform is unsupported context 2d")
        .unwrap()
        .dyn_into()
        .unwrap();

    ctx.fill_rect(25., 25., 100., 100.);
    ctx.clear_rect(45., 45., 60., 60.);
    ctx.stroke_rect(50., 50., 50., 50.);
    Ok(())
}
