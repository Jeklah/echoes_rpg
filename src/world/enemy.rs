use crate::character::Stats;
use crate::item::Item;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyType {
    Goblin,
    Orc,
    Skeleton,
    Ghost,
    Slime,
    Drake,
    Troll,
    Elemental,
    Golem,
    DarkMage,
    AncientGuardian,
}

impl EnemyType {
    pub fn get_base_stats(&self) -> Stats {
        let mut stats = Stats::new();

        match self {
            EnemyType::Goblin => {
                stats.set_strength(3);
                stats.set_intelligence(2);
                stats.set_dexterity(6);
                stats.set_constitution(3);
                stats.set_wisdom(2);
            }
            EnemyType::Orc => {
                stats.set_strength(7);
                stats.set_intelligence(2);
                stats.set_dexterity(4);
                stats.set_constitution(6);
                stats.set_wisdom(2);
            }
            EnemyType::Skeleton => {
                stats.set_strength(4);
                stats.set_intelligence(1);
                stats.set_dexterity(5);
                stats.set_constitution(4);
                stats.set_wisdom(1);
            }
            EnemyType::Ghost => {
                stats.set_strength(2);
                stats.set_intelligence(7);
                stats.set_dexterity(7);
                stats.set_constitution(3);
                stats.set_wisdom(5);
            }
            EnemyType::Slime => {
                stats.set_strength(3);
                stats.set_intelligence(1);
                stats.set_dexterity(2);
                stats.set_constitution(8);
                stats.set_wisdom(1);
            }
            EnemyType::Drake => {
                stats.set_strength(8);
                stats.set_intelligence(4);
                stats.set_dexterity(6);
                stats.set_constitution(7);
                stats.set_wisdom(3);
            }
            EnemyType::Troll => {
                stats.set_strength(9);
                stats.set_intelligence(2);
                stats.set_dexterity(3);
                stats.set_constitution(9);
                stats.set_wisdom(2);
            }
            EnemyType::Elemental => {
                stats.set_strength(5);
                stats.set_intelligence(8);
                stats.set_dexterity(5);
                stats.set_constitution(5);
                stats.set_wisdom(7);
            }
            EnemyType::Golem => {
                stats.set_strength(10);
                stats.set_intelligence(2);
                stats.set_dexterity(2);
                stats.set_constitution(12);
                stats.set_wisdom(3);
            }
            EnemyType::DarkMage => {
                stats.set_strength(3);
                stats.set_intelligence(10);
                stats.set_dexterity(5);
                stats.set_constitution(4);
                stats.set_wisdom(10);
            }
            EnemyType::AncientGuardian => {
                stats.set_strength(12);
                stats.set_intelligence(8);
                stats.set_dexterity(6);
                stats.set_constitution(12);
                stats.set_wisdom(8);
            }
        }

        stats
    }

    pub fn description(&self) -> &str {
        match self {
            EnemyType::Goblin => "A small, nimble creature with mischievous intent.",
            EnemyType::Orc => "A brutish humanoid with green skin and a powerful build.",
            EnemyType::Skeleton => "An animated pile of bones, rattling as it moves.",
            EnemyType::Ghost => "A translucent spirit floating eerily in the air.",
            EnemyType::Slime => "A gelatinous blob that oozes across the floor.",
            EnemyType::Drake => "A smaller cousin of dragons with tough scales and sharp teeth.",
            EnemyType::Troll => "A large, ugly creature with regenerative abilities.",
            EnemyType::Elemental => "A being of pure elemental energy.",
            EnemyType::Golem => "A massive construct of stone or metal, brought to life by magic.",
            EnemyType::DarkMage => "A corrupted spellcaster wielding forbidden magic.",
            EnemyType::AncientGuardian => "A powerful entity created to protect ancient treasures.",
        }
    }

    pub fn get_level_range(&self) -> Range<u32> {
        match self {
            EnemyType::Goblin | EnemyType::Skeleton | EnemyType::Slime => 1..6,
            EnemyType::Orc | EnemyType::Ghost => 3..9,
            EnemyType::Drake | EnemyType::Troll => 6..13,
            EnemyType::Elemental | EnemyType::Golem => 10..17,
            EnemyType::DarkMage => 14..21,
            EnemyType::AncientGuardian => 18..31,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub enemy_type: EnemyType,
    pub level: u32,
    pub stats: Stats,
    pub health: i32,
    pub max_health: i32,
    pub experience_reward: u32,
    pub gold_reward: u32,
    pub item_drop_chance: f32,
}

impl Enemy {
    pub fn new(name: String, enemy_type: EnemyType, level: u32) -> Self {
        let mut stats = enemy_type.get_base_stats();

        // Scale stats based on level
        for _ in 1..level {
            match enemy_type {
                EnemyType::Goblin => {
                    stats.increase_dexterity(1);
                    if level % 2 == 0 {
                        stats.increase_strength(1);
                    }
                    if level % 3 == 0 {
                        stats.increase_constitution(1);
                    }
                }
                EnemyType::Orc => {
                    stats.increase_strength(1);
                    if level % 2 == 0 {
                        stats.increase_constitution(1);
                    }
                    if level % 4 == 0 {
                        stats.increase_dexterity(1);
                    }
                }
                // Similar patterns for other enemy types...
                _ => {
                    // Generic scaling for other types
                    if level % 2 == 0 {
                        stats.increase_strength(1);
                        stats.increase_constitution(1);
                    }
                    if level % 3 == 0 {
                        stats.increase_dexterity(1);
                        stats.increase_intelligence(1);
                    }
                    if level % 4 == 0 {
                        stats.increase_wisdom(1);
                    }
                }
            }
        }

        let max_health = 10 + stats.constitution * 5;

        // Calculate rewards based on level and enemy type
        let experience_reward = level * 25
            + match enemy_type {
                EnemyType::AncientGuardian | EnemyType::DarkMage => 100,
                EnemyType::Golem | EnemyType::Elemental | EnemyType::Drake => 60,
                EnemyType::Troll | EnemyType::Ghost => 40,
                _ => 20,
            };

        let gold_reward = level * 5
            + match enemy_type {
                EnemyType::AncientGuardian | EnemyType::DarkMage => 50,
                EnemyType::Golem | EnemyType::Elemental | EnemyType::Drake => 30,
                EnemyType::Troll | EnemyType::Ghost => 20,
                _ => 10,
            };

        // Drop chance increases with enemy level and rarity
        let item_drop_chance = 0.1
            + (level as f32 * 0.02)
            + match enemy_type {
                EnemyType::AncientGuardian | EnemyType::DarkMage => 0.4,
                EnemyType::Golem | EnemyType::Elemental | EnemyType::Drake => 0.25,
                EnemyType::Troll | EnemyType::Ghost => 0.15,
                _ => 0.05,
            };

        Enemy {
            name,
            enemy_type,
            level,
            stats,
            health: max_health,
            max_health,
            experience_reward,
            gold_reward,
            item_drop_chance,
        }
    }

    pub fn generate_random(level: u32, difficulty: u32) -> Self {
        let mut rng = rand::thread_rng();

        // Determine what enemy types are appropriate for this level
        let possible_types: Vec<EnemyType> = vec![
            EnemyType::Goblin,
            EnemyType::Orc,
            EnemyType::Skeleton,
            EnemyType::Ghost,
            EnemyType::Slime,
            EnemyType::Drake,
            EnemyType::Troll,
            EnemyType::Elemental,
            EnemyType::Golem,
            EnemyType::DarkMage,
            EnemyType::AncientGuardian,
        ]
        .into_iter()
        .filter(|e_type| {
            let range = e_type.get_level_range();
            range.contains(&level)
        })
        .collect();

        if possible_types.is_empty() {
            // Fallback to basic enemies if no appropriate types
            return Enemy::new("Goblin".to_string(), EnemyType::Goblin, level);
        }

        let enemy_type = possible_types[rng.gen_range(0..possible_types.len())].clone();

        // Generate name with some variety
        let name = match enemy_type {
            EnemyType::Goblin => {
                let prefixes = ["Sneaky", "Crafty", "Nimble", "Wily"];
                format!("{} Goblin", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::Orc => {
                let prefixes = ["Brutal", "Fierce", "Battle-scarred", "Raging"];
                format!("{} Orc", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::Skeleton => {
                let prefixes = ["Ancient", "Brittle", "Rattling", "Undead"];
                format!("{} Skeleton", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::Ghost => {
                let prefixes = ["Haunting", "Ethereal", "Vengeful", "Tormented"];
                format!("{} Ghost", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::Slime => {
                let prefixes = ["Acidic", "Bubbling", "Viscous", "Sticky"];
                format!("{} Slime", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::Drake => {
                let prefixes = ["Fiery", "Scale-covered", "Winged", "Ferocious"];
                format!("{} Drake", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::Troll => {
                let prefixes = ["Hulking", "Regenerating", "Mossy", "Bridge"];
                format!("{} Troll", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::Elemental => {
                let elements = ["Fire", "Water", "Earth", "Air"];
                format!("{} Elemental", elements[rng.gen_range(0..elements.len())])
            }
            EnemyType::Golem => {
                let materials = ["Stone", "Iron", "Crystal", "Ancient"];
                format!("{} Golem", materials[rng.gen_range(0..materials.len())])
            }
            EnemyType::DarkMage => {
                let prefixes = ["Corrupted", "Shadow", "Void", "Forbidden"];
                format!("{} Mage", prefixes[rng.gen_range(0..prefixes.len())])
            }
            EnemyType::AncientGuardian => {
                let prefixes = ["Eternal", "Forgotten", "Colossal", "Primordial"];
                format!("{} Guardian", prefixes[rng.gen_range(0..prefixes.len())])
            }
        };

        // Adjust level based on difficulty
        let adjusted_level = level + rng.gen_range(0..=difficulty.min(5));

        Enemy::new(name, enemy_type, adjusted_level)
    }

    pub fn attack_damage(&self) -> i32 {
        let base_damage = match self.enemy_type {
            EnemyType::Goblin | EnemyType::Ghost => self.stats.dexterity,
            EnemyType::DarkMage | EnemyType::Elemental => self.stats.intelligence,
            _ => self.stats.strength,
        };

        let level_bonus = self.level as i32 / 2;

        base_damage + level_bonus
    }

    pub fn defense(&self) -> i32 {
        let base_defense = self.stats.constitution / 2;
        let level_bonus = self.level as i32 / 3;

        base_defense + level_bonus
    }

    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let defense = self.defense();
        let damage_taken = (amount - defense).max(1); // Always take at least 1 damage

        self.health -= damage_taken;

        damage_taken
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_drops(&self) -> (u32, u32, Option<Item>) {
        let mut rng = rand::thread_rng();

        // Randomize gold and experience a bit
        let exp_variation = rng.gen_range(0.8..1.2);
        let gold_variation = rng.gen_range(0.8..1.2);

        let experience = (self.experience_reward as f32 * exp_variation) as u32;
        let gold = (self.gold_reward as f32 * gold_variation) as u32;

        // Determine if an item drops
        let item = if rng.gen_bool(self.item_drop_chance as f64) {
            Some(Item::generate_random(self.level))
        } else {
            None
        };

        (experience, gold, item)
    }
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Level {} {}): HP: {}/{}",
            self.name,
            self.level,
            self.enemy_type.description(),
            self.health,
            self.max_health
        )
    }
}
