pub mod map1;
use crate::game::tiles::Tile;

pub struct Map {
    pub tiles: &'static[Tile],
    pub width: u32,
    pub player_spawns: &'static[(u32, u32)],
}