use crate::pokemon::types::Types;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    r#type: Types,
    base_power: Option<u32>,
    accuracy: Option<u32>,
    priority: i32,
    move_type: MoveType,
    makes_contact: bool,
    secondary_effects: fn(),
}

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Physical,
    Special,
    Status,
}
