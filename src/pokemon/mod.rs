pub mod data;
pub mod r#move;
pub mod types;

pub struct Pokemon {
    base_stats: Stats,
    IVs: IndividualValues,
    EVs: EffortValues,

    level: u32,
    nature: Nature,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    hp: u8,
    attack: u8,
    defense: u8,
    special_attack: u8,
    special_defense: u8,
    speed: u8,
}

impl Stats {
    pub fn new(
        hp: u8,
        attack: u8,
        defense: u8,
        special_attack: u8,
        special_defense: u8,
        speed: u8,
    ) -> Self {
        Self {
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        }
    }

    pub fn values(&self) -> [u8; 6] {
        [
            self.hp,
            self.attack,
            self.defense,
            self.special_attack,
            self.special_defense,
            self.speed,
        ]
    }
}

pub struct IndividualValues(Stats);

impl IndividualValues {
    pub fn new(stats: Stats) -> Option<Self> {
        for stat in stats.values() {
            if stat > 31 {
                return None;
            }
        }

        Some(Self(stats))
    }
}

pub struct EffortValues(Stats);

impl EffortValues {
    pub fn new(stats: Stats) -> Option<Self> {
        let mut sum: u16 = 0;
        for stat in stats.values() {
            sum += stat as u16;
        }

        if sum > 510 { None } else { Some(Self(stats)) }
    }
}

pub enum Nature {
    Hardy,
    Lonely,
    Adamant,
    Naughty,
    Brave,
    Bold,
    Docile,
    Impish,
    Lax,
    Relaxed,
    Modest,
    Mild,
    Bashful,
    Rash,
    Quiet,
    Calm,
    Gentle,
    Careful,
    Quirky,
    Sassy,
    Timid,
    Hasty,
    Jolly,
    Naive,
    Serious,
}
