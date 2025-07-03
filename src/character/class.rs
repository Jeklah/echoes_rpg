use crate::character::Stats;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassType {
    Warrior,
    Mage,
    Ranger,
    Cleric,
}

impl std::fmt::Display for ClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassType::Warrior => write!(f, "Warrior"),
            ClassType::Mage => write!(f, "Mage"),
            ClassType::Ranger => write!(f, "Ranger"),
            ClassType::Cleric => write!(f, "Cleric"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub class_type: ClassType,
    pub abilities: Vec<String>,
}

impl Class {
    pub fn new(class_type: ClassType) -> Self {
        let abilities = match class_type {
            ClassType::Warrior => vec!["Slash".to_string(), "Shield Block".to_string()],
            ClassType::Mage => vec!["Fireball".to_string(), "Magic Shield".to_string()],
            ClassType::Ranger => vec!["Aimed Shot".to_string(), "Evasion".to_string()],
            ClassType::Cleric => vec!["Heal".to_string(), "Divine Protection".to_string()],
        };

        Class {
            class_type,
            abilities,
        }
    }

    pub fn base_stats(&self) -> Stats {
        let mut stats = Stats::new();

        match self.class_type {
            ClassType::Warrior => {
                stats.set_strength(8);
                stats.set_intelligence(3);
                stats.set_dexterity(5);
                stats.set_constitution(8);
                stats.set_wisdom(4);
            }
            ClassType::Mage => {
                stats.set_strength(3);
                stats.set_intelligence(10);
                stats.set_dexterity(4);
                stats.set_constitution(4);
                stats.set_wisdom(7);
            }
            ClassType::Ranger => {
                stats.set_strength(5);
                stats.set_intelligence(4);
                stats.set_dexterity(9);
                stats.set_constitution(5);
                stats.set_wisdom(5);
            }
            ClassType::Cleric => {
                stats.set_strength(5);
                stats.set_intelligence(5);
                stats.set_dexterity(4);
                stats.set_constitution(6);
                stats.set_wisdom(8);
            }
        }

        stats
    }

    pub fn level_up_stats(&self, stats: &mut Stats) {
        let mut rng = rand::thread_rng();

        match self.class_type {
            ClassType::Warrior => {
                stats.increase_strength(1 + rng.gen_range(0..=1));
                if rng.gen_bool(0.3) {
                    stats.increase_intelligence(1);
                }
                if rng.gen_bool(0.5) {
                    stats.increase_dexterity(1);
                }
                stats.increase_constitution(1 + rng.gen_range(0..=1));
                if rng.gen_bool(0.4) {
                    stats.increase_wisdom(1);
                }
            }
            ClassType::Mage => {
                if rng.gen_bool(0.3) {
                    stats.increase_strength(1);
                }
                stats.increase_intelligence(1 + rng.gen_range(0..=2));
                if rng.gen_bool(0.4) {
                    stats.increase_dexterity(1);
                }
                if rng.gen_bool(0.5) {
                    stats.increase_constitution(1);
                }
                stats.increase_wisdom(1 + rng.gen_range(0..=1));
            }
            ClassType::Ranger => {
                if rng.gen_bool(0.5) {
                    stats.increase_strength(1);
                }
                if rng.gen_bool(0.4) {
                    stats.increase_intelligence(1);
                }
                stats.increase_dexterity(1 + rng.gen_range(0..=2));
                if rng.gen_bool(0.5) {
                    stats.increase_constitution(1);
                }
                if rng.gen_bool(0.5) {
                    stats.increase_wisdom(1);
                }
            }
            ClassType::Cleric => {
                if rng.gen_bool(0.5) {
                    stats.increase_strength(1);
                }
                if rng.gen_bool(0.5) {
                    stats.increase_intelligence(1);
                }
                if rng.gen_bool(0.4) {
                    stats.increase_dexterity(1);
                }
                stats.increase_constitution(1);
                stats.increase_wisdom(1 + rng.gen_range(0..=1));
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_description(&self) -> &str {
        match self.class_type {
            ClassType::Warrior => {
                "A powerful melee fighter specialized in direct combat with high health and defense."
            }
            ClassType::Mage => {
                "A wielder of arcane energies with powerful spells but lower health."
            }
            ClassType::Ranger => {
                "A skilled archer and tracker with balanced abilities and ranged attacks."
            }
            ClassType::Cleric => {
                "A divine spellcaster with healing abilities and protective magic."
            }
        }
    }

    #[allow(dead_code)]
    pub fn learn_ability(&mut self, ability: String) {
        self.abilities.push(ability);
    }

    pub fn use_ability(&self, ability_index: usize) -> Option<&str> {
        self.abilities.get(ability_index).map(|s| s.as_str())
    }
}
