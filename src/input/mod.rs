//! Input handling module for the Echoes RPG GUI
//!
//! This module centralizes all input processing to avoid duplicate key handling
//! and provides a clean interface for different game states.

use egui::{Event, Key};

#[derive(Debug, Clone, PartialEq)]
pub enum InputAction {
    // Character input for names, etc.
    Character(char),
    // Navigation and control
    Enter,
    Backspace,
    // Menu selections
    MenuOption(u8), // 1-9 for menu options
    // Game actions
    Move(Direction),
    Interact,
    Inventory,
    // Special actions
    Exit,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[cfg(feature = "gui")]
#[derive(Default)]
pub struct InputHandler {
    // Event queue for robust input handling
    action_queue: std::collections::VecDeque<InputAction>,
    // Track processed events to prevent duplicates within the same frame
    processed_events: std::collections::HashSet<String>,
    last_processed_frame: u64,
}

impl InputHandler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process egui input events and queue actions for retrieval
    /// This method prevents duplicate processing of the same input
    pub fn process_input(&mut self, ctx: &egui::Context, current_frame: u64) -> Vec<InputAction> {
        // Clear processed events if we're on a new frame
        if current_frame != self.last_processed_frame {
            self.processed_events.clear();
            self.last_processed_frame = current_frame;
        }

        ctx.input(|i| {
            // Process key press events (not text events to avoid duplicates)
            for event in &i.events {
                if let Event::Key {
                    key, pressed: true, ..
                } = event
                {
                    let event_id = format!("{key:?}");

                    // Skip if we already processed this event in this frame
                    if self.processed_events.contains(&event_id) {
                        continue;
                    }

                    self.processed_events.insert(event_id);

                    let action = self.key_to_action(key);
                    if action != InputAction::Unknown {
                        self.action_queue.push_back(action);
                    }
                }
            }
        });

        // Return and drain all queued actions
        self.drain_action_queue()
    }

    /// Drain all queued actions and return them
    /// This provides a more robust way to handle input events
    pub fn drain_action_queue(&mut self) -> Vec<InputAction> {
        let mut actions = Vec::new();
        while let Some(action) = self.action_queue.pop_front() {
            actions.push(action);
        }
        actions
    }

    /// Convert egui Key to InputAction
    fn key_to_action(&self, key: &Key) -> InputAction {
        match key {
            // Numbers for menu options
            Key::Num1 => InputAction::MenuOption(1),
            Key::Num2 => InputAction::MenuOption(2),
            Key::Num3 => InputAction::MenuOption(3),
            Key::Num4 => InputAction::MenuOption(4),
            Key::Num5 => InputAction::MenuOption(5),
            Key::Num6 => InputAction::MenuOption(6),
            Key::Num7 => InputAction::MenuOption(7),
            Key::Num8 => InputAction::MenuOption(8),
            Key::Num9 => InputAction::MenuOption(9),
            Key::Num0 => InputAction::Character('0'),

            // Letters (convert to lowercase for consistency)
            Key::A => InputAction::Character('a'),
            Key::B => InputAction::Character('b'),
            Key::C => InputAction::Character('c'),
            Key::D => InputAction::Character('d'),
            Key::E => InputAction::Character('e'),
            Key::F => InputAction::Character('f'),
            Key::G => InputAction::Character('g'),
            Key::H => InputAction::Character('h'),
            Key::I => InputAction::Character('i'),
            Key::J => InputAction::Character('j'),
            Key::K => InputAction::Character('k'),
            Key::L => InputAction::Character('l'),
            Key::M => InputAction::Character('m'),
            Key::N => InputAction::Character('n'),
            Key::O => InputAction::Character('o'),
            Key::P => InputAction::Character('p'),
            Key::Q => InputAction::Character('q'),
            Key::R => InputAction::Character('r'),
            Key::S => InputAction::Character('s'),
            Key::T => InputAction::Character('t'),
            Key::U => InputAction::Character('u'),
            Key::V => InputAction::Character('v'),
            Key::W => InputAction::Character('w'),
            Key::X => InputAction::Character('x'),
            Key::Y => InputAction::Character('y'),
            Key::Z => InputAction::Character('z'),

            // Special keys
            Key::Space => InputAction::Character(' '),
            Key::Enter => InputAction::Enter,
            Key::Backspace => InputAction::Backspace,
            Key::Escape => InputAction::Exit,

            // Movement keys (WASD and arrow keys)
            Key::ArrowUp => InputAction::Move(Direction::North),
            Key::ArrowDown => InputAction::Move(Direction::South),
            Key::ArrowLeft => InputAction::Move(Direction::West),
            Key::ArrowRight => InputAction::Move(Direction::East),

            _ => InputAction::Unknown,
        }
    }

    /// Clear all processed events (useful when changing game states)
    pub fn clear_state(&mut self) {
        self.processed_events.clear();
    }
}

/// Helper functions for common input patterns
impl InputHandler {
    /// Check if an action is a valid character for name entry
    pub fn is_name_character(action: &InputAction) -> bool {
        match action {
            InputAction::Character(c) => c.is_alphanumeric() || *c == ' ',
            _ => false,
        }
    }

    /// Extract character from action if it's a character action
    pub fn get_character(action: &InputAction) -> Option<char> {
        match action {
            InputAction::Character(c) => Some(*c),
            _ => None,
        }
    }

    /// Check if action is a menu selection
    pub fn is_menu_option(action: &InputAction) -> bool {
        matches!(action, InputAction::MenuOption(_))
    }

    /// Get menu option number
    pub fn get_menu_option(action: &InputAction) -> Option<u8> {
        match action {
            InputAction::MenuOption(n) => Some(*n),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_to_action() {
        let handler = InputHandler::new();

        assert_eq!(
            handler.key_to_action(&Key::Num1),
            InputAction::MenuOption(1)
        );
        assert_eq!(handler.key_to_action(&Key::A), InputAction::Character('a'));
        assert_eq!(
            handler.key_to_action(&Key::Space),
            InputAction::Character(' ')
        );
        assert_eq!(handler.key_to_action(&Key::Enter), InputAction::Enter);
        assert_eq!(
            handler.key_to_action(&Key::Backspace),
            InputAction::Backspace
        );
    }

    #[test]
    fn test_helper_functions() {
        assert!(InputHandler::is_name_character(&InputAction::Character(
            'a'
        )));
        assert!(InputHandler::is_name_character(&InputAction::Character(
            ' '
        )));
        assert!(!InputHandler::is_name_character(&InputAction::Enter));

        assert_eq!(
            InputHandler::get_character(&InputAction::Character('x')),
            Some('x')
        );
        assert_eq!(InputHandler::get_character(&InputAction::Enter), None);

        assert!(InputHandler::is_menu_option(&InputAction::MenuOption(1)));
        assert!(!InputHandler::is_menu_option(&InputAction::Character('1')));

        assert_eq!(
            InputHandler::get_menu_option(&InputAction::MenuOption(3)),
            Some(3)
        );
        assert_eq!(
            InputHandler::get_menu_option(&InputAction::Character('3')),
            None
        );
    }

    #[test]
    fn test_inventory_hotkey_mapping() {
        let handler = InputHandler::new();

        // Test that number keys 1-9 map to MenuOption actions (for inventory hotkeys)
        assert_eq!(
            handler.key_to_action(&Key::Num1),
            InputAction::MenuOption(1)
        );
        assert_eq!(
            handler.key_to_action(&Key::Num5),
            InputAction::MenuOption(5)
        );
        assert_eq!(
            handler.key_to_action(&Key::Num9),
            InputAction::MenuOption(9)
        );

        // Test that 0 still maps to Character action
        assert_eq!(
            handler.key_to_action(&Key::Num0),
            InputAction::Character('0')
        );
    }
}
