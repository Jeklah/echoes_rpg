pub mod consumable;
pub mod equipment;

// Re-exports
pub use consumable::Consumable;
pub use equipment::{Equipment, EquipmentSlot};

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Equipment(Equipment),
    Consumable(Consumable),
    Quest {
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
            Item::Quest { name, .. } => name,
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

    /// Generate an item specifically for a chest with guaranteed quality
    /// This helps ensure consistent behavior across all platforms
    pub fn generate_for_chest(level: u32) -> Self {
        let mut rng = rand::thread_rng();

        // For chests, slightly bias toward equipment (80%)
        // and ensure higher quality items
        let effective_level = level + 1; // Chests always contain better items

        if rng.gen_bool(0.8) {
            // Equipment with boosted stats for chests
            Item::Equipment(Equipment::generate_random(effective_level))
        } else {
            // Valuable consumables for chests
            Item::Consumable(Consumable::generate_random(effective_level))
        }
    }
}
