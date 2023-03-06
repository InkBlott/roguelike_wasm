use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct DisplayElement {
    d: Option<char>,
    color: String,
}

impl DisplayElement {
    pub fn new(d: Option<char>, color: String) -> Self {
        Self {d, color}
    }
}

const CELL_SIZE : f64 = 12.0;
pub const GAME_DISPLAY_ROWS : usize = 10;

pub fn draw_matrix_on_canvas<Matrix: AsRef<[DisplayElement]>>(matrix: Matrix, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let chunks = matrix.as_ref().chunks(GAME_DISPLAY_ROWS);
    for (row_index, row) in chunks.enumerate() {
        for (col_index, d_el) in row.as_ref().iter().enumerate() {
            crate::log!("{}", d_el.color);
            context.set_fill_style(&JsValue::from_str(&d_el.color));
            let x = col_index as f64 * CELL_SIZE;
            let y = row_index as f64 * CELL_SIZE;
            match d_el.d {
                Some(d) => {
                    context.set_font("10px sans-serif");
                    let text_metrics = context.measure_text(&d.to_string())?;
                    let text_width = text_metrics.width();
                    let text_x = x + (CELL_SIZE - text_width) / 2.0;
                    let text_y = y + CELL_SIZE / 2.0 + (text_metrics.actual_bounding_box_ascent() - text_metrics.actual_bounding_box_descent()) / 2.0;
                    context.fill_text(&d.to_string(), text_x, text_y)?;
                },
                None => {
                    context.fill_rect(x, y, CELL_SIZE, CELL_SIZE);
                }
            }
        }
    }
    Ok(())
}