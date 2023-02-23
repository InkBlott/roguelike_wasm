use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct DisplayElement {
    d: char,
    color: String,
}

impl DisplayElement {
    pub fn new(d: char, color: String) -> Self {
        Self {d, color}
    }
}

pub fn draw_matrix_on_canvas<Matrix: AsRef<[Row]>, Row: AsRef<[DisplayElement]>>(matrix: Matrix, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let matrix = matrix.as_ref();

    let context = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    
    let cell_size = canvas.width() as f64 /( matrix[0].as_ref().len() + 60) as f64;
    
    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, d_el) in row.as_ref().iter().enumerate() {
            context.set_fill_style(&JsValue::from_str(&d_el.color));
            let x = col_index as f64 * cell_size;
            let y = row_index as f64 * cell_size;
            context.set_font("12px sans-serif");
            let text_metrics = context.measure_text(&d_el.d.to_string())?;
            let text_width = text_metrics.width();
            let text_x = x + (cell_size - text_width) / 2.0;
            let text_y = y + cell_size / 2.0 + (text_metrics.actual_bounding_box_ascent() - text_metrics.actual_bounding_box_descent()) / 2.0;
            context.fill_text(&d_el.d.to_string(), text_x, text_y)?;
        }
    }
    Ok(())
}