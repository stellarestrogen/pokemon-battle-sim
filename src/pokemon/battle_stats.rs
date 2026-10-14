use crate::pokemon::stat::{StatType, Stats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleStats {
    stat_changes: Stats<i32>,
    accuracy: i32,
    evasiveness: i32,
}

impl BattleStats {
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

    pub fn get_stat_change(&self, stat: StatType) -> i32 {
        self.stat_changes.get_stat(stat)
    }

    pub fn get_accuracy(&self) -> i32 {
        self.accuracy
    }

    pub fn get_evasiveness(&self) -> i32 {
        self.evasiveness
    }
}

impl Default for BattleStats {
    fn default() -> Self {
        Self {
            stat_changes: Stats::new(0, 0, 0, 0, 0, 0),
            accuracy: 0,
            evasiveness: 0,
        }
    }
}
