use crate::item::{Equipment, EquipmentSlot, Item};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
                // Adjust indices for items after the removed one
                if *eq_idx > index {
                    // This would need to be updated, but we're returning an error above
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

            // Check level requirement
            // In a real implementation, you'd need to pass the player's level
            // For now, we'll skip this check

            // Unequip current item in that slot if any
            if let Some(Some(current_equipped_idx)) = self.equipped.get(&slot) {
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
