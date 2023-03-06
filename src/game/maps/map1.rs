use super::Map;
use crate::game::tiles;

pub const MAP1: Map = Map {
    map: &[
        tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::GROUND, tiles::WALL,
        tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL, tiles::WALL,
    ],
    width: 2,
    player_spawns: &[(0, 0)],
};
