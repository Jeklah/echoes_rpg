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
use crate::world::level::{Level, Room};
use crate::world::tile::Tile;
use crate::world::{Dungeon, Position, TileType};

// WASM-specific viewport constants (independent of desktop versions)
const VIEW_WIDTH: i32 = 50; // WASM: Number of tiles visible horizontally
const VIEW_HEIGHT: i32 = 20; // WASM: Number of tiles visible vertically
const CELL_SIZE: i32 = 10; // WASM: Base cell size for calculations
const CANVAS_WIDTH: i32 = 800; // WASM: Large canvas width for scaling
const CANVAS_HEIGHT: i32 = 600; // WASM: Large canvas height for scaling

// WASM-specific UI constants (separate from desktop)
const UI_PANEL_WIDTH: i32 = 250; // WASM: UI panel width
const MESSAGE_HEIGHT: i32 = 100; // WASM: Message area height

// WASM-specific scaling calculations (independent of desktop)
const SCALE_FACTOR_X: f64 = CANVAS_WIDTH as f64 / (VIEW_WIDTH * CELL_SIZE) as f64;
const SCALE_FACTOR_Y: f64 = CANVAS_HEIGHT as f64 / (VIEW_HEIGHT * CELL_SIZE) as f64;
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

/// WASM-specific game structure (separate from desktop Game)
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
    key_repeat_timeout: Option<i32>,
    last_movement_time: f64,
    movement_repeat_rate: f64,
    max_consecutive_movements: u32,
    consecutive_movement_count: u32,
    timer_count: u32,
    max_timers: u32,
    render_count: u32,
    last_render_check: f64,
}

#[wasm_bindgen]
/// WASM-specific implementation (independent of desktop versions)
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
            key_repeat_timeout: None,
            last_movement_time: 0.0,
            movement_repeat_rate: 120.0, // milliseconds between movement when holding key
            max_consecutive_movements: 50, // Prevent infinite timer loops
            consecutive_movement_count: 0,
            timer_count: 0,
            max_timers: 3, // Maximum concurrent timers allowed
            render_count: 0,
            last_render_check: 0.0,
        };

        Ok(web_game)
    }

    /// WASM-specific game start (separate from desktop start logic)
    #[wasm_bindgen]
    pub fn start_game(&mut self) -> Result<(), JsValue> {
        console::log_1(&"Starting visual dungeon crawler...".into());

        // Setup keyboard handlers with error handling
        if let Err(e) = self.setup_keyboard_handlers() {
            console::log_2(&"Error setting up keyboard handlers:".into(), &e);
            return Err(e);
        }

        // Show title screen with error handling
        if let Err(e) = self.show_title_screen() {
            console::log_2(&"Error showing title screen:".into(), &e);
            return Err(e);
        }

        Ok(())
    }

    fn show_main_menu(&mut self) -> Result<(), JsValue> {
        console::log_1(&"Showing main menu".into());
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

        // Handle keydown events - set key state and start processing loop
        let game_ptr1 = self as *mut WebGame;
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

            unsafe {
                if let Some(game) = game_ptr1.as_mut() {
                    // Add safety check to prevent recursive calls
                    if game.consecutive_movement_count < game.max_consecutive_movements {
                        if let Err(e) = game.handle_key_down(&key) {
                            console::log_2(&"Error in key down handler:".into(), &e);
                            game.clear_all_key_states();
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        document.add_event_listener_with_callback(
            "keydown",
            keydown_closure.as_ref().unchecked_ref(),
        )?;
        keydown_closure.forget();

        // Handle keyup events - clear key state
        let game_ptr2 = self as *mut WebGame;
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = event.key();
            unsafe {
                if let Some(game) = game_ptr2.as_mut() {
                    // Always try to handle key up to prevent stuck keys
                    if let Err(e) = game.handle_key_up(&key) {
                        console::log_2(&"Error in key up handler:".into(), &e);
                        // Force clear all key states on error
                        game.clear_all_key_states();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())?;
        keyup_closure.forget();

        Ok(())
    }

    fn handle_key_down(&mut self, key: &str) -> Result<(), JsValue> {
        console::log_2(&"Key down:".into(), &key.into());

        // Set key as pressed
        self.pressed_keys.insert(key.to_string(), true);

        // Handle all keys immediately
        match key {
            "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => {
                console::log_2(&"Movement key detected:".into(), &key.into());
                if matches!(self.game.game_state, GameState::Playing) {
                    console::log_1(&"Game state is Playing, handling movement".into());
                    self.handle_immediate_movement(key)?;
                    // Start simple repeat timer for movement
                    self.start_simple_repeat_timer()?;
                }
            }
            // Menu keys, action keys, and navigation keys
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" | "i" | "I" | "c" | "C"
            | "g" | "G" | "q" | "Q" | "f" | "F" | " " | "Enter" | "Escape" => {
                console::log_2(&"Action key detected:".into(), &key.into());
                self.handle_single_key_action(key)?;
            }
            _ => {
                console::log_2(&"Unknown key:".into(), &key.into());
            }
        }

        Ok(())
    }

    fn handle_key_up(&mut self, key: &str) -> Result<(), JsValue> {
        console::log_2(&"Key up:".into(), &key.into());

        // Clear key state
        self.pressed_keys.insert(key.to_string(), false);

        // Stop repeat timer if no movement keys are held
        if !self.any_movement_keys_pressed() {
            console::log_1(&"No movement keys pressed, stopping timer".into());
            self.stop_repeat_timer();
        }

        // Periodic cleanup
        let _ = self.cleanup_stuck_state();

        Ok(())
    }

    fn handle_immediate_movement(&mut self, key: &str) -> Result<(), JsValue> {
        console::log_2(&"Handling immediate movement:".into(), &key.into());

        // Rate limit movement to prevent overwhelming the system
        let now = js_sys::Date::now();
        if now - self.last_movement_time < 50.0 {
            console::log_1(&"Movement rate limited".into());
            return Ok(());
        }

        let moved = match key {
            "ArrowUp" => {
                console::log_1(&"Moving up".into());
                self.game.move_player(0, -1)
            }
            "ArrowDown" => {
                console::log_1(&"Moving down".into());
                self.game.move_player(0, 1)
            }
            "ArrowLeft" => {
                console::log_1(&"Moving left".into());
                self.game.move_player(-1, 0)
            }
            "ArrowRight" => {
                console::log_1(&"Moving right".into());
                self.game.move_player(1, 0)
            }
            _ => false,
        };

        if moved {
            console::log_1(&"Player moved successfully".into());
            self.process_movement()?;
            self.last_movement_time = now;
            self.consecutive_movement_count = 0; // Reset counter on successful movement
        } else {
            console::log_1(&"Player movement failed".into());
        }
        Ok(())
    }

    fn handle_single_key_action(&mut self, key: &str) -> Result<(), JsValue> {
        console::log_2(&"handle_single_key_action called with:".into(), &key.into());

        // Prevent key repeat spam for non-movement actions
        let now = js_sys::Date::now();
        let time_since_last = now - self.last_key_time;
        console::log_2(&"Time since last key:".into(), &time_since_last.into());

        if time_since_last < self.key_repeat_delay {
            console::log_1(&"Key blocked by repeat delay".into());
            return Ok(());
        }
        self.last_key_time = now;

        console::log_2(
            &"Current game state:".into(),
            &format!("{:?}", self.game.game_state).into(),
        );

        match self.game.game_state.clone() {
            GameState::Playing => {
                console::log_1(&"Calling handle_gameplay_input".into());
                self.handle_gameplay_input(key)
            }
            GameState::MainMenu => {
                console::log_1(&"Calling handle_menu_input".into());
                self.handle_menu_input(key)
            }
            GameState::Inventory => {
                console::log_1(&"Calling handle_inventory_input".into());
                self.handle_inventory_input(key)
            }
            GameState::Character => {
                console::log_1(&"Calling handle_character_input".into());
                self.handle_character_input(key)
            }
            GameState::Combat(pos) => {
                console::log_1(&"Calling handle_combat_input".into());
                self.handle_combat_input(key, pos)
            }
            _ => {
                console::log_1(&"Unknown game state, ignoring key".into());
                Ok(())
            }
        }
    }

    fn any_keys_pressed(&self) -> bool {
        self.pressed_keys.values().any(|&pressed| pressed)
    }

    fn any_movement_keys_pressed(&self) -> bool {
        ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"]
            .iter()
            .any(|&key| *self.pressed_keys.get(key).unwrap_or(&false))
    }

    fn clear_all_key_states(&mut self) {
        // Clear all key states to prevent stuck keys
        for key in ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].iter() {
            self.pressed_keys.insert(key.to_string(), false);
        }
        self.stop_repeat_timer();
        // Force reset timer count as safety measure
        self.timer_count = 0;
    }

    fn start_simple_repeat_timer(&mut self) -> Result<(), JsValue> {
        // Only start if not already running
        if self.key_repeat_timeout.is_some() {
            return Ok(());
        }

        // Prevent infinite timer loops by limiting consecutive movements
        if self.consecutive_movement_count >= self.max_consecutive_movements {
            console::log_1(&"Movement timer limit reached, stopping timer".into());
            self.consecutive_movement_count = 0;
            return Ok(());
        }

        // Prevent timer accumulation
        if self.timer_count >= self.max_timers {
            console::log_1(&"Too many timers active, skipping timer creation".into());
            return Ok(());
        }

        let window = window().unwrap();
        let game_ptr = self as *mut WebGame;

        let closure = Closure::wrap(Box::new(move || unsafe {
            if let Some(game) = game_ptr.as_mut() {
                if let Err(e) = game.process_movement_repeat() {
                    console::log_2(&"Error in movement repeat:".into(), &e);
                    // Clear key states on error to prevent infinite loops
                    game.clear_all_key_states();
                }
            }
        }) as Box<dyn FnMut()>);

        match window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            self.movement_repeat_rate as i32,
        ) {
            Ok(id) => {
                self.key_repeat_timeout = Some(id);
                self.timer_count += 1;
                closure.forget();
                Ok(())
            }
            Err(e) => {
                console::log_2(&"Failed to set movement timer:".into(), &e);
                self.clear_all_key_states();
                Err(e)
            }
        }
    }

    fn stop_repeat_timer(&mut self) {
        if let Some(id) = self.key_repeat_timeout.take() {
            if let Some(window) = window() {
                window.clear_timeout_with_handle(id);
            }
            if self.timer_count > 0 {
                self.timer_count -= 1;
            }
        }
        // Reset the consecutive movement counter when stopping
        self.consecutive_movement_count = 0;
    }

    fn process_movement_repeat(&mut self) -> Result<(), JsValue> {
        self.key_repeat_timeout = None;
        if self.timer_count > 0 {
            self.timer_count -= 1;
        }
        self.consecutive_movement_count += 1;

        if !matches!(self.game.game_state, GameState::Playing) {
            console::log_1(&"Game state not Playing, stopping repeat".into());
            self.consecutive_movement_count = 0;
            return Ok(());
        }

        // Rate limiting for repeated movement
        let now = js_sys::Date::now();
        if now - self.last_movement_time < 80.0 {
            // Still continue timer but skip this movement
            if self.any_movement_keys_pressed()
                && self.consecutive_movement_count < self.max_consecutive_movements
            {
                self.start_simple_repeat_timer()?;
            }
            return Ok(());
        }

        let mut moved = false;
        let mut movement_attempted = false;

        // Only process one direction at a time to avoid conflicts
        if *self.pressed_keys.get("ArrowUp").unwrap_or(&false) {
            movement_attempted = true;
            if self.game.move_player(0, -1) {
                moved = true;
            }
        } else if *self.pressed_keys.get("ArrowDown").unwrap_or(&false) {
            movement_attempted = true;
            if self.game.move_player(0, 1) {
                moved = true;
            }
        } else if *self.pressed_keys.get("ArrowLeft").unwrap_or(&false) {
            movement_attempted = true;
            if self.game.move_player(-1, 0) {
                moved = true;
            }
        } else if *self.pressed_keys.get("ArrowRight").unwrap_or(&false) {
            movement_attempted = true;
            if self.game.move_player(1, 0) {
                moved = true;
            }
        }

        if moved {
            self.process_movement()?;
            self.last_movement_time = now;
        }

        // Continue timer if movement keys are still pressed, but with safety limits
        if self.any_movement_keys_pressed()
            && self.consecutive_movement_count < self.max_consecutive_movements
            && movement_attempted
        {
            self.start_simple_repeat_timer()?;
        } else if self.consecutive_movement_count >= self.max_consecutive_movements {
            console::log_1(&"Maximum consecutive movements reached, resetting timer".into());
            self.consecutive_movement_count = 0;
        }

        Ok(())
    }

    // Add a safety mechanism to periodically clean up stuck state
    fn cleanup_stuck_state(&mut self) -> Result<(), JsValue> {
        let now = js_sys::Date::now();

        // If it's been too long since last movement and we have pressed keys, clear them
        if now - self.last_movement_time > 5000.0 && self.any_movement_keys_pressed() {
            console::log_1(&"Cleaning up stuck key states".into());
            self.clear_all_key_states();
        }

        // If consecutive count is too high, reset it
        if self.consecutive_movement_count > self.max_consecutive_movements + 10 {
            console::log_1(&"Resetting stuck consecutive movement counter".into());
            self.consecutive_movement_count = 0;
            self.stop_repeat_timer();
        }

        Ok(())
    }

    fn handle_gameplay_input(&mut self, key: &str) -> Result<(), JsValue> {
        match key {
            // Movement keys are now handled by the continuous processing loop
            "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => {
                // Movement is handled in process_held_keys()
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
        console::log_2(&"Menu input received:".into(), &key.into());
        match key {
            "1" => {
                console::log_1(&"Starting new game...".into());
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
        console::log_1(&"start_new_game() called".into());

        // Clear any existing timers to prevent conflicts
        self.stop_repeat_timer();
        self.clear_all_key_states();

        console::log_1(&"Starting WASM-optimized game initialization".into());

        // Try minimal game creation with detailed logging
        match self.create_minimal_wasm_game() {
            Ok(_) => {
                console::log_1(&"Minimal game creation successful".into());
                self.add_message("Welcome to the dungeon! Use arrow keys to move.");
                self.add_message("Press 'i' for inventory, 'c' for character, 'g' to get items.");

                // Schedule first render for next frame
                let game_ptr = self as *mut WebGame;
                let render_closure = Closure::wrap(Box::new(move || unsafe {
                    if let Some(game) = game_ptr.as_mut() {
                        console::log_1(&"Performing first render".into());
                        let _ = game.render_game();
                        console::log_1(&"First render complete".into());
                    }
                }) as Box<dyn FnMut()>);

                let window = web_sys::window().unwrap();
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    render_closure.as_ref().unchecked_ref(),
                    16, // Next frame
                );
                render_closure.forget();
            }
            Err(e) => {
                console::log_2(
                    &"Game creation failed, using emergency fallback:".into(),
                    &e,
                );
                self.create_emergency_fallback();
            }
        }

        Ok(())
    }

    fn create_minimal_wasm_game(&mut self) -> Result<(), JsValue> {
        console::log_1(&"Creating player...".into());
        let player = Player::new("Hero".to_string(), ClassType::Warrior);
        console::log_1(&"Player created successfully".into());

        console::log_1(&"Creating game with timeout protection...".into());

        // Create a minimal game state first
        self.game = Game {
            player,
            dungeons: vec![],
            current_dungeon_index: 0,
            game_state: GameState::Playing,
            combat_started: false,
            #[cfg(target_arch = "wasm32")]
            first_visibility_update_done: false,
            #[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
            last_render_time: None,
        };

        // Create minimal dungeon manually
        console::log_1(&"Creating minimal dungeon...".into());
        let minimal_dungeon = self.create_minimal_dungeon()?;
        self.game.dungeons.push(minimal_dungeon);
        console::log_1(&"Minimal dungeon created successfully".into());

        // Defer visibility update
        console::log_1(&"Deferring visibility update...".into());

        Ok(())
    }

    fn create_minimal_dungeon(&self) -> Result<Dungeon, JsValue> {
        console::log_1(&"Building minimal level...".into());

        // Create very simple level manually
        let mut level = Level::new(40, 25); // Larger but still manageable
        level.level_num = 1;

        // Create one room manually
        let room = Room::new(5, 5, 30, 15);

        // Fill the room with floor tiles
        for y in (room.y1 + 1)..room.y2 {
            for x in (room.x1 + 1)..room.x2 {
                if y < level.height as i32 && x < level.width as i32 {
                    level.tiles[y as usize][x as usize] = Tile::floor();
                }
            }
        }

        level.rooms.push(room);
        level.player_position = Position::new(10, 10);

        // Create minimal dungeon
        let dungeon = Dungeon {
            name: "Test Dungeon".to_string(),
            dungeon_type: crate::world::DungeonType::Ruins,
            levels: vec![level],
            current_level: 0,
            difficulty: 1,
        };

        console::log_1(&"Minimal dungeon built successfully".into());
        Ok(dungeon)
    }

    fn create_emergency_fallback(&mut self) {
        console::log_1(&"Creating emergency fallback game state".into());

        // Create absolute minimal state that cannot fail
        self.game = Game {
            player: Player::new("Hero".to_string(), ClassType::Warrior),
            dungeons: vec![],
            current_dungeon_index: 0,
            game_state: GameState::Playing,
            combat_started: false,
            #[cfg(target_arch = "wasm32")]
            first_visibility_update_done: false,
            #[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
            last_render_time: None,
        };

        self.add_message("Emergency mode: Minimal game state loaded");
        self.add_message("Some features may be limited");
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
        self.show_main_menu()
    }

    fn render_game(&mut self) -> Result<(), JsValue> {
        // Add comprehensive render safety checks
        let now = js_sys::Date::now();

        // Reset render count every second
        if now - self.last_render_check > 1000.0 {
            self.render_count = 0;
            self.last_render_check = now;
        }

        // Limit renders per second to prevent infinite loops
        if self.render_count > 120 {
            console::log_1(&"Warning: Too many renders per second, throttling".into());
            return Ok(());
        }

        // Frame rate limiting
        static mut LAST_RENDER_TIME: f64 = 0.0;
        unsafe {
            if now - LAST_RENDER_TIME < 16.0 {
                // Skip render if called too frequently (60 FPS limit)
                return Ok(());
            }
            LAST_RENDER_TIME = now;
        }

        self.render_count += 1;
        self.clear_canvas()?;

        // Update visibility only when necessary
        match self.game.game_state {
            GameState::Playing | GameState::Combat(_) => {
                self.update_visibility();
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

    /// WASM-specific map rendering with camera following (independent of desktop)
    fn render_map(&mut self) -> Result<(), JsValue> {
        // Extract all needed data first to avoid borrowing issues
        let level_width = self.game.current_level().width as i32;
        let level_height = self.game.current_level().height as i32;
        let player_pos = self.game.player_position();

        // WASM-specific camera system (can be modified independently of desktop)
        let center_x = VIEW_WIDTH / 2;
        let center_y = VIEW_HEIGHT / 2;

        // WASM-specific tile collection with camera offset
        let mut tile_data = Vec::new();
        let mut enemy_positions = Vec::new();
        let mut item_positions = Vec::new();

        {
            let level = self.game.current_level();
            // WASM: Render viewport centered on player (independent camera system)
            let mut tiles_processed = 0;
            const MAX_RENDER_TILES: usize = 2000; // Safety limit for WASM

            for screen_y in 0..VIEW_HEIGHT {
                for screen_x in 0..VIEW_WIDTH {
                    if tiles_processed >= MAX_RENDER_TILES {
                        console::log_1(
                            &"Warning: Render tile limit reached, stopping render".into(),
                        );
                        break;
                    }
                    // WASM: Calculate map coordinates with camera offset
                    let map_x = player_pos.x - center_x + screen_x;
                    let map_y = player_pos.y - center_y + screen_y;

                    // WASM: Bounds checking for camera viewport
                    if map_x >= 0 && map_x < level_width && map_y >= 0 && map_y < level_height {
                        let tile = &level.tiles[map_y as usize][map_x as usize];
                        // WASM: Store screen coords for rendering, map coords for data
                        tile_data.push((
                            screen_x,
                            screen_y,
                            map_x,
                            map_y,
                            tile.visible,
                            tile.explored,
                            tile.tile_type.clone(),
                        ));
                        tiles_processed += 1;
                    }
                }
            }
            if tiles_processed >= MAX_RENDER_TILES {
                return Ok(());
            }

            // WASM: Collect entity positions relative to camera viewport
            for (pos, _enemy) in &level.enemies {
                let screen_x = pos.x - player_pos.x + center_x;
                let screen_y = pos.y - player_pos.y + center_y;
                if screen_x >= 0 && screen_x < VIEW_WIDTH && screen_y >= 0 && screen_y < VIEW_HEIGHT
                {
                    enemy_positions.push((screen_x, screen_y));
                }
            }
            for (pos, _item) in &level.items {
                let screen_x = pos.x - player_pos.x + center_x;
                let screen_y = pos.y - player_pos.y + center_y;
                if screen_x >= 0 && screen_x < VIEW_WIDTH && screen_y >= 0 && screen_y < VIEW_HEIGHT
                {
                    item_positions.push((screen_x, screen_y));
                }
            }
        }

        // WASM: Render everything using camera-relative coordinates with bounds check
        let mut entities_rendered = 0;
        const MAX_RENDER_ENTITIES: usize = 500; // Safety limit for entities

        for (screen_x, screen_y, map_x, map_y, visible, explored, tile_type) in tile_data {
            if entities_rendered >= MAX_RENDER_ENTITIES {
                console::log_1(&"Warning: Entity render limit reached".into());
                break;
            }
            if visible {
                self.render_tile(screen_x, screen_y, &tile_type)?;

                // WASM: Render entities (player always centered in WASM version)
                if player_pos.x == map_x && player_pos.y == map_y {
                    self.render_player(screen_x, screen_y)?;
                } else if enemy_positions.contains(&(screen_x, screen_y)) {
                    self.render_enemy(screen_x, screen_y)?;
                } else if item_positions.contains(&(screen_x, screen_y)) {
                    self.render_item(screen_x, screen_y)?;
                }
                entities_rendered += 1;
            } else if explored {
                self.render_fog_tile(screen_x, screen_y)?;
                entities_rendered += 1;
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
