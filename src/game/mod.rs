use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io;

use crate::character::{ClassType, Player};
use crate::combat::{CombatAction, CombatResult, process_combat_turn};
use crate::item::Item;
use crate::ui::UI;
use crate::world::{Dungeon, DungeonType, Level, Position, Tile, TileType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    MainMenu,
    Playing,
    Combat(Position),
    Inventory,
    Character,
    GameOver,
    Victory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub player: Player,
    pub dungeons: Vec<Dungeon>,
    pub current_dungeon_index: usize,
    pub game_state: GameState,
}

impl Game {
    pub fn new(player: Player) -> Self {
        // Create initial dungeon
        let first_dungeon = Dungeon::generate_random(player.level);

        Game {
            player,
            dungeons: vec![first_dungeon],
            current_dungeon_index: 0,
            game_state: GameState::MainMenu,
        }
    }

    pub fn current_dungeon(&self) -> &Dungeon {
        &self.dungeons[self.current_dungeon_index]
    }

    pub fn current_dungeon_mut(&mut self) -> &mut Dungeon {
        &mut self.dungeons[self.current_dungeon_index]
    }

    pub fn current_level(&self) -> &Level {
        self.current_dungeon().current_level()
    }

    pub fn current_level_mut(&mut self) -> &mut Level {
        self.current_dungeon_mut().current_level_mut()
    }

    pub fn player_position(&self) -> Position {
        self.current_level().player_position
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) -> bool {
        // Get the current player position
        let current_pos = self.current_level().player_position;
        let new_pos = Position::new(current_pos.x + dx, current_pos.y + dy);

        // Check if the position is valid
        if !self.current_level().is_position_walkable(new_pos) {
            return false;
        }

        // Check for enemies
        if self.current_level().enemies.contains_key(&new_pos) {
            // Start combat
            self.game_state = GameState::Combat(new_pos);
            return true;
        }

        // Check for items on the ground
        if self.current_level().items.contains_key(&new_pos) {
            let item = self.current_level_mut().remove_item_at(&new_pos).unwrap();
            // Try to add to inventory
            if let Err(e) = self.player.inventory.add_item(item.clone()) {
                // Put the item back if inventory is full
                self.current_level_mut().items.insert(new_pos, item);
                return false;
            }
        }

        // Check for special tiles
        if let Some(tile) = self.current_level().get_tile(new_pos.x, new_pos.y) {
            match tile.tile_type {
                TileType::StairsDown => {
                    if let Err(_) = self.current_dungeon_mut().go_to_next_level() {
                        // Can't go further down
                        return false;
                    }
                    return true;
                }
                TileType::StairsUp => {
                    // Implement going up a level if needed
                    return false;
                }
                TileType::Exit => {
                    if self.current_dungeon().is_final_level() {
                        // Victory condition - player reached the exit of the final level
                        self.game_state = GameState::Victory;
                    }
                    return true;
                }
                TileType::Chest => {
                    // Generate loot from chest
                    if let Some(item) = self.current_level().get_item_at(&new_pos) {
                        let item_clone = item.clone();
                        if let Err(_) = self.player.inventory.add_item(item_clone) {
                            // Inventory full, can't loot the chest
                            return false;
                        }
                        // Remove the item and replace the chest with a floor tile
                        self.current_level_mut().remove_item_at(&new_pos);
                        if let Some(tile) =
                            self.current_level_mut().get_tile_mut(new_pos.x, new_pos.y)
                        {
                            *tile = Tile::floor();
                        }
                    }
                    return true;
                }
                _ => {}
            }
        }

        // Move the player
        self.current_level_mut().player_position = new_pos;
        true
    }

    pub fn handle_combat(&mut self, enemy_pos: Position) -> CombatResult {
        // Create a copy of UI
        let ui = UI::new();

        // Clone enemy and player for UI operations
        let mut enemy_clone = self
            .current_level()
            .get_enemy_at(&enemy_pos)
            .unwrap()
            .clone();
        let mut player_clone = self.player.clone();

        // Draw combat screen using the clones
        ui.draw_combat_screen(&player_clone, &enemy_clone).unwrap();

        // Get combat action from user
        let action = ui.handle_combat_action(&player_clone).unwrap();

        // Process combat with the clones
        let result = process_combat_turn(&mut player_clone, &mut enemy_clone, action);

        // Update the real player and enemy with the changes
        self.player = player_clone;

        // Update the enemy in the world if still alive
        if !result.enemy_defeated {
            if let Some(enemy) = self.current_level_mut().get_enemy_at_mut(&enemy_pos) {
                *enemy = enemy_clone;
            }
        }

        // Check if combat is over
        if result.enemy_defeated {
            self.current_level_mut().remove_enemy_at(&enemy_pos);
            self.game_state = GameState::Playing;
        } else if result.player_fled {
            self.game_state = GameState::Playing;
        } else if !self.player.is_alive() {
            self.game_state = GameState::GameOver;
        }

        result
    }

    pub fn process_turn(&mut self) {
        // Update game state, process enemy movements, etc.
        match self.game_state {
            GameState::Playing => {
                // Process enemy turns
                // This is a simple implementation - more complex AI would be better
                let mut rng = rand::thread_rng();

                // Clone enemy positions to avoid borrowing issues
                let enemy_positions: Vec<Position> =
                    self.current_level().enemies.keys().cloned().collect();

                for pos in enemy_positions {
                    // 50% chance enemy moves randomly
                    if rng.gen_bool(0.5) {
                        let dx = rng.gen_range(-1..=1);
                        let dy = rng.gen_range(-1..=1);

                        let new_pos = Position::new(pos.x + dx, pos.y + dy);

                        // Only move if position is valid and not occupied
                        if self.current_level().is_position_walkable(new_pos)
                            && !self.current_level().enemies.contains_key(&new_pos)
                            && new_pos != self.player_position()
                        {
                            if let Some(enemy) = self.current_level_mut().remove_enemy_at(&pos) {
                                self.current_level_mut().enemies.insert(new_pos, enemy);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn update_visibility(&mut self) {
        // Get the current level and player position
        let level = self.current_level_mut();
        let player_pos = level.player_position;

        // Set all tiles to not visible
        for row in &mut level.visible_tiles {
            for tile in row {
                *tile = false;
            }
        }

        // Reveal a circular area around the player
        let view_radius = 10; // Increased view radius to match UI display

        for dy in -view_radius..=view_radius {
            for dx in -view_radius..=view_radius {
                let x = player_pos.x + dx;
                let y = player_pos.y + dy;

                // Check if within bounds
                if x >= 0 && x < level.width as i32 && y >= 0 && y < level.height as i32 {
                    // Check if within view radius (circular area)
                    if dx * dx + dy * dy <= view_radius * view_radius {
                        level.visible_tiles[y as usize][x as usize] = true;
                        level.revealed_tiles[y as usize][x as usize] = true;

                        // Update tile to be explored
                        if let Some(tile) = level.get_tile_mut(x, y) {
                            tile.explored = true;
                            tile.visible = true;
                        }
                    }
                }
            }
        }

        // Add more tile visibility for the screen around the player
        // This ensures all tiles shown on screen are visible, even beyond the circular radius
        let screen_width = 30; // Half the screen width
        let screen_height = 10; // Half the screen height

        for dy in -screen_height..=screen_height {
            for dx in -screen_width..=screen_width {
                let x = player_pos.x + dx;
                let y = player_pos.y + dy;

                // Check if within bounds and not already visible
                if x >= 0 && x < level.width as i32 && y >= 0 && y < level.height as i32 {
                    level.revealed_tiles[y as usize][x as usize] = true;

                    // Only mark as explored, not necessarily visible (for fog of war effect)
                    if let Some(tile) = level.get_tile_mut(x, y) {
                        tile.explored = true;
                    }
                }
            }
        }
    }
}

pub fn run() {
    // Initialize UI
    let mut ui = UI::new();
    if let Err(e) = ui.initialize() {
        eprintln!("Error initializing UI: {}", e);
        return;
    }

    // Show title screen
    if let Err(e) = ui.draw_title_screen() {
        eprintln!("Error drawing title screen: {}", e);
        return;
    }

    // Main menu loop
    loop {
        match ui.wait_for_key() {
            Ok(key_event) => match key_event.code {
                crossterm::event::KeyCode::Char('1') => {
                    // Start new game
                    break;
                }
                crossterm::event::KeyCode::Char('2') => {
                    // Exit
                    if let Err(e) = ui.cleanup() {
                        eprintln!("Error cleaning up UI: {}", e);
                    }
                    return;
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Error reading key: {}", e);
                if let Err(e) = ui.cleanup() {
                    eprintln!("Error cleaning up UI: {}", e);
                }
                return;
            }
        }
    }

    // Character creation
    let player = match ui.character_creation() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error during character creation: {}", e);
            if let Err(e) = ui.cleanup() {
                eprintln!("Error cleaning up UI: {}", e);
            }
            return;
        }
    };

    // Create new game
    let mut game = Game::new(player);

    // Show combat tutorial
    if let Err(e) = ui.show_combat_tutorial() {
        eprintln!("Error showing combat tutorial: {}", e);
        if let Err(e) = ui.cleanup() {
            eprintln!("Error cleaning up UI: {}", e);
        }
        return;
    }

    game.game_state = GameState::Playing;

    // Game loop
    while match game.game_state {
        GameState::GameOver | GameState::Victory => false,
        _ => true,
    } {
        // Update visibility
        game.update_visibility();

        // Draw game screen
        if let Err(e) =
            ui.draw_game_screen(&game.player, game.current_level(), game.current_dungeon())
        {
            eprintln!("Error drawing game screen: {}", e);
            break;
        }

        // Handle input based on game state
        match game.game_state {
            GameState::Playing => match ui.wait_for_key() {
                Ok(key_event) => match key_event.code {
                    crossterm::event::KeyCode::Up => {
                        if game.move_player(0, -1) {
                            game.process_turn();
                        }
                    }
                    crossterm::event::KeyCode::Down => {
                        if game.move_player(0, 1) {
                            game.process_turn();
                        }
                    }
                    crossterm::event::KeyCode::Left => {
                        if game.move_player(-1, 0) {
                            game.process_turn();
                        }
                    }
                    crossterm::event::KeyCode::Right => {
                        if game.move_player(1, 0) {
                            game.process_turn();
                        }
                    }
                    crossterm::event::KeyCode::Char('i') => {
                        game.game_state = GameState::Inventory;
                    }
                    crossterm::event::KeyCode::Char('c') => {
                        game.game_state = GameState::Character;
                    }
                    crossterm::event::KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                Err(e) => {
                    eprintln!("Error reading key: {}", e);
                    break;
                }
            },
            GameState::Combat(enemy_pos) => {
                let result = game.handle_combat(enemy_pos);
                ui.add_messages_from_combat(&result);
            }
            GameState::Inventory => {
                if let Err(e) = ui.draw_inventory_screen(&game.player) {
                    eprintln!("Error drawing inventory screen: {}", e);
                    break;
                }

                match ui.wait_for_key() {
                    Ok(key_event) => match key_event.code {
                        crossterm::event::KeyCode::Char(c) if c >= '1' && c <= '9' => {
                            let index = c.to_digit(10).unwrap() as usize - 1;
                            if index < game.player.inventory.items.len() {
                                match game.player.inventory.items[index] {
                                    Item::Equipment(_) => {
                                        if let Err(e) = game.player.inventory.equip_item(index) {
                                            ui.add_message(e);
                                        } else {
                                            ui.add_message(format!("Equipped item."));
                                        }
                                    }
                                    Item::Consumable(_) => {
                                        // Handle consumable use directly to avoid borrowing conflicts
                                        if let Some(Item::Consumable(consumable)) =
                                            game.player.inventory.items.get(index).cloned()
                                        {
                                            // Remove from inventory
                                            game.player.inventory.items.remove(index);

                                            // Apply effect and get message
                                            let result = consumable.use_effect(&mut game.player);
                                            ui.add_message(result);
                                        } else {
                                            ui.add_message("This item cannot be used".to_string());
                                        }
                                    }
                                    Item::QuestItem { .. } => {
                                        ui.add_message("This is a quest item.".to_string());
                                    }
                                }
                            }
                        }
                        crossterm::event::KeyCode::Char('e') | crossterm::event::KeyCode::Esc => {
                            game.game_state = GameState::Playing;
                        }
                        _ => {}
                    },
                    Err(e) => {
                        eprintln!("Error reading key: {}", e);
                        break;
                    }
                }
            }
            GameState::Character => {
                if let Err(e) = ui.draw_character_screen(&game.player) {
                    eprintln!("Error drawing character screen: {}", e);
                    break;
                }

                // Any key returns to game
                if let Err(e) = ui.wait_for_key() {
                    eprintln!("Error reading key: {}", e);
                    break;
                }

                game.game_state = GameState::Playing;
            }
            _ => {}
        }
    }

    // Handle game end
    match game.game_state {
        GameState::GameOver => {
            if let Err(e) = ui.draw_game_over(&game.player) {
                eprintln!("Error drawing game over screen: {}", e);
            }
        }
        GameState::Victory => {
            if let Err(e) = ui.draw_victory_screen(&game.player) {
                eprintln!("Error drawing victory screen: {}", e);
            }
        }
        _ => {}
    }

    // Clean up
    if let Err(e) = ui.cleanup() {
        eprintln!("Error cleaning up UI: {}", e);
    }
}
