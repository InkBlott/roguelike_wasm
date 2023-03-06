use super::tiles::DisplayValues;

pub enum Entity {
    Player(EntityType),
}

pub struct EntityType {
    pub display_values: DisplayValues,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub health: u32,
    pub speed: u8,
}
