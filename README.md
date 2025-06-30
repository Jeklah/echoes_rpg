# Echoes of the Forgotten Realm

A cross-platform text-based RPG adventure game built with Rust, featuring turn-based combat, character progression, and procedurally generated dungeons.

## 🎮 About the Game

Echoes of the Forgotten Realm is a terminal-based RPG featuring:
- **Turn-based Combat**: Strategic combat with multiple abilities and items
- **Character Progression**: Level up your character and improve stats
- **Equipment System**: Find and equip weapons, armor, and accessories
- **Inventory Management**: Collect and manage items throughout your journey
- **Procedural Dungeons**: Explore randomly generated levels
- **Cross-Platform**: Runs seamlessly on Windows, macOS, and Linux
- **Terminal-Based**: Retro ASCII graphics in your favorite terminal

## 🖥️ Platform Support

| Platform | Status | Best Experience | Notes |
|----------|--------|-----------------|-------|
| **Windows 10+** | ✅ Full Support | GUI Launchers | Auto-opens new window, double-click friendly |
| **Windows 8.1/7** | ⚠️ Limited Support | Command Prompt | Basic functionality, limited colors |
| **macOS** | ✅ Full Support | iTerm2, Terminal.app | Excellent compatibility |
| **Linux** | ✅ Full Support | Most modern terminals | Wide compatibility |

## 🚀 Quick Start

### Windows (Recommended)
```batch
# Download and navigate to game folder, then:

# GUI Launcher (Double-click friendly)
launch_game.bat

# OR Advanced PowerShell launcher
launch_game.ps1

# OR Create desktop shortcut
create_desktop_shortcut.bat
```

### macOS/Linux
```bash
# Clone and run
git clone https://github.com/yourusername/echoes_rpg.git
cd echoes_rpg
./run_unix.sh
```

### Manual Installation (Any Platform)
```bash
# If you have Rust installed
cargo run --release
```

## 📦 Installation Guide

### Prerequisites

**Required:**
- **Rust** 1.70+ (install from [rustup.rs](https://rustup.rs/))
- **Git** for cloning the repository
- **Modern terminal** with ANSI color support

**Recommended Terminals:**
- **Windows**: Windows Terminal, PowerShell
- **macOS**: iTerm2, Terminal.app
- **Linux**: gnome-terminal, konsole, xterm

### Windows Installation

#### Option 1: GUI Launcher (Easiest - Recommended)
1. Download or clone the repository:
   ```bash
   git clone https://github.com/yourusername/echoes_rpg.git
   cd echoes_rpg
   ```
2. **Double-click** `launch_game.bat` - Opens automatically in new window
3. Or run `launch_game.ps1` in PowerShell for advanced GUI features

**Features:**
- ✅ Automatic new window creation
- ✅ System requirements checking
- ✅ Build progress indication
- ✅ Professional presentation
- ✅ Error handling with helpful tips

#### Option 2: Windows Terminal
1. Install [Windows Terminal](https://aka.ms/terminal) from Microsoft Store
2. Open Windows Terminal (PowerShell or Command Prompt)
3. Run the installation commands:
   ```bash
   git clone https://github.com/yourusername/echoes_rpg.git
   cd echoes_rpg
   
   # GUI launcher (recommended)
   launch_game.bat
   
   # OR manual build and run
   cargo run --release
   ```

#### Option 3: Command Prompt
1. Open Command Prompt as Administrator
2. Enable ANSI support (Windows 10 1511+):
   ```cmd
   reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
   ```
3. Follow the same git clone and cargo commands as above

#### Creating Desktop Shortcut
```batch
# After installation, run this to create a desktop shortcut:
create_desktop_shortcut.bat
```

### macOS Installation

1. Open Terminal.app or iTerm2
2. Install Rust if not already installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```
3. Clone and build:
   ```bash
   git clone https://github.com/yourusername/echoes_rpg.git
   cd echoes_rpg
   cargo run --release
   ```

### Linux Installation

#### Ubuntu/Debian
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/yourusername/echoes_rpg.git
cd echoes_rpg
cargo run --release
```

#### Fedora/RHEL
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/yourusername/echoes_rpg.git
cd echoes_rpg
cargo run --release
```

#### Arch Linux
```bash
# Install Rust
sudo pacman -S rustup
rustup default stable

# Clone and build
git clone https://github.com/yourusername/echoes_rpg.git
cd echoes_rpg
cargo run --release
```

## 🎯 Game Controls

### Exploration
- **Arrow Keys** - Move your character
- **G** - Get items or loot chests
- **I** - Open inventory
- **C** - View character stats
- **Q** - Quit game

### Combat
- **1** - Attack enemy
- **2** - Use special ability
- **3** - Use consumable item
- **4** - Attempt to flee

### Inventory
- **1-9** - Use or equip items
- **E/Esc** - Return to game

## 🎨 Game Symbols

| Symbol | Meaning |
|--------|---------|
| `@` | Player character |
| `E` | Enemy |
| `C` | Treasure chest |
| `!` | Item on ground |
| `#` | Wall |
| `.` | Floor |
| `+` | Door |
| `>` | Stairs down |
| `<` | Stairs up |

## 🛠️ Build Options

### Development Build
```bash
cargo run
```

### Optimized Release Build
```bash
cargo run --release
```

### Windows GUI Launchers
```batch
# Simple GUI launcher (double-click friendly)
launch_game.bat

# Advanced PowerShell GUI with progress dialogs
launch_game.ps1 [--debug] [--clean]

# Create desktop shortcut
create_desktop_shortcut.bat
```

### Platform-Specific Builds

#### Windows
```bash
# For Windows MSVC
cargo build --release --target x86_64-pc-windows-msvc

# For Windows GNU
cargo build --release --target x86_64-pc-windows-gnu
```

#### macOS
```bash
# For Intel Macs
cargo build --release --target x86_64-apple-darwin

# For Apple Silicon Macs
cargo build --release --target aarch64-apple-darwin
```

#### Linux
```bash
# For x86_64 Linux
cargo build --release --target x86_64-unknown-linux-gnu

# For ARM64 Linux
cargo build --release --target aarch64-unknown-linux-gnu
```

## 🔧 Troubleshooting

### Windows Issues

**Problem**: Colors not displaying correctly
**Solution**: 
1. Use the GUI launchers (`launch_game.bat` or `launch_game.ps1`)
2. Use Windows Terminal instead of Command Prompt
3. Enable ANSI support: `reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1`
4. Update to Windows 10 version 1511 or later

**Problem**: Game crashes on startup
**Solution**:
1. Use the GUI launchers which handle window creation automatically
2. Run as administrator
3. Check Windows Defender exclusions
4. Try PowerShell launcher instead of batch file

**Problem**: No new window opens when double-clicking .exe
**Solution**:
1. Use `launch_game.bat` instead of running the .exe directly
2. The game automatically creates a new console window when launched properly
3. Create a desktop shortcut using `create_desktop_shortcut.bat`

### macOS Issues

**Problem**: Terminal size detection fails
**Solution**:
1. Use Terminal.app or iTerm2
2. Ensure terminal window is large enough (minimum 80x25)
3. Check terminal preferences for compatibility mode

### Linux Issues

**Problem**: Input not working correctly
**Solution**:
1. Ensure your terminal supports raw mode
2. Try different terminal emulators (gnome-terminal, konsole, etc.)
3. Check locale settings: `export LANG=en_US.UTF-8`

**Problem**: Colors not displaying
**Solution**:
1. Check TERM environment variable: `echo $TERM`
2. Set TERM to a color-capable terminal: `export TERM=xterm-256color`

### General Issues

**Problem**: Compilation errors
**Solution**:
1. Update Rust: `rustup update`
2. Clean build cache: `cargo clean`
3. Check Rust version: `rustc --version` (should be 1.70+)

**Problem**: Performance issues
**Solution**:
1. Use release build: `cargo run --release`
2. Increase terminal buffer size
3. Close other applications using terminal resources

## 💾 Save Files

Game progress is saved automatically in platform-specific locations:

- **Windows**: `%APPDATA%\EchoesRPG\`
- **macOS**: `~/Library/Application Support/EchoesRPG/`
- **Linux**: `~/.local/share/echoes_rpg/`

## 🎨 Terminal Compatibility

### Recommended Terminals

#### Windows
- ✅ **GUI Launchers** - Best experience, auto-opens new window
- ✅ **Windows Terminal** - Excellent support, full color
- ✅ **PowerShell** - Good compatibility
- ⚠️ **Command Prompt** - Basic support, limited colors

#### macOS
- ✅ **iTerm2** - Excellent support, best performance
- ✅ **Terminal.app** - Good support
- ⚠️ **VS Code Terminal** - Limited support

#### Linux
- ✅ **gnome-terminal** - Excellent support
- ✅ **konsole** - Excellent support
- ✅ **xterm** - Good support
- ✅ **urxvt** - Good support
- ⚠️ **tmux/screen** - May have input issues

## 📊 System Requirements

### Minimum Requirements
- **OS**: Windows 7, macOS 10.12, Linux (any modern distribution)
- **Memory**: 4 MB RAM
- **Storage**: 50 MB available space
- **Terminal**: Any terminal with cursor control support

### Recommended Requirements
- **OS**: Windows 10+ (with GUI launchers), macOS 10.15+, Linux with modern terminal
- **Memory**: 8 MB RAM
- **Storage**: 100 MB available space
- **Terminal**: Modern terminal with full ANSI color support
- **Windows**: Use GUI launchers for optimal experience

## 🔗 Dependencies

The game uses these cross-platform Rust crates:
- `crossterm` - Cross-platform terminal manipulation
- `rand` - Random number generation
- `serde` - Serialization/deserialization
- `winapi` - Windows-specific APIs (Windows only)

## 📚 Game Guide

### Getting Started
1. Create your character by choosing a class
2. Learn the combat system with the tutorial
3. Explore the dungeon and fight enemies
4. Collect items and equipment to grow stronger
5. Progress through multiple dungeon levels

### Combat Tips
- Use special abilities wisely (limited uses)
- Keep healing items in your inventory
- Equipment significantly affects your combat effectiveness
- Sometimes fleeing is the better option

### Exploration Tips
- Search every room for chests and items
- Use the 'G' key to interact with chests and pick up items
- Check your character stats regularly to track progress
- Manage your inventory space efficiently

## 🛠️ Development

### Building from Source
```bash
# Development build (faster compilation)
cargo run

# Release build (optimized performance)
cargo run --release

# Run tests
cargo test

# Clean build cache
cargo clean
```

### Cross-Compilation
The project supports cross-compilation for all major platforms. See the GitHub Actions workflow for automated builds.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for:
- Bug fixes
- New features
- Platform-specific improvements
- Documentation updates

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on:
- Setting up development environment
- Platform-specific testing
- Cross-compilation guidelines
- Code style and standards

## 📄 License

This project is dual-licensed under the MIT OR Apache-2.0 license.

## 🆘 Support

If you encounter platform-specific issues:

1. Check this troubleshooting guide
2. Search existing [GitHub Issues](https://github.com/yourusername/echoes_rpg/issues)
3. Create a new issue with:
   - Your operating system and version
   - Terminal type and version
   - Rust version (`rustc --version`)
   - Complete error message
   - Steps to reproduce

## 🔗 Links

- [Cross-Platform Implementation Details](CROSS_PLATFORM_IMPLEMENTATION.md)
- [Windows GUI Features](WINDOWS_GUI_FEATURES.md)
- [Issue Tracker](https://github.com/yourusername/echoes_rpg/issues)
- [Rust Installation](https://rustup.rs/)

---

*Adventure awaits in the Echoes of the Forgotten Realm! 🗡️⚔️*

*Happy adventuring across all platforms! 🎮*