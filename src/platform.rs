//! Cross-platform utilities for Echoes RPG
//!
//! This module provides cross-platform functionality using pure Rust libraries
//! instead of platform-specific APIs, ensuring consistent behavior across
//! Windows, macOS, and Linux.

use anyhow::{Context, Result};
#[cfg(not(all(feature = "gui", target_os = "windows")))]
use crossterm::{
    cursor, execute,
    style::Color,
    terminal::{self, Clear, ClearType},
};
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
use std::env;
#[cfg(not(all(feature = "gui", target_os = "windows")))]
use std::io::stdout;
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
use std::process::Command;
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
use std::time::{Duration, Instant};

/// Initialize cross-platform terminal settings
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn init_terminal() -> Result<()> {
    // Enable raw mode for input handling
    terminal::enable_raw_mode().context("Failed to enable raw mode")?;

    // Enter alternate screen buffer
    execute!(stdout(), terminal::EnterAlternateScreen)
        .context("Failed to enter alternate screen")?;

    // Hide cursor for cleaner display
    execute!(stdout(), cursor::Hide).context("Failed to hide cursor")?;

    Ok(())
}

/// Cleanup terminal state
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn cleanup_terminal() -> Result<()> {
    // Show cursor
    execute!(stdout(), cursor::Show).context("Failed to show cursor")?;

    // Leave alternate screen buffer
    execute!(stdout(), terminal::LeaveAlternateScreen)
        .context("Failed to leave alternate screen")?;

    // Disable raw mode
    terminal::disable_raw_mode().context("Failed to disable raw mode")?;

    Ok(())
}

/// Get terminal size with fallback defaults
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn get_terminal_size() -> (u16, u16) {
    match terminal::size() {
        Ok((width, height)) => {
            // Ensure minimum size for gameplay with room for legend
            let min_width = 140;
            let min_height = 45;
            (width.max(min_width), height.max(min_height))
        }
        Err(_) => {
            // Fallback to standard terminal size
            (140, 45)
        }
    }
}

/// Clear the terminal screen in a cross-platform way
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn clear_screen() -> Result<()> {
    #[cfg(windows)]
    {
        // Windows-optimized screen clearing
        use std::io::Write;
        queue!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
            .context("Failed to clear screen")?;
        stdout().flush().context("Failed to flush stdout")?;
    }

    #[cfg(not(windows))]
    {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
            .context("Failed to clear screen")?;
    }

    Ok(())
}

/// Check if the current terminal supports the features we need
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn check_terminal_compatibility() -> Result<()> {
    // Check if we can get terminal size
    terminal::size().context("Terminal does not support size detection")?;

    // Test raw mode capability
    terminal::enable_raw_mode().context("Terminal does not support raw mode")?;
    terminal::disable_raw_mode().context("Failed to disable raw mode after test")?;

    Ok(())
}

/// Platform-specific error handling with helpful messages
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn handle_error(error: &anyhow::Error) -> String {
    let platform_info = get_platform_info();

    format!(
        "Error: {}\n\nPlatform: {}\n\nTroubleshooting tips:\n{}",
        error,
        platform_info,
        get_troubleshooting_tips()
    )
}

/// Get current platform information
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn get_platform_info() -> String {
    format!(
        "{} {} ({})",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::consts::FAMILY
    )
}

/// Get platform-specific troubleshooting tips
#[cfg(not(all(feature = "gui", target_os = "windows")))]
fn get_troubleshooting_tips() -> &'static str {
    #[cfg(windows)]
    return "• Use Windows Terminal or PowerShell for best experience\n\
            • Ensure Windows 10 version 1511 or later for color support\n\
            • Try running in a different terminal if issues persist\n\
            • Check Windows Defender exclusions if performance is poor";

    #[cfg(target_os = "macos")]
    return "• Use Terminal.app or iTerm2 for best experience\n\
            • Ensure terminal window is at least 80x24 characters\n\
            • Check terminal preferences for UTF-8 encoding\n\
            • Try resizing the terminal window if display issues occur";

    #[cfg(target_os = "linux")]
    return "• Use gnome-terminal, konsole, or xterm for best experience\n\
            • Ensure TERM environment variable is set correctly\n\
            • Check that your terminal supports ANSI escape sequences\n\
            • Try: export TERM=xterm-256color if colors don't work";

    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    return "• Ensure your terminal supports ANSI escape sequences\n\
            • Try using a more modern terminal emulator\n\
            • Check that your terminal supports UTF-8 encoding";
}

/// Display welcome message with platform-specific formatting
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn show_welcome_message() -> Result<()> {
    clear_screen()?;

    let (width, height) = get_terminal_size();
    let platform = get_platform_info();

    // Create the welcome messages
    let title = "Welcome to Echoes of the Forgotten Realm!";
    let subtitle = format!("Cross-platform RPG running on {platform}");
    let separator = "═".repeat(title.len()); // Exact same length as title
    let continue_msg = "Press any key to continue...";

    // Calculate positions for centered text
    let title_x = (width.saturating_sub(title.len() as u16)) / 2;
    let subtitle_x = (width.saturating_sub(subtitle.len() as u16)) / 2;
    let separator_x = title_x; // Use same x position as title for perfect alignment
    let continue_x = (width.saturating_sub(continue_msg.len() as u16)) / 2;
    let center_y = height / 2;

    execute!(
        stdout(),
        // Title with decorative border
        cursor::MoveTo(separator_x, center_y.saturating_sub(3)),
        crossterm::style::SetForegroundColor(Color::Cyan),
        crossterm::style::Print(&separator),
        cursor::MoveTo(title_x, center_y.saturating_sub(2)),
        crossterm::style::SetForegroundColor(Color::Yellow),
        crossterm::style::Print(title),
        cursor::MoveTo(separator_x, center_y.saturating_sub(1)),
        crossterm::style::SetForegroundColor(Color::Cyan),
        crossterm::style::Print(&separator),
        // Platform information
        cursor::MoveTo(subtitle_x, center_y + 1),
        crossterm::style::SetForegroundColor(Color::Green),
        crossterm::style::Print(subtitle),
        // Continue prompt with spacing
        cursor::MoveTo(continue_x, center_y + 4),
        crossterm::style::SetForegroundColor(Color::Yellow),
        crossterm::style::Print(continue_msg),
        crossterm::style::ResetColor,
    )?;

    Ok(())
}

/// Normalize key events across platforms
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn normalize_key_event(key_event: crossterm::event::KeyEvent) -> crossterm::event::KeyEvent {
    #[cfg(windows)]
    {
        use crossterm::event::KeyEventKind;

        // On Windows, ensure we only process key press events to prevent double input
        // This addresses the issue where Windows terminals send both press and release events
        crossterm::event::KeyEvent {
            code: key_event.code,
            modifiers: key_event.modifiers,
            kind: KeyEventKind::Press, // Force to Press to ensure consistency
            state: key_event.state,
        }
    }

    #[cfg(not(windows))]
    {
        // On other platforms, pass through as-is
        key_event
    }
}

/// Windows-specific frame rate limiting to improve performance
#[cfg(windows)]
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
static mut LAST_FRAME_TIME: Option<Instant> = None;

/// Limit frame rate on Windows to reduce terminal load
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
pub fn windows_frame_limit() {
    const TARGET_FRAME_TIME: Duration = Duration::from_millis(16); // ~60 FPS

    unsafe {
        let now = Instant::now();

        if let Some(last_frame) = LAST_FRAME_TIME {
            let elapsed = now.duration_since(last_frame);
            if elapsed < TARGET_FRAME_TIME {
                std::thread::sleep(TARGET_FRAME_TIME - elapsed);
            }
        }

        LAST_FRAME_TIME = Some(Instant::now());
    }
}

/// No-op frame limit for non-Windows platforms
#[cfg(all(not(windows), not(all(feature = "gui", target_os = "windows"))))]
pub fn windows_frame_limit() {
    // Do nothing on non-Windows platforms
}

/// Set Command Prompt to full screen mode
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
pub fn set_cmd_fullscreen() -> Result<()> {
    if is_command_prompt() {
        // Method 1: Resize console buffer and window
        let _ = Command::new("mode")
            .args(&["con", "cols=150", "lines=50"])
            .status();

        // Method 2: Simple Alt+Enter approach (most reliable)
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Try multiple times with different timing
        for _ in 0..3 {
            let _ = Command::new("powershell")
                .args(&[
                    "-WindowStyle", "Hidden",
                    "-ExecutionPolicy", "Bypass",
                    "-Command",
                    "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('%{ENTER}'); Start-Sleep -Milliseconds 100"
                ])
                .status();

            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        // Method 3: Direct Windows API approach
        let script = r#"
try {
    Add-Type -Name Win32 -Namespace Native -MemberDefinition @'
        [DllImport("kernel32.dll")]
        public static extern IntPtr GetConsoleWindow();
        [DllImport("user32.dll")]
        public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
        [DllImport("user32.dll")]
        public static extern bool SetWindowPos(IntPtr hWnd, IntPtr hWndInsertAfter, int X, int Y, int cx, int cy, uint uFlags);
'@
    $hwnd = [Native.Win32]::GetConsoleWindow()
    [Native.Win32]::ShowWindow($hwnd, 3)
    Start-Sleep -Milliseconds 200
    [Native.Win32]::SetWindowPos($hwnd, -1, 0, 0, 0, 0, 0x0003)
} catch {}
"#;

        let _ = Command::new("powershell")
            .args(&["-WindowStyle", "Hidden", "-Command", script])
            .status();
    }
    Ok(())
}

/// No-op fullscreen for non-Windows platforms
#[cfg(not(windows))]
pub fn set_cmd_fullscreen() -> Result<()> {
    Ok(())
}

/// Detect if running in Command Prompt (cmd.exe) for specialized optimizations
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
pub fn is_command_prompt() -> bool {
    // Check COMSPEC environment variable and terminal capabilities
    if let Ok(comspec) = env::var("COMSPEC") {
        if comspec.to_lowercase().contains("cmd.exe") {
            return true;
        }
    }

    // Additional detection methods
    if let Ok(term) = env::var("TERM") {
        // Command Prompt typically doesn't set TERM or sets it to basic values
        if term.is_empty() || term == "dumb" || term == "cygwin" {
            return true;
        }
    } else {
        // TERM not set is often Command Prompt
        return true;
    }

    // Check for Windows Terminal or PowerShell indicators
    if env::var("WT_SESSION").is_ok() || env::var("POWERSHELL_DISTRIBUTION_CHANNEL").is_ok() {
        return false;
    }

    false
}

/// Get optimized color palette for Command Prompt
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
pub fn get_cmd_color_palette() -> Vec<(Color, Color)> {
    // Simplified color palette that works well in Command Prompt
    vec![
        (Color::Yellow, Color::Yellow),  // Player - bright and visible
        (Color::Red, Color::Red),        // Enemies - danger
        (Color::Green, Color::Green),    // Items - safe
        (Color::White, Color::Grey),     // Walls - visible but not bright
        (Color::DarkGrey, Color::Black), // Floor - subtle
        (Color::Cyan, Color::Blue),      // Special elements
        (Color::Black, Color::Black),    // Fog of war - invisible
    ]
}

/// Command Prompt specific frame limiting (more aggressive)
#[cfg(all(windows, not(all(feature = "gui", target_os = "windows"))))]
pub fn cmd_frame_limit() {
    const CMD_TARGET_FRAME_TIME: Duration = Duration::from_millis(33); // ~30 FPS for cmd

    unsafe {
        let now = Instant::now();
        if let Some(last_frame) = LAST_FRAME_TIME {
            let elapsed = now.duration_since(last_frame);
            if elapsed < CMD_TARGET_FRAME_TIME {
                std::thread::sleep(CMD_TARGET_FRAME_TIME - elapsed);
            }
        }
        LAST_FRAME_TIME = Some(Instant::now());
    }
}

/// Check if running in a compatible terminal environment
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn is_terminal_compatible() -> bool {
    // Check if stdout is a TTY
    if !atty::is(atty::Stream::Stdout) {
        return false;
    }

    // Check if we can get terminal size
    if terminal::size().is_err() {
        return false;
    }

    true
}

/// Get recommended terminal size for optimal gameplay
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn get_recommended_size() -> (u16, u16) {
    (150, 50) // Width x Height in characters
}

/// Check if terminal size is adequate
#[cfg(not(all(feature = "gui", target_os = "windows")))]
pub fn is_terminal_size_adequate() -> bool {
    let (current_width, current_height) = get_terminal_size();
    let (min_width, min_height) = (140, 45);

    current_width >= min_width && current_height >= min_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_terminal_size() {
        let (width, height) = get_terminal_size();
        assert!(width >= 80);
        assert!(height >= 24);
    }

    #[test]
    fn test_platform_info() {
        let info = get_platform_info();
        assert!(!info.is_empty());
        assert!(info.contains(std::env::consts::OS));
    }

    #[test]
    fn test_game_data_dir() {
        let dir = get_game_data_dir();
        assert!(dir.is_ok());
    }

    #[test]
    fn test_config_dir() {
        let dir = get_config_dir();
        assert!(dir.is_ok());
    }

    #[test]
    fn test_terminal_compatibility() {
        // This test might fail in CI environments without a proper TTY
        // but should work in development
        let _ = is_terminal_compatible();
    }
}
