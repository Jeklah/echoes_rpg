#[cfg(not(target_arch = "wasm32"))]
use crossterm::event::KeyCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

use crate::character::Player;
#[cfg(all(
    not(all(feature = "gui", target_os = "windows")),
    not(target_arch = "wasm32")
))]
use crate::combat::process_combat_turn;
use crate::inventory::InventoryManager;
#[cfg(all(
    not(all(feature = "gui", target_os = "windows")),
    not(target_arch = "wasm32")
))]
use crate::item::Item;
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
use crate::platform;
#[cfg(all(
    not(all(feature = "gui", target_os = "windows")),
    not(target_arch = "wasm32")
))]
use crate::ui::UI;
use crate::world::{Dungeon, Level, Position, Tile, TileType};

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
    pub combat_started: bool,
    #[cfg(target_arch = "wasm32")]
    pub first_visibility_update_done: bool,
    #[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
    pub last_render_time: Option<std::time::Instant>,
}

impl Game {
    pub fn new(player: Player) -> Self {
        // Create initial dungeon
        let first_dungeon = Dungeon::generate_random(player.level);

        let game = Game {
            player,
            dungeons: vec![first_dungeon],
            current_dungeon_index: 0,
            game_state: GameState::MainMenu,
            combat_started: false,
            #[cfg(target_arch = "wasm32")]
            first_visibility_update_done: false,
            #[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
            last_render_time: None,
        };

        // Initialize visibility for the starting level (with WASM safety)
        #[cfg(target_arch = "wasm32")]
        {
            // Skip visibility update during initialization in WASM to prevent freezing
            console::log_1(&"WASM: Skipping initial visibility update".into());
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            game.update_visibility();
        }

        game
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

        // Check if the position is valid (tiles only, not enemies)
        let tile_walkable = self.current_level().is_position_valid(new_pos.x, new_pos.y)
            && self.current_level().tiles[new_pos.y as usize][new_pos.x as usize]
                .tile_type
                .is_walkable();

        if !tile_walkable {
            return false;
        }

        // Check for enemies
        if self.current_level().enemies.contains_key(&new_pos) {
            // Start combat - don't move the player into the enemy's position
            self.game_state = GameState::Combat(new_pos);

            // Mark that we're starting a new combat
            self.combat_started = true;

            return true;
        }

        // Check for items on the ground
        if self.current_level().items.contains_key(&new_pos) {
            let item = self.current_level_mut().remove_item_at(&new_pos).unwrap();
            // Try to add to inventory
            let add_result = InventoryManager::add_item(&mut self.player, item.clone());
            if !add_result.success {
                // Put the item back if inventory is full
                self.current_level_mut().items.insert(new_pos, item);
                return false;
            }
        }

        // Check for special tiles
        if let Some(tile) = self.current_level().get_tile(new_pos.x, new_pos.y) {
            match tile.tile_type {
                TileType::StairsDown => {
                    if self.current_dungeon_mut().go_to_next_level().is_err() {
                        // Can't go further down
                        return false;
                    }
                    // Move player to the starting position of the new level
                    let new_level_start = self.current_level().player_position;
                    self.current_level_mut().player_position = new_level_start;
                    return true;
                }
                TileType::StairsUp => {
                    if self.current_dungeon_mut().go_to_previous_level().is_err() {
                        // Can't go further up
                        return false;
                    }
                    // Move player to the starting position of the previous level
                    let new_level_start = self.current_level().player_position;
                    self.current_level_mut().player_position = new_level_start;
                    return true;
                }
                TileType::Exit => {
                    if self.current_dungeon().is_final_level() {
                        // Victory condition - player reached the exit of the final level
                        self.game_state = GameState::Victory;
                    }
                    // Allow player to move to the exit position
                    self.current_level_mut().player_position = new_pos;
                    return true;
                }
                TileType::Chest => {
                    // Generate loot from chest
                    if let Some(item) = self.current_level().get_item_at(&new_pos) {
                        let item_clone = item.clone();
                        #[cfg_attr(not(debug_assertions), allow(unused_variables))]
                        let item_name = item_clone.name().to_string();
                        let add_result = InventoryManager::add_item(&mut self.player, item_clone);
                        if !add_result.success {
                            // Inventory full, can't loot the chest
                            return false;
                        }
                        // Remove the item
                        self.current_level_mut().remove_item_at(&new_pos);

                        // This is auto-looting by walking into a chest
                        // We don't directly add a message here because the move_player method
                        // doesn't return messages, but we'll add a hook for it
                        #[cfg(debug_assertions)]
                        println!("DEBUG: Auto-looted chest at {new_pos:?}, found {item_name}");
                    }

                    // Always replace the chest with a floor tile (whether it had loot or not)
                    if let Some(tile) = self.current_level_mut().get_tile_mut(new_pos.x, new_pos.y)
                    {
                        *tile = Tile::floor();
                    }

                    // Don't return early - let the player move to the chest position
                    // after it's been converted to a floor tile
                }
                _ => {}
            }
        }

        // Move the player
        self.current_level_mut().player_position = new_pos;
        true
    }

    pub fn process_turn(&mut self) {
        // Update game state, process enemy movements, etc.
        if let GameState::Playing = self.game_state {
            // Process enemy turns
            // This is a simple implementation - more complex AI would be better
            let mut rng = rand::thread_rng();

            // Clone enemy positions to avoid borrowing issues
            let enemy_positions: Vec<Position> =
                self.current_level().enemies.keys().copied().collect();

            for pos in enemy_positions {
                // 50% chance enemy moves randomly
                if rng.gen_bool(0.5) {
                    let dx = rng.gen_range(-1..=1);
                    let dy = rng.gen_range(-1..=1);

                    let new_pos = Position::new(pos.x + dx, pos.y + dy);

                    // Only move if position is valid and not occupied
                    if self.current_level().is_tile_walkable(new_pos)
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
    }

    pub fn update_visibility(&mut self) {
        // Check if this is the first visibility update for WASM
        #[cfg(target_arch = "wasm32")]
        let is_first_update = !self.first_visibility_update_done;
        #[cfg(not(target_arch = "wasm32"))]
        let is_first_update = false;

        // WASM: Removed rate limiting to ensure all movement updates complete
        #[cfg(target_arch = "wasm32")]
        {
            console::log_1(&"Starting visibility update".into());
        }

        // Get the current level and player position
        let level = self.current_level_mut();
        let player_pos = level.player_position;

        // Safety bounds to prevent excessive computation
        let max_width = level.width.min(80); // Reduced for WASM performance
        let max_height = level.height.min(60);

        // Set all tiles to not visible with bounds checking and WASM safety
        if level.visible_tiles.len() > max_height
            || !level.visible_tiles.iter().all(|row| row.len() <= max_width)
        {
            #[cfg(target_arch = "wasm32")]
            {
                console::log_1(
                    &"Warning: Visibility tiles exceed safety bounds, skipping update".into(),
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                eprintln!("Warning: Visibility tiles exceed safety bounds, skipping update");
            }
            return;
        }

        let mut tiles_cleared = 0;
        for row in &mut level.visible_tiles {
            for tile in row {
                *tile = false;
                tiles_cleared += 1;
                // WASM: Don't yield during tile clearing to prevent inconsistent state
                // Early return will happen after clearing is complete
            }
        }

        // WASM: Removed early return to ensure visibility always completes
        // This prevents partial visibility updates that cause freezing

        // Reveal a circular area around the player
        let view_radius = if cfg!(target_arch = "wasm32") {
            20.min(max_width as i32 / 3).min(max_height as i32 / 3) // Large radius to cover 40x25 viewport
        } else {
            10.min(max_width as i32 / 4).min(max_height as i32 / 4)
        };

        let mut tiles_processed = 0;
        let max_tiles_per_update = if cfg!(target_arch = "wasm32") {
            // Large limit to handle 40x25 viewport with 20-tile radius
            5000
        } else {
            2000
        };

        for dy in -view_radius..=view_radius {
            for dx in -view_radius..=view_radius {
                if tiles_processed >= max_tiles_per_update {
                    #[cfg(target_arch = "wasm32")]
                    {
                        console::log_1(
                            &"Warning: Visibility update hit tile processing limit".into(),
                        );
                        // Continue processing for WASM to prevent incomplete visibility
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        eprintln!("Warning: Visibility update hit tile processing limit");
                        break;
                    }
                }

                let x = player_pos.x + dx;
                let y = player_pos.y + dy;

                // Check if within bounds
                if x >= 0 && x < level.width as i32 && y >= 0 && y < level.height as i32 {
                    // Check if within view radius (circular area)
                    if dx * dx + dy * dy <= view_radius * view_radius {
                        let ux = x as usize;
                        let uy = y as usize;

                        // Additional bounds checking for array access
                        if uy < level.visible_tiles.len()
                            && ux < level.visible_tiles[uy].len()
                            && uy < level.revealed_tiles.len()
                            && ux < level.revealed_tiles[uy].len()
                        {
                            level.visible_tiles[uy][ux] = true;
                            level.revealed_tiles[uy][ux] = true;

                            // Update tile to be explored
                            if let Some(tile) = level.get_tile_mut(x, y) {
                                tile.explored = true;
                                tile.visible = true;
                            }
                        }
                    }
                }
                tiles_processed += 1;
            }
            if tiles_processed >= max_tiles_per_update {
                break;
            }
        }

        // Add more tile visibility for the screen around the player
        // This ensures all tiles shown on screen are visible, even beyond the circular radius
        let screen_width = if cfg!(target_arch = "wasm32") {
            25.min(max_width as i32 / 2) // Cover full 40-tile width viewport
        } else {
            30.min(max_width as i32 / 2) // Bounded screen width
        };
        let screen_height = if cfg!(target_arch = "wasm32") {
            15.min(max_height as i32 / 2) // Cover full 25-tile height viewport
        } else {
            10.min(max_height as i32 / 2) // Bounded screen height
        };

        for dy in -screen_height..=screen_height {
            for dx in -screen_width..=screen_width {
                if tiles_processed >= max_tiles_per_update {
                    #[cfg(target_arch = "wasm32")]
                    {
                        console::log_1(
                            &"Warning: Screen visibility update hit tile processing limit".into(),
                        );
                        // Continue processing for WASM to prevent incomplete visibility
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        eprintln!("Warning: Screen visibility update hit tile processing limit");
                        break;
                    }
                }

                let x = player_pos.x + dx;
                let y = player_pos.y + dy;

                // Check if within bounds and not already visible
                if x >= 0 && x < level.width as i32 && y >= 0 && y < level.height as i32 {
                    let ux = x as usize;
                    let uy = y as usize;

                    // Additional bounds checking for array access
                    if uy < level.revealed_tiles.len() && ux < level.revealed_tiles[uy].len() {
                        level.revealed_tiles[uy][ux] = true;

                        // Only mark as explored, not necessarily visible (for fog of war effect)
                        if let Some(tile) = level.get_tile_mut(x, y) {
                            tile.explored = true;
                        }
                    }
                }
                tiles_processed += 1;
            }
            if tiles_processed >= max_tiles_per_update {
                break;
            }
        }

        // Mark first visibility update as complete for WASM
        #[cfg(target_arch = "wasm32")]
        {
            if !self.first_visibility_update_done {
                console::log_1(&"First visibility update completed".into());
            }
            self.first_visibility_update_done = true;
        }
    }

    /// Attempts to pick up an item at the player's position or loot a chest in an adjacent tile.
    /// Returns a message describing the result of the action.
    pub fn try_get_item(&mut self) -> Option<String> {
        let player_pos = self.current_level().player_position;

        // First check if there's an item at the current position
        if let Some(item) = self.current_level().get_item_at(&player_pos) {
            let item_clone = item.clone();
            let add_result = InventoryManager::add_item(&mut self.player, item_clone);
            if add_result.success {
                self.current_level_mut().remove_item_at(&player_pos);
                return Some("You picked up an item.".to_string());
            }
            return Some(add_result.message);
        }

        // Check adjacent positions for chests or items
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // up, down, left, right

        for (dx, dy) in &directions {
            let adj_pos = Position::new(player_pos.x + dx, player_pos.y + dy);

            // Check if position is valid
            if !self.current_level().is_position_valid(adj_pos.x, adj_pos.y) {
                continue;
            }

            // Check if there's a chest at this position
            if let Some(tile) = self.current_level().get_tile(adj_pos.x, adj_pos.y) {
                if tile.tile_type == TileType::Chest {
                    // Try to loot the chest
                    if let Some(item) = self.current_level().get_item_at(&adj_pos) {
                        let item_clone = item.clone();
                        // Get the item name before potentially moving item_clone
                        let item_name = item_clone.name().to_string();
                        // Also save the name for potential error message
                        let item_name_for_err = item_clone.name().to_string();
                        let add_result = InventoryManager::add_item(&mut self.player, item_clone);
                        if add_result.success {
                            // Item name is already saved
                            self.current_level_mut().remove_item_at(&adj_pos);
                            // Replace chest with floor
                            if let Some(tile_mut) =
                                self.current_level_mut().get_tile_mut(adj_pos.x, adj_pos.y)
                            {
                                *tile_mut = Tile::floor();
                            }
                            return Some(format!("You looted the chest and found {item_name}!"));
                        }
                        return Some(format!(
                            "Chest contains {}, but {}.",
                            item_name_for_err,
                            add_result.message.to_lowercase()
                        ));
                    }
                    // This could indicate an issue with chest item generation
                    // Add more detailed debug information
                    #[cfg(debug_assertions)]
                    println!("DEBUG: Found empty chest at position {adj_pos:?}");

                    // Replace chest with floor since it's empty
                    if let Some(tile_mut) =
                        self.current_level_mut().get_tile_mut(adj_pos.x, adj_pos.y)
                    {
                        *tile_mut = Tile::floor();
                    }

                    return Some("The chest is empty.".to_string());
                }
            }

            // Check if there's an item at this adjacent position
            if let Some(item) = self.current_level().get_item_at(&adj_pos) {
                let item_clone = item.clone();
                let add_result = InventoryManager::add_item(&mut self.player, item_clone);
                if add_result.success {
                    self.current_level_mut().remove_item_at(&adj_pos);
                    return Some("You picked up an item.".to_string());
                }
                return Some(add_result.message);
            }
        }

        Some("There's nothing here to pick up.".to_string())
    }
}

#[cfg(all(
    not(all(feature = "gui", target_os = "windows")),
    not(target_arch = "wasm32")
))]
pub fn run() {
    // Initialize UI
    let mut ui = UI::new();
    if let Err(e) = ui.initialize() {
        eprintln!("Error initializing UI: {e}");
        return;
    }

    // Show title screen
    if let Err(e) = ui.draw_title_screen() {
        eprintln!("Error drawing title screen: {e}");
        return;
    }

    // Main menu loop
    loop {
        match ui.wait_for_key() {
            Ok(key_event) => match key_event.code {
                KeyCode::Char('1') => {
                    // Start new game
                    break;
                }
                KeyCode::Char('2') => {
                    // Exit
                    if let Err(e) = ui.cleanup() {
                        eprintln!("Error cleaning up UI: {e}");
                    }
                    return;
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Error reading key: {e}");
                if let Err(e) = ui.cleanup() {
                    eprintln!("Error cleaning up UI: {e}");
                }
                return;
            }
        }
    }

    // Character creation
    let player = match ui.character_creation() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error during character creation: {e}");
            if let Err(e) = ui.cleanup() {
                eprintln!("Error cleaning up UI: {e}");
            }
            return;
        }
    };

    // Create new game
    let mut game = Game::new(player);

    // Show combat tutorial
    if let Err(e) = ui.show_combat_tutorial() {
        eprintln!("Error showing combat tutorial: {e}");
        if let Err(e) = ui.cleanup() {
            eprintln!("Error cleaning up UI: {e}");
        }
        return;
    }

    game.game_state = GameState::Playing;

    // Game loop with error recovery
    let mut consecutive_errors = 0;
    const MAX_CONSECUTIVE_ERRORS: u32 = 5;

    while !matches!(game.game_state, GameState::GameOver | GameState::Victory) {
        // Windows-specific frame rate limiting for better performance
        #[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
        {
            if platform::is_command_prompt() {
                platform::cmd_frame_limit();
            } else {
                platform::windows_frame_limit();
            }
        }

        // Update visibility
        game.update_visibility();

        // Windows-specific screen update optimization
        #[cfg(windows)]
        let should_redraw = {
            let now = std::time::Instant::now();
            let should_draw = game.last_render_time.map_or(true, |last| {
                now.duration_since(last).as_millis() > 16 // ~60 FPS max
            });
            if should_draw {
                game.last_render_time = Some(now);
            }
            should_draw
        };

        #[cfg(not(windows))]
        let should_redraw = true;

        // Draw game screen only when needed
        if should_redraw {
            if let Err(e) =
                ui.draw_game_screen(&game.player, game.current_level(), game.current_dungeon())
            {
                eprintln!("Error drawing game screen: {e}");
                break;
            }
        }

        // Handle input based on game state with error recovery
        match game.game_state {
            GameState::Playing => match ui.wait_for_key() {
                Ok(key_event) => {
                    consecutive_errors = 0; // Reset error counter on success
                    match key_event.code {
                        KeyCode::Up => {
                            if game.move_player(0, -1) {
                                match game.game_state {
                                    GameState::Combat(_) => {
                                        // Combat will be handled in the next loop iteration
                                    }
                                    _ => game.process_turn(),
                                }
                            }
                        }
                        KeyCode::Down => {
                            if game.move_player(0, 1) {
                                match game.game_state {
                                    GameState::Combat(_) => {
                                        // Combat will be handled in the next loop iteration
                                    }
                                    _ => game.process_turn(),
                                }
                            }
                        }
                        KeyCode::Left => {
                            if game.move_player(-1, 0) {
                                match game.game_state {
                                    GameState::Combat(_) => {
                                        // Combat will be handled in the next loop iteration
                                    }
                                    _ => game.process_turn(),
                                }
                            }
                        }
                        KeyCode::Right => {
                            if game.move_player(1, 0) {
                                match game.game_state {
                                    GameState::Combat(_) => {
                                        // Combat will be handled in the next loop iteration
                                    }
                                    _ => game.process_turn(),
                                }
                            }
                        }
                        KeyCode::Char('i') => {
                            game.game_state = GameState::Inventory;
                        }
                        KeyCode::Char('c') => {
                            game.game_state = GameState::Character;
                        }
                        KeyCode::Char('g') => {
                            // Try to get item at current position or adjacent chest
                            if let Some(result) = game.try_get_item() {
                                ui.add_message(result);
                            }
                        }
                        KeyCode::Char('q') => {
                            break;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    consecutive_errors += 1;
                    eprintln!(
                        "Error reading key (attempt {}/{}): {e}",
                        consecutive_errors, MAX_CONSECUTIVE_ERRORS
                    );

                    if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                        eprintln!("Too many consecutive input errors, exiting game loop");
                        break;
                    }

                    // Brief pause before retrying to prevent tight error loop
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            },
            GameState::Combat(enemy_pos) => {
                // Make sure the enemy still exists at this position
                if game.current_level().enemies.contains_key(&enemy_pos) {
                    // Check if we need to clear messages for a new combat
                    if game.combat_started {
                        // Clone the enemy name before clearing the UI to avoid borrowing issues
                        let enemy_name = game
                            .current_level()
                            .get_enemy_at(&enemy_pos)
                            .unwrap()
                            .name
                            .clone();
                        ui.clear_messages();
                        ui.add_message(format!("Combat started with {enemy_name}!"));
                        game.combat_started = false;
                    }

                    // Get the enemy reference after clearing messages
                    let enemy = game.current_level().get_enemy_at(&enemy_pos).unwrap();

                    // Draw the combat screen
                    if let Err(e) = ui.draw_combat_screen(&game.player, enemy) {
                        eprintln!("Error drawing combat screen: {e}");
                        break;
                    }

                    // Get the combat action from the user
                    let action = match ui.handle_combat_action(&game.player) {
                        Ok(a) => a,
                        Err(e) => {
                            eprintln!("Error handling combat action: {e}");
                            break;
                        }
                    };

                    // Apply the chosen action
                    let mut enemy_clone = enemy.clone();
                    let mut player_clone = game.player.clone();
                    let result = process_combat_turn(&mut player_clone, &mut enemy_clone, action);

                    // Update game state
                    game.player = player_clone;
                    if !result.enemy_defeated && !result.player_fled {
                        if let Some(enemy_ref) =
                            game.current_level_mut().get_enemy_at_mut(&enemy_pos)
                        {
                            *enemy_ref = enemy_clone;
                        }
                    }

                    // Add combat messages to UI
                    ui.add_messages_from_combat(&result);

                    // Check if combat is over
                    if result.enemy_defeated {
                        game.current_level_mut().remove_enemy_at(&enemy_pos);
                        game.game_state = GameState::Playing;
                        // Reset combat state and add victory message
                        game.combat_started = false;
                        ui.add_message("You were victorious!".to_string());
                    } else if result.player_fled {
                        game.game_state = GameState::Playing;
                        // Reset combat state and add fled message
                        game.combat_started = false;
                        ui.add_message("You fled from combat!".to_string());
                    } else if !game.player.is_alive() {
                        game.game_state = GameState::GameOver;
                    }
                } else {
                    // Enemy no longer exists at this position, return to playing
                    game.game_state = GameState::Playing;
                }
            }
            GameState::Inventory => {
                if let Err(e) = ui.draw_inventory_screen(&game.player) {
                    eprintln!("Error drawing inventory screen: {e}");
                    break;
                }

                match ui.wait_for_key() {
                    Ok(key_event) => {
                        consecutive_errors = 0; // Reset error counter on success
                        match key_event.code {
                            KeyCode::Char(c) if ('1'..='9').contains(&c) => {
                                let index = c.to_digit(10).unwrap() as usize - 1;
                                if index < InventoryManager::get_item_count(&game.player) {
                                    if let Some(item) =
                                        InventoryManager::get_item(&game.player, index)
                                    {
                                        match item {
                                            Item::Equipment(_) | Item::Consumable(_) => {
                                                let result = InventoryManager::use_item(
                                                    &mut game.player,
                                                    index,
                                                );
                                                ui.add_message(result.message);
                                            }
                                            Item::Quest { .. } => {
                                                ui.add_message(
                                                    "This item cannot be used".to_string(),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('e') | KeyCode::Esc => {
                                game.game_state = GameState::Playing;
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        eprintln!(
                            "Error reading key in inventory (attempt {}/{}): {e}",
                            consecutive_errors, MAX_CONSECUTIVE_ERRORS
                        );

                        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                            eprintln!("Too many consecutive input errors in inventory, returning to main menu");
                            break;
                        }

                        // Brief pause before retrying
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
            }
            GameState::Character => {
                if let Err(e) = ui.draw_character_screen(&game.player) {
                    eprintln!("Error drawing character screen: {e}");
                    break;
                }

                // Any key returns to game with error handling
                match ui.wait_for_key() {
                    Ok(_) => {
                        consecutive_errors = 0; // Reset error counter on success
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        eprintln!(
                            "Error reading key in character screen (attempt {}/{}): {e}",
                            consecutive_errors, MAX_CONSECUTIVE_ERRORS
                        );

                        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                            eprintln!("Too many consecutive input errors in character screen, returning to main menu");
                            break;
                        }

                        // Brief pause before retrying
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
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
                eprintln!("Error drawing game over screen: {e}");
            }
        }
        GameState::Victory => {
            if let Err(e) = ui.draw_victory_screen(&game.player) {
                eprintln!("Error drawing victory screen: {e}");
            }
        }
        _ => {}
    }

    // Clean up
    if let Err(e) = ui.cleanup() {
        eprintln!("Error cleaning up UI: {e}");
    }
}
