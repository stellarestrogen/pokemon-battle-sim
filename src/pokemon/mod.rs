use crate::pokemon::{
    battle_stats::BattleStats,
    r#move::Move,
    nature::Nature,
    stat::{EffortValues, IndividualValues, StatType, Stats},
    types::Types,
};

pub mod ability;
pub mod battle_stats;
pub mod effects;
pub mod item;
pub mod r#move;
pub mod nature;
pub mod stat;
pub mod types;

#[derive(Debug)]
pub struct Pokemon {
    species: String,
    base_stats: Stats<u8>,
    types: heapless::Vec<Types, 3>,
    ivs: IndividualValues,
    evs: EffortValues,
    level: u32,
    nature: Nature,
    battle_stats: BattleStats,
    moves: heapless::Vec<Move, 4>,
}

impl Pokemon {
    pub fn from_primitive(pokemon: PrimitivePokemon) -> PokemonBuilder {
        PokemonBuilder {
            species: pokemon.species,
            base_stats: pokemon.base_stats,
            types: pokemon.types,
            ivs: PokemonBuilder::default_ivs(),
            evs: PokemonBuilder::default_evs(),
            level: PokemonBuilder::default_level(),
            nature: PokemonBuilder::default_nature(),
            battle_stats: PokemonBuilder::default_battle_stats(),
            moves: PokemonBuilder::default_moves(),
        }
    }

    pub fn calculate_stats(&self) -> Stats<u32> {
        let hp = self.calculate_hp();
        let attack = self.calculate_other_stat(StatType::Attack);
        let defense = self.calculate_other_stat(StatType::Defense);
        let special_attack = self.calculate_other_stat(StatType::SpecialAttack);
        let special_defense = self.calculate_other_stat(StatType::SpecialDefense);
        let speed = self.calculate_other_stat(StatType::Speed);

        Stats::new(hp, attack, defense, special_attack, special_defense, speed)
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

pub struct PokemonBuilder {
    species: String,
    base_stats: Stats<u8>,
    types: heapless::Vec<Types, 3>,
    ivs: IndividualValues,
    evs: EffortValues,
    level: u32,
    nature: Nature,
    battle_stats: BattleStats,
    moves: heapless::Vec<Move, 4>,
}

impl PokemonBuilder {
    pub fn build(self) -> Pokemon {
        Pokemon {
            species: self.species,
            base_stats: self.base_stats,
            types: self.types,
            ivs: self.ivs,
            evs: self.evs,
            level: self.level,
            nature: self.nature,
            battle_stats: self.battle_stats,
            moves: self.moves,
        }
    }

    pub fn with_ivs(mut self, ivs: IndividualValues) -> Self {
        self.ivs = ivs;
        self
    }

    pub fn with_evs(mut self, evs: EffortValues) -> Self {
        self.evs = evs;
        self
    }

    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub fn with_nature(mut self, nature: Nature) -> Self {
        self.nature = nature;
        self
    }

    pub fn with_moves(mut self, moves: heapless::Vec<Move, 4>) -> Self {
        self.moves = moves;
        self
    }

    fn default_ivs() -> IndividualValues {
        IndividualValues::default()
    }

    fn default_evs() -> EffortValues {
        EffortValues::default()
    }

    fn default_level() -> u32 {
        1
    }

    fn default_nature() -> Nature {
        Nature::Hardy
    }

    fn default_battle_stats() -> BattleStats {
        BattleStats::default()
    }

    fn default_moves() -> heapless::Vec<Move, 4> {
        heapless::Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct PrimitivePokemon {
    species: String,
    base_stats: Stats<u8>,
    types: heapless::Vec<Types, 3>,
}
