use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{self, Color, Stylize},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write, stdout};
use std::thread;
use std::time::Duration;

use crate::character::{ClassType, Player};
use crate::combat::{CombatAction, CombatResult};
use crate::item::{ConsumableType, Equipment, EquipmentSlot, Item};
use crate::world::{Dungeon, Enemy, Level, Position, Tile, TileType};

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;
const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 20;
const UI_PANEL_WIDTH: usize = 30; // Increased panel width for longer content
const BORDER_PADDING: usize = 2; // Padding inside the border

pub struct UI {
    messages: Vec<String>,
    max_messages: usize,
}

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
        let (term_width, term_height) = terminal::size()?;
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

        // Draw bordered tutorial
        let border_width = 70;
        let border_height = 22;
        let (term_width, term_height) = terminal::size()?;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height as i32) / 2).max(0) as u16;

        self.draw_game_border(
            start_x as usize,
            start_y as usize,
            border_width as usize,
            border_height as usize,
        )?;

        let title = "Combat Tutorial";
        let title_pos_x = start_x + (border_width - title.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, start_y - 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print(title),
            style::SetForegroundColor(Color::White)
        )?;

        // Content positioning
        let text_x = start_x + 3;
        let mut text_y = start_y + 2;

        // Draw tutorial content
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Yellow),
            style::Print("Welcome to your first combat encounter!"),
            style::SetForegroundColor(Color::White)
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("Combat in Echoes RPG is turn-based. Here's how it works:")
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("1. Attack"),
            style::SetForegroundColor(Color::White),
            style::Print(" - Basic attack using your equipped weapon.")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("2. Use Ability"),
            style::SetForegroundColor(Color::White),
            style::Print(" - Use a special ability or spell (costs mana).")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("3. Use Item"),
            style::SetForegroundColor(Color::White),
            style::Print(" - Use a consumable item from your inventory.")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Cyan),
            style::Print("4. Flee"),
            style::SetForegroundColor(Color::White),
            style::Print(" - Attempt to escape combat (chance based on your dexterity).")
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("After you take an action, the enemy will counter-attack.")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print(
                "If you defeat an enemy, you'll gain experience, gold, and possibly items!"
            )
        )?;

        // Add simulated combat example
        text_y += 2;
        let example_x = text_x + 5;

        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
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
            style::Print("You encounter a Goblin (HP: 20/20)")
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
            style::Print("You attack the Goblin for 8 damage!")
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
            style::Print("The Goblin hits you for 5 damage!")
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
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
            style::Print("• Use healing potions when your health is low")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("• Special abilities can deal more damage but cost mana")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("• Sometimes fleeing is the best option if you're outmatched")
        )?;

        text_y += 2;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::Print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        )?;

        text_y += 1;
        execute!(
            stdout(),
            cursor::MoveTo(text_x, text_y),
            style::SetForegroundColor(Color::Green),
            style::Print("Press any key to continue your adventure...")
        )?;

        // Wait for key press
        self.wait_for_key()?;

        Ok(())
    }

    pub fn initialize(&self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), terminal::EnterAlternateScreen)?;
        execute!(stdout(), cursor::Hide)?;
        Ok(())
    }

    pub fn cleanup(&self) -> io::Result<()> {
        execute!(stdout(), terminal::LeaveAlternateScreen)?;
        execute!(stdout(), cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
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

    pub fn draw_title_screen(&self) -> io::Result<()> {
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

    pub fn character_creation(&self) -> io::Result<Player> {
        // Name selection screen
        let name = self.get_character_name()?;

        // Class selection screen
        let class_type = self.choose_character_class()?;

        let player = Player::new(name, class_type);
        Ok(player)
    }

    fn get_character_name(&self) -> io::Result<String> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Create a centered box for name input
        let border_width = 60;
        let border_height = 10;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height as i32) / 2).max(0) as u16;

        self.draw_game_border(
            start_x as usize,
            start_y as usize,
            border_width as usize,
            border_height as usize,
        )?;

        let title = "Character Creation";
        let title_pos_x = start_x + (border_width - title.len() as u16) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x, start_y - 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print(title),
            cursor::MoveTo(start_x + 5, start_y + 3),
            style::SetForegroundColor(Color::White),
            style::Print("Enter your character's name: "),
            cursor::MoveTo(start_x + 33, start_y + 3),
            cursor::Show
        )?;

        terminal::disable_raw_mode()?;
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        name = name.trim().to_string();
        terminal::enable_raw_mode()?;

        if name.is_empty() {
            name = "Hero".to_string();
        }

        Ok(name)
    }

    fn choose_character_class(&self) -> io::Result<ClassType> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Create a centered box for class selection
        let border_width = 70;
        let border_height = 14;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height as i32) / 2).max(0) as u16;

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
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('1') => break ClassType::Warrior,
                    KeyCode::Char('2') => break ClassType::Mage,
                    KeyCode::Char('3') => break ClassType::Ranger,
                    KeyCode::Char('4') => break ClassType::Cleric,
                    _ => continue,
                }
            }
        };

        Ok(class_type)
    }

    pub fn draw_game_screen(
        &self,
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

        // Draw the visible map portion
        for screen_y in 0..MAP_HEIGHT {
            for screen_x in 0..MAP_WIDTH {
                // Calculate map coordinates by offsetting from player position
                // This ensures player is always at the center
                let map_x = level.player_position.x - center_x as i32 + screen_x as i32;
                let map_y = level.player_position.y - center_y as i32 + screen_y as i32;

                // Ensure we're within map bounds
                if map_x < 0
                    || map_x >= level.width as i32
                    || map_y < 0
                    || map_y >= level.height as i32
                {
                    // Draw a space for out-of-bounds areas
                    execute!(
                        stdout(),
                        cursor::MoveTo(
                            (content_start_x + screen_x) as u16,
                            (content_start_y + screen_y) as u16
                        ),
                        style::SetForegroundColor(Color::Black),
                        style::Print(' ')
                    )?;
                    continue;
                }

                let pos = Position::new(map_x, map_y);

                // Determine what character to draw
                let char_to_draw = if pos == level.player_position {
                    '@'
                } else if level.enemies.contains_key(&pos) {
                    'E'
                } else if level.items.contains_key(&pos) {
                    '!'
                } else {
                    // Get the tile at this position
                    level.tiles[map_y as usize][map_x as usize].render()
                };

                let color = match char_to_draw {
                    '@' => Color::Yellow,
                    'E' => Color::Red,
                    '!' => Color::Green,
                    '#' => Color::White,
                    '.' => Color::DarkGrey,
                    '+' => Color::Magenta,
                    'C' => Color::Cyan,
                    '>' | '<' => Color::Blue,
                    _ => Color::Grey,
                };

                execute!(
                    stdout(),
                    cursor::MoveTo(
                        (content_start_x + screen_x) as u16,
                        (content_start_y + screen_y) as u16
                    ),
                    style::SetForegroundColor(color),
                    style::Print(char_to_draw)
                )?;
            }
        }

        // UI panel starts to the right of the map
        let ui_start_x = content_start_x + MAP_WIDTH;

        // Draw vertical divider between map and UI panel
        for y in 0..MAP_HEIGHT {
            execute!(
                stdout(),
                cursor::MoveTo(ui_start_x as u16, (content_start_y + y) as u16),
                style::SetForegroundColor(Color::White),
                style::Print("│")
            )?;
        }

        // Draw player stats in the UI panel
        let ui_text_x = ui_start_x + 2; // Offset from the divider

        execute!(
            stdout(),
            cursor::MoveTo(ui_text_x as u16, (content_start_y + 1) as u16),
            style::SetForegroundColor(Color::Cyan),
            style::Print(format!("{}", player.name)),
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

        // Location information
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

        // Position for Symbol Legend inside UI panel
        let legend_col_x = ui_text_x;
        let section_start_y = content_start_y + 11;

        // Position for Controls outside the game border
        let controls_col_x = border_start_x + outer_width + 2; // 2 spaces after border
        let controls_start_y = border_start_y + 2; // Starting near the top of the border

        // Draw symbol legend in the UI panel (left column)
        execute!(
            stdout(),
            cursor::MoveTo(legend_col_x as u16, section_start_y as u16),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Symbol Legend:")
        )?;

        // Create a legend of symbols and their meanings
        let symbols = [
            ('@', "You (the player)", Color::Yellow),
            ('E', "Enemy", Color::Red),
            ('!', "Item", Color::Green),
            ('#', "Wall", Color::White),
            ('.', "Floor", Color::DarkGrey),
            ('+', "Door", Color::Magenta),
            ('C', "Chest", Color::Cyan),
            ('>', "Stairs Down", Color::Blue),
            ('<', "Stairs Up", Color::Blue),
        ];

        for (i, (symbol, meaning, color)) in symbols.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(legend_col_x as u16, (section_start_y + 1 + i) as u16),
                style::SetForegroundColor(*color),
                style::Print(*symbol),
                style::SetForegroundColor(Color::White),
                style::Print(format!(" - {}", meaning))
            )?;
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

    pub fn draw_inventory_screen(&self, player: &Player) -> io::Result<()> {
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

        if player.inventory.items.is_empty() {
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

            for (i, item) in player.inventory.items.iter().enumerate() {
                let item_name = item.name();
                let equipped_marker = match item {
                    Item::Equipment(equipment) => {
                        if let Some(Some(idx)) = player.inventory.equipped.get(&equipment.slot) {
                            if *idx == i { " [E]" } else { "" }
                        } else {
                            ""
                        }
                    }
                    _ => "",
                };

                execute!(
                    stdout(),
                    cursor::MoveTo(5, 7 + i as u16),
                    style::Print(format!("{}. {}{}", i + 1, item_name, equipped_marker))
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

    pub fn draw_character_screen(&self, player: &Player) -> io::Result<()> {
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
            style::Print(format!(
                "Strength: {}",
                player.stats.get_stat(crate::character::StatType::Strength)
            )),
            cursor::MoveTo(10, 13),
            style::Print(format!(
                "Intelligence: {}",
                player
                    .stats
                    .get_stat(crate::character::StatType::Intelligence)
            )),
            cursor::MoveTo(10, 14),
            style::Print(format!(
                "Dexterity: {}",
                player.stats.get_stat(crate::character::StatType::Dexterity)
            )),
            cursor::MoveTo(10, 15),
            style::Print(format!(
                "Constitution: {}",
                player
                    .stats
                    .get_stat(crate::character::StatType::Constitution)
            )),
            cursor::MoveTo(10, 16),
            style::Print(format!(
                "Wisdom: {}",
                player.stats.get_stat(crate::character::StatType::Wisdom)
            )),
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

    pub fn draw_ability_selection(&self, player: &Player) -> io::Result<usize> {
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
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No abilities available",
            ));
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
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char(c) if c >= '1' && c <= '9' => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        if index < player.class.abilities.len() {
                            return Ok(index);
                        }
                    }
                    KeyCode::Esc => {
                        return Err(io::Error::new(io::ErrorKind::Other, "Cancelled"));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw_item_selection(&self, player: &Player) -> io::Result<usize> {
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
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No usable items available",
            ));
        }

        for (i, (item_index, item)) in consumables.iter().enumerate() {
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
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char(c) if c >= '1' && c <= '9' => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        if index < consumables.len() {
                            return Ok(consumables[index].0);
                        }
                    }
                    KeyCode::Esc => {
                        return Err(io::Error::new(io::ErrorKind::Other, "Cancelled"));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn handle_combat_action(&self, player: &Player) -> io::Result<CombatAction> {
        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('1') => return Ok(CombatAction::Attack),
                    KeyCode::Char('2') => match self.draw_ability_selection(player) {
                        Ok(ability_index) => return Ok(CombatAction::UseAbility(ability_index)),
                        Err(_) => continue,
                    },
                    KeyCode::Char('3') => match self.draw_item_selection(player) {
                        Ok(item_index) => return Ok(CombatAction::UseItem(item_index)),
                        Err(_) => continue,
                    },
                    KeyCode::Char('4') => return Ok(CombatAction::Flee),
                    _ => {}
                }
            }
        }
    }

    pub fn wait_for_key(&self) -> io::Result<KeyEvent> {
        loop {
            if let Event::Key(key_event) = event::read()? {
                return Ok(key_event);
            }
        }
    }

    pub fn draw_game_over(&self, player: &Player) -> io::Result<()> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Create a centered box for game over screen
        let border_width = 60;
        let border_height = 10;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height as i32) / 2).max(0) as u16;

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

    pub fn draw_victory_screen(&self, player: &Player) -> io::Result<()> {
        self.clear_screen()?;

        // Get actual terminal size
        let (term_width, term_height) = terminal::size()?;

        // Create a centered box for victory screen
        let border_width = 70;
        let border_height = 10;
        let start_x = ((term_width as i32 - border_width as i32) / 2).max(0) as u16;
        let start_y = ((term_height as i32 - border_height as i32) / 2).max(0) as u16;

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
