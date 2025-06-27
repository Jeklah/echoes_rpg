use crate::character::StatType;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    Chest,
    Hands,
    Feet,
    Weapon,
    Shield,
}

impl EquipmentSlot {
    pub fn iter() -> impl Iterator<Item = EquipmentSlot> {
        vec![
            EquipmentSlot::Head,
            EquipmentSlot::Chest,
            EquipmentSlot::Hands,
            EquipmentSlot::Feet,
            EquipmentSlot::Weapon,
            EquipmentSlot::Shield,
        ]
        .into_iter()
    }
}

impl fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EquipmentSlot::Head => write!(f, "Head"),
            EquipmentSlot::Chest => write!(f, "Chest"),
            EquipmentSlot::Hands => write!(f, "Hands"),
            EquipmentSlot::Feet => write!(f, "Feet"),
            EquipmentSlot::Weapon => write!(f, "Weapon"),
            EquipmentSlot::Shield => write!(f, "Shield"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipmentType {
    Armor,
    Weapon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub name: String,
    pub description: String,
    pub equipment_type: EquipmentType,
    pub slot: EquipmentSlot,
    pub power: i32,
    pub value: u32,
    pub stat_bonuses: HashMap<StatType, i32>,
    pub level_requirement: u32,
}

impl Equipment {
    pub fn new(
        name: String,
        description: String,
        equipment_type: EquipmentType,
        slot: EquipmentSlot,
        power: i32,
        value: u32,
        stat_bonuses: HashMap<StatType, i32>,
        level_requirement: u32,
    ) -> Self {
        Equipment {
            name,
            description,
            equipment_type,
            slot,
            power,
            value,
            stat_bonuses,
            level_requirement,
        }
    }

    pub fn generate_random(level: u32) -> Self {
        let mut rng = rand::thread_rng();

        // Randomly determine slot
        let slot = match rng.gen_range(0..6) {
            0 => EquipmentSlot::Head,
            1 => EquipmentSlot::Chest,
            2 => EquipmentSlot::Hands,
            3 => EquipmentSlot::Feet,
            4 => EquipmentSlot::Weapon,
            _ => EquipmentSlot::Shield,
        };

        let equipment_type = match slot {
            EquipmentSlot::Weapon => EquipmentType::Weapon,
            _ => EquipmentType::Armor,
        };

        // Generate name based on type and level
        let prefix = match level {
            1..=3 => match rng.gen_range(0..5) {
                0 => "Rusty",
                1 => "Worn",
                2 => "Simple",
                3 => "Basic",
                _ => "Crude",
            },
            4..=7 => match rng.gen_range(0..5) {
                0 => "Sturdy",
                1 => "Reliable",
                2 => "Balanced",
                3 => "Reinforced",
                _ => "Polished",
            },
            8..=12 => match rng.gen_range(0..5) {
                0 => "Superior",
                1 => "Enchanted",
                2 => "Gleaming",
                3 => "Magical",
                _ => "Exquisite",
            },
            13..=17 => match rng.gen_range(0..5) {
                0 => "Majestic",
                1 => "Arcane",
                2 => "Mystical",
                3 => "Blessed",
                _ => "Legendary",
            },
            _ => match rng.gen_range(0..5) {
                0 => "Ancient",
                1 => "Divine",
                2 => "Godly",
                3 => "Celestial",
                _ => "Mythical",
            },
        };

        let item_type = match slot {
            EquipmentSlot::Head => match rng.gen_range(0..3) {
                0 => "Helm",
                1 => "Cap",
                _ => "Hood",
            },
            EquipmentSlot::Chest => match rng.gen_range(0..3) {
                0 => "Breastplate",
                1 => "Armor",
                _ => "Robe",
            },
            EquipmentSlot::Hands => match rng.gen_range(0..3) {
                0 => "Gauntlets",
                1 => "Gloves",
                _ => "Bracers",
            },
            EquipmentSlot::Feet => match rng.gen_range(0..3) {
                0 => "Boots",
                1 => "Greaves",
                _ => "Sabatons",
            },
            EquipmentSlot::Weapon => match rng.gen_range(0..5) {
                0 => "Sword",
                1 => "Axe",
                2 => "Mace",
                3 => "Staff",
                _ => "Bow",
            },
            EquipmentSlot::Shield => match rng.gen_range(0..3) {
                0 => "Shield",
                1 => "Buckler",
                _ => "Barrier",
            },
        };

        let name = format!("{} {}", prefix, item_type);

        // Generate power based on level
        let power_base = 2 + level;
        let power_variation = rng.gen_range(0..=3);
        let power = power_base + power_variation;

        // Generate value based on level and power
        let value = (level * 10 + power as u32 * 5) * rng.gen_range(1..=3);

        // Generate stat bonuses
        let mut stat_bonuses = HashMap::new();
        let num_bonuses = rng.gen_range(1..=2).min((level / 3 + 1) as usize);

        let stat_types = [
            StatType::Strength,
            StatType::Intelligence,
            StatType::Dexterity,
            StatType::Constitution,
            StatType::Wisdom,
        ];

        for _ in 0..num_bonuses {
            let stat = stat_types[rng.gen_range(0..stat_types.len())];
            let bonus = rng.gen_range(1..=(level / 2 + 1));
            stat_bonuses.insert(stat, bonus as i32);
        }

        // Level requirement is usually level - 2, but never below 1
        let level_requirement = (level.saturating_sub(2)).max(1);

        // Generate description
        let description = match equipment_type {
            EquipmentType::Weapon => format!(
                "A {} that deals {} damage. Required level: {}",
                item_type.to_lowercase(),
                power,
                level_requirement
            ),
            EquipmentType::Armor => format!(
                "A piece of {} that provides {} protection. Required level: {}",
                item_type.to_lowercase(),
                power,
                level_requirement
            ),
        };

        Equipment {
            name,
            description,
            equipment_type,
            slot,
            power: power as i32,
            value,
            stat_bonuses,
            level_requirement,
        }
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Lvl {}): {} [Power: {}]",
            self.name, self.level_requirement, self.slot, self.power
        )
    }
}
