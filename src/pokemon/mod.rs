use crate::pokemon::{
    nature::Nature,
    stat::{EffortValues, IndividualValues, StatType, Stats},
    types::Types,
};

pub mod r#move;
pub mod nature;
pub mod stat;
pub mod types;

#[derive(Debug)]
pub struct Pokemon {
    species: String,
    base_stats: Stats<u8>,
    ivs: IndividualValues,
    evs: EffortValues,
    types: heapless::Vec<Types, 3>,
    level: u32,
    nature: Nature,
}

impl Pokemon {
    pub fn calculate_stats(&self) -> Stats<u32> {
        Stats::new(
            self.calculate_hp(),
            self.calculate_other_stat(StatType::Attack),
            self.calculate_other_stat(StatType::SpecialAttack),
            self.calculate_other_stat(StatType::SpecialAttack),
            self.calculate_other_stat(StatType::SpecialDefense),
            self.calculate_other_stat(StatType::Speed),
        )
    }

    pub fn get_base_stat(&self, stat: StatType) -> u8 {
        self.base_stats.get_stat(stat)
    }

    pub fn get_iv(&self, stat: StatType) -> u8 {
        self.ivs.get_stat(stat)
    }

    pub fn get_ev(&self, stat: StatType) -> u8 {
        self.evs.get_stat(stat)
    }

    fn calculate_hp(&self) -> u32 {
        if self.species.eq_ignore_ascii_case("Shedinja") {
            return 1;
        }

        let hp = StatType::HP;
        let hp_base: u32 = self.base_stats.get_stat(hp) as u32;
        let hp_iv: u32 = self.ivs.get_stat(hp) as u32;
        let hp_ev = self.evs.get_stat(hp) as u32;

        ((2 * hp_base + hp_iv + hp_ev / 4) * self.level) / 100 + self.level + 10
    }

    fn calculate_other_stat(&self, stat: StatType) -> u32 {
        let base = self.get_base_stat(stat) as u32;
        let iv = self.get_iv(stat) as u32;
        let ev = self.get_ev(stat) as u32;

        let nature_modifier: f32 = if self.nature.is_nature_neutral() {
            1.
        } else if self.nature.get_stat_boost() == stat {
            1.1
        } else if self.nature.get_stat_drop() == stat {
            0.9
        } else {
            1.
        };

        ((((2 * base + iv + ev / 4) * self.level) / 100 + 5) as f32 * nature_modifier) as u32
    }
}
