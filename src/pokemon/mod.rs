use crate::pokemon::{
    stat_modifier::StatModifier,
    r#move::Move,
    nature::Nature,
    stat::{EffortValues, IndividualValues, StatType, Stats},
    types::Types,
};

pub mod ability;
pub mod stat_modifier;
pub mod effects;
pub mod item;
pub mod r#move;
pub mod nature;
pub mod stat;
pub mod test;
pub mod types;

#[derive(Debug)]
pub struct Pokemon {
    species: String,
    effective_stats: Stats<u32>,
    types: heapless::Vec<Types, 3>,
    ivs: IndividualValues,
    evs: EffortValues,
    level: u32,
    nature: Nature,
    current_hp: u32,
    modifiers: StatModifier,
    moves: heapless::Vec<Move, 4>,
}

impl Pokemon {
    pub fn from_primitive(pokemon: PrimitivePokemon) -> PokemonBuilder {
        PokemonBuilder {
            pokemon,
            ivs: PokemonBuilder::default_ivs(),
            evs: PokemonBuilder::default_evs(),
            level: PokemonBuilder::default_level(),
            nature: PokemonBuilder::default_nature(),
            modifiers: PokemonBuilder::default_battle_stats(),
            moves: PokemonBuilder::default_moves(),
        }
    }

    pub fn get_stat(&self, stat: StatType) -> u32 {
        self.effective_stats.get_stat(stat)
    }

    pub fn get_iv(&self, stat: StatType) -> u8 {
        self.ivs.get_stat(stat)
    }

    pub fn get_ev(&self, stat: StatType) -> u8 {
        self.evs.get_stat(stat)
    }

    fn calculate_hp(pokemon: &PrimitivePokemon, iv: u8, ev: u8, level: u32) -> u32 {
        if pokemon.species.eq_ignore_ascii_case("Shedinja") {
            return 1;
        }

        let hp_base: u32 = pokemon.base_stats.get_stat(StatType::HP) as u32;
        let hp_iv: u32 = iv as u32;
        let hp_ev = ev as u32;

        ((2 * hp_base + hp_iv + hp_ev / 4) * level) / 100 + level + 10
    }

    fn calculate_other_stat(
        pokemon: &PrimitivePokemon,
        ivs: IndividualValues,
        evs: EffortValues,
        stat: StatType,
        nature: Nature,
        level: u32,
    ) -> u32 {
        let base = pokemon.base_stats.get_stat(stat) as u32;
        let iv = ivs.get_stat(stat) as u32;
        let ev = evs.get_stat(stat) as u32;

        let nature_modifier: f32 = if nature.is_nature_neutral() {
            1.
        } else if nature.get_stat_boost() == stat {
            1.1
        } else if nature.get_stat_drop() == stat {
            0.9
        } else {
            1.
        };

        ((((2 * base + iv + ev / 4) * level) / 100 + 5) as f32 * nature_modifier) as u32
    }

    fn calculate_effective_stats(
        pokemon: &PrimitivePokemon,
        ivs: IndividualValues,
        evs: EffortValues,
        nature: Nature,
        level: u32,
    ) -> Stats<u32> {
        let max_hp = Self::calculate_hp(
            &pokemon,
            ivs.get_stat(StatType::HP),
            evs.get_stat(StatType::HP),
            level,
        );
        let attack =
            Self::calculate_other_stat(&pokemon, ivs, evs, StatType::Attack, nature, level);
        let defense =
            Self::calculate_other_stat(&pokemon, ivs, evs, StatType::Defense, nature, level);
        let special_attack =
            Self::calculate_other_stat(&pokemon, ivs, evs, StatType::SpecialAttack, nature, level);
        let special_defense =
            Self::calculate_other_stat(&pokemon, ivs, evs, StatType::SpecialDefense, nature, level);
        let speed = Self::calculate_other_stat(&pokemon, ivs, evs, StatType::Speed, nature, level);

        Stats::new(
            max_hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        )
    }
}

pub struct PokemonBuilder {
    pokemon: PrimitivePokemon,
    ivs: IndividualValues,
    evs: EffortValues,
    level: u32,
    nature: Nature,
    modifiers: StatModifier,
    moves: heapless::Vec<Move, 4>,
}

impl PokemonBuilder {
    pub fn build(self) -> Pokemon {
        let effective_stats = Pokemon::calculate_effective_stats(
            &self.pokemon,
            self.ivs,
            self.evs,
            self.nature,
            self.level,
        );
        Pokemon {
            species: self.pokemon.species,
            effective_stats,
            types: self.pokemon.types,
            ivs: self.ivs,
            evs: self.evs,
            level: self.level,
            nature: self.nature,
            current_hp: effective_stats.get_stat(StatType::HP),
            modifiers: self.modifiers,
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

    fn default_battle_stats() -> StatModifier {
        StatModifier::default()
    }

    fn default_moves() -> heapless::Vec<Move, 4> {
        heapless::Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct PrimitivePokemon {
    pub species: String,
    pub base_stats: Stats<u8>,
    pub types: heapless::Vec<Types, 3>,
}
