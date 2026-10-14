use crate::pokemon::stat::{StatType, Stats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleEffects {
    stat_changes: Stats<i32>,
    accuracy: i32,
    evasiveness: i32,
}

impl BattleEffects {
    pub fn change_stat(&mut self, stat: StatType, stages: i32) {
        let stat_value = self.stat_changes.get_stat(stat) + stages;
        self.stat_changes.set_stat(stat, stat_value.clamp(-6, 6));
    }

    pub fn change_accuracy(&mut self, stages: i32) {
        self.accuracy += stages;
        self.accuracy = self.accuracy.clamp(-6, 6);
    }

    pub fn change_evasiveness(&mut self, stages: i32) {
        self.evasiveness += stages;
        self.evasiveness = self.evasiveness.clamp(-6, 6)
    }
}

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
