pub mod consumable;
pub mod equipment;
pub mod inventory;

// Re-exports
pub use consumable::{Consumable, ConsumableType};
pub use equipment::{Equipment, EquipmentSlot};
pub use inventory::Inventory;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Equipment(Equipment),
    Consumable(Consumable),
    QuestItem {
        id: String,
        name: String,
        description: String,
    },
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Equipment(equipment) => &equipment.name,
            Item::Consumable(consumable) => &consumable.name,
            Item::QuestItem { name, .. } => name,
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match self {
            Item::Equipment(equipment) => &equipment.description,
            Item::Consumable(consumable) => &consumable.description,
            Item::QuestItem { description, .. } => description,
        }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        match self {
            Item::Equipment(equipment) => equipment.value,
            Item::Consumable(consumable) => consumable.value,
            Item::QuestItem { .. } => 0, // Quest items have no sale value
        }
    }

    // Generate a random item with appropriate stats for the given level
    pub fn generate_random(level: u32) -> Self {
        let mut rng = rand::thread_rng();

        // Determine item type (70% equipment, 30% consumable)
        if rng.gen_bool(0.7) {
            // Generate equipment
            Item::Equipment(Equipment::generate_random(level))
        } else {
            // Generate consumable
            Item::Consumable(Consumable::generate_random(level))
        }
    }
}
