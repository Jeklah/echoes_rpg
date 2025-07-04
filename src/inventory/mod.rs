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
//! ## Use InventoryManager for Advanced Operations
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
pub mod screen;

pub use manager::{Inventory, InventoryManager};

use crate::item::Item;

/// Result type for inventory operations
pub type InventoryResult<T> = Result<T, InventoryError>;

/// Errors that can occur during inventory operations
#[derive(Debug, Clone)]
pub enum InventoryError {
    InvalidIndex,
    InventoryFull,
    CannotEquip(String),
    CannotUse(String),
    ItemNotFound,
    AlreadyEquipped,
    NotEquipped,
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::InvalidIndex => write!(f, "Invalid item index"),
            InventoryError::InventoryFull => write!(f, "Inventory is full"),
            InventoryError::CannotEquip(msg) => write!(f, "Cannot equip item: {}", msg),
            InventoryError::CannotUse(msg) => write!(f, "Cannot use item: {}", msg),
            InventoryError::ItemNotFound => write!(f, "Item not found"),
            InventoryError::AlreadyEquipped => write!(f, "Item is already equipped"),
            InventoryError::NotEquipped => write!(f, "No item equipped in that slot"),
        }
    }
}

impl std::error::Error for InventoryError {}

/// Information about an inventory item for display purposes
#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub index: usize,
    pub name: String,
    pub description: String,
    pub is_equipped: bool,
    pub item_type: ItemType,
    pub value: u32,
}

/// Type of item for categorization
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Equipment,
    Consumable,
    Quest,
}

impl From<&Item> for ItemType {
    fn from(item: &Item) -> Self {
        match item {
            Item::Equipment(_) => ItemType::Equipment,
            Item::Consumable(_) => ItemType::Consumable,
            Item::QuestItem { .. } => ItemType::Quest,
        }
    }
}

/// Action result from inventory operations
#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
    pub item_consumed: bool,
}

impl ActionResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            item_consumed: false,
        }
    }

    pub fn success_consumed(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            item_consumed: true,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            item_consumed: false,
        }
    }
}
