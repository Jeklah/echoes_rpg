use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlDivElement, HtmlInputElement};

use crate::character::{ClassType, Player};
use crate::game::Game;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct WebGame {
    game: Game,
    output_element: HtmlDivElement,
    input_element: HtmlInputElement,
    current_input: String,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebGame, JsValue> {
        console::log_1(&"Initializing Echoes RPG Web Version...".into());

        let window = window().unwrap();
        let document = window.document().unwrap();

        // Create the game container
        let container = document.create_element("div")?;
        container.set_id("game-container");
        container.set_attribute(
            "style",
            "font-family: 'Courier New', monospace;
             background: rgba(0, 17, 0, 0.9);
             color: #00ff00;
             padding: 10px;
             border: 2px solid #00ff00;
             border-radius: 10px;
             box-shadow: 0 0 20px rgba(0, 255, 0, 0.3);
             display: flex;
             flex-direction: column;
             overflow: hidden;
             box-sizing: border-box;
             position: absolute;
             top: 70px;
             left: 10px;
             right: 10px;
             bottom: 40px;
             width: auto;
             height: auto;
             max-width: 1200px;
             margin: 0 auto;",
        )?;

        // Create output area
        let output = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        output.set_id("game-output");
        output.set_attribute(
            "style",
            "white-space: pre-wrap;
             background: #000000;
             color: #00ff00;
             padding: 15px;
             border: 1px solid #006600;
             border-radius: 5px;
             flex: 1;
             overflow-y: auto;
             font-size: clamp(12px, 1.5vw, 16px);
             line-height: 1.4;
             margin-bottom: 10px;
             box-shadow: inset 0 0 10px rgba(0, 255, 0, 0.1);
             min-height: 0;
             max-height: 100%;
             box-sizing: border-box;",
        )?;

        // Create input area
        let input = document
            .create_element("input")?
            .dyn_into::<HtmlInputElement>()?;
        input.set_id("game-input");
        input.set_type("text");
        input.set_placeholder("Enter command...");
        input.set_attribute(
            "style",
            "width: 100%;
             background: #000;
             color: #00ff00;
             border: 2px solid #00ff00;
             border-radius: 5px;
             padding: 12px;
             font-family: 'Courier New', monospace;
             font-size: clamp(12px, 1.5vw, 16px);
             outline: none;
             transition: all 0.3s ease;
             flex-shrink: 0;
             box-sizing: border-box;",
        )?;

        // Add elements to container
        container.append_child(&output)?;
        container.append_child(&input)?;

        // Add container to body
        document.body().unwrap().append_child(&container)?;

        let player = Player::new("WebPlayer".to_string(), ClassType::Warrior);
        let game_instance = Game::new(player);

        let game = WebGame {
            game: game_instance,
            output_element: output,
            input_element: input,
            current_input: String::new(),
        };

        Ok(game)
    }

    #[wasm_bindgen]
    pub fn start_game(&mut self) -> Result<(), JsValue> {
        self.display_welcome_message()?;
        self.setup_input_handlers()?;
        self.display_main_menu()?;
        Ok(())
    }

    fn display_welcome_message(&self) -> Result<(), JsValue> {
        let welcome_text = r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║  ███████╗ ██████╗██╗  ██╗ ██████╗ ███████╗███████╗    ██████╗ ██████╗  ██████╗ ║
║  ██╔════╝██╔════╝██║  ██║██╔═══██╗██╔════╝██╔════╝    ██╔══██╗██╔══██╗██╔════╝ ║
║  █████╗  ██║     ███████║██║   ██║█████╗  ███████╗    ██████╔╝██████╔╝██║  ███╗║
║  ██╔══╝  ██║     ██╔══██║██║   ██║██╔══╝  ╚════██║    ██╔══██╗██╔═══╝ ██║   ██║║
║  ███████╗╚██████╗██║  ██║╚██████╔╝███████╗███████║    ██║  ██║██║     ╚██████╔╝║
║  ╚══════╝ ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝    ╚═╝  ╚═╝╚═╝      ╚═════╝ ║
║                                                                               ║
║                        A Cross-Platform Text Adventure                       ║
║                              Web Version                                     ║
╚═══════════════════════════════════════════════════════════════════════════════╝

Welcome to Echoes RPG - Web Edition!

This is the browser version of the text-based RPG adventure.
Use the input field below to enter commands and navigate through the world.

Press Enter or click outside this area to continue...
"#;

        self.output_element.set_inner_text(welcome_text);
        Ok(())
    }

    fn display_main_menu(&self) -> Result<(), JsValue> {
        let menu_text = r#"
═══════════════════════════════════════════════════════════════════════════════
                                MAIN MENU
═══════════════════════════════════════════════════════════════════════════════

Please choose an option:

1. Start New Game
2. Continue Game (if save exists)
3. View Instructions
4. Exit

Enter your choice (1-4): "#;

        self.append_output(menu_text)?;
        Ok(())
    }

    fn setup_input_handlers(&self) -> Result<(), JsValue> {
        let input_element = self.input_element.clone();
        let output_element = self.output_element.clone();

        // Create closure for handling input
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Enter" {
                let input_value = input_element.value();
                if !input_value.is_empty() {
                    // Display the command
                    let current_output = output_element.inner_text();
                    let new_output = format!("{}\n> {}\n", current_output, input_value);
                    output_element.set_inner_text(&new_output);

                    // Process the command (this would call your game logic)
                    WebGame::process_command(&input_value, &output_element);

                    // Clear input
                    input_element.set_value("");

                    // Scroll to bottom
                    output_element.set_scroll_top(output_element.scroll_height());
                }
            }
        }) as Box<dyn FnMut(_)>);

        self.input_element
            .add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Keep the closure alive

        Ok(())
    }

    fn append_output(&self, text: &str) -> Result<(), JsValue> {
        let current_output = self.output_element.inner_text();
        let new_output = format!("{}\n{}", current_output, text);
        self.output_element.set_inner_text(&new_output);
        self.output_element
            .set_scroll_top(self.output_element.scroll_height());
        Ok(())
    }

    // Static method to process commands (this would integrate with your game logic)
    fn process_command(input: &str, output_element: &HtmlDivElement) {
        let response = match input.trim() {
            "1" => {
                r#"Starting new game...

You find yourself standing at the edge of a mysterious forest.
The ancient trees tower above you, their branches swaying in an otherworldly breeze.
A narrow path leads deeper into the woods, while behind you lies the safety of the village.

What would you like to do?
- 'go forward' to enter the forest
- 'go back' to return to the village
- 'look' to examine your surroundings
- 'inventory' to check your items
- 'help' for more commands"#
            },
            "2" => "No save file found. Starting a new game instead...",
            "3" => {
                r#"═══════════════════════════════════════════════════════════════════════════════
                              GAME INSTRUCTIONS
═══════════════════════════════════════════════════════════════════════════════

BASIC COMMANDS:
- Movement: 'go [direction]', 'north', 'south', 'east', 'west'
- Interaction: 'look', 'examine [item]', 'take [item]', 'use [item]'
- Combat: 'attack', 'defend', 'flee'
- Character: 'inventory', 'stats', 'health'
- Game: 'save', 'load', 'help', 'quit'

TIPS:
- Type 'look' to get a description of your current location
- Use 'inventory' to see what items you're carrying
- Pay attention to your health and stamina
- Save your game frequently!

Type 'menu' to return to the main menu."#
            },
            "4" | "exit" | "quit" => "Thanks for playing Echoes RPG! Refresh the page to start again.",
            "menu" => {
                r#"═══════════════════════════════════════════════════════════════════════════════
                                MAIN MENU
═══════════════════════════════════════════════════════════════════════════════

Please choose an option:

1. Start New Game
2. Continue Game (if save exists)
3. View Instructions
4. Exit

Enter your choice (1-4):"#
            },
            "help" => {
                "Available commands: go, look, take, use, inventory, stats, attack, defend, flee, save, load, help, menu, quit"
            },
            "look" => {
                "You are in a dark forest. Tall trees surround you on all sides. A path leads north deeper into the woods."
            },
            "inventory" => {
                "Your inventory:\n- Rusty sword\n- Health potion x2\n- 50 gold coins"
            },
            "stats" => {
                "Your Stats:\nLevel: 1\nHealth: 100/100\nStamina: 50/50\nStrength: 10\nDefense: 8\nExperience: 0/100"
            },
            _ => {
                "I don't understand that command. Type 'help' for available commands."
            }
        };

        let current_output = output_element.inner_text();
        let new_output = format!("{}\n{}\n", current_output, response);
        output_element.set_inner_text(&new_output);
        output_element.set_scroll_top(output_element.scroll_height());
    }
}

// Initialize the game when the WASM module loads
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"WASM module loaded successfully!".into());

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

    console::log_1(&"Creating single game instance...".into());
    let mut game = WebGame::new()?;
    game.start_game()?;

    console::log_1(&"Game initialized successfully!".into());

    // Keep the game instance alive (in a real implementation, you'd want to store this properly)
    std::mem::forget(game);

    Ok(())
}
