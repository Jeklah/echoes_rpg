//! GUI module for Windows graphical interface using egui
//! Provides a native Windows application with text-based gameplay

#[cfg(feature = "gui")]
use crate::character::{ClassType, Player};
#[cfg(feature = "gui")]
use crate::game::Game;
#[cfg(feature = "gui")]
use crate::input::InputHandler;
use crate::inventory::{InventoryManager, ItemType};
use crate::item::{equipment, Item};
#[cfg(feature = "gui")]
use crate::world::{FogOfWar, Position};
#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use egui::{Color32, FontFamily, FontId, RichText};

#[cfg(feature = "gui")]
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum CharacterCreationState {
    EnteringName,
    SelectingClass,
}

#[allow(dead_code)]
pub struct EchoesApp {
    game: Option<Game>,
    terminal_buffer: Vec<Vec<char>>,
    color_buffer: Vec<Vec<Option<Color32>>>,
    input_buffer: String,
    last_key: Option<char>,
    show_combat_tutorial: bool,
    window_size: (f32, f32),
    font_size: f32,
    char_width: f32,
    char_height: f32,
    cursor_pos: (usize, usize),
    terminal_size: (usize, usize),
    ui_messages: Vec<String>,
    message_log: Vec<(String, f64)>, // Messages with timestamps for fading
    message_log_visible: bool,       // Toggle for message log visibility
    game_initialized: bool,
    character_name: String,
    character_class: Option<ClassType>,
    creating_character: bool,
    character_creation_state: CharacterCreationState,
    showing_inventory: bool, // Whether the inventory screen is shown
    showing_character: bool, // Whether the character screen is shown
    main_menu: bool,
    input_handler: InputHandler,
    frame_count: u64,
    in_combat: bool,
    combat_enemy_pos: Option<Position>,
    combat_messages: Vec<String>,
}

#[cfg(feature = "gui")]
impl Default for EchoesApp {
    fn default() -> Self {
        let mut app = Self {
            game: None,
            terminal_buffer: vec![vec![' '; 150]; 50],
            color_buffer: vec![vec![Some(Color32::from_rgb(192, 192, 192)); 150]; 50],
            input_buffer: String::new(),
            last_key: None,
            show_combat_tutorial: false,
            window_size: (1200.0, 800.0),
            font_size: 14.0,
            char_width: 8.0,
            char_height: 16.0,
            cursor_pos: (0, 0),
            terminal_size: (150, 50),
            ui_messages: Vec::with_capacity(25), // Pre-allocate more space for extended message history
            message_log: Vec::with_capacity(50), // Larger capacity for dedicated message log
            message_log_visible: true,           // Show message log by default
            game_initialized: false,
            character_name: String::new(),
            character_class: None,
            creating_character: false,
            character_creation_state: CharacterCreationState::EnteringName,
            showing_inventory: false,
            showing_character: false,
            main_menu: true,
            input_handler: InputHandler::new(),
            frame_count: 0,
            in_combat: false,
            combat_enemy_pos: None,
            combat_messages: Vec::new(),
        };
        app.init_terminal();
        app
    }
}

#[cfg(feature = "gui")]
#[allow(dead_code)]
impl EchoesApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure dark theme and colors for terminal appearance
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::BLACK;
        visuals.panel_fill = Color32::BLACK;
        visuals.extreme_bg_color = Color32::BLACK;
        visuals.faint_bg_color = Color32::from_gray(10);
        visuals.widgets.noninteractive.bg_stroke.color = Color32::from_gray(30);
        cc.egui_ctx.set_visuals(visuals);

        let mut app = Self::default();
        app.init_terminal();
        app
    }

    fn create_fog_of_war() -> FogOfWar {
        crate::world::create_standard_fog_of_war()
    }

    fn init_terminal(&mut self) {
        // Initialize terminal buffer and color buffer with larger size
        self.terminal_buffer = vec![vec![' '; self.terminal_size.0]; self.terminal_size.1];
        self.color_buffer = vec![
            vec![Some(Color32::from_rgb(192, 192, 192)); self.terminal_size.0];
            self.terminal_size.1
        ];
        self.clear_screen();
        self.show_main_menu();
    }

    fn clear_screen(&mut self) {
        for line in &mut self.terminal_buffer {
            *line = vec![' '; self.terminal_size.0];
        }
        for line in &mut self.color_buffer {
            for color in line {
                *color = Some(Color32::from_rgb(192, 192, 192));
            }
        }
        self.cursor_pos = (0, 0);
    }

    fn print_at(&mut self, x: usize, y: usize, text: &str, color: Option<Color32>) {
        if y < self.terminal_buffer.len() && x < self.terminal_size.0 {
            let line = &mut self.terminal_buffer[y];
            let end_x = (x + text.len()).min(line.len());
            if x < line.len() {
                for (i, c) in text[..end_x - x].chars().enumerate() {
                    if x + i < line.len() {
                        line[x + i] = c;
                    }
                }

                // Set colors for each character
                let color_to_use = color.unwrap_or(Color32::from_rgb(192, 192, 192));
                for i in x..end_x {
                    if i < self.color_buffer[y].len() {
                        self.color_buffer[y][i] = Some(color_to_use);
                    }
                }
            }
        }
    }

    fn show_main_menu(&mut self) {
        self.clear_screen();
        let title = "*** ECHOES OF THE FORGOTTEN REALM ***";
        let subtitle = "A Text-Based RPG Adventure";

        let center_x = (self.terminal_size.0.saturating_sub(title.len())) / 2;
        let center_y = self.terminal_size.1 / 2;

        self.print_at(center_x, center_y - 3, title, Some(Color32::YELLOW));
        self.print_at(
            (self.terminal_size.0 - subtitle.len()) / 2,
            center_y - 1,
            subtitle,
            Some(Color32::from_rgb(0, 255, 0)),
        );

        self.print_at(center_x, center_y + 2, "1. Start New Game", None);
        self.print_at(center_x, center_y + 3, "2. Exit", None);

        self.print_at(
            center_x,
            center_y + 6,
            "Press 1 to start or 2 to exit",
            Some(Color32::from_rgb(0, 255, 255)),
        );
    }

    fn handle_main_menu_input(&mut self, action: &crate::input::InputAction) {
        match action {
            crate::input::InputAction::MenuOption(1) => {
                self.main_menu = false;
                self.creating_character = true;
                self.character_name.clear(); // Clear any residual input
                self.character_class = None; // Reset class selection
                self.character_creation_state = CharacterCreationState::EnteringName; // Reset to name input
                self.input_handler.clear_state(); // Clear input state
                self.show_character_creation();
            }
            crate::input::InputAction::MenuOption(2) => {
                // Exit application - will be handled by the framework
                std::process::exit(0);
            }
            _ => {}
        }
    }

    fn show_character_creation(&mut self) {
        self.clear_screen();

        let title = "Character Creation";
        let center_x = (self.terminal_size.0 - title.len()) / 2;

        self.print_at(center_x, 5, title, Some(Color32::YELLOW));

        match self.character_creation_state {
            CharacterCreationState::EnteringName => {
                let display_name = if self.character_name.is_empty() {
                    "Name: _".to_string()
                } else {
                    format!("Name: {}_", self.character_name)
                };
                self.print_at(10, 10, &display_name, None);
                self.print_at(
                    10,
                    13,
                    "Type your character name and press Enter",
                    Some(Color32::from_rgb(0, 255, 255)),
                );
                self.print_at(
                    10,
                    15,
                    "(Use Backspace to delete, Esc to go back)",
                    Some(Color32::DARK_GRAY),
                );
            }
            CharacterCreationState::SelectingClass => {
                self.print_at(10, 10, &format!("Name: {}", self.character_name), None);
                self.print_at(
                    10,
                    13,
                    "Choose your class:",
                    Some(Color32::from_rgb(0, 255, 255)),
                );
                self.print_at(10, 15, "1. Warrior - Strong melee fighter", None);
                self.print_at(10, 16, "2. Mage - Powerful spellcaster", None);
                self.print_at(10, 17, "3. Ranger - Balanced archer", None);
                self.print_at(10, 18, "4. Cleric - Healer and support", None);
                self.print_at(
                    10,
                    20,
                    "(Press number key to select class)",
                    Some(Color32::DARK_GRAY),
                );
            }
        }
    }

    fn handle_character_creation_input(&mut self, action: &crate::input::InputAction) {
        match self.character_creation_state {
            CharacterCreationState::EnteringName => {
                match action {
                    crate::input::InputAction::Character(c) => {
                        // Add character to name if it's valid and we have space
                        if (c.is_alphanumeric() || *c == ' ') && self.character_name.len() < 20 {
                            self.character_name.push(*c);
                            self.show_character_creation();
                        }
                    }
                    crate::input::InputAction::Backspace => {
                        if !self.character_name.is_empty() {
                            self.character_name.pop();
                            self.show_character_creation();
                        }
                    }
                    crate::input::InputAction::Enter => {
                        // Proceed to class selection if we have a name
                        if !self.character_name.is_empty() {
                            self.character_creation_state = CharacterCreationState::SelectingClass;
                            self.show_character_creation();
                        } else {
                            // Set default name if empty
                            self.character_name = "Hero".to_string();
                            self.character_creation_state = CharacterCreationState::SelectingClass;
                            self.show_character_creation();
                        }
                    }
                    crate::input::InputAction::Exit => {
                        // Go back to main menu
                        self.main_menu = true;
                        self.creating_character = false;
                        self.character_name.clear();
                        self.character_creation_state = CharacterCreationState::EnteringName;
                        self.show_main_menu();
                    }
                    _ => {}
                }
            }
            CharacterCreationState::SelectingClass => {
                match action {
                    crate::input::InputAction::MenuOption(1) => {
                        self.character_class = Some(crate::character::ClassType::Warrior);
                        self.finish_character_creation();
                    }
                    crate::input::InputAction::MenuOption(2) => {
                        self.character_class = Some(crate::character::ClassType::Mage);
                        self.finish_character_creation();
                    }
                    crate::input::InputAction::MenuOption(3) => {
                        self.character_class = Some(crate::character::ClassType::Ranger);
                        self.finish_character_creation();
                    }
                    crate::input::InputAction::MenuOption(4) => {
                        self.character_class = Some(crate::character::ClassType::Cleric);
                        self.finish_character_creation();
                    }
                    crate::input::InputAction::Backspace | crate::input::InputAction::Exit => {
                        // Go back to name input
                        self.character_creation_state = CharacterCreationState::EnteringName;
                        self.show_character_creation();
                    }
                    _ => {}
                }
            }
        }
    }

    fn finish_character_creation(&mut self) {
        if let Some(class_type) = self.character_class {
            let _class = crate::character::Class::new(class_type);
            let player = Player::new(self.character_name.clone(), class_type);
            self.game = Some(Game::new(player));
            self.creating_character = false;
            self.game_initialized = true;
            self.show_combat_tutorial = true;
            self.display_combat_tutorial();
        }
    }

    fn display_combat_tutorial(&mut self) {
        self.clear_screen();

        let tutorial_lines = vec![
            "=== COMBAT TUTORIAL ===",
            "",
            "Welcome to your first combat encounter!",
            "",
            "Combat in Echoes RPG is turn-based:",
            "",
            "1. Attack - Basic attack with your weapon",
            "2. Use Ability - Special ability (costs mana)",
            "3. Use Item - Consumable from inventory",
            "4. Flee - Attempt to escape combat",
            "",
            "After your action, enemies counter-attack.",
            "Victory grants experience, gold, and items!",
            "",
            "Tips:",
            "• Use healing potions when health is low",
            "• Abilities deal more damage but cost mana",
            "• Sometimes fleeing is the best option",
            "",
            "Press any key to start your adventure...",
        ];

        for (i, line) in tutorial_lines.iter().enumerate() {
            let x = (self.terminal_size.0 - line.len()) / 2;
            let color = if line.starts_with("===") {
                Some(Color32::YELLOW)
            } else if line.starts_with("Tips:")
                || line.starts_with("1.")
                || line.starts_with("2.")
                || line.starts_with("3.")
                || line.starts_with("4.")
            {
                Some(Color32::from_rgb(0, 255, 255))
            } else {
                None
            };
            self.print_at(x, 5 + i, line, color);
        }
    }

    fn start_game(&mut self) {
        self.show_combat_tutorial = false;
        // Set game state to Playing so enemies can move
        if let Some(ref mut game) = self.game {
            game.game_state = crate::game::GameState::Playing;
        }
        // Don't render here, will be handled in main update loop
    }

    fn render_game_screen_safe(&mut self, game: &Game) {
        // Request a repaint to keep UI responsive
        if self.showing_inventory || self.showing_character {
            eframe::egui::Context::request_repaint(&eframe::egui::Context::default());
        }

        self.clear_screen();

        // Render game map using centralized fog of war system
        let level = game.current_level();
        let player_pos = level.player_position;
        let fog_of_war = Self::create_fog_of_war();

        // Calculate view area (centered on player) - use larger screen
        let view_width = 90;
        let view_height = 35;
        let start_x = 5;
        let start_y = 3;

        // Draw map
        for screen_y in 0..view_height {
            for screen_x in 0..view_width {
                let map_x = player_pos.x - view_width as i32 / 2 + screen_x as i32;
                let map_y = player_pos.y - view_height as i32 / 2 + screen_y as i32;
                let pos = Position::new(map_x, map_y);

                // Use centralized fog of war processing
                let fog_result = fog_of_war.process_position(level, pos, player_pos);

                // Convert fog color to egui color
                let egui_color = fog_result.color.map(|c| FogOfWar::to_egui_color(&c));

                if fog_result.should_render {
                    self.print_at(
                        start_x + screen_x,
                        start_y + screen_y,
                        &fog_result.character.to_string(),
                        egui_color,
                    );
                }
            }
        }

        // Draw UI panel
        let ui_x = start_x + view_width + 3;
        let player = &game.player;

        self.print_at(
            ui_x,
            start_y,
            &player.name,
            Some(Color32::from_rgb(0, 255, 255)),
        );
        self.print_at(
            ui_x,
            start_y + 1,
            &format!("Level {} {}", player.level, player.class.class_type),
            None,
        );
        self.print_at(
            ui_x,
            start_y + 2,
            &format!("HP: {}/{}", player.health, player.max_health),
            None,
        );
        self.print_at(
            ui_x,
            start_y + 3,
            &format!("MP: {}/{}", player.mana, player.max_mana),
            None,
        );
        self.print_at(
            ui_x,
            start_y + 4,
            &format!("XP: {}/{}", player.experience, player.level * 100),
            None,
        );
        self.print_at(ui_x, start_y + 5, &format!("Gold: {}", player.gold), None);

        // Draw controls
        let controls_y = start_y + 8;
        self.print_at(
            ui_x,
            controls_y,
            "Controls:",
            Some(Color32::from_rgb(0, 255, 255)),
        );
        self.print_at(ui_x, controls_y + 1, "WASD: Move", None);
        self.print_at(ui_x, controls_y + 2, "I: Toggle Inventory", None);
        self.print_at(ui_x, controls_y + 3, "C: Toggle Character", None);
        self.print_at(ui_x, controls_y + 4, "G: Get item", None);
        self.print_at(ui_x, controls_y + 5, "Q: Quit", None);

        // Draw legend
        let legend_y = controls_y + 8;
        self.print_at(
            ui_x,
            legend_y,
            "Legend:",
            Some(Color32::from_rgb(0, 255, 255)),
        );
        self.print_at(ui_x, legend_y + 1, "@ - You", None);
        self.print_at(ui_x, legend_y + 2, "E - Enemy", None);
        self.print_at(ui_x, legend_y + 3, "! - Item", None);
        self.print_at(ui_x, legend_y + 4, "# - Wall", None);
        self.print_at(ui_x, legend_y + 5, ". - Floor", None);
        self.print_at(ui_x, legend_y + 6, "+ - Door", None);
        self.print_at(ui_x, legend_y + 7, "C - Chest", None);
        self.print_at(ui_x, legend_y + 8, "> - Stairs Down", None);
        self.print_at(ui_x, legend_y + 9, "< - Stairs Up", None);
    }

    fn handle_game_input(&mut self, key: char) {
        if let Some(ref mut game) = self.game {
            if self.in_combat {
                self.handle_combat_input(key);
            } else {
                match key {
                    'w' | 'W' => {
                        if game.move_player(0, -1) {
                            game.update_visibility();
                            if !matches!(game.game_state, crate::game::GameState::Combat(_)) {
                                game.process_turn();
                            }
                            self.check_for_combat();
                        }
                    }
                    's' | 'S' => {
                        if game.move_player(0, 1) {
                            game.update_visibility();
                            if !matches!(game.game_state, crate::game::GameState::Combat(_)) {
                                game.process_turn();
                            }
                            self.check_for_combat();
                        }
                    }
                    'a' | 'A' => {
                        if game.move_player(-1, 0) {
                            game.update_visibility();
                            if !matches!(game.game_state, crate::game::GameState::Combat(_)) {
                                game.process_turn();
                            }
                            self.check_for_combat();
                        }
                    }
                    'd' | 'D' => {
                        if game.move_player(1, 0) {
                            game.update_visibility();
                            if !matches!(game.game_state, crate::game::GameState::Combat(_)) {
                                game.process_turn();
                            }
                            self.check_for_combat();
                        }
                    }
                    'g' | 'G' => {
                        // Try to get item at current position or adjacent chest
                        if let Some(result) = game.try_get_item() {
                            // Add a visual prefix for item/chest interactions with color coding
                            let message = if result.contains("chest") {
                                format!("📦 {}", result)
                            } else {
                                format!("🔍 {}", result)
                            };
                            self.add_message(message);
                        }
                    }
                    'i' | 'I' => {
                        // Toggle inventory screen
                        self.showing_inventory = !self.showing_inventory;
                        if self.showing_inventory {
                            self.showing_character = false; // Close character screen if open
                            self.add_message("🎒 Inventory opened - Press number keys 1-9 to equip items or use the Equip buttons".to_string());
                        } else {
                            self.add_message("🎒 Inventory closed".to_string());
                        }
                    }
                    'c' | 'C' => {
                        // Toggle character screen
                        self.showing_character = !self.showing_character;
                        if self.showing_character {
                            self.showing_inventory = false; // Close inventory screen if open
                            self.add_message("👤 Character screen opened".to_string());
                        } else {
                            self.add_message("👤 Character screen closed".to_string());
                        }
                    }
                    'm' | 'M' => {
                        // Toggle message log visibility
                        self.toggle_message_log();
                        self.add_message(
                            if self.message_log_visible {
                                "📜 Message log visible (press M to hide)"
                            } else {
                                "📜 Message log hidden (press M to show)"
                            }
                            .to_string(),
                        );
                    }
                    'q' | 'Q' => {
                        // Quit to main menu
                        self.game_initialized = false;
                        self.main_menu = true;
                        self.show_main_menu();
                    }
                    _ => {}
                }
            }
        }
    }

    fn check_for_combat(&mut self) {
        if let Some(ref mut game) = self.game {
            match game.game_state {
                crate::game::GameState::Combat(enemy_pos) => {
                    if !self.in_combat || game.combat_started {
                        self.in_combat = true;
                        self.combat_enemy_pos = Some(enemy_pos);
                        // Get enemy name before setting combat_started to false
                        let enemy_name = game
                            .current_level()
                            .get_enemy_at(&enemy_pos)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| "Unknown Enemy".to_string());

                        self.combat_messages.clear();
                        self.combat_messages
                            .push(format!("Combat started with {}!", enemy_name));
                        game.combat_started = false;
                    }
                }
                _ => {
                    if self.in_combat {
                        self.in_combat = false;
                        self.combat_enemy_pos = None;
                    }
                }
            }
        }
    }

    /// Handle inventory hotkey actions (1-9 keys for equipping/using items)
    fn handle_inventory_hotkey(&mut self, action: &crate::input::InputAction) {
        use crate::input::InputAction;

        let item_index = match action {
            // Handle MenuOption actions (number keys 1-9)
            InputAction::MenuOption(n) if *n >= 1 && *n <= 9 => Some(*n as usize - 1),
            // Handle Character actions for backward compatibility
            InputAction::Character(c) if c.is_ascii_digit() && *c != '0' => {
                c.to_digit(10).map(|d| d as usize - 1)
            }
            _ => None,
        };

        if let Some(index) = item_index {
            if let Some(game) = &mut self.game {
                if index < InventoryManager::get_item_count(&game.player) {
                    let result = InventoryManager::use_item(&mut game.player, index);
                    if result.success {
                        self.add_message("🎒 Item used successfully!".to_string());
                    } else {
                        self.add_message(format!("🎒 Error: {}", result.message));
                    }
                } else {
                    self.add_message("🎒 Invalid item number".to_string());
                }
            }
        }
    }

    fn handle_combat_input(&mut self, key: char) {
        if let Some(ref mut game) = self.game {
            if let Some(enemy_pos) = self.combat_enemy_pos {
                let action = match key {
                    '1' => Some(crate::combat::CombatAction::Attack),
                    '2' => {
                        // Use first ability if available
                        if !game.player.class.abilities.is_empty() {
                            Some(crate::combat::CombatAction::UseAbility(0))
                        } else {
                            self.combat_messages
                                .push("No abilities available!".to_string());
                            None
                        }
                    }
                    '3' => {
                        // Use first consumable if available
                        let consumables: Vec<_> = game
                            .player
                            .inventory
                            .items
                            .iter()
                            .enumerate()
                            .filter(|(_, item)| matches!(item, crate::item::Item::Consumable(_)))
                            .collect();
                        if !consumables.is_empty() {
                            Some(crate::combat::CombatAction::UseItem(consumables[0].0))
                        } else {
                            self.combat_messages
                                .push("No consumables available!".to_string());
                            None
                        }
                    }
                    '4' => Some(crate::combat::CombatAction::Flee),
                    _ => None,
                };

                if let Some(combat_action) = action {
                    self.process_combat_action(combat_action, enemy_pos);
                }
            }
        }
    }

    fn process_combat_action(&mut self, action: crate::combat::CombatAction, enemy_pos: Position) {
        if let Some(ref mut game) = self.game {
            if let Some(enemy) = game.current_level().get_enemy_at(&enemy_pos) {
                let mut enemy_clone = enemy.clone();
                let mut player_clone = game.player.clone();
                let result =
                    crate::combat::process_combat_turn(&mut player_clone, &mut enemy_clone, action);

                // Update game state
                game.player = player_clone;
                if !result.enemy_defeated && !result.player_fled {
                    if let Some(enemy_ref) = game.current_level_mut().get_enemy_at_mut(&enemy_pos) {
                        *enemy_ref = enemy_clone;
                    }
                }

                // Add combat messages
                for message in &result.messages {
                    self.combat_messages.push(message.clone());
                }

                // Check if combat is over
                if result.enemy_defeated {
                    game.current_level_mut().remove_enemy_at(&enemy_pos);
                    game.game_state = crate::game::GameState::Playing;
                    game.combat_started = false;
                    self.in_combat = false;
                    self.combat_enemy_pos = None;
                    // Add victory message directly to message log
                    self.add_message("⚔️ You were victorious!".to_string());

                    // Add any other combat messages to the message log
                    let messages: Vec<String> = self.combat_messages.drain(..).collect();
                    for msg in messages {
                        self.add_message(msg);
                    }
                } else if result.player_fled {
                    game.game_state = crate::game::GameState::Playing;
                    game.combat_started = false;
                    self.in_combat = false;
                    self.combat_enemy_pos = None;
                    // Add fled message directly to message log
                    self.add_message("🏃 You fled from combat!".to_string());

                    // Add any other combat messages to the message log
                    let messages: Vec<String> = self.combat_messages.drain(..).collect();
                    for msg in messages {
                        self.add_message(msg);
                    }
                } else if !game.player.is_alive() {
                    game.game_state = crate::game::GameState::GameOver;
                    self.in_combat = false;
                    self.combat_enemy_pos = None;
                }
            }
        }
    }

    fn handle_input(&mut self, action: &crate::input::InputAction) {
        // Skip processing character/inventory keys if those screens are already open
        if self.showing_inventory || self.showing_character {
            if let crate::input::InputAction::Character('i')
            | crate::input::InputAction::Character('I') = action
            {
                self.handle_game_input('i');
                return;
            }
            if let crate::input::InputAction::Character('c')
            | crate::input::InputAction::Character('C') = action
            {
                self.handle_game_input('c');
                return;
            }

            // Handle number keys for equipping items in inventory
            if self.showing_inventory {
                self.handle_inventory_hotkey(action);
            }
        }

        // Update last key for display purposes
        if let Some(c) = crate::input::InputHandler::get_character(action) {
            self.last_key = Some(c);
        } else {
            match action {
                crate::input::InputAction::Enter => self.last_key = Some('\r'),
                crate::input::InputAction::Backspace => self.last_key = Some('\u{8}'),
                crate::input::InputAction::MenuOption(n) => {
                    self.last_key = Some(char::from_digit(*n as u32, 10).unwrap_or('?'))
                }
                _ => {}
            }
        }

        if self.main_menu {
            self.handle_main_menu_input(action);
        } else if self.creating_character {
            self.handle_character_creation_input(action);
        } else if self.show_combat_tutorial {
            match action {
                crate::input::InputAction::Enter | crate::input::InputAction::Character(' ') => {
                    self.start_game();
                }
                _ => {}
            }
        } else if self.game_initialized {
            self.handle_game_input_legacy(action);
        }
    }

    fn handle_game_input_legacy(&mut self, action: &crate::input::InputAction) {
        // Convert action back to char for compatibility with existing game input
        let key_char = match action {
            crate::input::InputAction::Character(c) => *c,
            crate::input::InputAction::Enter => '\r',
            crate::input::InputAction::Backspace => '\u{8}',
            crate::input::InputAction::MenuOption(n) => {
                char::from_digit(*n as u32, 10).unwrap_or('0')
            }
            crate::input::InputAction::Move(direction) => {
                match direction {
                    crate::input::Direction::North => 'w',
                    crate::input::Direction::South => 's',
                    crate::input::Direction::West => 'a',
                    crate::input::Direction::East => 'd',
                    _ => return, // Ignore Up/Down for now
                }
            }
            _ => return, // Ignore other actions for now
        };

        self.handle_game_input(key_char);
    }

    fn render_combat_screen_safe(&mut self, game: &crate::game::Game) {
        self.clear_screen();

        // Draw combat UI
        self.print_at(5, 3, "=== COMBAT ===", Some(Color32::from_rgb(255, 255, 0)));

        if let Some(enemy_pos) = self.combat_enemy_pos {
            if let Some(enemy) = game.current_level().get_enemy_at(&enemy_pos) {
                // Display enemy info
                self.print_at(
                    5,
                    5,
                    &format!("Enemy: {}", enemy.name),
                    Some(Color32::from_rgb(255, 100, 100)),
                );
                self.print_at(
                    5,
                    6,
                    &format!("HP: {}/{}", enemy.health, enemy.max_health),
                    None,
                );

                // Display player info
                self.print_at(
                    5,
                    8,
                    &format!("Player: {}", game.player.name),
                    Some(Color32::from_rgb(100, 255, 100)),
                );
                self.print_at(
                    5,
                    9,
                    &format!("HP: {}/{}", game.player.health, game.player.max_health),
                    None,
                );
                self.print_at(
                    5,
                    10,
                    &format!("MP: {}/{}", game.player.mana, game.player.max_mana),
                    None,
                );

                // Display combat options
                self.print_at(
                    5,
                    12,
                    "Combat Actions:",
                    Some(Color32::from_rgb(255, 255, 255)),
                );
                self.print_at(5, 13, "1 - Attack", None);
                self.print_at(5, 14, "2 - Use Ability", None);
                self.print_at(5, 15, "3 - Use Item", None);
                self.print_at(5, 16, "4 - Flee", None);

                // Display combat messages
                self.print_at(5, 18, "Combat Log:", Some(Color32::from_rgb(255, 255, 255)));
                let start_line = 19;
                let max_messages = 10;
                let message_start = if self.combat_messages.len() > max_messages {
                    self.combat_messages.len() - max_messages
                } else {
                    0
                };

                // Clone the messages to avoid borrow checker issues
                let messages_to_display: Vec<String> = self
                    .combat_messages
                    .iter()
                    .skip(message_start)
                    .cloned()
                    .collect();
                for (i, message) in messages_to_display.iter().enumerate() {
                    if i < max_messages {
                        self.print_at(5, start_line + i, message, None);
                    }
                }
            }
        }
    }

    /// Displays the inventory screen with the player's items and equipment
    /// Allows equipping items and using consumables
    fn show_inventory_screen(&mut self, ui: &mut egui::Ui) {
        // Store indexes of items to equip or use
        let mut equip_item_index: Option<usize> = None;
        let mut use_item_index: Option<usize> = None;
        // Static variable to persist across frames for feedback messages
        static mut EQUIP_RESULT_MESSAGE: Option<(String, u64)> = None;

        if let Some(ref game) = self.game {
            let player = &game.player;

            // Create a window for the inventory
            let window = egui::Window::new("Inventory")
                .fixed_size([500.0, 500.0])
                .collapsible(false)
                .resizable(false);

            window.show(ui.ctx(), |ui| {
                ui.heading("Inventory");
                ui.add_space(10.0);

                ui.label(format!("Gold: {}", player.gold));
                ui.separator();

                // List inventory items
                if InventoryManager::is_empty(player) {
                    ui.label("Your inventory is empty.");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // Add keyboard shortcut hint at the top of the inventory
                        ui.label("Press 1-9 keys to quickly equip equipment items");
                        ui.separator();

                        let items = InventoryManager::get_items(player);
                        for (i, item_info) in items.iter().enumerate() {
                            ui.horizontal(|ui| {
                                let is_equipped = item_info.is_equipped;

                                let item_name = &item_info.name;
                                let prefix = format!("{}. ", i + 1);

                                // Create appropriate text with formatting
                                let text = if is_equipped {
                                    egui::RichText::new(prefix + item_name + " [Equipped]")
                                        .color(Color32::from_rgb(0, 255, 0))
                                        .strong()
                                } else {
                                    egui::RichText::new(prefix + item_name)
                                };

                                // Show item name
                                ui.label(text);

                                // Add interaction buttons based on item type
                                match item_info.item_type {
                                    ItemType::Equipment => {
                                        if !is_equipped {
                                            if ui.button("Equip").clicked() {
                                                equip_item_index = Some(i);
                                            }
                                        }
                                    }
                                    ItemType::Consumable => {
                                        if ui.button("Use").clicked() {
                                            use_item_index = Some(i);
                                        }
                                    }
                                    _ => {}
                                }
                            });
                        }
                    });
                }

                // Show keyboard shortcuts reminder
                ui.separator();
                ui.label("Keyboard shortcuts:");
                ui.label("• 1-9: Equip corresponding item");
                ui.label("• I or ESC: Close inventory");

                // Show feedback message if we have one
                unsafe {
                    // Use raw const to avoid shared reference to mutable static
                    let equip_message_ptr =
                        std::ptr::addr_of!(EQUIP_RESULT_MESSAGE) as *const Option<(String, u64)>;
                    if let Some((message, frame_count)) = &*equip_message_ptr {
                        ui.separator();
                        ui.colored_label(Color32::from_rgb(0, 255, 0), message);

                        // Clear message after 90 frames (~1.5 seconds at 60fps)
                        if self.frame_count > *frame_count + 90 {
                            EQUIP_RESULT_MESSAGE = None;
                        }
                    }
                }

                ui.separator();

                // Add a close button at the bottom
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("Close Inventory").clicked() {
                        self.showing_inventory = false;
                    }
                });
            });
        }

        // Process equip/use actions outside the UI closure to avoid borrow issues
        if let Some(index) = equip_item_index {
            if let Some(game) = &mut self.game {
                if index < game.player.inventory.items.len() {
                    match game.player.inventory.equip_item(index) {
                        Ok(()) => {
                            // Store message with current frame count for timing
                            unsafe {
                                // Directly write to mutable static
                                EQUIP_RESULT_MESSAGE = Some((
                                    "Item equipped successfully!".to_string(),
                                    self.frame_count,
                                ));
                            }
                            self.add_message("🎒 Item equipped successfully!".to_string());
                        }
                        Err(error) => {
                            // Store error message with current frame count for timing
                            unsafe {
                                // Directly write to mutable static
                                EQUIP_RESULT_MESSAGE =
                                    Some((format!("Error: {}", error), self.frame_count));
                            }
                            self.add_message(format!("🎒 Error equipping item: {}", error));
                        }
                    }
                }
            }
        }

        // Handle using consumable items
        if let Some(index) = use_item_index {
            // Handle consumable use for GUI
            if let Some(game) = &mut self.game {
                if index < InventoryManager::get_item_count(&game.player) {
                    if let Some(item) = InventoryManager::get_item(&game.player, index) {
                        if let Item::Consumable(_) = item {
                            let result = InventoryManager::use_item(&mut game.player, index);
                            self.add_message(format!("🧪 {}", result.message));
                        }
                    }
                }
            }
        }
    }

    /// Displays the character screen with player stats
    fn show_character_screen(&mut self, ui: &mut egui::Ui) {
        if let Some(ref game) = self.game {
            let player = &game.player;

            // Create a window for the character info
            let window = egui::Window::new("Character")
                .fixed_size([400.0, 500.0])
                .collapsible(false)
                .resizable(false);

            window.show(ui.ctx(), |ui| {
                ui.heading(format!("{} - Level {}", player.name, player.level));
                ui.label(format!("Class: {}", player.class.class_type));
                ui.add_space(10.0);

                // Stats section
                ui.heading("Stats");
                ui.label(format!("Health: {}/{}", player.health, player.max_health));
                ui.label(format!("Mana: {}/{}", player.mana, player.max_mana));
                ui.label(format!(
                    "Experience: {}/{}",
                    player.experience,
                    player.level * 100
                ));
                ui.label(format!("Gold: {}", player.gold));

                ui.add_space(10.0);

                // Equipment section
                ui.heading("Equipment");
                for slot in equipment::EquipmentSlot::iter() {
                    let equipped = if let Some(item_info) =
                        InventoryManager::get_equipped_item(player, slot)
                    {
                        format!("{} (+{})", item_info.name, item_info.value)
                    } else {
                        "None".to_string()
                    };

                    ui.label(format!("{}: {}", slot, equipped));
                }

                ui.separator();
                ui.label("Press C or ESC to close character screen");

                // Add a close button at the bottom
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("Close Character Screen").clicked() {
                        self.showing_character = false;
                    }
                });
            });
        }
    }

    /// Adds a message to both the UI messages list and the message log with timestamp
    fn add_message(&mut self, message: String) {
        // Add to UI messages (short-term display)
        self.ui_messages.push(message.clone());

        // Limit UI messages to 8 most recent for better history in status bar
        if self.ui_messages.len() > 8 {
            self.ui_messages.remove(0);
        }

        // Add to message log with current timestamp
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        self.message_log.push((message, current_time));

        // Limit message log size
        if self.message_log.len() > 100 {
            self.message_log.remove(0);
        }
    }

    /// Toggles visibility of the message log
    fn toggle_message_log(&mut self) {
        self.message_log_visible = !self.message_log_visible;
    }
}

#[cfg(feature = "gui")]
impl eframe::App for EchoesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Increment frame counter
        self.frame_count += 1;

        // Process input using centralized handler
        let actions = self.input_handler.process_input(ctx, self.frame_count);

        // Check if Escape key is pressed to close any open screens
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.showing_inventory {
                self.showing_inventory = false;
                self.add_message("🎒 Inventory closed".to_string());
            }
            if self.showing_character {
                self.showing_character = false;
                self.add_message("👤 Character screen closed".to_string());
            }
        }

        // Handle each action
        for action in actions {
            self.handle_input(&action);
        }

        // Main UI with dark terminal theme - remove borders and center content
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::BLACK))
            .show(ctx, |ui| {
                // Set dark background
                ui.visuals_mut().extreme_bg_color = Color32::BLACK;
                ui.visuals_mut().window_fill = Color32::BLACK;
                ui.visuals_mut().panel_fill = Color32::BLACK;

                // Set monospace font for terminal display
                let font_id = FontId::new(self.font_size, FontFamily::Monospace);

                // Calculate responsive sizing
                let available_size = ui.available_size();
                let char_width = self.font_size * 0.6;
                let char_height = self.font_size * 1.2;

                let max_cols = ((available_size.x * 0.9) / char_width) as usize;
                let max_rows = ((available_size.y * 0.9) / char_height) as usize;

                // Calculate content dimensions for proper centering
                let content_width = (max_cols as f32 * char_width).min(available_size.x * 0.8);
                let left_padding = (available_size.x - content_width) / 2.0 + 150.0; // Align with window title

                // Add horizontal spacing to center content properly
                ui.horizontal(|ui| {
                    ui.add_space(left_padding);

                    ui.vertical(|ui| {
                        // Center the title
                        ui.horizontal(|ui| {
                            ui.add_space((content_width - 500.0) / 2.0); // Align game title with window title
                            ui.heading(
                                RichText::new("Echoes of the Forgotten Realm")
                                    .size(24.0)
                                    .color(Color32::YELLOW),
                            );
                        });

                        ui.add_space(15.0);

                        // Terminal content with explicit centering
                        for (y, line) in self.terminal_buffer.iter().enumerate() {
                            if y >= max_rows.saturating_sub(3) {
                                break;
                            } // Leave space for UI elements

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;

                                // Group consecutive characters with same color into segments
                                let mut current_segment = String::new();
                                let mut current_color = Color32::from_rgb(192, 192, 192);
                                let mut segment_start = true;

                                for (x, &ch) in line.iter().enumerate() {
                                    if x >= max_cols.saturating_sub(5) {
                                        break;
                                    } // Prevent overflow with smaller margin

                                    let color = if y < self.color_buffer.len()
                                        && x < self.color_buffer[y].len()
                                    {
                                        self.color_buffer[y][x]
                                    } else {
                                        Some(Color32::from_rgb(192, 192, 192))
                                    };

                                    // If color changes or this is the first character, start new segment
                                    if segment_start || color.unwrap_or(Color32::from_rgb(192, 192, 192)) != current_color {
                                        // Render previous segment if it exists
                                        if !current_segment.is_empty() {
                                            let text = RichText::new(&current_segment)
                                                .font(font_id.clone())
                                                .color(current_color);
                                            ui.label(text);
                                        }

                                        // Start new segment
                                        current_segment = ch.to_string();
                                        current_color = color.unwrap_or(Color32::from_rgb(192, 192, 192));
                                        segment_start = false;
                                    } else {
                                        // Add to current segment
                                        current_segment.push(ch);
                                    }
                                }

                                // Render final segment
                                if !current_segment.is_empty() {
                                    let text = RichText::new(&current_segment)
                                        .font(font_id.clone())
                                        .color(current_color);
                                    ui.label(text);
                                }
                            });
                        }
                    });
                });

                // Render game if active
                if self.game_initialized && !self.show_combat_tutorial {
                    if self.game.is_some() {
                        // Clone the game data only at render time to avoid stale state
                        let game_clone = self.game.clone().unwrap();
                        if self.in_combat {
                            self.render_combat_screen_safe(&game_clone);
                        } else {
                            self.render_game_screen_safe(&game_clone);
                        }
                    }
                }

                // Compact status bar at bottom - no separators or borders
                // If inventory or character screen should be shown, render them on top of the game screen
                let mut close_inventory = false;
                let mut close_character = false;

                if self.showing_inventory && self.game_initialized {
                    self.show_inventory_screen(ui);
                    // Check if inventory was closed via button
                    if !self.showing_inventory {
                        close_inventory = true;
                    }

                    // Request repaint to ensure inventory updates properly
                    ctx.request_repaint();
                }

                if self.showing_character && self.game_initialized {
                    self.show_character_screen(ui);
                    // Check if character screen was closed via button
                    if !self.showing_character {
                        close_character = true;
                    }
                }

                // Handle screen closed events outside of the UI closures
                if close_inventory {
                    self.add_message("🎒 Inventory closed".to_string());
                }
                if close_character {
                    self.add_message("👤 Character screen closed".to_string());
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    // Compact message display at bottom
                    if !self.ui_messages.is_empty() {
                        ui.horizontal_centered(|ui| {
                            ui.label(
                                RichText::new("Recent: ").color(Color32::from_rgb(0, 255, 255)),
                            );
                            // Display recent messages in a horizontal bar
                            ui.horizontal_wrapped(|ui| {
                                for msg in &self.ui_messages {
                                    ui.label(
                                        RichText::new(format!("{} | ", msg)).color(Color32::WHITE),
                                    );
                                }
                            });
                        });
                    }

                    // Show help for message log and item pickup
                    ui.horizontal_centered(|ui| {
                        ui.label(
                            RichText::new("Press M to toggle message log | Press G to pick up items or loot chests")
                                .color(Color32::from_rgb(180, 180, 180))
                                .small(),
                        );
                    });

                    // Full message log (when visible)
                    if self.message_log_visible && !self.message_log.is_empty() {
                        // Calculate current time to fade old messages
                        let current_time = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs_f64();

                        // Create a scrollable message log area
                        egui::ScrollArea::vertical()
                            .max_height(150.0)
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.push_id("message_log", |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.heading(RichText::new("Message Log").color(Color32::from_rgb(0, 255, 255)));
                                    });

                                    // Display message log with timestamps
                                    ui.add_space(5.0);
                                    for (i, (msg, time)) in self.message_log.iter().enumerate() {
                                        // Fade older messages (30 seconds to full fade)
                                        let age = current_time - time;
                                        let alpha = (1.0 - (age / 30.0)).max(0.3).min(1.0);
                                        let color = if msg.contains("chest") || msg.contains("item") {
                                            Color32::from_rgba_premultiplied(200, 255, 200, (alpha * 255.0) as u8)
                                        } else if msg.contains("combat") || msg.contains("attack") || msg.contains("damage") {
                                            Color32::from_rgba_premultiplied(255, 200, 200, (alpha * 255.0) as u8)
                                        } else {
                                            Color32::from_rgba_premultiplied(255, 255, 255, (alpha * 255.0) as u8)
                                        };

                                        ui.horizontal(|ui| {
                                            // Add small indicator for message type
                                            let indicator = if i == self.message_log.len() - 1 { "➤ " } else { "• " };
                                            ui.label(RichText::new(indicator).color(color));
                                            ui.label(RichText::new(msg).color(color));
                                        });
                                    }
                                });
                            });

                        ui.add_space(10.0);
                    }

                    // Status bar - compact and centered
                    ui.horizontal_centered(|ui| {
                        ui.label(RichText::new("Status: ").color(Color32::from_rgb(0, 255, 255)));
                        if self.main_menu {
                            ui.label(RichText::new("Main Menu").color(Color32::YELLOW));
                        } else if self.creating_character {
                            ui.label(RichText::new("Character Creation").color(Color32::YELLOW));
                        } else if self.show_combat_tutorial {
                            ui.label(RichText::new("Combat Tutorial").color(Color32::YELLOW));
                        } else if self.game_initialized {
                            ui.label(RichText::new("In Game").color(Color32::GREEN));
                        }

                        if let Some(key) = self.last_key {
                            ui.label(
                                RichText::new(format!(" | Last key: {}", key))
                                    .color(Color32::LIGHT_GRAY)
                            );
                        }
                    });
                });
            });

        // Request repaint for smooth updates
        ctx.request_repaint();
    }
}

#[cfg(feature = "gui")]
#[allow(dead_code)]
pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("Echoes of the Forgotten Realm") // This still is not centered correctly.
            .with_resizable(true)
            .with_maximize_button(true)
            .with_minimize_button(true),
        ..Default::default()
    };

    eframe::run_native(
        "Echoes of the Forgotten Realm",
        options,
        Box::new(|cc| Box::new(EchoesApp::new(cc))),
    )
}

// Stub implementations for when GUI feature is not enabled
#[cfg(not(feature = "gui"))]
pub fn run_gui() -> Result<(), Box<dyn std::error::Error>> {
    Err("GUI feature not enabled. Compile with --features gui".into())
}

#[cfg(feature = "gui")]
#[allow(dead_code)]
impl EchoesApp {
    fn get_game_info(&self) -> Option<(String, i32, i32, i32, i32, i32, i32)> {
        if let Some(ref game) = self.game {
            let player = &game.player;
            Some((
                player.name.clone(),
                player.level as i32,
                player.health,
                player.max_health,
                player.mana,
                player.max_mana,
                player.gold as i32,
            ))
        } else {
            None
        }
    }
}
