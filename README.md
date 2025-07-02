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

### GUI Version (Windows)
- Native Windows application using GNU toolchain
- Improved input handling
- Better visual integration
- Self-contained executable (no Visual Studio dependencies)

## 🖥️ Platform Support

| Platform | GUI Version | Best Terminal | Notes |
|----------|-------------|---------------|-------|
| **Windows 10+** | ✅ Available | Windows Terminal | GUI version with enhanced input handling |
| **Windows 8.1/7** | ✅ Available | Command Prompt | GUI recommended for older Windows |
| **macOS** | ❌ Not Available | iTerm2, Terminal.app | Excellent terminal compatibility |
| **Linux** | ❌ Not Available | Most modern terminals | Wide terminal compatibility |

## 🚀 Quick Start

Download the appropriate version for your platform from the releases page and run the executable.

## 📦 Installation

Download the appropriate version for your platform from the [releases page](https://github.com/yourusername/echoes_rpg/releases):

### Windows (GUI Version - Recommended)
- Download `echoes_rpg-windows-gui.exe` (compiled with GNU target)
- Run the executable directly - no additional dependencies required

### Windows (Terminal Version)
- Download `echoes_rpg-windows-terminal.exe`
- Run from Command Prompt or Windows Terminal

### macOS
- Download `echoes_rpg-macos`
- Make executable: `chmod +x echoes_rpg-macos`
- Run: `./echoes_rpg-macos`

### Linux
- Download `echoes_rpg-linux`
- Make executable: `chmod +x echoes_rpg-linux`
- Run: `./echoes_rpg-linux`

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



## 🔧 Troubleshooting

### Windows Issues

**Problem**: Game crashes on startup
**Solution**:
1. Run as administrator
2. Check Windows Defender exclusions
3. Ensure you have the GUI version: `cargo build --features gui --release`

### General Issues

**Problem**: Performance issues
**Solution**:
1. Close other applications
2. Ensure sufficient system resources
3. Try running from a terminal if using the GUI becomes unresponsive

## 💾 Save Files

Game progress is saved automatically in platform-specific locations:

- **Windows**: `%APPDATA%\EchoesRPG\`
- **macOS**: `~/Library/Application Support/EchoesRPG/`
- **Linux**: `~/.local/share/echoes_rpg/`



## 🛠️ Building from Source

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)

### Quick Build Commands

#### Linux/macOS Terminal Version
```bash
cargo build --release
```

#### Windows GUI Version (GNU Target - Recommended)
```bash
# Install target (one-time setup)
rustup target add x86_64-pc-windows-gnu

# Build GUI version
cargo build --release --target x86_64-pc-windows-gnu --features gui
```

### Why GNU Target for Windows?
This project uses the `x86_64-pc-windows-gnu` target for Windows builds because:
- **Self-contained**: No Visual Studio dependencies required
- **Cross-compilation friendly**: Can build Windows binaries from Linux/macOS
- **Smaller toolchain**: Easier to set up in CI/CD environments
- **Consistent experience**: Same GNU toolchain across all platforms

### Build Output Locations
- **Windows GUI**: `target/x86_64-pc-windows-gnu/release/echoes_rpg.exe`
- **Linux/macOS**: `target/release/echoes_rpg`

## 📊 System Requirements

### Minimum Requirements
- **OS**: Windows 7, macOS 10.12, Linux (any modern distribution)
- **Memory**: 4 MB RAM
- **Storage**: 50 MB available space

### Recommended Requirements
- **OS**: Windows 10+, macOS 10.15+, Linux with modern desktop environment
- **Memory**: 8 MB RAM
- **Storage**: 100 MB available space



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
   - Game version
   - Complete error message
   - Steps to reproduce

## 🔗 Links

- [Issue Tracker](https://github.com/yourusername/echoes_rpg/issues)

- [Implementation Documentation](implementation_readme/)

---

*Adventure awaits in the Echoes of the Forgotten Realm! 🗡️⚔️*

*Start your GUI adventure today! 🎮*