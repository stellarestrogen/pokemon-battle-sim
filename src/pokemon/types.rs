use paste::paste;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PkmnType {
    Normal,
    Grass,
    Fire,
    Water,
    Electric,
    Bug,
    Flying,
    Rock,
    Poison,
    Ground,
    Ice,
    Fighting,
    Psychic,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl PkmnType {
    pub fn get_resistances(r#type: PkmnType) -> &'static [PkmnType] {
        match r#type {
            Self::Normal => NORMAL_RESISTANCES,
            Self::Grass => GRASS_RESISTANCES,
            Self::Fire => FIRE_RESISTANCES,
            Self::Water => WATER_RESISTANCES,
            Self::Electric => ELECTRIC_RESISTANCES,
            Self::Bug => BUG_RESISTANCES,
            Self::Flying => FLYING_RESISTANCES,
            Self::Rock => ROCK_RESISTANCES,
            Self::Poison => POISON_RESISTANCES,
            Self::Ground => GROUND_RESISTANCES,
            Self::Ice => ICE_RESISTANCES,
            Self::Fighting => FIGHTING_RESISTANCES,
            Self::Psychic => PSYCHIC_RESISTANCES,
            Self::Ghost => GHOST_RESISTANCES,
            Self::Dragon => DRAGON_RESISTANCES,
            Self::Dark => DARK_RESISTANCES,
            Self::Steel => STEEL_RESISTANCES,
            Self::Fairy => FAIRY_RESISTANCES,
        }
    }

    pub fn get_weaknesses(r#type: PkmnType) -> &'static [PkmnType] {
        match r#type {
            Self::Normal => NORMAL_WEAKNESSES,
            Self::Grass => GRASS_WEAKNESSES,
            Self::Fire => FIRE_WEAKNESSES,
            Self::Water => WATER_WEAKNESSES,
            Self::Electric => ELECTRIC_WEAKNESSES,
            Self::Bug => BUG_WEAKNESSES,
            Self::Flying => FLYING_WEAKNESSES,
            Self::Rock => ROCK_WEAKNESSES,
            Self::Poison => POISON_WEAKNESSES,
            Self::Ground => GROUND_WEAKNESSES,
            Self::Ice => ICE_WEAKNESSES,
            Self::Fighting => FIGHTING_WEAKNESSES,
            Self::Psychic => PSYCHIC_WEAKNESSES,
            Self::Ghost => GHOST_WEAKNESSES,
            Self::Dragon => DRAGON_WEAKNESSES,
            Self::Dark => DARK_WEAKNESSES,
            Self::Steel => STEEL_WEAKNESSES,
            Self::Fairy => FAIRY_WEAKNESSES,
        }
    }

    pub fn get_immunities(r#type: PkmnType) -> &'static [PkmnType] {
        match r#type {
            Self::Normal => NORMAL_IMMUNITIES,
            Self::Grass => GRASS_IMMUNITIES,
            Self::Fire => FIRE_IMMUNITIES,
            Self::Water => WATER_IMMUNITIES,
            Self::Electric => ELECTRIC_IMMUNITIES,
            Self::Bug => BUG_IMMUNITIES,
            Self::Flying => FLYING_IMMUNITIES,
            Self::Rock => ROCK_IMMUNITIES,
            Self::Poison => POISON_IMMUNITIES,
            Self::Ground => GROUND_IMMUNITIES,
            Self::Ice => ICE_IMMUNITIES,
            Self::Fighting => FIGHTING_IMMUNITIES,
            Self::Psychic => PSYCHIC_IMMUNITIES,
            Self::Ghost => GHOST_IMMUNITIES,
            Self::Dragon => DRAGON_IMMUNITIES,
            Self::Dark => DARK_IMMUNITIES,
            Self::Steel => STEEL_IMMUNITIES,
            Self::Fairy => FAIRY_IMMUNITIES,
        }
    }
}

macro_rules! define_type {
    ($type:ty, [$($resist:ty),*], [$($weak:ty),*], [$($immune:ty),*] ) => {
        paste! {
            const [<$type:upper _RESISTANCES>]: &[PkmnType] = &[$(PkmnType::$resist),*];
            const [<$type:upper _WEAKNESSES>]: &[PkmnType] = &[$(PkmnType::$weak),*];
            const [<$type:upper _IMMUNITIES>]: &[PkmnType] = &[$(PkmnType::$immune),*];
        }

    }
}

define_type!(Normal, [], [Fighting], [Ghost]);
define_type!(
    Grass,
    [Grass, Water, Electric, Ground],
    [Fire, Bug, Flying, Poison, Ice],
    []
);
define_type!(
    Fire,
    [Grass, Fire, Bug, Ice, Steel, Fairy],
    [Water, Rock, Ground],
    []
);
define_type!(Water, [Fire, Water, Ice, Steel], [Grass, Electric], []);
define_type!(Electric, [Electric, Flying, Steel], [Ground], []);
define_type!(Bug, [Grass, Ground, Fighting], [Fire, Flying, Rock], []);
define_type!(
    Flying,
    [Grass, Bug, Fighting],
    [Electric, Rock, Ice],
    [Ground]
);
define_type!(
    Rock,
    [Normal, Fire, Flying, Poison],
    [Grass, Water, Ground, Fighting, Steel],
    []
);
define_type!(
    Poison,
    [Grass, Poison, Fighting, Fairy],
    [Ground, Psychic],
    []
);
define_type!(Ground, [Rock, Poison], [Grass, Water, Ice], [Electric]);
define_type!(Ice, [Ice], [Fire, Rock, Fighting, Steel], []);
define_type!(Fighting, [Bug, Rock, Dark], [Flying, Psychic, Fairy], []);
define_type!(Psychic, [Fighting, Psychic], [Bug, Ghost, Dark], []);
define_type!(Ghost, [Bug, Poison], [Ghost, Dark], [Normal, Fighting]);
define_type!(
    Dragon,
    [Grass, Fire, Water, Electric],
    [Ice, Dragon, Fairy],
    []
);
define_type!(Dark, [Bug, Fighting, Fairy], [Ghost, Dark], [Psychic]);
define_type!(
    Steel,
    [
        Normal, Grass, Bug, Flying, Rock, Ice, Psychic, Dragon, Steel, Fairy
    ],
    [Fire, Ground, Fighting],
    [Poison]
);
define_type!(Fairy, [Bug, Fighting, Dark], [Poison, Steel], [Dragon]);
