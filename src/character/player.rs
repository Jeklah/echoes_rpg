use crate::character::{Class, ClassType, Stats};
use crate::inventory::manager::Inventory;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub stats: Stats,
    pub level: u32,
    pub experience: u32,
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub inventory: Inventory,
    pub gold: u32,
}

impl Player {
    pub fn new(name: String, class_type: ClassType) -> Self {
        let class = Class::new(class_type);
        let stats = class.base_stats();
        let max_health = 10 + (stats.constitution * 5);
        let max_mana = 5 + (stats.wisdom * 3);

        Player {
            name,
            class,
            stats,
            level: 1,
            experience: 0,
            health: max_health,
            max_health,
            mana: max_mana,
            max_mana,
            inventory: Inventory::new(20), // Start with 20 slots
            gold: 50,
        }
    }

    pub fn gain_experience(&mut self, exp: u32) -> bool {
        self.experience += exp;
        let level_up_threshold = self.level * 100;

        if self.experience >= level_up_threshold {
            self.level_up();
            return true;
        }

        false
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.class.level_up_stats(&mut self.stats);

        // Recalculate max health and mana
        self.max_health = 10 + (self.stats.constitution * 5);
        self.max_mana = 5 + (self.stats.wisdom * 3);

        // Restore health and mana on level up
        self.health = self.max_health;
        self.mana = self.max_mana;
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn attack_damage(&self) -> i32 {
        let base_damage = match self.class.class_type {
            ClassType::Warrior => self.stats.strength,
            ClassType::Mage => self.stats.intelligence / 2,
            ClassType::Ranger => self.stats.dexterity,
            ClassType::Cleric => self.stats.wisdom / 2,
        };

        // Add weapon damage if equipped
        let weapon_damage = if let Some(weapon) = self.inventory.get_equipped_weapon() {
            weapon.power
        } else {
            1 // Base damage without weapon
        };

        base_damage + weapon_damage
    }

    pub fn defense(&self) -> i32 {
        let base_defense = self.stats.constitution / 2;

        // Add armor defense
        let armor_defense = self.inventory.get_total_armor_defense();

        base_defense + armor_defense
    }

    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let defense = self.defense();
        let damage_taken = (amount - defense).max(1); // Always take at least 1 damage
        self.health -= damage_taken;
        damage_taken
    }

    pub fn use_ability(&mut self, ability_index: usize) -> Result<String, String> {
        let _rng = rand::thread_rng();

        if let Some(ability_name) = self.class.use_ability(ability_index) {
            match ability_name {
                "Heal" => {
                    let heal_amount = self.stats.wisdom * 2;
                    let mana_cost = 5;

                    if self.mana >= mana_cost {
                        self.mana -= mana_cost;
                        self.heal(heal_amount);
                        Ok(format!("You cast Heal and restored {heal_amount} health"))
                    } else {
                        Err("Not enough mana to cast Heal".to_string())
                    }
                }
                "Fireball" => {
                    let damage = self.stats.intelligence * 3;
                    let mana_cost = 8;

                    if self.mana >= mana_cost {
                        self.mana -= mana_cost;
                        Ok(format!("You cast Fireball for {damage} damage"))
                    } else {
                        Err("Not enough mana to cast Fireball".to_string())
                    }
                }
                "Shield Block" | "Magic Shield" | "Divine Protection" => {
                    let mana_cost = 4;

                    if self.mana >= mana_cost {
                        self.mana -= mana_cost;
                        Ok(format!("You cast {ability_name} and increase your defense"))
                    } else {
                        Err(format!("Not enough mana to cast {ability_name}"))
                    }
                }
                "Slash" | "Aimed Shot" => {
                    let damage = self.attack_damage() * 2;
                    Ok(format!("You use {ability_name} for {damage} damage"))
                }
                "Evasion" => Ok("You use Evasion, increasing your chance to dodge".to_string()),
                _ => Ok(format!("You use {ability_name}")),
            }
        } else {
            Err("Invalid ability index".to_string())
        }
    }
}
