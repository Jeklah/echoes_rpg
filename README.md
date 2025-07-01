# Echoes of the Forgotten Realm

A cross-platform text-based RPG adventure game built with Rust, featuring turn-based combat, character progression, and procedurally generated dungeons. Available in both terminal and GUI versions.

## 🎮 About the Game

Echoes of the Forgotten Realm is a cross-platform RPG featuring:
- **Turn-based Combat**: Strategic combat with multiple abilities and items
- **Character Progression**: Level up your character and improve stats
- **Equipment System**: Find and equip weapons, armor, and accessories
- **Inventory Management**: Collect and manage items throughout your journey
- **Procedural Dungeons**: Explore randomly generated levels
- **Cross-Platform**: Runs seamlessly on Windows, macOS, and Linux
- **Dual Interface**: Choose between terminal-based or GUI versions
- **Retro ASCII Graphics**: Classic text-based gaming experience

## 🖥️ Interface Options

### Terminal Version (Default)
- Classic terminal-based interface
- Runs in any modern terminal
- Full keyboard controls
- Cross-platform compatibility

### GUI Version (Windows)
- Native Windows application
- Improved input handling
- Better visual integration
- Optimized for Windows users

## 🖥️ Platform Support

| Platform | Terminal Version | GUI Version | Best Terminal | Notes |
|----------|------------------|-------------|---------------|-------|
| **Windows 10+** | ✅ Full Support | ✅ Available | Windows Terminal | GUI version with enhanced input handling |
| **Windows 8.1/7** | ⚠️ Limited Support | ✅ Available | Command Prompt | GUI recommended for older Windows |
| **macOS** | ✅ Full Support | ❌ Not Available | iTerm2, Terminal.app | Excellent terminal compatibility |
| **Linux** | ✅ Full Support | ❌ Not Available | Most modern terminals | Wide terminal compatibility |

## 🚀 Quick Start

### Windows (GUI Version - Recommended)
```bash
git clone https://github.com/yourusername/echoes_rpg.git
cd echoes_rpg
cargo build --features gui --release
./target/release/echoes_rpg.exe
```

### Windows (Terminal Version)
```bash
git clone https://github.com/yourusername/echoes_rpg.git
cd echoes_rpg
cargo run --release
```

### macOS/Linux
```bash
git clone https://github.com/yourusername/echoes_rpg.git
cd echoes_rpg
cargo run --release
```

### Prerequisites
- **Rust** 1.70+ (see installation instructions below)
- **Git** for cloning the repository

## 🔧 Installing Rust

### Using Rustup (Recommended)

**Rustup** is the official Rust toolchain installer and version manager. It's the easiest way to install and manage Rust.

#### Windows
1. **Download and run the installer:**
   - Visit [rustup.rs](https://rustup.rs/) 
   - Download `rustup-init.exe`
   - Run the installer and follow the prompts
   - **OR** run this command in PowerShell:
   ```powershell
   Invoke-WebRequest -Uri "https://win.rustup.rs/" -OutFile "rustup-init.exe"; .\rustup-init.exe
   ```

2. **Follow the installation prompts:**
   - Choose option 1 (default installation)
   - Restart your terminal or run: `source $env:USERPROFILE\.cargo\env`

3. **Verify installation:**
   ```bash
   rustc --version
   cargo --version
   ```

#### macOS
1. **Install via terminal:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Follow the installation prompts:**
   - Choose option 1 (default installation)
   - Restart your terminal or run: `source ~/.cargo/env`

3. **Verify installation:**
   ```bash
   rustc --version
   cargo --version
   ```

#### Linux
1. **Install via terminal:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Follow the installation prompts:**
   - Choose option 1 (default installation)
   - Restart your terminal or run: `source ~/.cargo/env`

3. **Verify installation:**
   ```bash
   rustc --version
   cargo --version
   ```

#### Alternative Installation Methods

**Package Managers (Not Recommended):**
- **Windows**: `winget install Rustlang.Rust.MSVC` or `scoop install rust`
- **macOS**: `brew install rust`
- **Linux**: `sudo apt install rustc cargo` (Ubuntu/Debian) or `sudo pacman -S rust` (Arch)

**Note**: Package manager versions may be outdated. Rustup is preferred for latest versions and easy updates.

#### Updating Rust
```bash
rustup update
```

#### Troubleshooting
- **Windows**: Ensure you have Visual Studio Build Tools or Visual Studio Community installed
- **All platforms**: If `cargo` command not found, restart your terminal or add `~/.cargo/bin` to your PATH
- **Permission issues**: Don't use `sudo` with rustup installations

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

1. Install [Windows Terminal](https://aka.ms/terminal) from Microsoft Store (recommended)
2. Open Windows Terminal, PowerShell, or Command Prompt
3. Run the installation commands:
   ```bash
   git clone https://github.com/yourusername/echoes_rpg.git
   cd echoes_rpg
   cargo run --release
   ```

**Note**: For Command Prompt on older Windows versions, enable ANSI support:
```cmd
reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
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

1. Install Rust if not already installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```
   
   **Arch Linux**: `sudo pacman -S rustup && rustup default stable`

2. Clone and build:
   ```bash
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

### Cross-Platform Building
```bash
# Development build (faster compilation)
cargo run

# Release build (optimized performance)
cargo run --release

# GUI version (Windows only)
cargo build --features gui --release
```

### Platform-Specific Builds

#### Windows
```bash
# Terminal version (MSVC)
cargo build --release --target x86_64-pc-windows-msvc

# Terminal version (GNU)
cargo build --release --target x86_64-pc-windows-gnu

# GUI version (recommended for Windows)
cargo build --features gui --release --target x86_64-pc-windows-msvc
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
1. Use Windows Terminal instead of Command Prompt
2. Enable ANSI support: `reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1`
3. Update to Windows 10 version 1511 or later

**Problem**: Game crashes on startup
**Solution**:
1. Run as administrator
2. Check Windows Defender exclusions
3. Try Windows Terminal or PowerShell instead of Command Prompt
4. Ensure terminal supports ANSI escape sequences

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
4. Verify internet connection for dependency downloads

**Problem**: Performance issues
**Solution**:
1. Use release build: `cargo run --release`
2. Increase terminal buffer size
3. Close other applications using terminal resources
4. Try a different terminal if issues persist

## 💾 Save Files

Game progress is saved automatically in platform-specific locations:

- **Windows**: `%APPDATA%\EchoesRPG\`
- **macOS**: `~/Library/Application Support/EchoesRPG/`
- **Linux**: `~/.local/share/echoes_rpg/`

## 🎨 Terminal Compatibility

### Recommended Terminals

#### Windows
- ✅ **Windows Terminal** - Best experience, full color support
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

## 🔗 Dependencies

The game uses these cross-platform Rust crates:

### Core Dependencies
- `crossterm` - Cross-platform terminal manipulation
- `rand` - Random number generation
- `serde` - Serialization/deserialization
- `dirs` - Cross-platform directory detection
- `anyhow` - Error handling
- `atty` - TTY detection

### GUI Dependencies (Windows)
- `eframe` - Cross-platform GUI framework
- `egui` - Immediate mode GUI library
- `egui_extras` - Additional GUI components

## 🎯 Recent Improvements

### Input Handling Enhancements (v0.1.0)
- **Fixed Triple Input Processing**: Resolved issue where key presses registered multiple times
- **Clean Character Creation**: Eliminated residual '1' character appearing in player names
- **Centralized Input System**: Implemented robust input handler with frame-based duplicate prevention
- **Enhanced GUI Experience**: Windows GUI version now provides smooth, predictable input behavior

For technical details, see [implementation documentation](implementation_readme/).

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

Development guidelines:
- Test on multiple platforms when possible
- Follow Rust best practices and conventions
- Maintain cross-platform compatibility
- Document any significant changes

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

- [Issue Tracker](https://github.com/yourusername/echoes_rpg/issues)
- [Rust Installation](https://rustup.rs/)
- [Implementation Documentation](implementation_readme/)

---

*Adventure awaits in the Echoes of the Forgotten Realm! 🗡️⚔️*

*Choose your interface and start your journey - whether in terminal or GUI! 🎮*