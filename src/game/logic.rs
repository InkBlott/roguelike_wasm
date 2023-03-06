use super::tiles::Tile;
use super::entities::Entity;
use super::maps::Map;

pub struct Game {
    map: Map,
    player: Entity,
}

impl Game {
    pub fn new(map: Map, player: Entity) -> Self {
        Self { map, player }
    } 
}

