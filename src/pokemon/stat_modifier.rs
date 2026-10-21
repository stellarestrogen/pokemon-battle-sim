#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierType {
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Accuracy,
    Evasiveness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatModifier {
    attack: i32,
    defense: i32,
    special_attack: i32,
    special_defense: i32,
    speed: i32,
    accuracy: i32,
    evasiveness: i32,
}

impl StatModifier {
    pub fn change_stat(&mut self, stat: ModifierType, stages: i32) {
        match stat {
            ModifierType::Attack => {
                self.attack += stages;
                self.attack = self.attack.clamp(-6, 6);
            }
            ModifierType::Defense => {
                self.defense += stages;
                self.defense = self.defense.clamp(-6, 6);
            }
            ModifierType::SpecialAttack => {
                self.special_attack += stages;
                self.special_attack = self.special_attack.clamp(-6, 6);
            }
            ModifierType::SpecialDefense => {
                self.special_defense += stages;
                self.special_defense = self.special_defense.clamp(-6, 6);
            }
            ModifierType::Speed => {
                self.speed += stages;
                self.speed = self.speed.clamp(-6, 6);
            }
            ModifierType::Accuracy => {
                self.accuracy += stages;
                self.accuracy = self.accuracy.clamp(-6, 6);
            }
            ModifierType::Evasiveness => {
                self.evasiveness += stages;
                self.evasiveness = self.evasiveness.clamp(-6, 6);
            }
        }
    }

    pub fn get_stat_change(&self, stat: ModifierType) -> i32 {
        match stat {
            ModifierType::Attack => self.attack,
            ModifierType::Defense => self.defense,
            ModifierType::SpecialAttack => self.special_attack,
            ModifierType::SpecialDefense => self.special_defense,
            ModifierType::Speed => self.speed,
            ModifierType::Accuracy => self.accuracy,
            ModifierType::Evasiveness => self.evasiveness,
        }
    }
}

impl Default for StatModifier {
    fn default() -> Self {
        Self {
            attack: 0,
            defense: 0,
            special_attack: 0,
            special_defense: 0,
            speed: 0,
            accuracy: 0,
            evasiveness: 0,
        }
    }
}
