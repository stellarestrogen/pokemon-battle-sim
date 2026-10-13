use crate::pokemon::types::Types;

pub struct Move {
    r#type: Types,
    base_power: Option<u32>,
    accuracy: Option<u32>,
    move_type: MoveType,
    makes_contact: bool,
    secondary_effects: fn(),
}

pub enum MoveType {
    Physical,
    Special,
    Status,
}
