use crate::pokemon::stat::StatType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Nature {
    pub fn get_stat_boost(&self) -> StatType {
        match self {
            Self::Hardy | Self::Lonely | Self::Adamant | Self::Naughty | Self::Brave => {
                StatType::Attack
            }
            Self::Bold | Self::Docile | Self::Impish | Self::Lax | Self::Relaxed => {
                StatType::Defense
            }
            Self::Modest | Self::Mild | Self::Bashful | Self::Rash | Self::Quiet => {
                StatType::SpecialAttack
            }
            Self::Calm | Self::Gentle | Self::Careful | Self::Quirky | Self::Sassy => {
                StatType::SpecialDefense
            }
            Self::Timid | Self::Hasty | Self::Jolly | Self::Naive | Self::Serious => {
                StatType::Speed
            }
        }
    }

    pub fn get_stat_drop(&self) -> StatType {
        match self {
            Self::Hardy | Self::Bold | Self::Modest | Self::Calm | Self::Timid => StatType::Attack,
            Self::Lonely | Self::Docile | Self::Mild | Self::Gentle | Self::Hasty => {
                StatType::Defense
            }
            Self::Adamant | Self::Impish | Self::Bashful | Self::Careful | Self::Jolly => {
                StatType::SpecialAttack
            }
            Self::Naughty | Self::Lax | Self::Rash | Self::Quirky | Self::Naive => {
                StatType::SpecialDefense
            }
            Self::Brave | Self::Relaxed | Self::Quiet | Self::Sassy | Self::Serious => {
                StatType::Speed
            }
        }
    }

    pub fn is_nature_neutral(&self) -> bool {
        self.get_stat_boost() == self.get_stat_drop()
    }
}
