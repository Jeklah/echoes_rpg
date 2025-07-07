//! Fog of War rendering system for consistent visibility handling across different UI implementations.
//!
//! This module provides a centralized way to handle fog of war rendering, ensuring consistent
//! behavior between GUI and terminal versions of the game.

use crate::world::{Level, Position, Tile};
use serde::{Deserialize, Serialize};

/// Represents the visibility state of a tile from the player's perspective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisibilityState {
    /// Tile has never been explored by the player
    Unexplored,
    /// Tile was previously explored but is not currently visible
    ExploredHidden,
    /// Tile is currently visible to the player
    Visible,
}

/// Color information for rendering fog of war
#[derive(Debug, Clone, Copy)]
pub struct FogColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl FogColor {
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const DARK_GREY: Self = Self {
        r: 64,
        g: 64,
        b: 64,
        a: 255,
    };
    pub const GREY: Self = Self {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };

    /// Create a dimmed version of this color
    pub fn dimmed(&self, factor: f32) -> Self {
        Self {
            r: (self.r as f32 * factor).clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * factor).clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * factor).clamp(0.0, 255.0) as u8,
            a: self.a,
        }
    }
}

/// Configuration for fog of war rendering behavior
#[derive(Debug, Clone)]
pub struct FogOfWarConfig {
    /// Whether unexplored areas should be completely hidden (black)
    pub hide_unexplored: bool,
    /// Whether explored but not visible tiles should show dimmed symbols
    pub show_explored_dimmed: bool,
    /// Dimming factor for explored but not visible tiles (0.0 = black, 1.0 = full brightness)
    pub dimming_factor: f32,
    /// Color to use for unexplored areas
    pub unexplored_color: FogColor,
}

impl Default for FogOfWarConfig {
    fn default() -> Self {
        Self {
            hide_unexplored: true,
            show_explored_dimmed: true,
            dimming_factor: 0.5,
            unexplored_color: FogColor::BLACK,
        }
    }
}

/// Result of fog of war processing for a tile
#[derive(Debug, Clone)]
pub struct FogRenderResult {
    /// Character to render for this tile
    pub character: char,
    /// Color to use for rendering
    pub color: Option<FogColor>,
    /// Whether this tile should be rendered at all
    pub should_render: bool,
}

/// Main fog of war processor
pub struct FogOfWar {
    config: FogOfWarConfig,
}

impl FogOfWar {
    /// Create a new fog of war processor with the given configuration
    pub fn new(config: FogOfWarConfig) -> Self {
        Self { config }
    }

    /// Determine the visibility state of a tile
    pub fn get_visibility_state(&self, tile: &Tile) -> VisibilityState {
        if !tile.explored {
            VisibilityState::Unexplored
        } else if !tile.visible {
            VisibilityState::ExploredHidden
        } else {
            VisibilityState::Visible
        }
    }

    /// Process a tile for fog of war rendering
    pub fn process_tile(
        &self,
        tile: &Tile,
        base_character: char,
        base_color: Option<FogColor>,
    ) -> FogRenderResult {
        match self.get_visibility_state(tile) {
            VisibilityState::Unexplored => {
                // Hide walls (#) and floors (.) completely in unexplored areas
                if base_character == '#' || base_character == '.' {
                    FogRenderResult {
                        character: ' ',
                        color: Some(FogColor::BLACK),
                        should_render: false,
                    }
                } else if self.config.hide_unexplored {
                    FogRenderResult {
                        character: ' ',
                        color: Some(self.config.unexplored_color),
                        should_render: true,
                    }
                } else {
                    FogRenderResult {
                        character: ' ',
                        color: Some(self.config.unexplored_color),
                        should_render: false,
                    }
                }
            }
            VisibilityState::ExploredHidden => {
                if self.config.show_explored_dimmed {
                    // Hide walls (#) and floors (.) in explored but not visible areas
                    // This only affects the in-game map rendering
                    if base_character == '#' || base_character == '.' {
                        FogRenderResult {
                            character: ' ',
                            color: Some(self.config.unexplored_color),
                            should_render: false,
                        }
                    } else {
                        let dimmed_color = base_color
                            .map(|c| c.dimmed(self.config.dimming_factor))
                            .or(Some(FogColor::DARK_GREY));

                        FogRenderResult {
                            character: base_character,
                            color: dimmed_color,
                            should_render: true,
                        }
                    }
                } else {
                    FogRenderResult {
                        character: ' ',
                        color: Some(self.config.unexplored_color),
                        should_render: true,
                    }
                }
            }
            VisibilityState::Visible => FogRenderResult {
                character: base_character,
                color: base_color,
                should_render: true,
            },
        }
    }

    /// Process a position on the map, handling entities and tiles
    pub fn process_position(
        &self,
        level: &Level,
        pos: Position,
        player_pos: Position,
    ) -> FogRenderResult {
        // Player is always visible
        if pos == player_pos {
            return FogRenderResult {
                character: '@',
                color: Some(FogColor {
                    r: 255,
                    g: 255,
                    b: 0,
                    a: 255,
                }), // Yellow
                should_render: true,
            };
        }

        // Check if position is within map bounds
        if pos.x < 0 || pos.x >= level.width as i32 || pos.y < 0 || pos.y >= level.height as i32 {
            return FogRenderResult {
                character: ' ',
                color: Some(self.config.unexplored_color),
                should_render: true,
            };
        }

        let tile = &level.tiles[pos.y as usize][pos.x as usize];
        let visibility_state = self.get_visibility_state(tile);

        // For unexplored tiles, return early
        if visibility_state == VisibilityState::Unexplored {
            return self.process_tile(tile, ' ', Some(self.config.unexplored_color));
        }

        // Check for entities (only visible if tile is visible)
        if tile.visible {
            if level.enemies.contains_key(&pos) {
                return FogRenderResult {
                    character: 'E',
                    color: Some(FogColor {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 255,
                    }), // Red
                    should_render: true,
                };
            }

            if level.items.contains_key(&pos) {
                return FogRenderResult {
                    character: '!',
                    color: Some(FogColor {
                        r: 0,
                        g: 255,
                        b: 255,
                        a: 255,
                    }), // Cyan
                    should_render: true,
                };
            }
        }

        // Get base tile rendering info
        let base_character = tile.tile_type.symbol();
        let base_color = self.get_tile_color(&tile.tile_type);

        self.process_tile(tile, base_character, Some(base_color))
    }

    /// Get the base color for a tile type
    fn get_tile_color(&self, tile_type: &crate::world::TileType) -> FogColor {
        match tile_type {
            crate::world::TileType::Wall => FogColor::GREY,
            crate::world::TileType::Floor => FogColor::WHITE,
            crate::world::TileType::Door => FogColor {
                r: 139,
                g: 69,
                b: 19,
                a: 255,
            }, // Brown
            crate::world::TileType::Chest => FogColor {
                r: 255,
                g: 215,
                b: 0,
                a: 255,
            }, // Gold
            crate::world::TileType::StairsDown | crate::world::TileType::StairsUp => {
                FogColor {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 255,
                } // Green
            }
            crate::world::TileType::Exit => FogColor {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            }, // Green
        }
    }
}

/// Utility functions for different rendering backends
impl FogOfWar {
    /// Convert FogColor to egui Color32 for GUI rendering
    #[cfg(feature = "gui")]
    pub fn to_egui_color(color: &FogColor) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a)
    }

    /// Convert FogColor to crossterm Color for terminal rendering
    #[cfg(not(all(feature = "gui", target_os = "windows")))]
    pub fn to_terminal_color(color: &FogColor) -> crossterm::style::Color {
        crossterm::style::Color::Rgb {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{Tile, TileType};

    #[test]
    fn test_visibility_states() {
        let fog = FogOfWar::new(FogOfWarConfig::default());

        // Test unexplored tile
        let unexplored_tile = Tile::new(TileType::Floor);
        assert_eq!(
            fog.get_visibility_state(&unexplored_tile),
            VisibilityState::Unexplored
        );

        // Test explored but not visible tile
        let mut explored_tile = Tile::new(TileType::Floor);
        explored_tile.explored = true;
        explored_tile.visible = false;
        assert_eq!(
            fog.get_visibility_state(&explored_tile),
            VisibilityState::ExploredHidden
        );

        // Test visible tile
        let mut visible_tile = Tile::new(TileType::Floor);
        visible_tile.explored = true;
        visible_tile.visible = true;
        assert_eq!(
            fog.get_visibility_state(&visible_tile),
            VisibilityState::Visible
        );
    }

    #[test]
    fn test_fog_processing() {
        let fog = FogOfWar::new(FogOfWarConfig::default());

        // Test unexplored tile processing
        let unexplored_tile = Tile::new(TileType::Wall);
        let result = fog.process_tile(&unexplored_tile, '#', Some(FogColor::GREY));
        assert_eq!(result.character, ' ');
        assert_eq!(result.color.unwrap().r, 0); // Should be black
        assert!(!result.should_render); // Unexplored walls should not render
    }

    #[test]
    fn test_color_dimming() {
        let bright_color = FogColor::WHITE;
        let dimmed = bright_color.dimmed(0.5);
        assert_eq!(dimmed.r, 127); // Half brightness
        assert_eq!(dimmed.g, 127);
        assert_eq!(dimmed.b, 127);
        assert_eq!(dimmed.a, 255); // Alpha unchanged
    }
}
