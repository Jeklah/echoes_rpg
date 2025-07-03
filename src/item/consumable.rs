use crate::character::Player;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumableType {
    HealthPotion,
    ManaPotion,
    Antidote,
    StrengthElixir,
    IntelligenceElixir,
    DexterityElixir,
    ConstitutionElixir,
    WisdomElixir,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumable {
    pub name: String,
    pub description: String,
    pub consumable_type: ConsumableType,
    pub potency: i32,
    pub value: u32,
}

impl Consumable {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        description: String,
        consumable_type: ConsumableType,
        potency: i32,
        value: u32,
    ) -> Self {
        Consumable {
            name,
            description,
            consumable_type,
            potency,
            value,
        }
    }

    pub fn use_effect(&self, player: &mut Player) -> String {
        match self.consumable_type {
            ConsumableType::HealthPotion => {
                let heal_amount = self.potency;
                player.heal(heal_amount);
                format!("You restored {} health points", heal_amount)
            }
            ConsumableType::ManaPotion => {
                let mana_amount = self.potency;
                player.mana = (player.mana + mana_amount).min(player.max_mana);
                format!("You restored {} mana points", mana_amount)
            }
            ConsumableType::Antidote => {
                // In a more complex game, this would remove poison status
                "You feel purified".to_string()
            }
            ConsumableType::StrengthElixir => {
                use crate::character::StatType;
                player.stats.modify_stat(StatType::Strength, 1);
                "Your strength increases permanently by 1".to_string()
            }
            ConsumableType::IntelligenceElixir => {
                use crate::character::StatType;
                player.stats.modify_stat(StatType::Intelligence, 1);
                "Your intelligence increases permanently by 1".to_string()
            }
            ConsumableType::DexterityElixir => {
                use crate::character::StatType;
                player.stats.modify_stat(StatType::Dexterity, 1);
                "Your dexterity increases permanently by 1".to_string()
            }
            ConsumableType::ConstitutionElixir => {
                use crate::character::StatType;
                player.stats.modify_stat(StatType::Constitution, 1);
                player.max_health = 10 + (player.stats.get_stat(StatType::Constitution) * 5);
                "Your constitution increases permanently by 1".to_string()
            }
            ConsumableType::WisdomElixir => {
                use crate::character::StatType;
                player.stats.modify_stat(StatType::Wisdom, 1);
                player.max_mana = 5 + (player.stats.get_stat(StatType::Wisdom) * 3);
                "Your wisdom increases permanently by 1".to_string()
            }
        }
    }

    pub fn generate_random(level: u32) -> Self {
        let mut rng = rand::thread_rng();

        // Choose consumable type
        let consumable_type = match rng.gen_range(0..8) {
            0 => ConsumableType::HealthPotion,
            1 => ConsumableType::ManaPotion,
            2 => ConsumableType::Antidote,
            3 => ConsumableType::StrengthElixir,
            4 => ConsumableType::IntelligenceElixir,
            5 => ConsumableType::DexterityElixir,
            6 => ConsumableType::ConstitutionElixir,
            _ => ConsumableType::WisdomElixir,
        };

        // Generate potency based on level
        let potency = match consumable_type {
            ConsumableType::HealthPotion | ConsumableType::ManaPotion => {
                20 + level as i32 * 10 + rng.gen_range(0..10)
            }
            ConsumableType::Antidote => 1, // Antidotes don't have variable potency
            _ => 1,                        // Stat elixirs always give +1 to the stat
        };

        // Set name and description based on type
        let (name, description) = match consumable_type {
            ConsumableType::HealthPotion => {
                let quality = if potency < 50 {
                    "Minor"
                } else if potency < 100 {
                    "Regular"
                } else if potency < 150 {
                    "Greater"
                } else {
                    "Superior"
                };

                (
                    format!("{} Health Potion", quality),
                    format!("Restores {} health points when consumed", potency),
                )
            }
            ConsumableType::ManaPotion => {
                let quality = if potency < 50 {
                    "Minor"
                } else if potency < 100 {
                    "Regular"
                } else if potency < 150 {
                    "Greater"
                } else {
                    "Superior"
                };

                (
                    format!("{} Mana Potion", quality),
                    format!("Restores {} mana points when consumed", potency),
                )
            }
            ConsumableType::Antidote => ("Antidote".to_string(), "Cures poison status".to_string()),
            ConsumableType::StrengthElixir => (
                "Elixir of Strength".to_string(),
                "Permanently increases Strength by 1".to_string(),
            ),
            ConsumableType::IntelligenceElixir => (
                "Elixir of Intelligence".to_string(),
                "Permanently increases Intelligence by 1".to_string(),
            ),
            ConsumableType::DexterityElixir => (
                "Elixir of Dexterity".to_string(),
                "Permanently increases Dexterity by 1".to_string(),
            ),
            ConsumableType::ConstitutionElixir => (
                "Elixir of Constitution".to_string(),
                "Permanently increases Constitution by 1".to_string(),
            ),
            ConsumableType::WisdomElixir => (
                "Elixir of Wisdom".to_string(),
                "Permanently increases Wisdom by 1".to_string(),
            ),
        };

        // Generate value based on type and potency
        let value = match consumable_type {
            ConsumableType::HealthPotion | ConsumableType::ManaPotion => potency as u32 / 2,
            ConsumableType::Antidote => 30,
            _ => 100 + level * 20, // Stat elixirs are valuable
        };

        Consumable {
            name,
            description,
            consumable_type,
            potency,
            value,
        }
    }
}

impl fmt::Display for Consumable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
