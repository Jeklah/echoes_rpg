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

    pub fn get_stat(&self, stat_type: StatType) -> i32 {
        match stat_type {
            StatType::Strength => self.strength,
            StatType::Intelligence => self.intelligence,
            StatType::Dexterity => self.dexterity,
            StatType::Constitution => self.constitution,
            StatType::Wisdom => self.wisdom,
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

    // Direct access methods for performance-critical code
    pub fn strength(&self) -> i32 {
        self.strength
    }

    pub fn intelligence(&self) -> i32 {
        self.intelligence
    }

    pub fn dexterity(&self) -> i32 {
        self.dexterity
    }

    pub fn constitution(&self) -> i32 {
        self.constitution
    }

    pub fn wisdom(&self) -> i32 {
        self.wisdom
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
    pub fn total_stats(&self) -> i32 {
        self.strength + self.intelligence + self.dexterity + self.constitution + self.wisdom
    }

    pub fn average_stat(&self) -> f32 {
        self.total_stats() as f32 / 5.0
    }

    pub fn highest_stat(&self) -> i32 {
        *[
            self.strength,
            self.intelligence,
            self.dexterity,
            self.constitution,
            self.wisdom,
        ]
        .iter()
        .max()
        .unwrap_or(&0)
    }

    pub fn lowest_stat(&self) -> i32 {
        *[
            self.strength,
            self.intelligence,
            self.dexterity,
            self.constitution,
            self.wisdom,
        ]
        .iter()
        .min()
        .unwrap_or(&0)
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

// Implement common stat operations
impl Stats {
    /// Apply a stat modifier array [str, int, dex, con, wis]
    pub fn apply_modifiers(&mut self, modifiers: [i32; 5]) {
        self.strength += modifiers[0];
        self.intelligence += modifiers[1];
        self.dexterity += modifiers[2];
        self.constitution += modifiers[3];
        self.wisdom += modifiers[4];
    }

    /// Get stats as an array [str, int, dex, con, wis]
    pub fn as_array(&self) -> [i32; 5] {
        [
            self.strength,
            self.intelligence,
            self.dexterity,
            self.constitution,
            self.wisdom,
        ]
    }

    /// Create stats from an array [str, int, dex, con, wis]
    pub fn from_array(stats: [i32; 5]) -> Self {
        Stats {
            strength: stats[0],
            intelligence: stats[1],
            dexterity: stats[2],
            constitution: stats[3],
            wisdom: stats[4],
        }
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
    fn test_get_stat() {
        let stats = Stats::new();
        assert_eq!(stats.get_stat(StatType::Strength), 5);
        assert_eq!(stats.get_stat(StatType::Intelligence), 5);
        assert_eq!(stats.get_stat(StatType::Dexterity), 5);
        assert_eq!(stats.get_stat(StatType::Constitution), 5);
        assert_eq!(stats.get_stat(StatType::Wisdom), 5);
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

    #[test]
    fn test_direct_access() {
        let stats = Stats::new();
        assert_eq!(stats.strength(), 5);
        assert_eq!(stats.intelligence(), 5);
        assert_eq!(stats.dexterity(), 5);
        assert_eq!(stats.constitution(), 5);
        assert_eq!(stats.wisdom(), 5);
    }

    #[test]
    fn test_utility_methods() {
        let stats = Stats::new();
        assert_eq!(stats.total_stats(), 25);
        assert_eq!(stats.average_stat(), 5.0);
        assert_eq!(stats.highest_stat(), 5);
        assert_eq!(stats.lowest_stat(), 5);
    }

    #[test]
    fn test_array_operations() {
        let stats = Stats::from_array([10, 8, 12, 9, 11]);
        assert_eq!(stats.as_array(), [10, 8, 12, 9, 11]);

        let mut stats = Stats::new();
        stats.apply_modifiers([2, 1, 3, 0, -1]);
        assert_eq!(stats.as_array(), [7, 6, 8, 5, 4]);
    }
}
