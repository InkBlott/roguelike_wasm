use super::tiles::DisplayValues;

#[derive(Debug)]
pub enum Entity {
    Player(EntityType),
}

impl Entity {
    pub fn display_values(&self) -> &DisplayValues {
        match self {
            Entity::Player(entity_type) => &entity_type.display_values,
            _ => unreachable!("Unsupported entity variant: {:?}", self),
        }
    }

    pub fn x(&self) -> Option<u32> {
        match self {
            Entity::Player(entity_type) => entity_type.x,
            _ => unreachable!("Unsupported entity variant: {:?}", self),
        }
    }

    pub fn y(&self) -> Option<u32> {
        match self {
            Entity::Player(entity_type) => entity_type.y,
            _ => unreachable!("Unsupported entity variant: {:?}", self),
        }
    }
}

#[derive(Debug)]
pub struct EntityType {
    pub display_values: DisplayValues,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub health: u32,
    pub speed: u8,
}
