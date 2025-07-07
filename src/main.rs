mod character;
mod combat;
mod game;
mod inventory;
mod item;
mod platform;
mod ui;
mod world;

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "gui")]
mod input;

fn main() {
    // Check if GUI feature is enabled and we're on Windows
    #[cfg(all(feature = "gui", target_os = "windows"))]
    {
        // Run GUI version on Windows when feature is enabled
        if let Err(e) = gui::run_gui() {
            eprintln!("Failed to run GUI: {e}");
            std::process::exit(1);
        }
    }
    #[cfg(not(all(feature = "gui", target_os = "windows")))]
    {
        // Fall back to terminal version
        run_terminal_version();
    }
}

#[cfg(not(all(feature = "gui", target_os = "windows")))]
fn run_terminal_version() {
    // Check if running in a compatible terminal
    if !platform::is_terminal_compatible() {
        eprintln!("Error: This game requires a terminal environment to run.");
        eprintln!("Please run from a terminal/command prompt.");
        std::process::exit(1);
    }

    // Check terminal compatibility
    if let Err(e) = platform::check_terminal_compatibility() {
        eprintln!("{}", platform::handle_error(&e));
        std::process::exit(1);
    }

    // Set Command Prompt to full screen if applicable
    if let Err(e) = platform::set_cmd_fullscreen() {
        eprintln!("Warning: Could not set fullscreen mode: {e}");
    }

    // Check terminal size
    if !platform::is_terminal_size_adequate() {
        let (current_w, current_h) = platform::get_terminal_size();
        let (rec_w, rec_h) = platform::get_recommended_size();
        eprintln!(
            "Warning: Terminal size ({current_w}, {current_h}) is smaller than recommended ({rec_w}, {rec_h})"
        );
        eprintln!("Game may not display correctly. Consider resizing your terminal.");
    }

    // Initialize terminal for the game
    if let Err(e) = platform::init_terminal() {
        eprintln!("{}", platform::handle_error(&e));
        std::process::exit(1);
    }

    // Show welcome message
    if let Err(e) = platform::show_welcome_message() {
        eprintln!("Failed to display welcome message: {e}");
        platform::cleanup_terminal().ok();
        std::process::exit(1);
    }

    // Wait for user input before starting
    if crossterm::event::read().is_err() {
        eprintln!("Failed to read user input");
        platform::cleanup_terminal().ok();
        std::process::exit(1);
    }

    // Run the game
    let result = std::panic::catch_unwind(|| {
        game::run();
    });

    // Ensure cleanup happens even if game panics
    if let Err(e) = platform::cleanup_terminal() {
        eprintln!("Failed to cleanup terminal: {e}");
    }

    // Handle any panics that occurred
    if let Err(panic) = result {
        eprintln!("Game crashed: {panic:?}");
        std::process::exit(1);
    }
}
