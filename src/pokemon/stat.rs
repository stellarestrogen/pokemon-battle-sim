use num_traits::Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    HP,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stats<T> {
    max_hp: T,
    attack: T,
    defense: T,
    special_attack: T,
    special_defense: T,
    speed: T,
}

impl<T: Num + Clone + Copy> Stats<T> {
    pub fn new(
        max_hp: T,
        attack: T,
        defense: T,
        special_attack: T,
        special_defense: T,
        speed: T,
    ) -> Self {
        Self {
            max_hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        }
    }

    pub fn values(&self) -> [T; 6] {
        [
            self.max_hp,
            self.attack,
            self.defense,
            self.special_attack,
            self.special_defense,
            self.speed,
        ]
    }

    pub fn get_stat(&self, stat: StatType) -> T {
        match stat {
            StatType::HP => self.max_hp,
            StatType::Attack => self.attack,
            StatType::Defense => self.defense,
            StatType::SpecialAttack => self.special_attack,
            StatType::SpecialDefense => self.special_defense,
            StatType::Speed => self.speed,
        }
    }

    pub fn set_stat(&mut self, stat: StatType, value: T) {
        match stat {
            StatType::HP => self.max_hp = value,
            StatType::Attack => self.attack = value,
            StatType::Defense => self.defense = value,
            StatType::SpecialAttack => self.special_attack = value,
            StatType::SpecialDefense => self.special_defense = value,
            StatType::Speed => self.speed = value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndividualValues(Stats<u8>);

impl IndividualValues {
    pub fn new(hp: u8, attack: u8, defense: u8, special_attack: u8, special_defense: u8, speed: u8) -> Option<Self> {
        let stats = Stats::new(hp, attack, defense, special_attack, special_defense, speed);
        Self::from_stats(stats)
    }

    pub fn from_stats(stats: Stats<u8>) -> Option<Self> {
        for stat in stats.values() {
            if stat > 31 {
                return None;
            }
        }

        Some(Self(stats))
    }

    pub fn get_stat(&self, stat: StatType) -> u8 {
        self.0.get_stat(stat)
    }
}

impl Default for IndividualValues {
    fn default() -> Self {
        Self(Stats::new(31, 31, 31, 31, 31, 31))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffortValues(Stats<u8>);

impl EffortValues {
    pub fn new(hp: u8, attack: u8, defense: u8, special_attack: u8, special_defense: u8, speed: u8) -> Option<Self> {
        let stats = Stats::new(hp, attack, defense, special_attack, special_defense, speed);
        Self::from_stats(stats)
    }

    pub fn from_stats(stats: Stats<u8>) -> Option<Self> {
        let mut sum: u16 = 0;
        for stat in stats.values() {
            sum += stat as u16;
        }

        if sum > 510 { None } else { Some(Self(stats)) }
    }

    pub fn get_stat(&self, stat: StatType) -> u8 {
        self.0.get_stat(stat)
    }
}

impl Default for EffortValues {
    fn default() -> Self {
        Self(Stats::new(0, 0, 0, 0, 0, 0))
    }
}
