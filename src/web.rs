use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    console, window, CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlDivElement,
    HtmlElement, KeyboardEvent,
};

use crate::character::{ClassType, Player};
use crate::combat::CombatAction;
use crate::game::{Game, GameState};
use crate::inventory::InventoryManager;
use crate::world::{Position, TileType};

// Game display constants - responsive sizing
const MAP_WIDTH: i32 = 50;
const MAP_HEIGHT: i32 = 20;
const CELL_SIZE: i32 = 10;
const CANVAS_WIDTH: i32 = 800; // Large canvas width for scaling
const CANVAS_HEIGHT: i32 = 600; // Large canvas height for scaling
const UI_PANEL_WIDTH: i32 = 250;
const MESSAGE_HEIGHT: i32 = 100;

// Calculate scale factor to fill canvas
const SCALE_FACTOR_X: f64 = CANVAS_WIDTH as f64 / (MAP_WIDTH * CELL_SIZE) as f64;
const SCALE_FACTOR_Y: f64 = CANVAS_HEIGHT as f64 / (MAP_HEIGHT * CELL_SIZE) as f64;
const SCALE_FACTOR: f64 = if SCALE_FACTOR_X < SCALE_FACTOR_Y {
    SCALE_FACTOR_X
} else {
    SCALE_FACTOR_Y
};
const SCALED_CELL_SIZE: f64 = CELL_SIZE as f64 * SCALE_FACTOR;

// Colors for different elements
const PLAYER_COLOR: &str = "#FFD700"; // Gold
const WALL_COLOR: &str = "#808080"; // Gray
const FLOOR_COLOR: &str = "#2F4F2F"; // Dark green
const DOOR_COLOR: &str = "#8B4513"; // Brown
const ENEMY_COLOR: &str = "#FF0000"; // Red
const ITEM_COLOR: &str = "#00FFFF"; // Cyan
const CHEST_COLOR: &str = "#DAA520"; // Goldenrod
const EXIT_COLOR: &str = "#32CD32"; // Lime green
const FOG_COLOR: &str = "#1a1a1a"; // Very dark gray
const BACKGROUND_COLOR: &str = "#000000"; // Black
const TEXT_COLOR: &str = "#00FF00"; // Green terminal text
const BORDER_COLOR: &str = "#00FF00"; // Green border

#[wasm_bindgen]
pub struct WebGame {
    game: Game,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    ui_panel: HtmlDivElement,
    message_area: HtmlDivElement,
    pressed_keys: HashMap<String, bool>,
    last_key_time: f64,
    key_repeat_delay: f64,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebGame, JsValue> {
        console::log_1(&"Creating Echoes RPG Visual Dungeon Crawler...".into());

        let window = window().unwrap();
        let document = window.document().unwrap();

        // Create main game container
        let container = Self::create_game_container(&document)?;

        // Create canvas for map rendering
        let canvas = Self::create_canvas(&document)?;
        let context = Self::get_canvas_context(&canvas)?;

        // Create UI panel
        let ui_panel = Self::create_ui_panel(&document)?;

        // Create message area
        let message_area = Self::create_message_area(&document)?;

        // Add elements to container
        let map_area = Self::create_map_area(&document)?;
        map_area.append_child(&canvas)?;

        container.append_child(&map_area)?;
        container.append_child(&ui_panel)?;
        container.append_child(&message_area)?;

        // Add container to main-content div if it's not already there
        let main_content = document
            .get_element_by_id("main-content")
            .ok_or("Could not find main-content element")?;

        // Only append if container is not already a child of main-content
        let needs_append = match container.parent_element() {
            Some(parent) => parent != main_content,
            None => true,
        };

        if needs_append {
            main_content.append_child(&container)?;
        }

        // Create game instance with default player for now
        let player = Player::new("WebHero".to_string(), ClassType::Warrior);
        let game = Game::new(player);

        let web_game = WebGame {
            game,
            canvas,
            context,
            ui_panel,
            message_area,
            pressed_keys: HashMap::new(),
            last_key_time: 0.0,
            key_repeat_delay: 150.0, // milliseconds
        };

        Ok(web_game)
    }

    #[wasm_bindgen]
    pub fn start_game(&mut self) -> Result<(), JsValue> {
        console::log_1(&"Starting visual dungeon crawler...".into());

        self.setup_keyboard_handlers()?;
        self.show_title_screen()?;

        Ok(())
    }

    fn create_game_container(document: &Document) -> Result<HtmlDivElement, JsValue> {
        // Check if game-container already exists
        if let Some(existing_container) = document.get_element_by_id("game-container") {
            // Clear existing content and reuse the container
            existing_container.set_inner_html("");
            return Ok(existing_container.dyn_into::<HtmlDivElement>()?);
        }

        // Create new container if none exists
        let container = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        container.set_id("game-container");
        // Let existing CSS handle the styling
        Ok(container)
    }

    fn create_canvas(document: &Document) -> Result<HtmlCanvasElement, JsValue> {
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        canvas.set_id("game-canvas");
        canvas.set_width(CANVAS_WIDTH as u32);
        canvas.set_height(CANVAS_HEIGHT as u32);

        let style = canvas.style();
        style.set_property("border", &format!("1px solid {}", BORDER_COLOR))?;
        style.set_property("background", BACKGROUND_COLOR)?;
        style.set_property("image-rendering", "pixelated")?;
        style.set_property("width", "95%")?;
        style.set_property("height", "95%")?;
        style.set_property("object-fit", "contain")?;

        Ok(canvas)
    }

    fn get_canvas_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, JsValue> {
        Ok(canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?)
    }

    fn create_map_area(document: &Document) -> Result<HtmlDivElement, JsValue> {
        let map_area = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        map_area.set_id("map-area");

        let style = map_area.dyn_ref::<HtmlElement>().unwrap().style();
        style.set_property("display", "flex")?;
        style.set_property("justify-content", "center")?;
        style.set_property("align-items", "center")?;
        style.set_property("flex", "1")?;
        style.set_property("min-height", "0")?;
        style.set_property("overflow", "hidden")?;
        style.set_property("width", "100%")?;

        Ok(map_area)
    }

    fn create_ui_panel(document: &Document) -> Result<HtmlDivElement, JsValue> {
        let panel = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        panel.set_id("ui-panel");

        let style = panel.dyn_ref::<HtmlElement>().unwrap().style();
        style.set_property("width", "100%")?;
        style.set_property("height", "120px")?;
        style.set_property("background", "rgba(0, 20, 0, 0.8)")?;
        style.set_property("border", &format!("1px solid {}", BORDER_COLOR))?;
        style.set_property("padding", "8px")?;
        style.set_property("color", TEXT_COLOR)?;
        style.set_property("font-size", "11px")?;
        style.set_property("font-family", "'Courier New', monospace")?;
        style.set_property("overflow-y", "auto")?;
        style.set_property("display", "flex")?;
        style.set_property("flex-direction", "column")?;

        Ok(panel)
    }

    fn create_message_area(document: &Document) -> Result<HtmlDivElement, JsValue> {
        let messages = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        messages.set_id("message-area");

        let style = messages.dyn_ref::<HtmlElement>().unwrap().style();
        style.set_property("width", "100%")?;
        style.set_property("height", "80px")?;
        style.set_property("max-height", "120px")?;
        style.set_property("background", "rgba(0, 20, 0, 0.8)")?;
        style.set_property("border", &format!("1px solid {}", BORDER_COLOR))?;
        style.set_property("padding", "8px")?;
        style.set_property("color", TEXT_COLOR)?;
        style.set_property("font-size", "10px")?;
        style.set_property("font-family", "'Courier New', monospace")?;
        style.set_property("overflow-y", "auto")?;
        style.set_property("margin-top", "5px")?;

        Ok(messages)
    }

    fn setup_keyboard_handlers(&mut self) -> Result<(), JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        // Prevent default browser shortcuts
        let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = event.key();

            // Prevent browser shortcuts for game keys
            match key.as_str() {
                "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | "i" | "I" | "c" | "C"
                | "g" | "G" | "q" | "Q" | " " | "Enter" | "Escape" => {
                    event.prevent_default();
                    event.stop_propagation();
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        document.add_event_listener_with_callback(
            "keydown",
            keydown_closure.as_ref().unchecked_ref(),
        )?;
        keydown_closure.forget();

        // Handle key processing
        let game_ptr = self as *mut WebGame;
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = event.key();
            unsafe {
                if let Some(game) = game_ptr.as_mut() {
                    let _ = game.handle_key_input(&key);
                }
            }
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())?;
        keyup_closure.forget();

        Ok(())
    }

    fn handle_key_input(&mut self, key: &str) -> Result<(), JsValue> {
        // Prevent key repeat spam
        let now = js_sys::Date::now();
        if now - self.last_key_time < self.key_repeat_delay {
            return Ok(());
        }
        self.last_key_time = now;

        match self.game.game_state.clone() {
            GameState::Playing => self.handle_gameplay_input(key),
            GameState::MainMenu => self.handle_menu_input(key),
            GameState::Inventory => self.handle_inventory_input(key),
            GameState::Character => self.handle_character_input(key),
            GameState::Combat(pos) => self.handle_combat_input(key, pos),
            _ => Ok(()),
        }
    }

    fn handle_gameplay_input(&mut self, key: &str) -> Result<(), JsValue> {
        match key {
            "ArrowUp" => {
                if self.game.move_player(0, -1) {
                    self.process_movement()?;
                }
            }
            "ArrowDown" => {
                if self.game.move_player(0, 1) {
                    self.process_movement()?;
                }
            }
            "ArrowLeft" => {
                if self.game.move_player(-1, 0) {
                    self.process_movement()?;
                }
            }
            "ArrowRight" => {
                if self.game.move_player(1, 0) {
                    self.process_movement()?;
                }
            }
            "i" | "I" => {
                self.game.game_state = GameState::Inventory;
                self.render_game()?;
            }
            "c" | "C" => {
                self.game.game_state = GameState::Character;
                self.render_game()?;
            }
            "g" | "G" => {
                if let Some(message) = self.game.try_get_item() {
                    self.add_message(&message);
                    self.render_game()?;
                }
            }
            "q" | "Q" => {
                self.add_message("Thanks for playing!");
                // Could add exit confirmation here
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_menu_input(&mut self, key: &str) -> Result<(), JsValue> {
        match key {
            "1" => {
                self.start_new_game()?;
            }
            "2" => {
                self.add_message("Load game not implemented yet.");
            }
            "3" => {
                self.show_instructions()?;
            }
            "4" | "q" | "Q" => {
                self.add_message("Thanks for playing!");
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_inventory_input(&mut self, key: &str) -> Result<(), JsValue> {
        match key {
            "Escape" | "i" | "I" => {
                self.game.game_state = GameState::Playing;
                self.render_game()?;
            }
            key if key.len() == 1 && key.chars().next().unwrap().is_ascii_digit() && key != "0" => {
                if let Ok(index) = key.parse::<usize>() {
                    let index = index - 1; // Convert to 0-based
                    if index < InventoryManager::get_item_count(&self.game.player) {
                        let result = InventoryManager::use_item(&mut self.game.player, index);
                        self.add_message(&result.message);
                        if result.success {
                            self.render_game()?;
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_character_input(&mut self, key: &str) -> Result<(), JsValue> {
        match key {
            "Escape" | "c" | "C" => {
                self.game.game_state = GameState::Playing;
                self.render_game()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_combat_input(&mut self, key: &str, pos: Position) -> Result<(), JsValue> {
        match key {
            "1" | " " => {
                // Attack
                self.execute_combat_action(CombatAction::Attack, pos)?;
            }
            "2" => {
                // Use ability (if implemented)
                self.execute_combat_action(CombatAction::UseAbility(0), pos)?;
            }
            "3" => {
                // Use item (if implemented)
                self.execute_combat_action(CombatAction::UseItem(0), pos)?;
            }
            "4" | "f" | "F" => {
                // Flee
                self.execute_combat_action(CombatAction::Flee, pos)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn process_movement(&mut self) -> Result<(), JsValue> {
        match self.game.game_state {
            GameState::Combat(_) => {
                // Combat will be handled in the next update
                self.render_game()?;
            }
            _ => {
                self.game.process_turn();
                self.render_game()?;
            }
        }
        Ok(())
    }

    fn execute_combat_action(
        &mut self,
        action: CombatAction,
        _pos: Position,
    ) -> Result<(), JsValue> {
        // This would integrate with the actual combat system
        // For now, just add a placeholder message
        match action {
            CombatAction::Attack => {
                self.add_message("You attack the enemy!");
            }
            CombatAction::Flee => {
                self.add_message("You attempt to flee!");
                self.game.game_state = GameState::Playing;
            }
            _ => {
                self.add_message("Combat action not yet implemented.");
            }
        }
        self.render_game()
    }

    fn start_new_game(&mut self) -> Result<(), JsValue> {
        // For now, start with a simple character creation
        // In full implementation, this would show character creation screen
        let player = Player::new("Hero".to_string(), ClassType::Warrior);
        self.game = Game::new(player);
        self.game.game_state = GameState::Playing;

        self.add_message("Welcome to the dungeon! Use arrow keys to move.");
        self.add_message("Press 'i' for inventory, 'c' for character, 'g' to get items.");
        self.render_game()
    }

    fn show_instructions(&mut self) -> Result<(), JsValue> {
        self.add_message("=== GAME INSTRUCTIONS ===");
        self.add_message("Arrow Keys: Move your character");
        self.add_message("I: Open inventory");
        self.add_message("C: View character stats");
        self.add_message("G: Pick up items");
        self.add_message("Q: Quit game");
        self.add_message("In combat: 1=Attack, 4=Flee");
        self.add_message("Press any key to continue...");
        Ok(())
    }

    fn show_title_screen(&mut self) -> Result<(), JsValue> {
        self.clear_canvas()?;
        self.game.game_state = GameState::MainMenu;

        // Draw title on canvas
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(TEXT_COLOR));
        self.context.set_font("20px 'Courier New'");
        self.context.fill_text("ECHOES RPG", 200.0, 100.0)?;

        self.context.set_font("12px 'Courier New'");
        self.context
            .fill_text("Web Dungeon Crawler", 220.0, 130.0)?;

        // Update UI panel with menu
        self.ui_panel.set_inner_html(&format!(
            "<div style='text-align: center; margin-top: 50px;'>
                <div style='font-size: 16px; margin-bottom: 20px;'>MAIN MENU</div>
                <div>1. Start New Game</div>
                <div>2. Load Game</div>
                <div>3. Instructions</div>
                <div>4. Exit</div>
                <div style='margin-top: 30px; font-size: 10px;'>Press number key to select</div>
            </div>"
        ));

        self.add_message("Welcome to Echoes RPG!");
        self.add_message("Use number keys to navigate the menu.");

        Ok(())
    }

    fn render_game(&mut self) -> Result<(), JsValue> {
        self.clear_canvas()?;
        self.update_visibility();

        match self.game.game_state {
            GameState::Playing | GameState::Combat(_) => {
                self.render_map()?;
                self.render_ui_panel()?;
            }
            GameState::Inventory => {
                self.render_map()?;
                self.render_inventory_panel()?;
            }
            GameState::Character => {
                self.render_map()?;
                self.render_character_panel()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn clear_canvas(&mut self) -> Result<(), JsValue> {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(BACKGROUND_COLOR));
        self.context
            .fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
        Ok(())
    }

    fn update_visibility(&mut self) {
        self.game.update_visibility();
    }

    fn render_map(&mut self) -> Result<(), JsValue> {
        // Extract all needed data first to avoid borrowing issues
        let level_width = self.game.current_level().width as i32;
        let level_height = self.game.current_level().height as i32;
        let player_pos = self.game.player_position();

        // Collect tile data
        let mut tile_data = Vec::new();
        let mut enemy_positions = Vec::new();
        let mut item_positions = Vec::new();

        {
            let level = self.game.current_level();
            for y in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    if x < level_width && y < level_height {
                        let tile = &level.tiles[y as usize][x as usize];
                        tile_data.push((x, y, tile.visible, tile.explored, tile.tile_type.clone()));
                    }
                }
            }

            // Collect entity positions
            for (pos, _enemy) in &level.enemies {
                enemy_positions.push((pos.x, pos.y));
            }
            for (pos, _item) in &level.items {
                item_positions.push((pos.x, pos.y));
            }
        }

        // Now render everything
        for (x, y, visible, explored, tile_type) in tile_data {
            if visible {
                self.render_tile(x, y, &tile_type)?;

                // Render entities at this position
                if player_pos.x == x && player_pos.y == y {
                    self.render_player(x, y)?;
                } else if enemy_positions.contains(&(x, y)) {
                    self.render_enemy(x, y)?;
                } else if item_positions.contains(&(x, y)) {
                    self.render_item(x, y)?;
                }
            } else if explored {
                self.render_fog_tile(x, y)?;
            }
        }

        Ok(())
    }

    fn render_tile(&mut self, x: i32, y: i32, tile_type: &TileType) -> Result<(), JsValue> {
        let color = match tile_type {
            TileType::Wall => WALL_COLOR,
            TileType::Floor => FLOOR_COLOR,
            TileType::Door => DOOR_COLOR,
            TileType::Chest => CHEST_COLOR,
            TileType::Exit => EXIT_COLOR,
            TileType::StairsDown => EXIT_COLOR,
            TileType::StairsUp => EXIT_COLOR,
        };

        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(color));
        self.context.fill_rect(
            (x as f64) * SCALED_CELL_SIZE,
            (y as f64) * SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
        );

        Ok(())
    }

    fn render_fog_tile(&mut self, x: i32, y: i32) -> Result<(), JsValue> {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(FOG_COLOR));
        self.context.fill_rect(
            (x as f64) * SCALED_CELL_SIZE,
            (y as f64) * SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
        );
        Ok(())
    }

    fn render_player(&mut self, x: i32, y: i32) -> Result<(), JsValue> {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(PLAYER_COLOR));
        self.context.fill_rect(
            (x as f64) * SCALED_CELL_SIZE,
            (y as f64) * SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
        );

        // Add @ symbol for player
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
        self.context
            .set_font(&format!("{}px monospace", (SCALED_CELL_SIZE - 2.0) as i32));
        self.context.fill_text(
            "@",
            (x as f64) * SCALED_CELL_SIZE + 2.0,
            (y as f64) * SCALED_CELL_SIZE + SCALED_CELL_SIZE - 2.0,
        )?;

        Ok(())
    }

    fn render_enemy(&mut self, x: i32, y: i32) -> Result<(), JsValue> {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(ENEMY_COLOR));
        self.context.fill_rect(
            (x as f64) * SCALED_CELL_SIZE,
            (y as f64) * SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
        );
        Ok(())
    }

    fn render_item(&mut self, x: i32, y: i32) -> Result<(), JsValue> {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(ITEM_COLOR));
        self.context.fill_rect(
            (x as f64) * SCALED_CELL_SIZE,
            (y as f64) * SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
            SCALED_CELL_SIZE,
        );
        Ok(())
    }

    fn render_ui_panel(&mut self) -> Result<(), JsValue> {
        let player = &self.game.player;
        let dungeon = self.game.current_dungeon();

        let ui_content = format!(
            "<div style='color: {}; font-family: monospace; height: 100%; display: flex; flex-direction: column;'>
                <div style='display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; border-bottom: 1px solid {}; padding-bottom: 4px;'>
                    <div style='font-weight: bold;'>{} (Lv.{}) | HP: {}/{}</div>
                    <div>XP: {} | Gold: {}</div>
                    <div>Floor: {} ({:?})</div>
                </div>
                <div style='display: flex; justify-content: space-between; flex: 1; align-items: center;'>
                    <div style='font-size: 10px;'>
                        <div>↑↓←→ Move | I-Inv | C-Char | G-Get | Q-Quit</div>
                    </div>
                </div>
            </div>",
            TEXT_COLOR,
            BORDER_COLOR,
            player.name,
            player.level,
            player.health,
            player.max_health,
            player.experience,
            player.gold,
            self.game.current_dungeon_index + 1,
            dungeon.dungeon_type
        );

        self.ui_panel.set_inner_html(&ui_content);
        Ok(())
    }

    fn render_inventory_panel(&mut self) -> Result<(), JsValue> {
        let player = &self.game.player;
        let item_count = InventoryManager::get_item_count(player);

        let mut content = format!(
            "<div style='color: {}; font-family: monospace;'>
                <div style='font-size: 14px; margin-bottom: 10px; text-align: center;'>INVENTORY</div>",
            TEXT_COLOR
        );

        if item_count == 0 {
            content.push_str("<div>Your inventory is empty.</div>");
        } else {
            for i in 0..item_count {
                if let Some(item) = InventoryManager::get_item(player, i) {
                    content.push_str(&format!("<div>{}. {}</div>", i + 1, item.name()));
                }
            }
        }

        content.push_str(
            "
            <div style='margin-top: 15px;'>
                <div>Press 1-9 to use item</div>
                <div>Press I or ESC to close</div>
            </div>
        </div>",
        );

        self.ui_panel.set_inner_html(&content);
        Ok(())
    }

    fn render_character_panel(&mut self) -> Result<(), JsValue> {
        let player = &self.game.player;

        let content = format!(
            "<div style='color: {}; font-family: monospace;'>
                <div style='font-size: 14px; margin-bottom: 10px; text-align: center;'>CHARACTER</div>
                <div>Name: {}</div>
                <div>Class: {:?}</div>
                <div>Level: {}</div>
                <div>Health: {}/{}</div>
                <div>Experience: {}</div>
                <div>Gold: {}</div>
                <div style='margin-top: 10px;'>
                    <div style='font-size: 12px; margin-bottom: 5px;'>STATS</div>
                    <div>Strength: {}</div>
                    <div>Intelligence: {}</div>
                    <div>Dexterity: {}</div>
                    <div>Constitution: {}</div>
                    <div>Wisdom: {}</div>
                </div>
                <div style='margin-top: 15px;'>
                    <div>Press C or ESC to close</div>
                </div>
            </div>",
            TEXT_COLOR,
            player.name,
            player.class.class_type,
            player.level,
            player.health,
            player.max_health,
            player.experience,
            player.gold,
            player.stats.strength,
            player.stats.intelligence,
            player.stats.dexterity,
            player.stats.constitution,
            player.stats.wisdom
        );

        self.ui_panel.set_inner_html(&content);
        Ok(())
    }

    fn add_message(&mut self, message: &str) {
        let current_content = self.message_area.inner_html();
        let new_content = if current_content.is_empty() {
            message.to_string()
        } else {
            format!("{}<br>{}", current_content, message)
        };
        self.message_area.set_inner_html(&new_content);

        // Auto-scroll to bottom
        self.message_area
            .set_scroll_top(self.message_area.scroll_height());
    }
}

// Initialize the game when the WASM module loads
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(
        &"WASM module loaded - initializing Echoes RPG Visual Dungeon Crawler...".into(),
    );

    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Auto-initialize game when WASM loads
    initialize_game()?;

    Ok(())
}

fn initialize_game() -> Result<(), JsValue> {
    // Wait a bit to ensure DOM is ready
    let window = window().unwrap();
    let document = window.document().unwrap();

    // Check if game is already initialized to prevent duplicates
    if document.get_element_by_id("game-container").is_some() {
        console::log_1(&"Game already initialized, skipping duplicate initialization.".into());
        return Ok(());
    }

    console::log_1(&"Creating visual dungeon crawler game instance...".into());
    let mut game = WebGame::new()?;
    game.start_game()?;

    console::log_1(&"Visual dungeon crawler initialized successfully!".into());

    // Keep the game instance alive (in a real implementation, you'd want to store this properly)
    std::mem::forget(game);

    Ok(())
}
