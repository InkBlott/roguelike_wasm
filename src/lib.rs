mod utils;
mod graphics;
mod macros;
mod game;

pub use graphics::{draw_matrix_on_canvas, DisplayElement};
use wasm_bindgen::prelude::*;
use web_sys::*;
use game::{maps, entities::*, tiles::*, logic::Game};
use rgb::RGB8;

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
    canvas.set_height(800);

    let player = Entity::Player(EntityType{
        display_values: DisplayValues::new(Some('@'), RGB8::new(255, 255, 255)),
        x: Some(5),
        y: Some(5),
        health: 100,
        speed: 1,
    });


    let game = Game::new(maps::map1::MAP1, player);

    
    // let letters = [
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(None, String::from("white")), DisplayElement::new(None, String::from("white")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(None, String::from("white")), DisplayElement::new(None, String::from("white")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some('@'), String::from("yellow")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(None, String::from("white")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    //     DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")), DisplayElement::new(Some(' '), String::from("black")),
    // ];
    // draw_matrix_on_canvas(letters, &canvas)?;
    body.append_child(&canvas)?;
    Ok(())
}