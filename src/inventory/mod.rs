//! Inventory Management Module
//!
//! This module provides a unified interface for inventory operations
//! that can be used by both GUI and terminal interfaces.
//!
//! # Example Usage
//!
//! ## Display Inventory
//! ```rust
//! use crate::inventory::InventoryScreen;
//!
//! // Get display data for rendering
//! let display_data = InventoryScreen::get_display_data(player);
//! println!("{}", display_data.gold_display());
//!
//! if !display_data.is_empty {
//!     for (display_index, item) in display_data.items_with_display_index() {
//!         let item_line = InventoryScreen::format_item_line(item, display_index);
//!         println!("{}", item_line);
//!     }
//! }
//! ```
//!
//! ## Handle User Input
//! ```rust
//! use crate::inventory::InventoryScreen;
//!
//! let input = '1'; // User pressed '1'
//! let action = InventoryScreen::process_input(input);
//!
//! match InventoryScreen::handle_action(player, action) {
//!     Some(result) => println!("{}", result.message),
//!     None => println!("Exiting inventory..."),
//! }
//! ```
//!
//! ## Use `InventoryManager` for Advanced Operations
//! ```rust
//! use crate::inventory::InventoryManager;
//!
//! // Check if player can equip an item
//! if InventoryManager::can_equip(player, 0) {
//!     let result = InventoryManager::use_item(player, 0);
//!     println!("{}", result.message);
//! }
//!
//! // Get equipped items
//! let equipped = InventoryManager::get_equipped_items(player);
//! for (slot, item) in equipped {
//!     println!("{:?}: {}", slot, item.name);
//! }
//! ```

pub mod manager;

pub use manager::InventoryManager;

/// Information about an inventory item for display purposes
#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub name: String,

    pub is_equipped: bool,
}

/// Action result from inventory operations
#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
}

impl ActionResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
        }
    }

    pub fn success_consumed(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
        }
    }
}
