//! Cross-platform terminal and system utilities
//!
//! This module provides platform-specific functionality to ensure
//! the game works consistently across Windows, macOS, and Linux.

use std::io::{self, Result};

#[cfg(windows)]
use std::ffi::CString;
#[cfg(windows)]
use winapi::um::consoleapi::AllocConsole;
#[cfg(windows)]
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
#[cfg(windows)]
use winapi::um::processenv::GetStdHandle;
#[cfg(windows)]
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
#[cfg(windows)]
use winapi::um::wincon::{ENABLE_VIRTUAL_TERMINAL_INPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING};
#[cfg(windows)]
use winapi::um::winuser::{GetConsoleWindow, SetConsoleTitle, ShowWindow, SW_SHOW};

/// Initialize platform-specific terminal settings
pub fn init_terminal() -> Result<()> {
    #[cfg(windows)]
    {
        // Enable ANSI escape sequences on Windows 10+
        unsafe {
            let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let stdin_handle = GetStdHandle(STD_INPUT_HANDLE);

            if stdout_handle != std::ptr::null_mut() {
                let mut mode: u32 = 0;
                if GetConsoleMode(stdout_handle, &mut mode) != 0 {
                    let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
                    SetConsoleMode(stdout_handle, new_mode);
                }
            }

            if stdin_handle != std::ptr::null_mut() {
                let mut mode: u32 = 0;
                if GetConsoleMode(stdin_handle, &mut mode) != 0 {
                    let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_INPUT;
                    SetConsoleMode(stdin_handle, new_mode);
                }
            }
        }
    }

    Ok(())
}

/// Launch game in new console window (Windows only)
#[cfg(windows)]
pub fn launch_in_new_window() -> Result<()> {
    unsafe {
        // Check if we already have a console window
        let console_window = GetConsoleWindow();

        if console_window.is_null() {
            // No console window exists, create one
            if AllocConsole() == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to allocate console",
                ));
            }
        }

        // Set console window title
        let title = CString::new("Echoes of the Forgotten Realm - RPG Adventure").unwrap();
        SetConsoleTitle(title.as_ptr());

        // Show the console window
        let console_window = GetConsoleWindow();
        if !console_window.is_null() {
            ShowWindow(console_window, SW_SHOW);
        }

        // Reinitialize stdio handles after console allocation
        init_terminal()?;
    }

    Ok(())
}

/// Launch game in new console window (non-Windows platforms)
#[cfg(not(windows))]
pub fn launch_in_new_window() -> Result<()> {
    // On non-Windows platforms, this is a no-op since the game
    // already runs in the terminal where it was launched
    Ok(())
}

/// Get the appropriate terminal size for the current platform
pub fn get_terminal_size() -> (u16, u16) {
    match crossterm::terminal::size() {
        Ok((width, height)) => (width, height),
        Err(_) => {
            // Fallback to common terminal sizes
            #[cfg(windows)]
            return (120, 30); // Windows Command Prompt default

            #[cfg(not(windows))]
            return (80, 24); // Unix terminal default
        }
    }
}

/// Clear the terminal screen in a cross-platform way
pub fn clear_screen() -> Result<()> {
    #[cfg(windows)]
    {
        // On Windows, we might need additional clearing
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )?;
    }

    #[cfg(not(windows))]
    {
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )?;
    }

    Ok(())
}

/// Platform-specific error handling
pub fn handle_platform_error(error: &str) -> String {
    #[cfg(windows)]
    {
        format!("Windows Error: {}\n\nTip: Make sure you're using Windows 10 or later with a modern terminal (Windows Terminal, PowerShell, or Command Prompt).", error)
    }

    #[cfg(target_os = "macos")]
    {
        format!(
            "macOS Error: {}\n\nTip: Make sure you're using Terminal.app or iTerm2.",
            error
        )
    }

    #[cfg(target_os = "linux")]
    {
        format!(
            "Linux Error: {}\n\nTip: Make sure your terminal supports ANSI escape sequences.",
            error
        )
    }

    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    {
        format!("Platform Error: {}", error)
    }
}

/// Check if the current terminal supports the features we need
pub fn check_terminal_compatibility() -> Result<()> {
    // Check if we can get terminal size
    if crossterm::terminal::size().is_err() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Terminal does not support size detection",
        ));
    }

    // Test basic crossterm functionality
    match crossterm::terminal::enable_raw_mode() {
        Ok(_) => {
            let _ = crossterm::terminal::disable_raw_mode();
            Ok(())
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("Terminal does not support raw mode: {}", e),
        )),
    }
}

/// Get platform-specific game data directory
pub fn get_game_data_dir() -> Option<std::path::PathBuf> {
    #[cfg(windows)]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let mut path = std::path::PathBuf::from(appdata);
            path.push("EchoesRPG");
            return Some(path);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            let mut path = std::path::PathBuf::from(home);
            path.push("Library");
            path.push("Application Support");
            path.push("EchoesRPG");
            return Some(path);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            let mut path = std::path::PathBuf::from(home);
            path.push(".local");
            path.push("share");
            path.push("echoes_rpg");
            return Some(path);
        }
    }

    // Fallback to current directory
    std::env::current_dir().ok()
}

/// Platform-specific key handling adjustments
pub fn normalize_key_event(key_event: crossterm::event::KeyEvent) -> crossterm::event::KeyEvent {
    #[cfg(windows)]
    {
        // Windows might send different key codes for some keys
        match key_event.code {
            crossterm::event::KeyCode::Enter => {
                // Ensure Enter key works consistently
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Enter,
                    modifiers: key_event.modifiers,
                    kind: key_event.kind,
                    state: key_event.state,
                }
            }
            _ => key_event,
        }
    }

    #[cfg(not(windows))]
    {
        key_event
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_size() {
        let (width, height) = get_terminal_size();
        assert!(width > 0);
        assert!(height > 0);
    }

    #[test]
    fn test_game_data_dir() {
        let dir = get_game_data_dir();
        assert!(dir.is_some());
    }

    #[test]
    fn test_error_handling() {
        let error_msg = handle_platform_error("test error");
        assert!(error_msg.contains("test error"));
    }
}
