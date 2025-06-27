use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{self, Color, Stylize},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write, stdout};

use crate::character::{ClassType, Player};
use crate::combat::{CombatAction, CombatResult};
use crate::item::{ConsumableType, Equipment, EquipmentSlot, Item};
use crate::world::{Dungeon, Enemy, Level, Position, Tile, TileType};

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

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

    pub fn add_messages_from_combat(&mut self, result: &CombatResult) {
        for message in &result.messages {
            self.add_message(message.clone());
        }
    }

    pub fn draw_title_screen(&self) -> io::Result<()> {
        self.clear_screen()?;

        let title = "Echoes of the Forgotten Realm";
        let author = "A Rusty Adventure";

        let title_pos_x = (SCREEN_WIDTH - title.len()) / 2;
        let author_pos_x = (SCREEN_WIDTH - author.len()) / 2;

        execute!(
            stdout(),
            cursor::MoveTo(title_pos_x as u16, 5),
            style::SetForegroundColor(Color::Cyan),
            style::Print(title),
            cursor::MoveTo(author_pos_x as u16, 7),
            style::SetForegroundColor(Color::White),
            style::Print(author),
            cursor::MoveTo(30, 12),
            style::Print("1. New Game"),
            cursor::MoveTo(30, 14),
            style::Print("2. Exit"),
            cursor::MoveTo(0, SCREEN_HEIGHT as u16 - 1),
            style::Print("Press the corresponding key to select an option..."),
        )?;

        Ok(())
    }

    pub fn character_creation(&self) -> io::Result<Player> {
        self.clear_screen()?;

        execute!(
            stdout(),
            cursor::MoveTo(20, 2),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Character Creation"),
            style::SetForegroundColor(Color::White),
        )?;

        // Get character name
        execute!(
            stdout(),
            cursor::MoveTo(10, 5),
            style::Print("Enter your character's name: "),
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

        // Choose class
        self.clear_screen()?;
        execute!(
            stdout(),
            cursor::MoveTo(20, 2),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Choose Your Class"),
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(10, 5),
            style::Print("1. Warrior - A powerful melee fighter with high health"),
            cursor::MoveTo(10, 6),
            style::Print("2. Mage - A spellcaster with powerful magical abilities"),
            cursor::MoveTo(10, 7),
            style::Print("3. Ranger - A skilled archer with balanced stats"),
            cursor::MoveTo(10, 8),
            style::Print("4. Cleric - A healer with supportive abilities"),
            cursor::MoveTo(10, 10),
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

        let player = Player::new(name, class_type);
        Ok(player)
    }

    pub fn draw_game_screen(
        &self,
        player: &Player,
        level: &Level,
        dungeon: &Dungeon,
    ) -> io::Result<()> {
        self.clear_screen()?;

        // Draw the map
        for y in 0..level.height.min(20) {
            for x in 0..level.width.min(60) {
                let pos = Position::new(x as i32, y as i32);
                let tile = &level.tiles[y][x];

                let char_to_draw = if pos == level.player_position {
                    '@'
                } else if level.enemies.contains_key(&pos) {
                    'E'
                } else if level.items.contains_key(&pos) {
                    '!'
                } else {
                    tile.render()
                };

                let color = match char_to_draw {
                    '@' => Color::Yellow,
                    'E' => Color::Blue,
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
                    cursor::MoveTo(x as u16, y as u16),
                    style::SetForegroundColor(color),
                    style::Print(char_to_draw)
                )?;
            }
        }

        // Draw UI borders
        for x in 0..SCREEN_WIDTH {
            execute!(
                stdout(),
                cursor::MoveTo(x as u16, 20),
                style::SetForegroundColor(Color::White),
                style::Print("-")
            )?;
        }

        for y in 0..SCREEN_HEIGHT {
            execute!(
                stdout(),
                cursor::MoveTo(60, y as u16),
                style::SetForegroundColor(Color::White),
                style::Print("|")
            )?;
        }

        // Draw player stats
        execute!(
            stdout(),
            cursor::MoveTo(62, 1),
            style::SetForegroundColor(Color::Cyan),
            style::Print(format!("{}", player.name)),
            cursor::MoveTo(62, 2),
            style::SetForegroundColor(Color::White),
            style::Print(format!(
                "Level {} {}",
                player.level, player.class.class_type
            )),
            cursor::MoveTo(62, 3),
            style::Print(format!("HP: {}/{}", player.health, player.max_health)),
            cursor::MoveTo(62, 4),
            style::Print(format!("MP: {}/{}", player.mana, player.max_mana)),
            cursor::MoveTo(62, 5),
            style::Print(format!("XP: {}/{}", player.experience, player.level * 100)),
            cursor::MoveTo(62, 6),
            style::Print(format!("Gold: {}", player.gold)),
            cursor::MoveTo(62, 8),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Location:"),
            cursor::MoveTo(62, 9),
            style::SetForegroundColor(Color::White),
            style::Print(format!(
                "{} - Level {}",
                dungeon.name,
                dungeon.current_level + 1
            ))
        )?;

        // Draw message log
        execute!(
            stdout(),
            cursor::MoveTo(1, 21),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Message Log:")
        )?;

        for (i, message) in self.messages.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(1, 22 + i as u16),
                style::SetForegroundColor(Color::White),
                style::Print(message)
            )?;
        }

        // Draw controls
        execute!(
            stdout(),
            cursor::MoveTo(62, 18),
            style::SetForegroundColor(Color::Cyan),
            style::Print("Controls:"),
            cursor::MoveTo(62, 19),
            style::SetForegroundColor(Color::White),
            style::Print("Arrow keys: Move"),
            cursor::MoveTo(62, 20),
            style::Print("I: Inventory"),
            cursor::MoveTo(62, 21),
            style::Print("C: Character"),
            cursor::MoveTo(62, 22),
            style::Print("G: Get item"),
            cursor::MoveTo(62, 23),
            style::Print("Q: Quit")
        )?;

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

    pub fn draw_combat_screen(&self, player: &Player, enemy: &Enemy) -> io::Result<()> {
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

        execute!(
            stdout(),
            cursor::MoveTo(30, 10),
            style::SetForegroundColor(Color::Red),
            style::Print("Game Over"),
            cursor::MoveTo(20, 12),
            style::SetForegroundColor(Color::White),
            style::Print(format!(
                "{} died at level {} after a brave adventure.",
                player.name, player.level
            )),
            cursor::MoveTo(25, 14),
            style::Print("Press any key to exit...")
        )?;

        self.wait_for_key()?;
        Ok(())
    }

    pub fn draw_victory_screen(&self, player: &Player) -> io::Result<()> {
        self.clear_screen()?;

        execute!(
            stdout(),
            cursor::MoveTo(25, 10),
            style::SetForegroundColor(Color::Green),
            style::Print("Congratulations! You've won!"),
            cursor::MoveTo(15, 12),
            style::SetForegroundColor(Color::White),
            style::Print(format!(
                "{} completed the adventure at level {} and saved the realm!",
                player.name, player.level
            )),
            cursor::MoveTo(25, 14),
            style::Print("Press any key to exit...")
        )?;

        self.wait_for_key()?;
        Ok(())
    }
}
