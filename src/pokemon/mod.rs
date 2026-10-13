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
        let hp = self.calculate_hp();

        
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
}
