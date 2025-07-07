//! Inventory Screen Module
//!
//! Provides unified screen rendering and input handling for inventory operations
//! that can be used by both GUI and terminal interfaces.

use super::{ActionResult, InventoryManager, ItemInfo, ItemType};
use crate::character::Player;

/// Actions that can be performed on the inventory screen
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryAction {
    /// Use/equip item by index (1-based for display)
    UseItem(usize),
    /// Exit the inventory screen
    Exit,
    /// Invalid action
    Invalid,
}

/// Inventory screen controller that handles display and interaction
pub struct InventoryScreen;

impl InventoryScreen {
    /// Process input character and return the corresponding action
    pub fn process_input(input: char) -> InventoryAction {
        match input {
            'e' | 'E' => InventoryAction::Exit,
            c if c.is_ascii_digit() && c != '0' => {
                let digit = c.to_digit(10).unwrap() as usize;
                if (1..=9).contains(&digit) {
                    InventoryAction::UseItem(digit)
                } else {
                    InventoryAction::Invalid
                }
            }
            _ => InventoryAction::Invalid,
        }
    }

    /// Handle an inventory action and return the result
    pub fn handle_action(player: &mut Player, action: InventoryAction) -> Option<ActionResult> {
        match action {
            InventoryAction::UseItem(display_index) => {
                let array_index = display_index - 1; // Convert from 1-based to 0-based
                if array_index < InventoryManager::get_item_count(player) {
                    Some(InventoryManager::use_item(player, array_index))
                } else {
                    Some(ActionResult::failure("Invalid item number"))
                }
            }
            InventoryAction::Exit => None, // Signal to exit inventory screen
            InventoryAction::Invalid => Some(ActionResult::failure("Invalid input")),
        }
    }

    /// Get formatted inventory data for display
    pub fn get_display_data(player: &Player) -> InventoryDisplayData {
        let items = InventoryManager::get_items(player);
        let gold = InventoryManager::get_gold(player);
        let is_empty = InventoryManager::is_empty(player);
        let (current_size, max_size) = InventoryManager::get_size_info(player);

        InventoryDisplayData {
            gold,
            items,
            is_empty,
            current_size,
            max_size,
        }
    }

    /// Format an item for display with its number
    pub fn format_item_line(item: &ItemInfo, display_index: usize) -> String {
        let equipped_marker = if item.is_equipped { " [E]" } else { "" };
        format!("{}. {}{}", display_index, item.name, equipped_marker)
    }

    /// Get help text for inventory screen
    pub fn get_help_text() -> &'static str {
        "Press number key to use/equip item, E to exit..."
    }

    /// Get title for inventory screen
    pub fn get_title() -> &'static str {
        "Inventory"
    }

    /// Check if an item index is valid for the current inventory
    pub fn is_valid_item_index(player: &Player, display_index: usize) -> bool {
        if display_index == 0 {
            return false;
        }
        let array_index = display_index - 1;
        array_index < InventoryManager::get_item_count(player)
    }

    /// Get item type description for display
    pub fn get_item_type_description(item_type: &ItemType) -> &'static str {
        match item_type {
            ItemType::Equipment => "Equipment",
            ItemType::Consumable => "Consumable",
            ItemType::Quest => "Quest Item",
        }
    }

    /// Format gold display
    pub fn format_gold(gold: u32) -> String {
        format!("Gold: {gold}")
    }

    /// Format inventory size display
    pub fn format_inventory_size(current: usize, max: usize) -> String {
        format!("Items: {current}/{max}")
    }

    /// Get empty inventory message
    pub fn get_empty_message() -> &'static str {
        "Your inventory is empty."
    }

    /// Get items section header
    pub fn get_items_header() -> &'static str {
        "Items:"
    }

    /// Get items section separator
    pub fn get_items_separator() -> &'static str {
        "------"
    }
}

/// Data structure containing all information needed to display the inventory
#[derive(Debug, Clone)]
pub struct InventoryDisplayData {
    pub gold: u32,
    pub items: Vec<ItemInfo>,
    pub is_empty: bool,
    pub current_size: usize,
    pub max_size: usize,
}

impl InventoryDisplayData {
    /// Get formatted gold string
    pub fn gold_display(&self) -> String {
        InventoryScreen::format_gold(self.gold)
    }

    /// Get formatted size string
    pub fn size_display(&self) -> String {
        InventoryScreen::format_inventory_size(self.current_size, self.max_size)
    }

    /// Get iterator over items with display indices (1-based)
    pub fn items_with_display_index(&self) -> impl Iterator<Item = (usize, &ItemInfo)> {
        self.items.iter().enumerate().map(|(i, item)| (i + 1, item))
    }

    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.current_size >= self.max_size
    }

    /// Get available space
    pub fn available_space(&self) -> usize {
        self.max_size.saturating_sub(self.current_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        assert_eq!(
            InventoryScreen::process_input('1'),
            InventoryAction::UseItem(1)
        );
        assert_eq!(
            InventoryScreen::process_input('9'),
            InventoryAction::UseItem(9)
        );
        assert_eq!(InventoryScreen::process_input('e'), InventoryAction::Exit);
        assert_eq!(InventoryScreen::process_input('E'), InventoryAction::Exit);
        assert_eq!(
            InventoryScreen::process_input('0'),
            InventoryAction::Invalid
        );
        assert_eq!(
            InventoryScreen::process_input('a'),
            InventoryAction::Invalid
        );
    }

    #[test]
    fn test_format_item_line() {
        let item = ItemInfo {
            index: 0,
            name: "Test Sword".to_string(),
            description: "A test sword".to_string(),
            is_equipped: true,
            item_type: ItemType::Equipment,
            value: 100,
        };

        assert_eq!(
            InventoryScreen::format_item_line(&item, 1),
            "1. Test Sword [E]"
        );

        let item_unequipped = ItemInfo {
            index: 1,
            name: "Health Potion".to_string(),
            description: "Restores health".to_string(),
            is_equipped: false,
            item_type: ItemType::Consumable,
            value: 50,
        };

        assert_eq!(
            InventoryScreen::format_item_line(&item_unequipped, 2),
            "2. Health Potion"
        );
    }

    #[test]
    fn test_valid_item_index() {
        // This would require creating a mock player, which is complex
        // In a real implementation, you might want to add helper methods
        // or use a mocking framework for more comprehensive testing
        assert!(!InventoryScreen::is_valid_item_index(
            &create_empty_player(),
            1
        ));
        assert!(!InventoryScreen::is_valid_item_index(
            &create_empty_player(),
            0
        ));
    }

    // Helper function for testing (simplified)
    fn create_empty_player() -> Player {
        // This is a simplified version - in real tests you'd want proper player creation
        use crate::character::{Class, ClassType, Stats};
        use crate::inventory::Inventory;

        Player {
            name: "Test".to_string(),
            class: Class::new(ClassType::Warrior),
            stats: Stats::new(),
            level: 1,
            experience: 0,
            health: 100,
            max_health: 100,
            mana: 50,
            max_mana: 50,
            inventory: Inventory::new(20),
            gold: 0,
        }
    }
}
