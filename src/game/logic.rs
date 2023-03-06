use super::tiles::Tile;
use super::entities::Entity;
use super::maps::Map;
use web_sys::*;
use wasm_bindgen::prelude::*;

pub struct Game {
    map: Map,
    player: Entity,
    display_cell_size: f64,
    display_game_rows: usize,
    canvas: HtmlCanvasElement,
}

impl Game { 
    pub fn draw_on_canvas(&self) -> Result<(), JsValue>  {
        let context = &self.canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;
        context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        // let mut m_vals = self.map.tiles.to_owned();
        let mut display_vals = self.map.tiles.iter().map(|tile| tile.display_values()).collect::<Vec<_>>();
        let playerIndex = (self.player.y().unwrap() * self.map.width + self.player.x().unwrap()) as usize;
        display_vals[playerIndex] = self.player.display_values();
        let chunks = display_vals.chunks(self.display_game_rows);
        for (row_index, row) in chunks.enumerate() {
            let y = row_index as f64 * self.display_cell_size;
            for (col_index, d_vals) in row.iter().enumerate() {
                context.set_fill_style(&JsValue::from_str(&d_vals.get_js_color()));
                let x = col_index as f64 * self.display_cell_size;
                match d_vals.d {
                    Some(d) => {
                        context.set_font("10px sans-serif");
                        let text_metrics = context.measure_text(&d.to_string())?;
                        let text_width = text_metrics.width();
                        let text_x = x + (self.display_cell_size - text_width) / 2.0;
                        let text_y = y + self.display_cell_size / 2.0 + (text_metrics.actual_bounding_box_ascent() - text_metrics.actual_bounding_box_descent()) / 2.0;
                        context.fill_text(&d.to_string(), text_x, text_y)?;
                    },
                    None => {
                        context.fill_rect(x, y, self.display_cell_size, self.display_cell_size);
                    }
                }
            }
        }
        Ok(())
    }
}