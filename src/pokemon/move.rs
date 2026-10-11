pub struct Move {
    move_type: MoveType,
    makes_contact: bool,
    secondary_effects: fn(),
    
}

pub enum MoveType {
    Physical,
    Special,
    Status,
}
