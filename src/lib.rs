#![cfg_attr(target_arch = "wasm32", no_main)]

mod character;
mod inventory;
mod item;
mod world;

// Combat module is safe for WASM (no terminal dependencies)
mod combat;

// Only include terminal-specific modules for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
mod platform;
#[cfg(not(target_arch = "wasm32"))]
mod ui;

// Game module has conditional compilation internally
mod game;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;

// Re-export key types for WASM usage
#[cfg(target_arch = "wasm32")]
pub use game::{Game, GameState};

#[cfg(target_arch = "wasm32")]
pub use character::Player;
