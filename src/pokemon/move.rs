use crate::pokemon::types::Types;

pub struct Move {
    r#type: Types,
    base_power: u32,
    move_type: MoveType,
    makes_contact: bool,
    secondary_effects: fn(),
    
}

pub enum MoveType {
    Physical,
    Special,
    Status,
}
