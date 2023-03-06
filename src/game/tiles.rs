
use rgb::RGB8;

pub enum Tile {
    Terrain(TerrainType),
}

pub struct DisplayValues {
    pub d: Option<char>,
    pub color: RGB8,
}

impl DisplayValues {
    pub fn new(d: Option<char>, color: RGB8) -> Self {
        Self { d, color }
    }
}

pub struct TerrainType {
    pub display_values: DisplayValues,
    pub passable: bool,
}

pub const GRASS: Tile = Tile::Terrain(TerrainType {
    display_values: DisplayValues {
        d: Some('.'),
        color: RGB8::new(0, 255, 0),
    },
    passable: true,
});

pub const WALL: Tile = Tile::Terrain(TerrainType {
    display_values: DisplayValues {
        d: None,
        color: RGB8::new(255, 255, 255),
    },
    passable: false,
});

pub const GROUND: Tile = Tile::Terrain(TerrainType {
    display_values: DisplayValues {
        d: None,
        color: RGB8::new(0, 0, 0),
    },
    passable: true,
});
