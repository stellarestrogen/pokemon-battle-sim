#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weather {
    Rain,
    Sun,
    Sand,
    Snow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
    Grassy,
    Electric,
    Psychic,
    Fairy,
}

pub trait TriggerEffect {
    fn on_trigger();
}

pub trait PassiveEffect {
    fn effect();
}
