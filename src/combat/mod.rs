use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::character::Player;
use crate::inventory::InventoryManager;
use crate::item::Item;
use crate::world::Enemy;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CombatAction {
    Attack,
    UseAbility(usize),
    UseItem(usize),
    Flee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatResult {
    pub player_damage_dealt: i32,
    pub enemy_damage_dealt: i32,
    pub experience_gained: u32,
    pub gold_gained: u32,
    pub items_gained: Vec<Item>,
    pub player_level_up: bool,
    pub enemy_defeated: bool,
    pub player_fled: bool,
    pub messages: Vec<String>,
}

impl CombatResult {
    pub fn new() -> Self {
        CombatResult {
            player_damage_dealt: 0,
            enemy_damage_dealt: 0,
            experience_gained: 0,
            gold_gained: 0,
            items_gained: Vec::new(),
            player_level_up: false,
            enemy_defeated: false,
            player_fled: false,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }
}

pub fn process_combat_turn(
    player: &mut Player,
    enemy: &mut Enemy,
    action: CombatAction,
) -> CombatResult {
    let mut result = CombatResult::new();
    let mut rng = rand::thread_rng();

    match action {
        CombatAction::Attack => {
            // Player attacks first
            let player_damage = player.attack_damage();
            let damage_dealt = enemy.take_damage(player_damage);
            result.player_damage_dealt = damage_dealt;
            result.add_message(format!(
                "You attack the {} for {} damage!",
                enemy.name, damage_dealt
            ));

            if !enemy.is_alive() {
                handle_enemy_defeat(player, enemy, &mut result);
                return result;
            }

            // Enemy counterattack
            let enemy_damage = enemy.attack_damage();
            let damage_taken = player.take_damage(enemy_damage);
            result.enemy_damage_dealt = damage_taken;
            result.add_message(format!(
                "The {} hits you for {} damage!",
                enemy.name, damage_taken
            ));
        }
        CombatAction::UseAbility(ability_index) => {
            // Player uses ability
            match player.use_ability(ability_index) {
                Ok(message) => {
                    // Some abilities might do damage to the enemy
                    let message_clone = message.clone();
                    result.add_message(message_clone);

                    if message.contains("damage") {
                        // Extract the damage value from the message
                        if let Some(damage_str) = message.split("damage").next() {
                            if let Some(damage_value) = damage_str
                                .split_whitespace()
                                .last()
                                .and_then(|s| s.parse::<i32>().ok())
                            {
                                let damage_dealt = enemy.take_damage(damage_value);
                                result.player_damage_dealt = damage_dealt;

                                if !enemy.is_alive() {
                                    handle_enemy_defeat(player, enemy, &mut result);
                                    return result;
                                }
                            }
                        }
                    }

                    // Enemy counterattack
                    let enemy_damage = enemy.attack_damage();
                    let damage_taken = player.take_damage(enemy_damage);
                    result.enemy_damage_dealt = damage_taken;
                    result.add_message(format!(
                        "The {} hits you for {} damage!",
                        enemy.name, damage_taken
                    ));
                }
                Err(err) => {
                    result.add_message(err);
                }
            }
        }
        CombatAction::UseItem(item_index) => {
            // Player uses an item - get a clone of the item first
            let item_message = if item_index < InventoryManager::get_item_count(player) {
                let result = InventoryManager::use_item(player, item_index);
                result.message
            } else {
                "Invalid item or item cannot be used.".to_string()
            };

            // Add message about item use
            result.add_message(item_message);

            // Enemy counterattack
            let enemy_damage = enemy.attack_damage();
            let damage_taken = player.take_damage(enemy_damage);
            result.enemy_damage_dealt = damage_taken;
            result.add_message(format!(
                "The {} hits you for {} damage!",
                enemy.name, damage_taken
            ));
        }
        CombatAction::Flee => {
            // Player attempts to flee
            let flee_chance = 0.3 + (player.stats.dexterity as f32 * 0.03);

            if rng.gen_bool(f64::from(flee_chance)) {
                result.player_fled = true;
                result.add_message("You successfully fled from combat!".to_string());
            } else {
                result.add_message("You failed to escape!".to_string());

                // Enemy gets a free attack
                let enemy_damage = enemy.attack_damage();
                let damage_taken = player.take_damage(enemy_damage);
                result.enemy_damage_dealt = damage_taken;
                result.add_message(format!(
                    "The {} hits you for {} damage as you try to escape!",
                    enemy.name, damage_taken
                ));
            }
        }
    }

    result
}

fn handle_enemy_defeat(player: &mut Player, enemy: &Enemy, result: &mut CombatResult) {
    // Get enemy drops
    let (exp, gold, possible_item) = enemy.get_drops();

    // Add experience and check for level up
    result.experience_gained = exp;
    result.gold_gained = gold;
    let leveled_up = player.gain_experience(exp);

    // Add rewards to player
    player.gold += gold;

    if let Some(item) = possible_item {
        // Try to add item to inventory
        let add_result = InventoryManager::add_item(player, item.clone());
        if add_result.success {
            let item_name = item.name().to_string();
            result.items_gained.push(item.clone());
            result.add_message(format!("You found: {item_name}"));
        } else {
            result.add_message("You found an item but your inventory is full!".to_string());
        }
    }

    // Record results
    result.enemy_defeated = true;
    result.player_level_up = leveled_up;

    result.add_message(format!("You defeated the {}!", enemy.name));
    result.add_message(format!("You gained {exp} experience and {gold} gold."));

    if leveled_up {
        result.add_message(format!("You leveled up to level {}!", player.level));
    }
}
