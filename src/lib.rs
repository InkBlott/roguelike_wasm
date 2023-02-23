mod utils;
mod graphics;

pub use graphics::{draw_matrix_on_canvas, DisplayElement};
use wasm_bindgen::prelude::*;
use web_sys::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_attribute("style", "background-color: black;")?;
    canvas.set_width(800);
    canvas.set_height(600);
    let letters = [
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new('@', String::from("blue")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
        [DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white")), DisplayElement::new(' ', String::from("white"))],
    ];

    // Draw the matrix on the canvas in white text
    draw_matrix_on_canvas(letters, &canvas)?;
    body.append_child(&canvas)?;
    Ok(())
}