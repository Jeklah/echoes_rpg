//! GUI module for Windows graphical interface using egui
//! Provides a native Windows application with text-based gameplay

#[cfg(feature = "gui")]
#[cfg(feature = "gui")]
use crate::character::Player;
#[cfg(feature = "gui")]
use crate::game::Game;
#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use egui::{Color32, FontFamily, FontId, RichText};

#[cfg(feature = "gui")]
pub struct EchoesApp {
    game: Option<Game>,
    terminal_buffer: Vec<String>,
    color_buffer: Vec<Vec<Color32>>,
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
    game_initialized: bool,
    character_name: String,
    character_class: Option<crate::character::ClassType>,
    creating_character: bool,
    main_menu: bool,
}

#[cfg(feature = "gui")]
impl Default for EchoesApp {
    fn default() -> Self {
        Self {
            game: None,
            terminal_buffer: vec![" ".repeat(120); 50],
            color_buffer: vec![vec![Color32::from_rgb(192, 192, 192); 120]; 50],
            input_buffer: String::new(),
            last_key: None,
            show_combat_tutorial: false,
            window_size: (1200.0, 800.0),
            font_size: 14.0,
            char_width: 8.0,
            char_height: 16.0,
            cursor_pos: (0, 0),
            terminal_size: (120, 50),
            ui_messages: Vec::new(),
            game_initialized: false,
            character_name: String::new(),
            character_class: None,
            creating_character: false,
            main_menu: true,
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

        let center_x = (self.terminal_size.0 - title.len()) / 2;
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

    fn handle_main_menu_input(&mut self, key: char) {
        match key {
            '1' => {
                self.main_menu = false;
                self.creating_character = true;
                self.show_character_creation();
            }
            '2' => {
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

        self.print_at(10, 10, &format!("Name: {}_", self.character_name), None);

        if !self.character_name.is_empty() {
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
        } else {
            self.print_at(
                10,
                13,
                "Type your character name and press Enter",
                Some(Color32::from_rgb(0, 255, 255)),
            );
        }
    }

    fn handle_character_creation_input(&mut self, key: char) {
        if self.character_name.is_empty() {
            // Getting character name
            if key.is_alphanumeric() || key == ' ' {
                if self.character_name.len() < 20 {
                    self.character_name.push(key);
                    self.show_character_creation();
                }
            } else if key == '\u{8}' {
                // Backspace
                self.character_name.pop();
                self.show_character_creation();
            } else if key == '\r' || key == '\n' {
                if !self.character_name.is_empty() {
                    self.show_character_creation();
                }
            }
        } else {
            // Choosing class
            match key {
                '1' => {
                    self.character_class = Some(crate::character::ClassType::Warrior);
                    self.finish_character_creation();
                }
                '2' => {
                    self.character_class = Some(crate::character::ClassType::Mage);
                    self.finish_character_creation();
                }
                '3' => {
                    self.character_class = Some(crate::character::ClassType::Ranger);
                    self.finish_character_creation();
                }
                '4' => {
                    self.character_class = Some(crate::character::ClassType::Cleric);
                    self.finish_character_creation();
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
        let view_width = 60;
        let view_height = 25;
        let start_x = 5;
        let start_y = 5;

        // Draw map
        for screen_y in 0..view_height {
            for screen_x in 0..view_width {
                let map_x = player_pos.x - view_width as i32 / 2 + screen_x as i32;
                let map_y = player_pos.y - view_height as i32 / 2 + screen_y as i32;

                let char_to_draw = if map_x == player_pos.x && map_y == player_pos.y {
                    '@'
                } else if map_x >= 0
                    && map_x < level.width as i32
                    && map_y >= 0
                    && map_y < level.height as i32
                {
                    let tile = &level.tiles[map_y as usize][map_x as usize];
                    if !tile.explored {
                        ' '
                    } else if tile.visible {
                        tile.tile_type.symbol()
                    } else {
                        tile.tile_type.symbol() // Show explored areas
                    }
                } else {
                    ' '
                };

                self.print_at(
                    start_x + screen_x,
                    start_y + screen_y,
                    &char_to_draw.to_string(),
                    None,
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
                }
                's' | 'S' => {
                    game.move_player(0, 1);
                }
                'a' | 'A' => {
                    game.move_player(-1, 0);
                }
                'd' | 'D' => {
                    game.move_player(1, 0);
                }
                'i' | 'I' => {
                    // Show inventory (simplified for GUI)
                    self.ui_messages.push("Inventory opened".to_string());
                }
                'c' | 'C' => {
                    // Show character screen
                    self.ui_messages.push("Character screen opened".to_string());
                }
                'g' | 'G' => {
                    // Get item
                    self.ui_messages.push("Looking for items...".to_string());
                }
                'q' | 'Q' => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }

    fn handle_input(&mut self, key: char) {
        self.last_key = Some(key);

        if self.main_menu {
            self.handle_main_menu_input(key);
        } else if self.creating_character {
            self.handle_character_creation_input(key);
        } else if self.show_combat_tutorial {
            self.start_game();
        } else if self.game_initialized {
            self.handle_game_input(key);
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for EchoesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Key {
                    key, pressed: true, ..
                } = event
                {
                    match key {
                        egui::Key::A => self.handle_input('a'),
                        egui::Key::B => self.handle_input('b'),
                        egui::Key::C => self.handle_input('c'),
                        egui::Key::D => self.handle_input('d'),
                        egui::Key::E => self.handle_input('e'),
                        egui::Key::F => self.handle_input('f'),
                        egui::Key::G => self.handle_input('g'),
                        egui::Key::H => self.handle_input('h'),
                        egui::Key::I => self.handle_input('i'),
                        egui::Key::J => self.handle_input('j'),
                        egui::Key::K => self.handle_input('k'),
                        egui::Key::L => self.handle_input('l'),
                        egui::Key::M => self.handle_input('m'),
                        egui::Key::N => self.handle_input('n'),
                        egui::Key::O => self.handle_input('o'),
                        egui::Key::P => self.handle_input('p'),
                        egui::Key::Q => self.handle_input('q'),
                        egui::Key::R => self.handle_input('r'),
                        egui::Key::S => self.handle_input('s'),
                        egui::Key::T => self.handle_input('t'),
                        egui::Key::U => self.handle_input('u'),
                        egui::Key::V => self.handle_input('v'),
                        egui::Key::W => self.handle_input('w'),
                        egui::Key::X => self.handle_input('x'),
                        egui::Key::Y => self.handle_input('y'),
                        egui::Key::Z => self.handle_input('z'),
                        egui::Key::Num1 => self.handle_input('1'),
                        egui::Key::Num2 => self.handle_input('2'),
                        egui::Key::Num3 => self.handle_input('3'),
                        egui::Key::Num4 => self.handle_input('4'),
                        egui::Key::Num5 => self.handle_input('5'),
                        egui::Key::Num6 => self.handle_input('6'),
                        egui::Key::Num7 => self.handle_input('7'),
                        egui::Key::Num8 => self.handle_input('8'),
                        egui::Key::Num9 => self.handle_input('9'),
                        egui::Key::Num0 => self.handle_input('0'),
                        egui::Key::Space => self.handle_input(' '),
                        egui::Key::Enter => self.handle_input('\r'),
                        egui::Key::Backspace => self.handle_input('\u{8}'),
                        _ => {}
                    }
                }
            }

            // Handle text input for character name
            if self.creating_character && self.character_name.is_empty() {
                for ch in &i.events {
                    if let egui::Event::Text(text) = ch {
                        for c in text.chars() {
                            if c.is_alphanumeric() || c == ' ' {
                                self.handle_input(c);
                            }
                        }
                    }
                }
            }
        });

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

                ui.heading(
                    RichText::new("Echoes of the Forgotten Realm")
                        .size(20.0)
                        .color(Color32::YELLOW),
                );
                ui.separator();

                // Terminal display area with dark background
                egui::Frame::none()
                    .fill(Color32::BLACK)
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_source("terminal")
                            .show(ui, |ui| {
                                ui.style_mut().visuals.extreme_bg_color = Color32::BLACK;

                                for (y, line) in self.terminal_buffer.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        // Render each character with its individual color
                                        for (x, ch) in line.chars().enumerate() {
                                            let color = if y < self.color_buffer.len()
                                                && x < self.color_buffer[y].len()
                                            {
                                                self.color_buffer[y][x]
                                            } else {
                                                Color32::from_rgb(192, 192, 192)
                                            };

                                            let text = RichText::new(ch.to_string())
                                                .font(font_id.clone())
                                                .color(color);
                                            ui.label(text);
                                        }
                                    });
                                }
                            });
                    });

                // Render game if active
                if self.game_initialized && !self.show_combat_tutorial {
                    // Clone the necessary data to avoid borrow checker issues
                    if let Some(info) = self.get_game_info() {
                        // Render game screen with cloned data
                        self.clear_screen();

                        // Simple display for now
                        self.print_at(10, 10, &format!("Player: {}", info.0), None);
                        self.print_at(10, 11, &format!("Level: {}", info.1), None);
                        self.print_at(10, 12, &format!("HP: {}/{}", info.2, info.3), None);
                        self.print_at(10, 13, &format!("MP: {}/{}", info.4, info.5), None);
                        self.print_at(10, 14, &format!("Gold: {}", info.6), None);

                        self.print_at(10, 16, "Use WASD to move, Q to quit", None);
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
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
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
