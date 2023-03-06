
use rgb::RGB8;

#[derive(Debug)]
pub enum Tile {
    Terrain(TerrainType),
}

impl Tile {
    pub fn display_values(&self) -> &DisplayValues {
        match self {
            Tile::Terrain(terrain_type) => &terrain_type.display_values,
            _ => unreachable!("Unsupported tile variant: {:?}", self),
        }
    }
}

#[derive(Debug)]
pub struct DisplayValues {
    pub d: Option<char>,
    pub color: RGB8,
}

impl DisplayValues {
    pub fn new(d: Option<char>, color: RGB8) -> Self {
        Self { d, color }
    }

    pub fn get_js_color(&self) -> String {
        format!("rgb({},{},{})", self.color.r, self.color.g, self.color.b)
    }
}
#[derive(Debug)]
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
