//! Inventory Manager - Core inventory data structure and operations

use super::{ActionResult, InventoryError, InventoryResult, ItemInfo, ItemType};
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

    pub fn remove_item(&mut self, index: usize) -> Result<Item, String> {
        if index >= self.items.len() {
            return Err("Invalid item index".to_string());
        }

        // Check if the item is equipped
        for (_, equipped_index) in self.equipped.iter() {
            if let Some(eq_idx) = equipped_index {
                if *eq_idx == index {
                    return Err("Cannot remove equipped item".to_string());
                }
            }
        }

        // Remove the item and return it
        let item = self.items.remove(index);

        // Update equipped indices after removal
        for equipped_index in self.equipped.values_mut() {
            if let Some(idx) = equipped_index {
                if *idx > index {
                    *idx -= 1;
                }
            }
        }

        Ok(item)
    }

    pub fn get_item(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
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

    pub fn unequip_item(&mut self, slot: EquipmentSlot) -> Result<(), String> {
        if let Some(Some(_)) = self.equipped.get(&slot) {
            // Mark as unequipped
            self.equipped.insert(slot, None);
            Ok(())
        } else {
            Err("No item equipped in that slot".to_string())
        }
    }

    pub fn get_equipped(&self, slot: EquipmentSlot) -> Option<&Item> {
        if let Some(Some(index)) = self.equipped.get(&slot) {
            self.items.get(*index)
        } else {
            None
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

    pub fn use_item(
        &mut self,
        index: usize,
        player: &mut crate::character::Player,
    ) -> Result<String, String> {
        if index >= self.items.len() {
            return Err("Invalid item index".to_string());
        }

        match &self.items[index] {
            Item::Consumable(consumable) => {
                let result = consumable.use_effect(player);
                // Remove the item after use
                self.items.remove(index);

                // Update equipped indices after removal
                for equipped_index in self.equipped.values_mut() {
                    if let Some(idx) = equipped_index {
                        if *idx > index {
                            *idx -= 1;
                        }
                    }
                }

                Ok(result)
            }
            _ => Err("This item cannot be used".to_string()),
        }
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
                    index,
                    name: item.name().to_string(),
                    description: item.description().to_string(),
                    is_equipped,
                    item_type: ItemType::from(item),
                    value: item.value(),
                }
            })
            .collect()
    }

    /// Get player's gold
    pub fn get_gold(player: &Player) -> u32 {
        player.gold
    }

    /// Check if inventory is empty
    pub fn is_empty(player: &Player) -> bool {
        player.inventory.items.is_empty()
    }

    /// Get inventory size information
    pub fn get_size_info(player: &Player) -> (usize, usize) {
        (player.inventory.items.len(), player.inventory.max_size)
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
            Item::QuestItem { .. } => ActionResult::failure("Quest items cannot be used"),
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
        for equipped_index in player.inventory.equipped.values_mut() {
            if let Some(idx) = equipped_index {
                if *idx > index {
                    *idx -= 1;
                }
            }
        }

        // Apply effect and get message
        let result = consumable.use_effect(player);
        ActionResult::success_consumed(result)
    }

    /// Get equipped item in a specific slot
    pub fn get_equipped_item(player: &Player, slot: EquipmentSlot) -> Option<ItemInfo> {
        if let Some(Some(index)) = player.inventory.equipped.get(&slot) {
            if let Some(item) = player.inventory.items.get(*index) {
                return Some(ItemInfo {
                    index: *index,
                    name: item.name().to_string(),
                    description: item.description().to_string(),
                    is_equipped: true,
                    item_type: ItemType::from(item),
                    value: item.value(),
                });
            }
        }
        None
    }

    /// Get all equipped items
    pub fn get_equipped_items(player: &Player) -> Vec<(EquipmentSlot, ItemInfo)> {
        let mut equipped = Vec::new();

        for (slot, maybe_index) in &player.inventory.equipped {
            if let Some(index) = maybe_index {
                if let Some(item) = player.inventory.items.get(*index) {
                    equipped.push((
                        *slot,
                        ItemInfo {
                            index: *index,
                            name: item.name().to_string(),
                            description: item.description().to_string(),
                            is_equipped: true,
                            item_type: ItemType::from(item),
                            value: item.value(),
                        },
                    ));
                }
            }
        }

        equipped
    }

    /// Unequip an item from a specific slot
    pub fn unequip_item(player: &mut Player, slot: EquipmentSlot) -> ActionResult {
        match player.inventory.unequip_item(slot) {
            Ok(()) => ActionResult::success(format!("Unequipped item from {:?} slot", slot)),
            Err(err) => ActionResult::failure(err),
        }
    }

    /// Add an item to inventory
    pub fn add_item(player: &mut Player, item: Item) -> ActionResult {
        match player.inventory.add_item(item) {
            Ok(()) => ActionResult::success("Item added to inventory"),
            Err(err) => ActionResult::failure(err),
        }
    }

    /// Remove an item from inventory by index
    pub fn remove_item(player: &mut Player, index: usize) -> InventoryResult<Item> {
        player
            .inventory
            .remove_item(index)
            .map_err(|_| InventoryError::InvalidIndex)
    }

    /// Check if an item can be equipped
    pub fn can_equip(player: &Player, index: usize) -> bool {
        if let Some(Item::Equipment(_)) = player.inventory.items.get(index) {
            true
        } else {
            false
        }
    }

    /// Check if an item can be used
    pub fn can_use(player: &Player, index: usize) -> bool {
        if let Some(item) = player.inventory.items.get(index) {
            match item {
                Item::Consumable(_) => true,
                Item::Equipment(_) => true,
                Item::QuestItem { .. } => false,
            }
        } else {
            false
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

    /// Check if inventory has space
    pub fn has_space(player: &Player) -> bool {
        player.inventory.items.len() < player.inventory.max_size
    }

    /// Get available space
    pub fn get_available_space(player: &Player) -> usize {
        player
            .inventory
            .max_size
            .saturating_sub(player.inventory.items.len())
    }
}
