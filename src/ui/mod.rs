#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
use crossterm::event::KeyEventKind;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Color},
    terminal::{self},
};

#[cfg(not(all(feature = "gui", target_os = "windows")))]
use std::io::{self, stdout, Write};

#[cfg(not(all(feature = "gui", target_os = "windows")))]
use crate::character::{ClassType, Player};
#[cfg(not(all(feature = "gui", target_os = "windows")))]
use crate::combat::{CombatAction, CombatResult};
#[cfg(not(all(feature = "gui", target_os = "windows")))]
use crate::inventory::InventoryManager;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
use crate::item::Item;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
#[cfg(not(target_arch = "wasm32"))]
use crate::platform;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
use crate::world::{Dungeon, Enemy, FogOfWar, Level, Position};

#[cfg(not(all(feature = "gui", target_os = "windows")))]
const SCREEN_HEIGHT: usize = 35;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
const MAP_WIDTH: usize = 70;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
const MAP_HEIGHT: usize = 25;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
const UI_PANEL_WIDTH: usize = 35; // Increased panel width for better readability
#[cfg(not(all(feature = "gui", target_os = "windows")))]
const BORDER_PADDING: usize = 4; // Increased padding inside the border

/// Create fog of war configuration for terminal rendering
#[cfg(not(all(feature = "gui", target_os = "windows")))]
fn create_fog_of_war() -> FogOfWar {
    crate::world::create_standard_fog_of_war()
}

#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub struct UI {
    pub messages: Vec<String>,
    pub max_messages: usize,
}

#[cfg(not(all(feature = "gui", target_os = "windows")))]
impl UI {
    pub fn new() -> Self {
        UI {
            messages: Vec::new(),
            max_messages: 5,
        }
    }

    pub fn show_combat_tutorial(&mut self) -> io::Result<()> {
        self.clear_screen()?;

        // Draw a flashy combat intro
        let (term_width, term_height) = platform::get_terminal_size();
        let title = "*** COMBAT TUTORIAL ***";
        let title_pos_x = (term_width - title.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, term_height / 2 - 2),
            style::SetForegroundColor(Color::Red),
            style::Print(title)
        )?;

        let subtitle = "Prepare for battle!";
        let subtitle_pos_x = (term_width - subtitle.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(subtitle_pos_x, term_height / 2),
            style::SetForegroundColor(Color::Yellow),
            style::Print(subtitle),
            style::SetForegroundColor(Color::White)
        )?;

        // Pause for dramatic effect
        std::thread::sleep(std::time::Duration::from_millis(1500));

        self.clear_screen()?;

        // Draw bordered tutorial with responsive sizing
        let (term_width, term_height) = terminal::size()?;
        let max_border_width = 80;
        let border_width = (max_border_width).min(term_width as usize - 10);
        let border_height = 24;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height as i32) / 2).max(0) as u16;

        self.draw_game_border(
            start_x as usize,
            start_y as usize,
            border_width,
            border_height,
        )?;

        let title = "Combat Tutorial";
        let title_pos_x = start_x + (border_width as u16 - title.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, start_y - 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print(title),
            style::SetForegroundColor(Color::White)
        )?;

        // Content positioning with responsive width
        let text_x = start_x + 3;
        let mut text_y = start_y + 2;
        let available_width = border_width - 6; // 3 chars padding on each side
        let separator = "─".repeat(available_width);

        // Helper function to wrap text to available width
        let wrap_text = |text: &str, max_width: usize| -> String {
            if text.len() <= max_width {
                text.to_string()
            } else {
                format!("{}...", &text[0..max_width.saturating_sub(3)])
            }
        };

        // Draw tutorial content
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Yellow),
            style::Print(wrap_text(
                "Welcome to your first combat encounter!",
                available_width
            )),
            style::SetForegroundColor(Color::White)
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(&separator)
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(wrap_text(
                "Combat in Echoes RPG is turn-based. Here's how it works:",
                available_width
            ))
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("1. Attack"),
            style::SetForegroundColor(Color::White),
            style::Print(&wrap_text(
                " - Basic attack using your weapon.",
                available_width - 10
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("2. Use Ability"),
            style::SetForegroundColor(Color::White),
            style::Print(&wrap_text(
                " - Use special ability (costs mana).",
                available_width - 14
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("3. Use Item"),
            style::SetForegroundColor(Color::White),
            style::Print(&wrap_text(
                " - Use consumable from inventory.",
                available_width - 12
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("4. Flee"),
            style::SetForegroundColor(Color::White),
            style::Print(&wrap_text(
                " - Attempt to escape (chance based on dexterity).",
                available_width - 8
            ))
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(wrap_text(
                "After you act, the enemy will counter-attack.",
                available_width
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(wrap_text(
                "Victory grants experience, gold, and possibly items!",
                available_width
            ))
        )?;

        // Add simulated combat example
        text_y += 2;
        let example_x = text_x + 2;

        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(&separator)
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Yellow),
            style::Print("Combat Example:"),
            style::SetForegroundColor(Color::White)
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(example_x, text_y),
            style::Print(wrap_text(
                "You encounter a Goblin (HP: 20/20)",
                available_width - 2
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(example_x, text_y),
            style::SetForegroundColor(Color::Green),
            style::Print("You"),
            style::SetForegroundColor(Color::White),
            style::Print(": Attack")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(example_x, text_y),
            style::Print(wrap_text(
                "You attack the Goblin for 8 damage!",
                available_width - 2
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(example_x, text_y),
            style::SetForegroundColor(Color::Red),
            style::Print("Goblin"),
            style::SetForegroundColor(Color::White),
            style::Print(": Counter-attack")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(example_x, text_y),
            style::Print(wrap_text(
                "The Goblin hits you for 5 damage!",
                available_width - 2
            ))
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(&separator)
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Yellow),
            style::Print("Combat Tips:"),
            style::SetForegroundColor(Color::White)
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(wrap_text(
                "• Use healing potions when health is low",
                available_width
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(wrap_text(
                "• Special abilities deal more damage but cost mana",
                available_width
            ))
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(wrap_text(
                "• Sometimes fleeing is the best option",
                available_width
            ))
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(&separator)
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Green),
            style::Print(wrap_text(
                "Press any key to continue your adventure...",
                available_width
            ))
        )?;

        // Wait for key press
        self.wait_for_key()?;

        Ok(())
    }

    pub fn initialize(&mut self) -> io::Result<()> {
        // Terminal initialization is now handled by platform module
        Ok(())
    }

    pub fn cleanup(&self) -> io::Result<()> {
        execute!(stdout(), terminal::LeaveAlternateScreen)?;
        execute!(stdout(), cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> io::Result<()> {
        platform::clear_screen().map_err(io::Error::other)?;
        Ok(())
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn add_messages_from_combat(&mut self, result: &CombatResult) {
        for message in &result.messages {
            self.add_message(message.clone());
        }
    }

    pub fn draw_title_screen(&mut self) -> io::Result<()> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        let title = "Echoes of the Forgotten Realm";
        let author = "A Rusty Adventure";

        // Draw a decorative border around the title area
        let border_width = 60;
        let border_height = 16;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height as i32) / 2).max(0) as u16;

        self.draw_game_border(
            start_x as usize,
            start_y as usize,
            border_width as usize,
            border_height as usize,
        )?;

        // Calculate centered positions relative to the border
        let title_pos_x = start_x + (border_width - title.len() as u16) / 2;
        let author_pos_x = start_x + (border_width - author.len() as u16) / 2;
        let option_pos_x = start_x + border_width / 4;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, start_y + 3),
            style::SetForegroundColor(Color::Cyan),
            style::Print(title),
            cursor::MoveTo(author_pos_x, start_y + 5),
            style::SetForegroundColor(Color::White),
            style::Print(author),
            cursor::MoveTo(option_pos_x + 5, start_y + 8),
            style::Print("1. New Game"),
            cursor::MoveTo(option_pos_x + 5, start_y + 10),
            style::Print("2. Exit"),
            cursor::MoveTo(start_x + 5, start_y + border_height - 2),
            style::Print("Press the corresponding key to select an option..."),
        )?;

        Ok(())
    }

    pub fn character_creation(&mut self) -> io::Result<Player> {
        // Name selection screen
        let name = self.get_character_name()?;

        // Class selection screen
        let class_type = self.choose_character_class()?;

        let player = Player::new(name, class_type);
        Ok(player)
    }

    fn get_character_name(&mut self) -> io::Result<String> {
        let mut name = String::new();
        let max_name_length = 20;

        loop {
            self.clear_screen()?;

            // Get actual terminal size
            let (term_width, term_height) = terminal::size()?;

            // Create a centered box for name input
            let border_width = 60;
            let border_height = 12;
            let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
            let start_y = ((term_height as i32 - border_height) / 2).max(0) as u16;

            self.draw_game_border(
                start_x as usize,
                start_y as usize,
                border_width as usize,
                border_height as usize,
            )?;

            let title = "Character Creation";
            let title_pos_x = start_x + (border_width - title.len() as u16) / 2;

            // Display current name with cursor
            let display_name = if name.is_empty() {
                "_".to_string()
            } else {
                format!("{name}_")
            };

            execute!(
                stdout(),
                cursor::MoveTo(title_pos_x, start_y - 1),
                style::SetForegroundColor(Color::Cyan),
                style::Print(title),
                cursor::MoveTo(start_x + 5, start_y + 3),
                style::SetForegroundColor(Color::White),
                style::Print("Enter your character's name:"),
                cursor::MoveTo(start_x + 5, start_y + 5),
                style::SetForegroundColor(Color::Yellow),
                style::Print(&display_name),
                cursor::MoveTo(start_x + 5, start_y + 8),
                style::SetForegroundColor(Color::Green),
                style::Print("Press ENTER to confirm"),
                cursor::MoveTo(start_x + 5, start_y + 9),
                style::SetForegroundColor(Color::DarkGrey),
                style::Print("(or type a name and press ENTER)"),
                cursor::Hide
            )?;

            // Read input in raw mode
            if let Event::Key(key_event) = event::read()? {
                // On Windows, only process key press events to avoid duplicates
                #[cfg(windows)]
                {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                }

                match key_event.code {
                    KeyCode::Enter => {
                        // Confirm name entry
                        if name.is_empty() {
                            name = "Hero".to_string();
                        }
                        break;
                    }
                    KeyCode::Backspace => {
                        // Remove last character
                        name.pop();
                    }
                    KeyCode::Char(c) => {
                        // Add character if name isn't too long and character is valid
                        if name.len() < max_name_length && (c.is_alphanumeric() || c == ' ') {
                            name.push(c);
                        }
                    }
                    KeyCode::Esc => {
                        // Exit character creation
                        return Err(io::Error::new(
                            io::ErrorKind::Interrupted,
                            "Character creation cancelled",
                        ));
                    }
                    _ => {
                        // Ignore other keys
                    }
                }
            }
        }

        // Flush any remaining input to prevent interference with next screen
        self.flush_input_buffer()?;

        Ok(name.trim().to_string())
    }

    fn choose_character_class(&mut self) -> io::Result<ClassType> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Create a centered box for class selection
        let border_width = 70;
        let border_height = 14;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height) / 2).max(0) as u16;

        self.draw_game_border(
            start_x as usize,
            start_y as usize,
            border_width as usize,
            border_height as usize,
        )?;

        let title = "Choose Your Class";
        let title_pos_x = start_x + (border_width - title.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, start_y - 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print(title),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(start_x + 5, start_y + 3),
            style::Print("1. Warrior - A powerful melee fighter with high health"),
            cursor::MoveTo(start_x + 5, start_y + 5),
            style::Print("2. Mage - A spellcaster with powerful magical abilities"),
            cursor::MoveTo(start_x + 5, start_y + 7),
            style::Print("3. Ranger - A skilled archer with balanced stats"),
            cursor::MoveTo(start_x + 5, start_y + 9),
            style::Print("4. Cleric - A healer with supportive abilities"),
            cursor::MoveTo(start_x + 5, start_y + 12),
            style::Print("Press the number key to select your class..."),
            cursor::Hide
        )?;

        let class_type = loop {
            if let Event::Key(key_event) = event::read()? {
                // On Windows, only process key press events
                #[cfg(windows)]
                {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                }

                match key_event.code {
                    KeyCode::Char('1') => break ClassType::Warrior,
                    KeyCode::Char('2') => break ClassType::Mage,
                    KeyCode::Char('3') => break ClassType::Ranger,
                    KeyCode::Char('4') => break ClassType::Cleric,
                    _ => {}
                }
            }
        };

        // Flush any remaining input to prevent interference with main game
        self.flush_input_buffer()?;

        Ok(class_type)
    }

    /// Flush any remaining input events from the buffer to prevent interference
    fn flush_input_buffer(&mut self) -> io::Result<()> {
        use crossterm::event::{poll, read};
        use std::time::Duration;

        // Read and discard any pending input events
        while poll(Duration::from_millis(1))? {
            let _ = read()?;
        }

        Ok(())
    }

    pub fn draw_game_screen(
        &mut self,
        player: &Player,
        level: &Level,
        dungeon: &Dungeon,
    ) -> io::Result<()> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Define our game dimensions with added padding
        let content_width = MAP_WIDTH + UI_PANEL_WIDTH;
        let content_height = MAP_HEIGHT;

        // Add border padding to create outer border dimensions
        let outer_width = content_width + (BORDER_PADDING * 2);
        let outer_height = content_height + (BORDER_PADDING * 2);

        // Make sure we have enough space
        if term_width < (outer_width as u16 + 2) || term_height < (outer_height as u16 + 2) {
            // Terminal too small, display error message
            execute!(
                stdout(),
                cursor::MoveTo(0, 0),
                style::SetForegroundColor(Color::Red),
                style::Print(format!(
                    "Terminal too small! Need at least {}x{}",
                    outer_width + 2,
                    outer_height + 2
                ))
            )?;
            return Ok(());
        }

        // Calculate the starting coordinates to center the game (for outer border)
        let border_start_x = ((term_width as usize - outer_width) / 2).max(2);
        let border_start_y = ((term_height as usize - outer_height) / 2).max(2);

        // Calculate inner content starting position (inside the border)
        let content_start_x = border_start_x + BORDER_PADDING;
        let content_start_y = border_start_y + BORDER_PADDING;

        // Draw border around the game area
        self.draw_game_border(border_start_x, border_start_y, outer_width, outer_height)?;

        // Calculate center point of our view
        let center_x = MAP_WIDTH / 2;
        let center_y = MAP_HEIGHT / 2;

        // Windows-specific optimized rendering
        #[cfg(windows)]
        {
            // Check if running in Command Prompt for specialized optimization
            let is_cmd = platform::is_command_prompt();

            if is_cmd {
                // Command Prompt specialized rendering - line-by-line with minimal colors
                self.render_cmd_optimized(
                    level,
                    center_x,
                    center_y,
                    content_start_x,
                    content_start_y,
                )?;
            } else {
                // Standard Windows Terminal/PowerShell rendering with centralized fog of war
                // Batch all rendering operations for better Windows performance
                let mut render_buffer = Vec::new();
                let fog_of_war = create_fog_of_war();

                for screen_y in 0..MAP_HEIGHT {
                    for screen_x in 0..MAP_WIDTH {
                        // Calculate map coordinates by offsetting from player position
                        let map_x = level.player_position.x - center_x as i32 + screen_x as i32;
                        let map_y = level.player_position.y - center_y as i32 + screen_y as i32;
                        let pos = Position::new(map_x, map_y);

                        // Use centralized fog of war processing
                        let fog_result =
                            fog_of_war.process_position(level, pos, level.player_position);

                        // Convert fog color to terminal color
                        let terminal_color = if let Some(fog_color) = fog_result.color {
                            FogOfWar::to_terminal_color(&fog_color)
                        } else {
                            Color::Black
                        };

                        if fog_result.should_render {
                            render_buffer.push((
                                (content_start_x + screen_x) as u16,
                                (content_start_y + screen_y) as u16,
                                terminal_color,
                                fog_result.character,
                            ));
                        }
                    }
                }

                // Batch render all characters with minimal color changes
                let mut current_color = Color::White;
                for (x, y, color, ch) in render_buffer {
                    queue!(stdout(), cursor::MoveTo(x, y))?;
                    if color != current_color {
                        queue!(stdout(), style::SetForegroundColor(color))?;
                        current_color = color;
                    }
                    queue!(stdout(), style::Print(ch))?;
                }
                stdout().flush()?;
            }
        }

        // Non-Windows platforms use original rendering
        // Non-Windows systems with full ANSI support using centralized fog of war
        #[cfg(not(windows))]
        {
            let fog_of_war = create_fog_of_war();

            for screen_y in 0..MAP_HEIGHT {
                for screen_x in 0..MAP_WIDTH {
                    // Calculate map coordinates by offsetting from player position
                    let map_x = level.player_position.x - center_x as i32 + screen_x as i32;
                    let map_y = level.player_position.y - center_y as i32 + screen_y as i32;
                    let pos = Position::new(map_x, map_y);

                    // Use centralized fog of war processing
                    let fog_result = fog_of_war.process_position(level, pos, level.player_position);

                    if fog_result.should_render {
                        // Convert fog color to terminal color
                        let terminal_color = if let Some(fog_color) = fog_result.color {
                            FogOfWar::to_terminal_color(&fog_color)
                        } else {
                            Color::Black
                        };

                        execute!(
                            stdout(),
                            cursor::MoveTo(
                                (content_start_x + screen_x) as u16,
                                (content_start_y + screen_y) as u16
                            ),
                            style::SetForegroundColor(terminal_color),
                            style::Print(fog_result.character)
                        )?;
                    }
                }
            }
        }

        // UI panel starts to the right of the map
        let ui_start_x = content_start_x + MAP_WIDTH;

        // Draw vertical divider between map and UI panel
        #[cfg(windows)]
        {
            // Batch vertical divider rendering on Windows
            queue!(stdout(), style::SetForegroundColor(Color::White))?;
            for y in 0..MAP_HEIGHT {
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_start_x as u16, (content_start_y + y) as u16),
                    style::Print("│")
                )?;
            }
            stdout().flush()?;
        }
        #[cfg(not(windows))]
        {
            for y in 0..MAP_HEIGHT {
                execute!(
                    stdout(),
                    cursor::MoveTo(ui_start_x as u16, (content_start_y + y) as u16),
                    style::SetForegroundColor(Color::White),
                    style::Print("│")
                )?;
            }
        }

        // Draw player stats in the UI panel
        let ui_text_x = ui_start_x + 2; // Offset from the divider

        // Player stats rendering with Windows optimization
        #[cfg(windows)]
        {
            let is_cmd = platform::is_command_prompt();

            if is_cmd {
                // Simplified UI for Command Prompt - fewer colors, simpler layout
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 1) as u16),
                    style::SetForegroundColor(Color::White),
                    style::Print(format!("{} L{}", player.name, player.level))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 2) as u16),
                    style::Print(format!("HP:{}/{}", player.health, player.max_health))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 3) as u16),
                    style::Print(format!("MP:{}/{}", player.mana, player.max_mana))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 4) as u16),
                    style::Print(format!("Gold:{}", player.gold))
                )?;
                stdout().flush()?;
            } else {
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 1) as u16),
                    style::SetForegroundColor(Color::Cyan),
                    style::Print(format!("{}", player.name))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 2) as u16),
                    style::SetForegroundColor(Color::White)
                )?;
                queue!(
                    stdout(),
                    style::Print(format!(
                        "Level {} {}",
                        player.level, player.class.class_type
                    ))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 3) as u16),
                    style::Print(format!("HP: {}/{}", player.health, player.max_health))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 4) as u16),
                    style::Print(format!("MP: {}/{}", player.mana, player.max_mana))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 5) as u16),
                    style::Print(format!("XP: {}/{}", player.experience, player.level * 100))
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 6) as u16),
                    style::Print(format!("Gold: {}", player.gold))
                )?;
                stdout().flush()?;
            }
        }
        #[cfg(not(windows))]
        {
            execute!(
                stdout(),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 1) as u16),
                style::SetForegroundColor(Color::Cyan),
                style::Print(player.name.to_string()),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 2) as u16),
                style::SetForegroundColor(Color::White),
                style::Print(format!(
                    "Level {} {}",
                    player.level, player.class.class_type
                )),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 3) as u16),
                style::Print(format!("HP: {}/{}", player.health, player.max_health)),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 4) as u16),
                style::Print(format!("MP: {}/{}", player.mana, player.max_mana)),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 5) as u16),
                style::Print(format!("XP: {}/{}", player.experience, player.level * 100)),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 6) as u16),
                style::Print(format!("Gold: {}", player.gold))
            )?;
        }

        // Location information with Windows optimization
        #[cfg(windows)]
        {
            let is_cmd = platform::is_command_prompt();

            if is_cmd {
                // Simplified location info for Command Prompt
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 6) as u16),
                    style::SetForegroundColor(Color::White),
                    style::Print(format!("{} L{}", dungeon.name, dungeon.current_level + 1))
                )?;
                stdout().flush()?;
            } else {
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 8) as u16),
                    style::SetForegroundColor(Color::Cyan),
                    style::Print("Location:")
                )?;
                queue!(
                    stdout(),
                    cursor::MoveTo(ui_text_x as u16, (content_start_y + 9) as u16),
                    style::SetForegroundColor(Color::White),
                    style::Print(format!(
                        "{} - Level {}",
                        dungeon.name,
                        dungeon.current_level + 1
                    ))
                )?;
                stdout().flush()?;
            }
        }
        #[cfg(not(windows))]
        {
            execute!(
                stdout(),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 8) as u16),
                style::SetForegroundColor(Color::Cyan),
                style::Print("Location:"),
                cursor::MoveTo(ui_text_x as u16, (content_start_y + 9) as u16),
                style::SetForegroundColor(Color::White),
                style::Print(format!(
                    "{} - Level {}",
                    dungeon.name,
                    dungeon.current_level + 1
                ))
            )?;
        }

        // Draw message log below the border
        let log_start_y = border_start_y + outer_height + 1; // Position below the border

        // Draw message log header
        execute!(
            stdout(),
            cursor::MoveTo(border_start_x as u16, log_start_y as u16),
            style::SetForegroundColor(Color::Cyan),
            style::Print(format!(
                "Message Log: [{}/{}]",
                self.messages.len().min(2),
                self.messages.len()
            ))
        )?;

        // Calculate available width for messages
        let available_width = outer_width;

        // Show the most recent messages first (reversed)
        let recent_messages: Vec<&String> = self.messages.iter().rev().take(2).collect();

        for (i, message) in recent_messages.iter().enumerate() {
            // Truncate long messages
            let truncated_message = if message.len() > available_width {
                format!("{}...", &message[0..available_width.saturating_sub(3)])
            } else {
                message.to_string()
            };

            execute!(
                stdout(),
                cursor::MoveTo(border_start_x as u16, log_start_y as u16 + 1 + i as u16),
                style::SetForegroundColor(Color::White),
                style::Print(truncated_message)
            )?;
        }

        // Position for Symbol Legend outside the game border (right side)
        let legend_col_x = border_start_x + outer_width + 2; // 2 spaces after border
        let legend_start_y = border_start_y + 10; // Below controls

        // Position for Controls outside the game border
        let controls_col_x = border_start_x + outer_width + 2; // 2 spaces after border
        let controls_start_y = border_start_y + 2; // Starting near the top of the border

        // Draw symbol legend outside the game border (right side)
        execute!(
            stdout(),
            cursor::MoveTo(legend_col_x as u16, legend_start_y as u16),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Symbol Legend:")
        )?;

        // Create a legend of symbols and their meanings
        #[cfg(windows)]
        let symbols = if platform::is_command_prompt() {
            // Simplified legend for Command Prompt
            vec![
                ('@', "You", Color::Yellow),
                ('E', "Enemy", Color::Red),
                ('!', "Item", Color::Green),
                ('#', "Wall", Color::White),
                ('.', "Floor", Color::White),
                ('E', "Exit", Color::Green),
            ]
        } else {
            vec![
                ('@', "You (the player)", Color::Yellow),
                ('E', "Enemy", Color::Red),
                ('!', "Item", Color::Green),
                ('#', "Wall", Color::White),
                ('.', "Floor", Color::DarkGrey),
                ('+', "Door", Color::Magenta),
                ('C', "Chest", Color::Cyan),
                ('>', "Stairs Down", Color::Blue),
                ('<', "Stairs Up", Color::Blue),
                ('E', "Exit", Color::Green),
            ]
        };

        #[cfg(not(windows))]
        let symbols = vec![
            ('@', "You (the player)", Color::Yellow),
            ('E', "Enemy", Color::Red),
            ('!', "Item", Color::Green),
            ('#', "Wall", Color::White),
            ('.', "Floor", Color::DarkGrey),
            ('+', "Door", Color::Magenta),
            ('C', "Chest", Color::Cyan),
            ('>', "Stairs Down", Color::Blue),
            ('<', "Stairs Up", Color::Blue),
            ('E', "Exit", Color::Green),
        ];

        for (i, (symbol, meaning, color)) in symbols.iter().enumerate() {
            if !meaning.is_empty() {
                execute!(
                    stdout(),
                    cursor::MoveTo(legend_col_x as u16, (legend_start_y + 1 + i) as u16),
                    style::SetForegroundColor(*color),
                    style::Print(*symbol),
                    style::SetForegroundColor(Color::White),
                    style::Print(format!(" - {meaning}"))
                )?;
            }
        }

        // Draw controls outside the game border
        execute!(
            stdout(),
            cursor::MoveTo(controls_col_x as u16, controls_start_y as u16),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Controls:"),
            cursor::MoveTo(controls_col_x as u16, (controls_start_y + 1) as u16),
            style::SetForegroundColor(Color::White),
            style::Print("↑↓←→: Move"),
            cursor::MoveTo(controls_col_x as u16, (controls_start_y + 2) as u16),
            style::Print("I: Inventory"),
            cursor::MoveTo(controls_col_x as u16, (controls_start_y + 3) as u16),
            style::Print("C: Character"),
            cursor::MoveTo(controls_col_x as u16, (controls_start_y + 4) as u16),
            style::Print("G: Get item"),
            cursor::MoveTo(controls_col_x as u16, (controls_start_y + 5) as u16),
            style::Print("Q: Quit")
        )?;

        Ok(())
    }

    // Helper function to draw a border around the game area
    fn draw_game_border(
        &self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
    ) -> io::Result<()> {
        // Check terminal dimensions
        let (term_width, term_height) = terminal::size()?;

        // Ensure we don't start drawing outside the terminal
        let safe_start_x = start_x.min(term_width as usize - 1);
        let safe_start_y = start_y.min(term_height as usize - 1);

        // Draw title at the top of the border
        let title = "Echoes of the Forgotten Realm";
        let title_start = safe_start_x + (width - title.len()) / 2;

        // Draw top border with title
        execute!(
            stdout(),
            cursor::MoveTo(safe_start_x as u16, (safe_start_y - 1) as u16),
            style::SetForegroundColor(Color::White),
            style::Print("┌")
        )?;

        for x in 1..width - 1 {
            let pos_x = start_x + x;
            if pos_x >= title_start && pos_x < title_start + title.len() {
                // Part of the title
                let char_idx = pos_x - title_start;
                execute!(
                    stdout(),
                    cursor::MoveTo((safe_start_x + x) as u16, (safe_start_y - 1) as u16),
                    style::SetForegroundColor(Color::Cyan),
                    style::Print(title.chars().nth(char_idx).unwrap_or(' '))
                )?;
            } else {
                // Regular border
                execute!(
                    stdout(),
                    cursor::MoveTo(pos_x as u16, (start_y - 1) as u16),
                    style::SetForegroundColor(Color::White),
                    style::Print("─")
                )?;
            }
        }

        execute!(
            stdout(),
            cursor::MoveTo((safe_start_x + width - 1) as u16, (safe_start_y - 1) as u16),
            style::SetForegroundColor(Color::White),
            style::Print("┐")
        )?;

        // Draw bottom border
        execute!(
            stdout(),
            cursor::MoveTo(safe_start_x as u16, (safe_start_y + height) as u16),
            style::SetForegroundColor(Color::White),
            style::Print("└")
        )?;

        for x in 1..width - 1 {
            execute!(
                stdout(),
                cursor::MoveTo((safe_start_x + x) as u16, (safe_start_y + height) as u16),
                style::SetForegroundColor(Color::White),
                style::Print("─")
            )?;
        }

        execute!(
            stdout(),
            cursor::MoveTo(
                (safe_start_x + width - 1) as u16,
                (safe_start_y + height) as u16
            ),
            style::SetForegroundColor(Color::White),
            style::Print("┘")
        )?;

        // Draw left and right borders
        for y in 0..height {
            execute!(
                stdout(),
                cursor::MoveTo(safe_start_x as u16, (safe_start_y + y) as u16),
                style::SetForegroundColor(Color::White),
                style::Print("│")
            )?;

            execute!(
                stdout(),
                cursor::MoveTo((safe_start_x + width - 1) as u16, (safe_start_y + y) as u16),
                style::SetForegroundColor(Color::White),
                style::Print("│")
            )?;
        }

        Ok(())
    }

    pub fn draw_inventory_screen(&mut self, player: &Player) -> io::Result<()> {
        self.clear_screen()?;

        execute!(
            stdout(),
            cursor::MoveTo(30, 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Inventory"),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(10, 3),
            style::Print(format!("Gold: {}", player.gold))
        )?;

        if InventoryManager::is_empty(player) {
            execute!(
                stdout(),
                cursor::MoveTo(10, 5),
                style::Print("Your inventory is empty.")
            )?;
        } else {
            execute!(
                stdout(),
                cursor::MoveTo(5, 5),
                style::Print("Items:"),
                cursor::MoveTo(5, 6),
                style::Print("------")
            )?;

            let items = InventoryManager::get_items(player);
            for (i, item_info) in items.iter().enumerate() {
                let equipped_marker = if item_info.is_equipped { " [E]" } else { "" };

                execute!(
                    stdout(),
                    cursor::MoveTo(5, 7 + i as u16),
                    style::Print(format!("{}. {}{}", i + 1, item_info.name, equipped_marker))
                )?;
            }
        }

        execute!(
            stdout(),
            cursor::MoveTo(10, SCREEN_HEIGHT as u16 - 3),
            style::Print("Press a number key to use/equip an item, E to exit...")
        )?;

        Ok(())
    }

    pub fn draw_character_screen(&mut self, player: &Player) -> io::Result<()> {
        self.clear_screen()?;

        execute!(
            stdout(),
            cursor::MoveTo(30, 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Character Sheet"),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(10, 3),
            style::Print(format!("Name: {}", player.name)),
            cursor::MoveTo(10, 4),
            style::Print(format!("Class: {}", player.class.class_type)),
            cursor::MoveTo(10, 5),
            style::Print(format!("Level: {}", player.level)),
            cursor::MoveTo(10, 6),
            style::Print(format!(
                "Experience: {}/{}",
                player.experience,
                player.level * 100
            )),
            cursor::MoveTo(10, 7),
            style::Print(format!("Health: {}/{}", player.health, player.max_health)),
            cursor::MoveTo(10, 8),
            style::Print(format!("Mana: {}/{}", player.mana, player.max_mana)),
            cursor::MoveTo(10, 9),
            style::Print(format!("Gold: {}", player.gold)),
            cursor::MoveTo(10, 11),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Stats:"),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(10, 12),
            style::Print(format!("Strength: {}", player.stats.strength)),
            cursor::MoveTo(10, 13),
            style::Print(format!("Intelligence: {}", player.stats.intelligence)),
            cursor::MoveTo(10, 14),
            style::Print(format!("Dexterity: {}", player.stats.dexterity)),
            cursor::MoveTo(10, 15),
            style::Print(format!("Constitution: {}", player.stats.constitution)),
            cursor::MoveTo(10, 16),
            style::Print(format!("Wisdom: {}", player.stats.wisdom)),
            cursor::MoveTo(40, 11),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Abilities:"),
            style::SetForegroundColor(Color::White)
        )?;

        // Display abilities
        for (i, ability) in player.class.abilities.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(40, 12 + i as u16),
                style::Print(format!("{}. {}", i + 1, ability))
            )?;
        }

        // Display derived stats
        execute!(
            stdout(),
            cursor::MoveTo(40, 18),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Combat Stats:"),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(40, 19),
            style::Print(format!("Attack: {}", player.attack_damage())),
            cursor::MoveTo(40, 20),
            style::Print(format!("Defense: {}", player.defense()))
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(10, SCREEN_HEIGHT as u16 - 3),
            style::Print("Press any key to return...")
        )?;

        Ok(())
    }

    pub fn draw_combat_screen(&mut self, player: &Player, enemy: &Enemy) -> io::Result<()> {
        self.clear_screen()?;

        execute!(
            stdout(),
            cursor::MoveTo(30, 1),
            style::SetForegroundColor(Color::Red),
            style::Print("Combat!"),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(10, 3),
            style::Print(format!("You are fighting a {}!", enemy.name)),
            cursor::MoveTo(10, 5),
            style::Print(format!(
                "Player HP: {}/{}",
                player.health, player.max_health
            )),
            cursor::MoveTo(10, 6),
            style::Print(format!("Player MP: {}/{}", player.mana, player.max_mana)),
            cursor::MoveTo(10, 8),
            style::Print(format!("Enemy HP: {}/{}", enemy.health, enemy.max_health)),
            cursor::MoveTo(10, 10),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Actions:"),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(10, 11),
            style::Print("1. Attack"),
            cursor::MoveTo(10, 12),
            style::Print("2. Use Ability"),
            cursor::MoveTo(10, 13),
            style::Print("3. Use Item"),
            cursor::MoveTo(10, 14),
            style::Print("4. Flee")
        )?;

        // Display message log
        execute!(
            stdout(),
            cursor::MoveTo(10, 16),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Combat Log:"),
            style::SetForegroundColor(Color::White)
        )?;

        for (i, message) in self.messages.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(10, 17 + i as u16),
                style::Print(message)
            )?;
        }

        Ok(())
    }

    pub fn draw_ability_selection(&mut self, player: &Player) -> io::Result<usize> {
        self.clear_screen()?;

        execute!(
            stdout(),
            cursor::MoveTo(30, 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Select Ability"),
            style::SetForegroundColor(Color::White)
        )?;

        if player.class.abilities.is_empty() {
            execute!(
                stdout(),
                cursor::MoveTo(10, 5),
                style::Print("You don't have any abilities yet!")
            )?;

            execute!(
                stdout(),
                cursor::MoveTo(10, 7),
                style::Print("Press any key to return to combat...")
            )?;

            event::read()?;
            return Err(io::Error::other("No abilities available"));
        }

        for (i, ability) in player.class.abilities.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(10, 5 + i as u16),
                style::Print(format!("{}. {}", i + 1, ability))
            )?;
        }

        execute!(
            stdout(),
            cursor::MoveTo(10, 5 + player.class.abilities.len() as u16 + 2),
            style::Print("Press the number key to select an ability, or ESC to cancel...")
        )?;

        loop {
            if let Event::Key(key_event) = event::read()? {
                // On Windows, only process key press events
                #[cfg(windows)]
                {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                }

                match key_event.code {
                    KeyCode::Char(c) if ('1'..='9').contains(&c) => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        if index < player.class.abilities.len() {
                            return Ok(index);
                        }
                    }
                    KeyCode::Esc => {
                        return Err(io::Error::other("Cancelled"));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw_item_selection(&mut self, player: &Player) -> io::Result<usize> {
        self.clear_screen()?;

        execute!(
            stdout(),
            cursor::MoveTo(30, 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Select Item"),
            style::SetForegroundColor(Color::White)
        )?;

        let consumables: Vec<(usize, &Item)> = player
            .inventory
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| matches!(item, Item::Consumable(_)))
            .collect();

        if consumables.is_empty() {
            execute!(
                stdout(),
                cursor::MoveTo(10, 5),
                style::Print("You don't have any usable items!")
            )?;

            execute!(
                stdout(),
                cursor::MoveTo(10, 7),
                style::Print("Press any key to return to combat...")
            )?;

            event::read()?;
            return Err(io::Error::other("No usable items available"));
        }

        for (i, (_item_index, item)) in consumables.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(10, 5 + i as u16),
                style::Print(format!("{}. {}", i + 1, item.name()))
            )?;
        }

        execute!(
            stdout(),
            cursor::MoveTo(10, 5 + consumables.len() as u16 + 2),
            style::Print("Press the number key to select an item, or ESC to cancel...")
        )?;

        loop {
            if let Event::Key(key_event) = event::read()? {
                // On Windows, only process key press events
                #[cfg(windows)]
                {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                }

                match key_event.code {
                    KeyCode::Char(c) if ('1'..='9').contains(&c) => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        if index < consumables.len() {
                            return Ok(consumables[index].0);
                        }
                    }
                    KeyCode::Esc => {
                        return Err(io::Error::other("Cancelled"));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn handle_combat_action(&mut self, player: &Player) -> io::Result<CombatAction> {
        loop {
            if let Event::Key(key_event) = event::read()? {
                // On Windows, only process key press events
                #[cfg(windows)]
                {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                }

                match key_event.code {
                    KeyCode::Char('1') => return Ok(CombatAction::Attack),
                    KeyCode::Char('2') => {
                        if let Ok(ability_index) = self.draw_ability_selection(player) {
                            return Ok(CombatAction::UseAbility(ability_index));
                        }
                    }
                    KeyCode::Char('3') => {
                        if let Ok(item_index) = self.draw_item_selection(player) {
                            return Ok(CombatAction::UseItem(item_index));
                        }
                    }
                    KeyCode::Char('4') => return Ok(CombatAction::Flee),
                    _ => {}
                }
            }
        }
    }

    pub fn wait_for_key(&mut self) -> io::Result<KeyEvent> {
        use crossterm::event::{poll, read};
        use std::time::Duration;

        loop {
            // Poll with timeout to prevent infinite blocking
            if poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = read()? {
                    // On Windows, filter out key release events to prevent double input
                    #[cfg(windows)]
                    {
                        if key_event.kind == KeyEventKind::Press {
                            return Ok(platform::normalize_key_event(key_event));
                        }
                    }
                    #[cfg(not(windows))]
                    {
                        // On other platforms, use the original behavior
                        return Ok(platform::normalize_key_event(key_event));
                    }
                }
            } else {
                // No input available, yield CPU briefly and continue polling
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }

    /// Command Prompt optimized rendering - renders line by line with minimal colors
    #[cfg(windows)]
    fn render_cmd_optimized(
        &mut self,
        level: &Level,
        center_x: usize,
        center_y: usize,
        content_start_x: usize,
        content_start_y: usize,
    ) -> io::Result<()> {
        use crossterm::style::Color;

        // Build entire screen as strings to minimize terminal operations
        let mut screen_lines = Vec::new();

        for screen_y in 0..MAP_HEIGHT {
            let mut line_chars = Vec::new();
            let mut line_colors = Vec::new();

            for screen_x in 0..MAP_WIDTH {
                let map_x = level.player_position.x - center_x as i32 + screen_x as i32;
                let map_y = level.player_position.y - center_y as i32 + screen_y as i32;

                if map_x < 0
                    || map_x >= level.width as i32
                    || map_y < 0
                    || map_y >= level.height as i32
                {
                    line_chars.push(' ');
                    line_colors.push(Color::Black);
                    continue;
                }

                let pos = Position::new(map_x, map_y);
                let tile = &level.tiles[map_y as usize][map_x as usize];

                let (char_to_draw, color) = if pos == level.player_position {
                    ('@', Color::Yellow)
                } else if !tile.explored {
                    (' ', Color::Black)
                } else if tile.visible && level.enemies.contains_key(&pos) {
                    ('E', Color::Red)
                } else if tile.visible && level.items.contains_key(&pos) {
                    ('!', Color::Green)
                } else if !tile.visible {
                    (' ', Color::Black) // Complete fog of war for Command Prompt
                } else {
                    // Simplified tile rendering for Command Prompt
                    match tile.tile_type {
                        crate::world::TileType::Wall => ('#', Color::White),
                        crate::world::TileType::Floor => ('.', Color::DarkGrey),
                        crate::world::TileType::Door => ('+', Color::Cyan),
                        crate::world::TileType::StairsDown => ('>', Color::Blue),
                        crate::world::TileType::StairsUp => ('<', Color::Blue),
                        crate::world::TileType::Chest => ('C', Color::Cyan),
                        crate::world::TileType::Exit => ('E', Color::Green),
                    }
                };

                line_chars.push(char_to_draw);
                line_colors.push(color);
            }

            screen_lines.push((line_chars, line_colors));
        }

        // Render line by line with color optimization for Command Prompt
        for (y, (chars, colors)) in screen_lines.iter().enumerate() {
            queue!(
                stdout(),
                cursor::MoveTo(content_start_x as u16, (content_start_y + y) as u16)
            )?;

            let mut current_color = Color::White;
            let mut line_buffer = String::new();
            let mut buffer_color = Color::White;

            for (i, (&ch, &color)) in chars.iter().zip(colors.iter()).enumerate() {
                if color != buffer_color || i == chars.len() - 1 {
                    // Flush current buffer if color changes or at end
                    if !line_buffer.is_empty() {
                        if buffer_color != current_color {
                            queue!(stdout(), style::SetForegroundColor(buffer_color))?;
                            current_color = buffer_color;
                        }
                        queue!(stdout(), style::Print(&line_buffer))?;
                        line_buffer.clear();
                    }

                    if i == chars.len() - 1 {
                        // Handle last character
                        if color != current_color {
                            queue!(stdout(), style::SetForegroundColor(color))?;
                        }
                        queue!(stdout(), style::Print(ch))?;
                    } else {
                        buffer_color = color;
                        line_buffer.push(ch);
                    }
                } else {
                    line_buffer.push(ch);
                }
            }
        }

        stdout().flush()?;
        Ok(())
    }

    pub fn draw_game_over(&mut self, player: &Player) -> io::Result<()> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Create a centered box for game over screen
        let border_width = 60;
        let border_height = 10;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height) / 2).max(0) as u16;

        self.draw_game_border(
            start_x as usize,
            start_y as usize,
            border_width as usize,
            border_height as usize,
        )?;

        let title = "Game Over";
        let title_pos_x = start_x + (border_width - title.len() as u16) / 2;

        let message = format!(
            "{} died at level {} after a brave adventure.",
            player.name, player.level
        );
        let message_pos_x = start_x + (border_width - message.len() as u16) / 2;

        let prompt = "Press any key to exit...";
        let prompt_pos_x = start_x + (border_width - prompt.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, start_y + 2),
            style::SetForegroundColor(Color::Red),
            style::Print(title),
            cursor::MoveTo(message_pos_x, start_y + 5),
            style::SetForegroundColor(Color::White),
            style::Print(message),
            cursor::MoveTo(prompt_pos_x, start_y + 8),
            style::Print(prompt)
        )?;

        self.wait_for_key()?;
        Ok(())
    }

    pub fn draw_victory_screen(&mut self, player: &Player) -> io::Result<()> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Create a centered box for victory screen
        let border_width = 70;
        let border_height = 10;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height) / 2).max(0) as u16;

        self.draw_game_border(
            start_x as usize,
            start_y as usize,
            border_width as usize,
            border_height as usize,
        )?;

        let title = "Congratulations! You've won!";
        let title_pos_x = start_x + (border_width - title.len() as u16) / 2;

        let message = format!(
            "{} completed the adventure at level {} and saved the realm!",
            player.name, player.level
        );
        let message_pos_x = start_x + (border_width - message.len() as u16) / 2;

        let prompt = "Press any key to exit...";
        let prompt_pos_x = start_x + (border_width - prompt.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, start_y + 2),
            style::SetForegroundColor(Color::Green),
            style::Print(title),
            cursor::MoveTo(message_pos_x, start_y + 5),
            style::SetForegroundColor(Color::White),
            style::Print(message),
            cursor::MoveTo(prompt_pos_x, start_y + 8),
            style::Print(prompt)
        )?;

        self.wait_for_key()?;
        Ok(())
    }
}
