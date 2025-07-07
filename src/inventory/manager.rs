//! Inventory Manager - Core inventory data structure and operations

use super::{ActionResult, ItemInfo};
use crate::character::Player;
use crate::item::{Equipment, EquipmentSlot, Item};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core inventory data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub max_size: usize,
    pub equipped: HashMap<EquipmentSlot, Option<usize>>, // Stores index to items vec
}

impl Inventory {
    pub fn new(max_size: usize) -> Self {
        let mut equipped = HashMap::new();
        for slot in EquipmentSlot::iter() {
            equipped.insert(slot, None);
        }

        Inventory {
            items: Vec::new(),
            max_size,
            equipped,
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.items.len() >= self.max_size {
            return Err("Inventory is full".to_string());
        }

        self.items.push(item);
        Ok(())
    }

    pub fn equip_item(&mut self, index: usize) -> Result<(), String> {
        if index >= self.items.len() {
            return Err("Invalid item index".to_string());
        }

        // Check if item is equipment
        if let Item::Equipment(ref equipment) = self.items[index] {
            let slot = equipment.slot;

            // Unequip current item in that slot if any
            if let Some(Some(_current_equipped_idx)) = self.equipped.get(&slot) {
                // Mark as unequipped
                self.equipped.insert(slot, None);
            }

            // Equip new item
            self.equipped.insert(slot, Some(index));

            Ok(())
        } else {
            Err("This item cannot be equipped".to_string())
        }
    }

    pub fn get_equipped_weapon(&self) -> Option<&Equipment> {
        if let Some(Some(index)) = self.equipped.get(&EquipmentSlot::Weapon) {
            if let Some(Item::Equipment(equipment)) = self.items.get(*index) {
                return Some(equipment);
            }
        }
        None
    }

    pub fn get_total_armor_defense(&self) -> i32 {
        let mut total = 0;

        // Check each armor slot
        for slot in [
            EquipmentSlot::Head,
            EquipmentSlot::Chest,
            EquipmentSlot::Hands,
            EquipmentSlot::Feet,
            EquipmentSlot::Shield,
        ] {
            if let Some(Some(index)) = self.equipped.get(&slot) {
                if let Some(Item::Equipment(equipment)) = self.items.get(*index) {
                    total += equipment.power;
                }
            }
        }

        total
    }
}

/// High-level inventory manager that provides a clean interface
/// for inventory operations while maintaining existing behavior
pub struct InventoryManager;

impl InventoryManager {
    /// Get all items in the inventory with display information
    pub fn get_items(player: &Player) -> Vec<ItemInfo> {
        player
            .inventory
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let is_equipped = match item {
                    Item::Equipment(equipment) => {
                        if let Some(Some(equipped_idx)) =
                            player.inventory.equipped.get(&equipment.slot)
                        {
                            *equipped_idx == index
                        } else {
                            false
                        }
                    }
                    _ => false,
                };

                ItemInfo {
                    name: item.name().to_string(),
                    is_equipped,
                }
            })
            .collect()
    }

    /// Check if inventory is empty
    pub fn is_empty(player: &Player) -> bool {
        player.inventory.items.is_empty()
    }

    /// Use or equip an item by index
    pub fn use_item(player: &mut Player, index: usize) -> ActionResult {
        if index >= player.inventory.items.len() {
            return ActionResult::failure("Invalid item index");
        }

        // Clone the item to avoid borrowing issues
        let item = player.inventory.items[index].clone();

        match item {
            Item::Equipment(equipment) => Self::equip_item(player, index, equipment),
            Item::Consumable(consumable) => Self::use_consumable(player, index, consumable),
            Item::Quest { .. } => ActionResult::failure("Quest items cannot be used"),
        }
    }

    /// Equip an equipment item
    fn equip_item(
        player: &mut Player,
        index: usize,
        equipment: crate::item::Equipment,
    ) -> ActionResult {
        match player.inventory.equip_item(index) {
            Ok(()) => ActionResult::success(format!("Equipped {}", equipment.name)),
            Err(err) => ActionResult::failure(err),
        }
    }

    /// Use a consumable item
    fn use_consumable(
        player: &mut Player,
        index: usize,
        consumable: crate::item::Consumable,
    ) -> ActionResult {
        // Remove from inventory first
        player.inventory.items.remove(index);

        // Update equipped indices after removal
        for idx in player.inventory.equipped.values_mut().flatten() {
            if *idx > index {
                *idx -= 1;
            }
        }

        // Apply effect and get message
        let result = consumable.use_effect(player);
        ActionResult::success_consumed(result)
    }

    /// Get equipped item in a specific slot
    #[cfg(all(feature = "gui", target_os = "windows"))]
    pub fn get_equipped_item(player: &Player, slot: EquipmentSlot) -> Option<ItemInfo> {
        if let Some(Some(index)) = player.inventory.equipped.get(&slot) {
            if let Some(item) = player.inventory.items.get(*index) {
                return Some(ItemInfo {
                    name: item.name().to_string(),
                    is_equipped: true,
                });
            }
        }
        None
    }

    /// Add an item to inventory
    pub fn add_item(player: &mut Player, item: Item) -> ActionResult {
        match player.inventory.add_item(item) {
            Ok(()) => ActionResult::success("Item added to inventory"),
            Err(err) => ActionResult::failure(err),
        }
    }

    /// Get item by index
    pub fn get_item(player: &Player, index: usize) -> Option<&Item> {
        player.inventory.items.get(index)
    }

    /// Get item count
    pub fn get_item_count(player: &Player) -> usize {
        player.inventory.items.len()
    }
}
