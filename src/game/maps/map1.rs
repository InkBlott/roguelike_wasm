use super::Map;
use crate::game::tiles;

pub const MAP1: Map = Map {
    tiles: &[
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
    width: 10,
    player_spawns: &[(5, 5)],
};
