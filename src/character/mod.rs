pub mod class;
pub mod player;
pub mod stats;

pub use class::{Class, ClassType};
pub use player::Player;
pub use stats::StatType;
pub use stats::Stats;

use crate::item::{Equipment, EquipmentSlot, Item};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub stats: Stats,
    pub level: u32,
    pub experience: u32,
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub equipment: HashMap<EquipmentSlot, Option<Equipment>>,
    pub inventory: Vec<Item>,
    pub gold: u32,
}

impl Character {
    #[allow(dead_code)]
    pub fn new(name: String, class: Class) -> Self {
        let base_stats = class.base_stats();
        let max_health = 10 + (base_stats.constitution * 5);
        let max_mana = 5 + (base_stats.wisdom * 3);

        let mut equipment = HashMap::new();
        for slot in EquipmentSlot::iter() {
            equipment.insert(slot, None);
        }

        Character {
            name,
            class,
            stats: base_stats,
            level: 1,
            experience: 0,
            health: max_health,
            max_health,
            mana: max_mana,
            max_mana,
            equipment,
            inventory: Vec::new(),
            gold: 50,
        }
    }

    #[allow(dead_code)]
    pub fn gain_experience(&mut self, exp: u32) -> bool {
        self.experience += exp;
        let level_up_threshold = self.level * 100;

        if self.experience >= level_up_threshold {
            self.level_up();
            return true;
        }

        false
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn equip(&mut self, item_index: usize) -> Result<(), String> {
        if item_index >= self.inventory.len() {
            return Err("Invalid item index".to_string());
        }

        let item = &self.inventory[item_index];

        match item {
            Item::Equipment(equipment) => {
                let slot = equipment.slot;

                // If there's already an item in that slot, move it to inventory
                if let Some(Some(old_equipment)) = self.equipment.get(&slot) {
                    self.inventory.push(Item::Equipment(old_equipment.clone()));
                }

                // Remove the equipped item from inventory
                let equip_item = self.inventory.remove(item_index);

                if let Item::Equipment(equip) = equip_item {
                    // Apply stat bonuses from equipment
                    for (stat_type, bonus) in &equip.stat_bonuses {
                        self.stats.modify_stat(*stat_type, *bonus);
                    }

                    // Equip the item
                    self.equipment.insert(slot, Some(equip));

                    // Recalculate derived stats
                    self.max_health = 10 + (self.stats.constitution * 5);
                    self.max_mana = 5 + (self.stats.wisdom * 3);

                    Ok(())
                } else {
                    Err("Failed to equip item".to_string())
                }
            }
            _ => Err("This item cannot be equipped".to_string()),
        }
    }

    #[allow(dead_code)]
    pub fn unequip(&mut self, slot: EquipmentSlot) -> Result<(), String> {
        if let Some(Some(equipment)) = self.equipment.get(&slot) {
            // Check if there's room in the inventory
            if self.inventory.len() >= 20 {
                return Err("Inventory is full".to_string());
            }

            // Remove stat bonuses
            for (stat_type, bonus) in &equipment.stat_bonuses {
                self.stats.modify_stat(*stat_type, -bonus);
            }

            // Add to inventory
            self.inventory.push(Item::Equipment(equipment.clone()));

            // Remove from equipment slot
            self.equipment.insert(slot, None);

            // Recalculate derived stats
            self.max_health = 10 + (self.stats.constitution * 5);
            self.max_mana = 5 + (self.stats.wisdom * 3);

            Ok(())
        } else {
            Err("No equipment in that slot".to_string())
        }
    }

    #[allow(dead_code)]
    pub fn use_item(&mut self, item_index: usize) -> Result<String, String> {
        if item_index >= self.inventory.len() {
            return Err("Invalid item index".to_string());
        }

        // Clone the item to avoid borrowing issues
        let item_clone = self.inventory[item_index].clone();

        match item_clone {
            Item::Consumable(consumable) => {
                // Apply the consumable effect directly
                let message = match consumable.consumable_type {
                    crate::item::ConsumableType::HealthPotion => {
                        let potency = consumable.potency;
                        let name = consumable.name.clone();
                        self.heal(potency);
                        format!("You used {name} and healed for {potency} health.")
                    }
                    crate::item::ConsumableType::ManaPotion => {
                        let potency = consumable.potency;
                        let name = consumable.name.clone();
                        let before_mana = self.mana;
                        self.mana = (self.mana + potency).min(self.max_mana);
                        let restored = self.mana - before_mana;
                        format!("You used {name} and restored {restored} mana.")
                    }
                    crate::item::ConsumableType::StrengthElixir => {
                        let name = consumable.name.clone();
                        self.stats.modify_stat(StatType::Strength, 1);
                        format!("You used {name}. Your strength has increased!")
                    }
                    crate::item::ConsumableType::IntelligenceElixir => {
                        let name = consumable.name.clone();
                        self.stats.modify_stat(StatType::Intelligence, 1);
                        format!("You used {name}. Your intelligence has increased!")
                    }
                    crate::item::ConsumableType::DexterityElixir => {
                        let name = consumable.name.clone();
                        self.stats.modify_stat(StatType::Dexterity, 1);
                        format!("You used {name}. Your dexterity has increased!")
                    }
                    crate::item::ConsumableType::ConstitutionElixir => {
                        let name = consumable.name.clone();
                        self.stats.modify_stat(StatType::Constitution, 1);
                        format!("You used {name}. Your constitution has increased!")
                    }
                    crate::item::ConsumableType::WisdomElixir => {
                        let name = consumable.name.clone();
                        self.stats.modify_stat(StatType::Wisdom, 1);
                        format!("You used {name}. Your wisdom has increased!")
                    }
                    _ => {
                        let name = consumable.name.clone();
                        format!("You used {name} with no effect.")
                    }
                };

                self.inventory.remove(item_index);
                Ok(message)
            }
            _ => Err("This item cannot be used".to_string()),
        }
    }

    #[allow(dead_code)]
    pub fn attack_damage(&self) -> i32 {
        let base_damage = match self.class.class_type {
            ClassType::Warrior => self.stats.strength,
            ClassType::Mage => self.stats.intelligence / 2,
            ClassType::Ranger => self.stats.dexterity,
            ClassType::Cleric => self.stats.wisdom / 2,
        };

        // Add weapon damage if equipped
        let weapon_damage = if let Some(Some(weapon)) = self.equipment.get(&EquipmentSlot::Weapon) {
            weapon.power
        } else {
            0
        };

        base_damage + weapon_damage
    }

    #[allow(dead_code)]
    pub fn defense(&self) -> i32 {
        let base_defense = self.stats.constitution / 2;

        // Add armor defense
        let mut armor_defense = 0;
        for slot in [
            EquipmentSlot::Head,
            EquipmentSlot::Chest,
            EquipmentSlot::Hands,
            EquipmentSlot::Feet,
        ] {
            if let Some(Some(armor)) = self.equipment.get(&slot) {
                armor_defense += armor.power;
            }
        }

        base_defense + armor_defense
    }

    #[allow(dead_code)]
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    #[allow(dead_code)]
    pub fn spend_mana(&mut self, amount: i32) -> bool {
        if self.mana >= amount {
            self.mana -= amount;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    #[allow(dead_code)]
    pub fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.inventory.len() >= 20 {
            return Err("Inventory is full".to_string());
        }

        self.inventory.push(item);
        Ok(())
    }
}
