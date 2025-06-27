use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatType {
    Strength,
    Intelligence,
    Dexterity,
    Constitution,
    Wisdom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    stats: HashMap<StatType, i32>,
}

impl Stats {
    pub fn new() -> Self {
        let mut stats = HashMap::new();
        stats.insert(StatType::Strength, 5);
        stats.insert(StatType::Intelligence, 5);
        stats.insert(StatType::Dexterity, 5);
        stats.insert(StatType::Constitution, 5);
        stats.insert(StatType::Wisdom, 5);

        Stats { stats }
    }

    pub fn get_stat(&self, stat_type: StatType) -> i32 {
        *self.stats.get(&stat_type).unwrap_or(&0)
    }

    pub fn set_strength(&mut self, value: i32) {
        self.stats.insert(StatType::Strength, value);
    }

    pub fn set_intelligence(&mut self, value: i32) {
        self.stats.insert(StatType::Intelligence, value);
    }

    pub fn set_dexterity(&mut self, value: i32) {
        self.stats.insert(StatType::Dexterity, value);
    }

    pub fn set_constitution(&mut self, value: i32) {
        self.stats.insert(StatType::Constitution, value);
    }

    pub fn set_wisdom(&mut self, value: i32) {
        self.stats.insert(StatType::Wisdom, value);
    }

    pub fn increase_strength(&mut self, amount: i32) {
        *self.stats.entry(StatType::Strength).or_insert(0) += amount;
    }

    pub fn increase_intelligence(&mut self, amount: i32) {
        *self.stats.entry(StatType::Intelligence).or_insert(0) += amount;
    }

    pub fn increase_dexterity(&mut self, amount: i32) {
        *self.stats.entry(StatType::Dexterity).or_insert(0) += amount;
    }

    pub fn increase_constitution(&mut self, amount: i32) {
        *self.stats.entry(StatType::Constitution).or_insert(0) += amount;
    }

    pub fn increase_wisdom(&mut self, amount: i32) {
        *self.stats.entry(StatType::Wisdom).or_insert(0) += amount;
    }

    pub fn modify_stat(&mut self, stat_type: StatType, amount: i32) {
        *self.stats.entry(stat_type).or_insert(0) += amount;
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
