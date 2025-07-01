//! GUI module for Windows graphical interface using egui
//! Provides a native Windows application with text-based gameplay

#[cfg(feature = "gui")]
use crate::character::Player;
#[cfg(feature = "gui")]
use crate::game::Game;
#[cfg(feature = "gui")]
use crate::input::{InputAction, InputHandler};
#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use egui::{Color32, FontFamily, FontId, RichText};

#[cfg(feature = "gui")]
pub struct EchoesApp {
    game: Option<crate::game::Game>,
    terminal_buffer: Vec<String>,
    color_buffer: Vec<Vec<Color32>>,
    input_buffer: String,
    last_key: Option<char>,
    show_combat_tutorial: bool,
    window_size: (usize, usize),
    font_size: f32,
    char_width: f32,
    char_height: f32,
    cursor_pos: (usize, usize),
    terminal_size: (usize, usize),
    ui_messages: Vec<String>,
    game_initialized: bool,
    character_name: String,
    character_class: Option<crate::character::ClassType>,
    creating_character: bool,
    main_menu: bool,
    input_handler: crate::input::InputHandler,
    frame_count: u64,
}

#[cfg(feature = "gui")]
impl Default for EchoesApp {
    fn default() -> Self {
        Self {
            game: None,
            terminal_buffer: vec![String::new(); 30],
            color_buffer: vec![vec![Color32::from_rgb(192, 192, 192); 80]; 30],
            input_buffer: String::new(),
            last_key: None,
            show_combat_tutorial: false,
            window_size: (800, 600),
            font_size: 14.0,
            char_width: 8.0,
            char_height: 16.0,
            cursor_pos: (0, 0),
            terminal_size: (80, 30),
            ui_messages: Vec::new(),
            game_initialized: false,
            character_name: String::new(),
            character_class: None,
            creating_character: false,
            main_menu: true,
            input_handler: crate::input::InputHandler::new(),
            frame_count: 0,
        }
    }
}

#[cfg(feature = "gui")]
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

    fn init_terminal(&mut self) {
        // Initialize terminal buffer and color buffer
        self.terminal_buffer = vec![" ".repeat(self.terminal_size.0); self.terminal_size.1];
        self.color_buffer = vec![
            vec![Color32::from_rgb(192, 192, 192); self.terminal_size.0];
            self.terminal_size.1
        ];
        self.clear_screen();
        self.show_main_menu();
    }

    fn clear_screen(&mut self) {
        for line in &mut self.terminal_buffer {
            *line = " ".repeat(self.terminal_size.0);
        }
        for line in &mut self.color_buffer {
            for color in line {
                *color = Color32::from_rgb(192, 192, 192);
            }
        }
        self.cursor_pos = (0, 0);
    }

    fn print_at(&mut self, x: usize, y: usize, text: &str, color: Option<Color32>) {
        if y < self.terminal_buffer.len() && x < self.terminal_size.0 {
            let line = &mut self.terminal_buffer[y];
            let end_x = (x + text.len()).min(line.len());
            if x < line.len() {
                line.replace_range(x..end_x, &text[..end_x - x]);

                // Set colors for each character
                let color_to_use = color.unwrap_or(Color32::from_rgb(192, 192, 192));
                for i in x..end_x {
                    if i < self.color_buffer[y].len() {
                        self.color_buffer[y][i] = color_to_use;
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

        if self.character_name.is_empty() {
            self.print_at(10, 10, "Name: _", None);
            self.print_at(
                10,
                13,
                "Type your character name and press Enter",
                Some(Color32::from_rgb(0, 255, 255)),
            );
        } else if self.character_class.is_none() {
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
        }
    }

    fn handle_character_creation_input(&mut self, action: &crate::input::InputAction) {
        if self.character_class.is_none() {
            // Still in character creation phase
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
                    // Can't proceed without a name
                    if !self.character_name.is_empty() {
                        // Move to class selection - show updated screen
                        self.show_character_creation();
                    }
                }
                crate::input::InputAction::MenuOption(1) => {
                    if !self.character_name.is_empty() {
                        self.character_class = Some(crate::character::ClassType::Warrior);
                        self.finish_character_creation();
                    }
                }
                crate::input::InputAction::MenuOption(2) => {
                    if !self.character_name.is_empty() {
                        self.character_class = Some(crate::character::ClassType::Mage);
                        self.finish_character_creation();
                    }
                }
                crate::input::InputAction::MenuOption(3) => {
                    if !self.character_name.is_empty() {
                        self.character_class = Some(crate::character::ClassType::Ranger);
                        self.finish_character_creation();
                    }
                }
                crate::input::InputAction::MenuOption(4) => {
                    if !self.character_name.is_empty() {
                        self.character_class = Some(crate::character::ClassType::Cleric);
                        self.finish_character_creation();
                    }
                }
                _ => {}
            }
        }
    }

    fn finish_character_creation(&mut self) {
        if let Some(class_type) = self.character_class {
            let class = crate::character::Class::new(class_type);
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
        // Don't render here, will be handled in main update loop
    }

    fn render_game_screen_safe(&mut self, game: &Game) {
        self.clear_screen();

        // Render game map
        let level = game.current_level();
        let player_pos = level.player_position;

        // Calculate view area (centered on player)
        let view_width = 70;
        let view_height = 25;
        let start_x = 5;
        let start_y = 5;

        // Draw map
        for screen_y in 0..view_height {
            for screen_x in 0..view_width {
                let map_x = player_pos.x - view_width as i32 / 2 + screen_x as i32;
                let map_y = player_pos.y - view_height as i32 / 2 + screen_y as i32;

                let (char_to_draw, color) = if map_x == player_pos.x && map_y == player_pos.y {
                    ('@', Some(Color32::YELLOW))
                } else if map_x >= 0
                    && map_x < level.width as i32
                    && map_y >= 0
                    && map_y < level.height as i32
                {
                    let tile = &level.tiles[map_y as usize][map_x as usize];
                    if !tile.explored {
                        (' ', None)
                    } else {
                        let pos = crate::world::Position::new(map_x, map_y);

                        // Check for enemies first
                        let has_enemy = level.enemies.contains_key(&pos);

                        // Check for items
                        let has_item = level.items.contains_key(&pos);

                        let (symbol, tile_color) = if has_enemy && tile.visible {
                            ('E', Some(Color32::RED))
                        } else if has_item && tile.visible {
                            ('!', Some(Color32::from_rgb(0, 255, 255)))
                        } else {
                            let symbol = tile.tile_type.symbol();
                            let color = match tile.tile_type.symbol() {
                                '#' => Some(Color32::GRAY),                  // Wall
                                '.' => Some(Color32::WHITE),                 // Floor
                                '+' => Some(Color32::from_rgb(139, 69, 19)), // Door (brown)
                                'C' => Some(Color32::from_rgb(255, 215, 0)), // Chest (gold)
                                '>' => Some(Color32::GREEN),                 // Stairs
                                _ => Some(Color32::WHITE),
                            };
                            (symbol, color)
                        };

                        if tile.visible {
                            (symbol, tile_color)
                        } else {
                            // Explored but not visible - dimmed
                            let dimmed_color = tile_color.map(|c| {
                                Color32::from_rgba_unmultiplied(
                                    (c.r() as f32 * 0.5) as u8,
                                    (c.g() as f32 * 0.5) as u8,
                                    (c.b() as f32 * 0.5) as u8,
                                    255,
                                )
                            });
                            (symbol, dimmed_color)
                        }
                    }
                } else {
                    (' ', None)
                };

                self.print_at(
                    start_x + screen_x,
                    start_y + screen_y,
                    &char_to_draw.to_string(),
                    color,
                );
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
        self.print_at(ui_x, controls_y + 2, "I: Inventory", None);
        self.print_at(ui_x, controls_y + 3, "C: Character", None);
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
        self.print_at(ui_x, legend_y + 8, "> - Stairs", None);
    }

    fn handle_game_input(&mut self, key: char) {
        if let Some(ref mut game) = self.game {
            match key {
                'w' | 'W' => {
                    game.move_player(0, -1);
                    game.update_visibility();
                }
                's' | 'S' => {
                    game.move_player(0, 1);
                    game.update_visibility();
                }
                'a' | 'A' => {
                    game.move_player(-1, 0);
                    game.update_visibility();
                }
                'd' | 'D' => {
                    game.move_player(1, 0);
                    game.update_visibility();
                }
                'i' | 'I' => {
                    // Show inventory (simplified for GUI)
                    self.ui_messages.push("Inventory opened".to_string());
                }
                'c' | 'C' => {
                    // Show character screen (simplified for GUI)
                    self.ui_messages.push("Character screen opened".to_string());
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

    fn handle_input(&mut self, action: &crate::input::InputAction) {
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
            _ => return, // Ignore other actions for now
        };

        self.handle_game_input(key_char);
    }
}

#[cfg(feature = "gui")]
impl eframe::App for EchoesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Increment frame counter
        self.frame_count += 1;

        // Process input using centralized handler
        let actions = self.input_handler.process_input(ctx, self.frame_count);

        // Handle each action
        for action in actions {
            self.handle_input(&action);
        }

        // Main UI with dark terminal theme
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::BLACK))
            .show(ctx, |ui| {
                // Set dark background
                ui.visuals_mut().extreme_bg_color = Color32::BLACK;
                ui.visuals_mut().window_fill = Color32::BLACK;
                ui.visuals_mut().panel_fill = Color32::BLACK;

                // Set monospace font for terminal display
                let font_id = FontId::new(self.font_size, FontFamily::Monospace);

                // Center the content vertically and horizontally
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Echoes of the Forgotten Realm")
                            .size(20.0)
                            .color(Color32::YELLOW),
                    );
                    ui.separator();

                    // Add some spacing
                    ui.add_space(20.0);

                    // Terminal display area with dark background and centered content
                    let available_width = ui.available_width();
                    let terminal_width =
                        (self.terminal_size.0 as f32 * 8.0).min(available_width - 40.0);

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(terminal_width, ui.available_height() - 100.0),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            egui::Frame::none()
                                .fill(Color32::BLACK)
                                .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                                .inner_margin(egui::Margin::same(10.0))
                                .show(ui, |ui| {
                                    egui::ScrollArea::vertical().id_source("terminal").show(
                                        ui,
                                        |ui| {
                                            ui.style_mut().visuals.extreme_bg_color =
                                                Color32::BLACK;

                                            for (y, line) in self.terminal_buffer.iter().enumerate()
                                            {
                                                ui.horizontal(|ui| {
                                                    ui.spacing_mut().item_spacing.x = 0.0; // Remove horizontal spacing

                                                    // Group consecutive characters with same color into segments
                                                    let mut current_segment = String::new();
                                                    let mut current_color =
                                                        Color32::from_rgb(192, 192, 192);
                                                    let mut segment_start = true;

                                                    for (x, ch) in line.chars().enumerate() {
                                                        let color = if y < self.color_buffer.len()
                                                            && x < self.color_buffer[y].len()
                                                        {
                                                            self.color_buffer[y][x]
                                                        } else {
                                                            Color32::from_rgb(192, 192, 192)
                                                        };

                                                        // If color changes or this is the first character, start new segment
                                                        if segment_start || color != current_color {
                                                            // Render previous segment if it exists
                                                            if !current_segment.is_empty() {
                                                                let text =
                                                                    RichText::new(&current_segment)
                                                                        .font(font_id.clone())
                                                                        .color(current_color);
                                                                ui.label(text);
                                                            }

                                                            // Start new segment
                                                            current_segment = ch.to_string();
                                                            current_color = color;
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
                                        },
                                    );
                                });
                        },
                    );
                });

                // Render game if active
                if self.game_initialized && !self.show_combat_tutorial {
                    if self.game.is_some() {
                        // Clone the game data to avoid borrow checker issues
                        let game_clone = self.game.clone().unwrap();
                        self.render_game_screen_safe(&game_clone);
                    }
                }

                // Status bar with dark theme
                ui.separator();
                egui::Frame::none()
                    .fill(Color32::from_gray(20))
                    .inner_margin(egui::Margin::same(5.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Status:").color(Color32::from_rgb(0, 255, 255)),
                            );
                            if self.main_menu {
                                ui.label(RichText::new("Main Menu").color(Color32::YELLOW));
                            } else if self.creating_character {
                                ui.label(
                                    RichText::new("Character Creation").color(Color32::YELLOW),
                                );
                            } else if self.show_combat_tutorial {
                                ui.label(RichText::new("Combat Tutorial").color(Color32::YELLOW));
                            } else if self.game_initialized {
                                ui.label(RichText::new("In Game").color(Color32::GREEN));
                            }

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if let Some(key) = self.last_key {
                                        ui.label(
                                            RichText::new(format!("Last key: {}", key))
                                                .color(Color32::LIGHT_GRAY),
                                        );
                                    }
                                },
                            );
                        });
                    });

                // Messages area with dark theme
                if !self.ui_messages.is_empty() {
                    ui.separator();
                    egui::Frame::none()
                        .fill(Color32::from_gray(15))
                        .inner_margin(egui::Margin::same(5.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("Messages:").color(Color32::from_rgb(0, 255, 255)),
                            );
                            for msg in &self.ui_messages {
                                ui.label(RichText::new(format!("• {}", msg)).color(Color32::WHITE));
                            }
                        });

                    // Clear old messages
                    if self.ui_messages.len() > 5 {
                        self.ui_messages.remove(0);
                    }
                }
            });

        // Request repaint for smooth updates
        ctx.request_repaint();
    }
}

#[cfg(feature = "gui")]
pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 1000.0])
            .with_min_inner_size([1200.0, 800.0])
            .with_title("Echoes of the Forgotten Realm"),
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
