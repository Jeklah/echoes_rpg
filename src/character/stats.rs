use serde::{Deserialize, Serialize};

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
    pub strength: i32,
    pub intelligence: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub wisdom: i32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            strength: 5,
            intelligence: 5,
            dexterity: 5,
            constitution: 5,
            wisdom: 5,
        }
    }

    pub fn modify_stat(&mut self, stat_type: StatType, amount: i32) {
        match stat_type {
            StatType::Strength => self.strength += amount,
            StatType::Intelligence => self.intelligence += amount,
            StatType::Dexterity => self.dexterity += amount,
            StatType::Constitution => self.constitution += amount,
            StatType::Wisdom => self.wisdom += amount,
        }
    }

    // Individual setter methods (keeping for compatibility)
    pub fn set_strength(&mut self, value: i32) {
        self.strength = value;
    }

    pub fn set_intelligence(&mut self, value: i32) {
        self.intelligence = value;
    }

    pub fn set_dexterity(&mut self, value: i32) {
        self.dexterity = value;
    }

    pub fn set_constitution(&mut self, value: i32) {
        self.constitution = value;
    }

    pub fn set_wisdom(&mut self, value: i32) {
        self.wisdom = value;
    }

    // Individual increase methods (keeping for compatibility)
    pub fn increase_strength(&mut self, amount: i32) {
        self.strength += amount;
    }

    pub fn increase_intelligence(&mut self, amount: i32) {
        self.intelligence += amount;
    }

    pub fn increase_dexterity(&mut self, amount: i32) {
        self.dexterity += amount;
    }

    pub fn increase_constitution(&mut self, amount: i32) {
        self.constitution += amount;
    }

    pub fn increase_wisdom(&mut self, amount: i32) {
        self.wisdom += amount;
    }

    // Utility methods for common operations
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stats() {
        let stats = Stats::new();
        assert_eq!(stats.strength, 5);
        assert_eq!(stats.intelligence, 5);
        assert_eq!(stats.dexterity, 5);
        assert_eq!(stats.constitution, 5);
        assert_eq!(stats.wisdom, 5);
    }

    #[test]
    fn test_direct_field_access() {
        let stats = Stats::new();
        assert_eq!(stats.strength, 5);
        assert_eq!(stats.intelligence, 5);
        assert_eq!(stats.dexterity, 5);
        assert_eq!(stats.constitution, 5);
        assert_eq!(stats.wisdom, 5);
    }

    #[test]
    fn test_modify_stat() {
        let mut stats = Stats::new();
        stats.modify_stat(StatType::Strength, 3);
        assert_eq!(stats.strength, 8);

        stats.modify_stat(StatType::Intelligence, -2);
        assert_eq!(stats.intelligence, 3);
    }

    #[test]
    fn test_set_and_increase() {
        let mut stats = Stats::new();
        stats.set_strength(10);
        assert_eq!(stats.strength, 10);

        stats.increase_strength(5);
        assert_eq!(stats.strength, 15);
    }
}
