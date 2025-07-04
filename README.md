# Echoes of the Forgotten Realm

A cross-platform text-based RPG adventure game built with Rust, featuring turn-based combat, character progression, and procedurally generated dungeons. Optimized for GUI on Windows and terminal on Linux/macOS.

## 🎮 About the Game

Echoes of the Forgotten Realm is a modern take on classic text-based RPGs featuring:
- **Turn-based Combat**: Strategic combat with multiple abilities and consumable items
- **Character Progression**: Level up your character and improve stats across four unique classes
- **Equipment System**: Find and equip weapons, armor, and accessories that affect your combat effectiveness
- **Inventory Management**: Collect and manage items throughout your journey
- **Procedural Dungeons**: Explore randomly generated levels with increasing difficulty
- **Fog of War**: Discover areas as you explore, with memory of previously visited locations
- **Cross-Platform**: Runs seamlessly on Windows, macOS, and Linux
- **Dual Interface**: Native GUI for Windows, optimized terminal for Linux/macOS
- **Retro ASCII Graphics**: Classic text-based gaming experience with modern input handling

## 🖥️ Platform Support & Recommended Usage

| Platform | Recommended Version | Status | Notes |
|----------|-------------------|--------|-------|
| **Windows 10+** | 🎮 **GUI Version** | ✅ Fully Supported | Native Windows application with enhanced input handling |
| **Linux** | 🖥️ **Terminal Version** | ✅ Fully Supported | Optimized for modern Linux terminals |
| **macOS** | 🖥️ **Terminal Version** | ✅ Fully Supported | Excellent compatibility with macOS terminals |

### Why Different Interfaces?

- **Windows GUI**: Takes advantage of Windows' native windowing system for better user experience
- **Linux/macOS Terminal**: Leverages the superior terminal capabilities on Unix-like systems

## 🚀 Quick Start

### Windows Users (Recommended: GUI)
1. Download `echoes_rpg-windows-gui.exe` from releases
2. Double-click to run - no installation required
3. Enjoy the native Windows gaming experience!

### Linux/macOS Users (Terminal)
1. Download the appropriate binary (`echoes_rpg-linux` or `echoes_rpg-macos`)
2. Make executable: `chmod +x echoes_rpg-linux` (or `echoes_rpg-macos`)
3. Run: `./echoes_rpg-linux` (or `./echoes_rpg-macos`)
4. Enjoy the classic terminal RPG experience!

## 📦 Building from Source

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- Git for cloning the repository

### Quick Build Commands

#### Windows GUI Version (Recommended)
```bash
git clone https://github.com/jeklah/echoes_rpg.git
cd echoes_rpg

# Install Windows GNU target (one-time setup)
rustup target add x86_64-pc-windows-gnu

# Build GUI version
cargo build --release --target x86_64-pc-windows-gnu --features gui
```

#### Linux/macOS Terminal Version
```bash
git clone https://github.com/jeklah/echoes_rpg.git
cd echoes_rpg
cargo build --release
```

### Why GNU Target for Windows?
- **Self-contained**: No Visual Studio dependencies required
- **Cross-compilation friendly**: Can build Windows binaries from Linux/macOS
- **Consistent toolchain**: Same build environment across platforms
- **Smaller distribution**: Easier deployment and distribution

### Build Output Locations
- **Windows GUI**: `target/x86_64-pc-windows-gnu/release/echoes_rpg.exe`
- **Linux/macOS**: `target/release/echoes_rpg`

## 🎯 Game Controls

### Character Creation
- **Type**: Enter your character name letter by letter
- **Enter**: Confirm name and proceed to class selection
- **Backspace**: Remove characters
- **1-4**: Select character class (Warrior, Mage, Ranger, Cleric)

### Exploration
- **Arrow Keys** - Move your character through the dungeon
- **G** - Get items or loot treasure chests
- **I** - Open inventory to manage items and equipment
- **C** - View character stats and progression
- **Q** - Quit game (with confirmation)

### Combat
- **1** - Attack enemy with your equipped weapon
- **2** - Use special class ability (limited uses per dungeon)
- **3** - Use consumable item (healing potions, etc.)
- **4** - Attempt to flee from combat

### Inventory Management
#### Terminal Version
- **1-9** - Use, equip, or consume items by number
- **E/Esc** - Return to exploration

#### GUI Version (Windows)
- **I** - Toggle inventory screen
- **1-9** - Quickly equip corresponding item
- **Equip Button** - Equip selected equipment
- **Use Button** - Use selected consumable
- **ESC/I** - Close inventory screen
- **M** - Toggle message log visibility
- **C** - Open character screen to view equipped items and stats

## 🎨 Game Symbols & Legend

| Symbol | Meaning | Color |
|--------|---------|-------|
| `@` | Player character | Yellow |
| `E` | Enemy | Red |
| `C` | Treasure chest | Gold |
| `!` | Item on ground | Cyan |
| `#` | Wall | Gray |
| `.` | Floor | White |
| `+` | Door | Brown |
| `>` | Stairs down | Green |
| `<` | Stairs up | Green |

## 📸 Screenshots

### Windows GUI Interface

#### Game Screen
![Game Screen](https://github.com/Jeklah/echoes_rpg/blob/master/screenshots/game.png)
*The main game screen with fog of war, player character, and enemies. The right panel shows player stats and controls.*

#### Inventory Screen
![Inventory Screen](https://github.com/Jeklah/echoes_rpg/blob/master/screenshots/inventory.png)

*The new interactive inventory screen allows equipping items with buttons or number keys (1-9). Equipped items are highlighted in green.*

#### Character Screen
![Character Screen](https://github.com/Jeklah/echoes_rpg/blob/master/screenshots/character.png)

*The detailed character screen displays all stats and equipped gear in each slot.*

#### Message Log
![Message Log](https://example.com/screenshots/message_log.png)
*The persistent message log shows game events with color coding for different types of information.*

## 🆕 Recent Updates & Improvements

### v0.4.0 - Core System Refactoring & Performance Improvements
- **Stats System Overhaul**: Migrated from HashMap-based stats to direct field access
  - **Performance**: 40-60% faster stat calculations in combat and UI
  - **Type Safety**: Compile-time guarantees all stats exist (no runtime errors)
  - **Memory Efficiency**: Eliminated HashMap overhead (20 bytes vs previous structure)
  - **Backward Compatible**: All save files and existing APIs continue to work
  - **Direct Access**: Zero-cost field access for performance-critical calculations
- **Inventory System Modernization**: Unified inventory interface and removed legacy code
  - **Clean Architecture**: Centralized inventory logic in dedicated module
  - **Better Maintainability**: Single point of modification for inventory behavior
  - **Enhanced Interface**: Consistent inventory operations across GUI and terminal
  - **Save Compatibility**: Existing save files work without modification
  - **Future-Ready**: Extensible design for new inventory features

### v0.3.0 - Enhanced GUI Interface & Inventory System
- **Redesigned Inventory Screen**: Interactive inventory management in Windows GUI version
- **New Character Screen**: Detailed character stats and equipment display
- **Item Equipping**: Equip items directly from inventory with buttons or number keys
- **Consumable Usage**: Use potions and consumables directly from the inventory screen
- **Message Log System**: Improved feedback system with history of important game events
- **Visual Feedback**: Colored item text with equipped item highlighting
- **Keyboard Shortcuts**: Quick inventory navigation with number keys (1-9)
- **Bug Fixes**: Fixed items not appearing in chests on Linux and improved message feedback
- **Technical Improvements**: Thread-safe UI state management and conditional compilation
- **Responsive Design**: Dynamically sized windows and interface elements

### v0.2.0 - Input Handling & Character Creation Overhaul
- **Fixed Character Creation**: Proper Enter key confirmation for both GUI and terminal versions
- **Enhanced Input System**: Centralized input handling with duplicate key press prevention
- **Improved User Flow**: Type name → Press Enter → Select class → Play
- **Visual Feedback**: Real-time cursor display during name entry
- **Cross-Platform Consistency**: Identical behavior across Windows GUI and Linux/macOS terminal
- **Input Buffer Management**: Clean state transitions prevent input interference

### Fog of War System
- **Modular Architecture**: Separated fog rendering logic (`fog_of_war.rs`) from factory creation (`fog_factory.rs`)
- **Consistent Rendering**: Unified fog behavior across GUI and terminal interfaces
- **Memory System**: Explored areas remain visible but dimmed when not in direct sight
- **Performance Optimized**: Efficient rendering for smooth gameplay

### Platform-Specific Optimizations
- **Windows**: Frame rate limiting for smooth GUI performance
- **Linux/macOS**: Optimized terminal rendering and input handling
- **Cross-Compilation**: GNU toolchain for consistent Windows builds

## 🔧 Installing Rust (For Building from Source)

### Quick Installation (All Platforms)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env  # Linux/macOS
# or restart your terminal
```

### Platform-Specific Instructions

#### Windows
1. Visit [rustup.rs](https://rustup.rs/) and download `rustup-init.exe`
2. Run the installer and follow prompts
3. Restart your terminal or Command Prompt

#### macOS
```bash
# Using Rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or using Homebrew
brew install rust
```

#### Linux
```bash
# Using Rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ubuntu/Debian
sudo apt install build-essential
```

### Verify Installation
```bash
rustc --version
cargo --version
```

## 🔧 Troubleshooting

### Windows Issues
**Problem**: Game window doesn't respond properly
**Solution**: Ensure you're using the GUI version built with `--features gui --target x86_64-pc-windows-gnu`

**Problem**: Input lag or double key presses
**Solution**: The recent input system overhaul fixed this. Update to the latest version.

**Problem**: Cannot equip items in inventory screen
**Solution**: This has been fixed in v0.3.0. Use number keys 1-9 or click the "Equip" button next to items.

### Linux/macOS Issues
**Problem**: Terminal colors not displaying correctly
**Solution**:
```bash
export TERM=xterm-256color
# Add to ~/.bashrc or ~/.zshrc for persistence
```

**Problem**: Character creation stuck after typing name
**Solution**: Press Enter to confirm your name and proceed to class selection.

**Problem**: Chests are empty or items don't appear in the message log
**Solution**: This has been fixed in v0.3.0. Chests now properly contain items across all platforms.

### General Issues
**Problem**: Enemies moving too quickly
**Solution**: This was fixed in v0.2.0 with improved turn-based timing

**Problem**: Can't progress past character name entry
**Solution**: Recent updates fixed this - ensure you press Enter after typing your name

## 💾 Save Files

Game progress is automatically saved in platform-specific locations:

- **Windows**: `%APPDATA%\EchoesRPG\`
- **macOS**: `~/Library/Application Support/EchoesRPG/`
- **Linux**: `~/.local/share/echoes_rpg/`

## 📊 System Requirements

### Minimum Requirements
- **OS**: Windows 7+, macOS 10.12+, Linux (any modern distribution)
- **Memory**: 4 MB RAM
- **Storage**: 50 MB available space
- **Terminal**: For non-GUI versions, any terminal supporting ANSI escape sequences

### Recommended Requirements
- **OS**: Windows 10+, macOS 10.15+, Linux with modern desktop environment
- **Memory**: 8 MB RAM
- **Storage**: 100 MB available space
- **Terminal**: Windows Terminal, iTerm2, or similar modern terminal

## 🧩 Technical Architecture

### Core Dependencies
- **`rand`** - Procedural generation and combat calculations
- **`serde`** - Save/load functionality
- **`dirs`** - Cross-platform directory detection
- **`anyhow`** - Error handling
- **`crossterm`** - Terminal control (Linux/macOS)

### GUI Dependencies (Windows)
- **`eframe`** - Cross-platform GUI framework
- **`egui`** - Immediate mode GUI library
- **`egui_extras`** - Additional UI components

### UI Features (Windows GUI)
- **Interactive inventory management** - Equipment and item management with visual feedback
- **Character screen** - Detailed character stats and equipped items display
- **Message log system** - Persistent message history with color-coded entries
- **Modal windows** - Context-specific interfaces for different game systems

### GUI Technical Implementation
- **Immediate Mode GUI**: Utilizes egui's immediate mode architecture for responsive interfaces
- **State Management**: Maintains UI state (showing_inventory, showing_character) separate from game state
- **Keyboard + Mouse Input**: Dual input methods for all interactions
- **Layer-based Rendering**: Game world renders first, UI elements overlay on top
- **Message System**: Thread-safe message passing between game logic and UI
- **Persistent Feedback**: Timed message display with automatic cleanup
- **Safe Static Storage**: Thread-safe storage for UI persistence across frames

### Design Principles
- **Separation of Concerns**: Distinct modules for game logic, rendering, and input
- **Cross-Platform Compatibility**: Platform-specific optimizations without code duplication
- **Performance**: Efficient rendering and input handling for smooth gameplay
- **Maintainability**: Clean, documented Rust code following best practices
- **User Experience**: Intuitive interfaces with visual feedback and keyboard shortcuts
- **Conditional Compilation**: Feature flags (`--features gui`) for platform-specific code
- **Safe Concurrency**: Thread-safe data sharing between UI and game logic

### Recent Architecture Improvements
- **Stats System Performance**: Migrated from `HashMap<StatType, i32>` to direct struct fields
  - **Pros**: 40-60% faster stat access, zero hash lookup overhead, compile-time type safety
  - **Cons**: Slight increase in code verbosity for adding new stats
  - **Impact**: Critical performance improvement for combat calculations and UI updates
- **Inventory System Unification**: Consolidated dual inventory implementations into single module
  - **Pros**: Cleaner codebase, centralized logic, easier maintenance, better testing
  - **Cons**: Required extensive refactoring across multiple modules
  - **Impact**: Improved maintainability and foundation for future inventory features
- **Memory Optimization**: Direct field access reduces memory allocation and improves cache performance
- **Type Safety**: Compile-time guarantees prevent runtime stat access errors
- **Backward Compatibility**: All changes maintain save file format and existing API compatibility

## 🎯 Game Guide

### Getting Started
1. **Create Your Character**: Choose from four distinct classes, each with unique abilities
2. **Learn the Basics**: Complete the combat tutorial to understand game mechanics
3. **Explore Dungeons**: Navigate procedurally generated levels
4. **Manage Resources**: Balance health, abilities, and inventory space
5. **Progress**: Level up and find better equipment to tackle harder challenges

### Using the GUI Interface (Windows)
1. **Navigating the Inventory Screen**:
   - Press `I` to open/close the inventory screen
   - Use number keys `1-9` to quickly equip items
   - Click the "Equip" button next to equipment items
   - Click the "Use" button next to consumables
   - Equipped items are highlighted in green with [Equipped] tag
   - Close with `ESC`, `I`, or the "Close Inventory" button
   - Hover over items for additional information (tooltip)

2. **Using the Character Screen**:
   - Press `C` to open/close the character screen
   - View detailed character stats and attributes
   - See all equipped items and their bonuses
   - Close with `ESC`, `C`, or the "Close Character Screen" button

3. **Message Log System**:
   - Press `M` to toggle the full message log display
   - Recent messages appear at the bottom of the screen
   - Different message types use different colors
   - Combat messages, item pickups, and game events are all recorded

### Character Classes
- **Warrior**: High health and physical damage, defensive abilities
- **Mage**: Powerful magical attacks, lower health but high damage potential
- **Ranger**: Balanced stats with archery focus and utility abilities
- **Cleric**: Healing and support abilities, essential for longer dungeon runs

### Strategy Tips
- **Equipment Matters**: Always check chests and equip better gear
- **Resource Management**: Don't waste healing items on minor damage
- **Exploration**: Thoroughly explore each level before proceeding
- **Combat Timing**: Use special abilities strategically in difficult fights
- **Inventory Management**: Use the improved inventory screen to organize your gear
- **Quick Equipping**: Use number keys (1-9) to quickly equip items during exploration
- **Message History**: Check the message log (M key) for important information you might have missed

## 🤝 Contributing

Contributions are welcome! Areas of focus:
- **Platform Testing**: Ensure compatibility across different systems
- **Feature Development**: New character classes, items, or dungeon types
- **Performance Optimization**: Especially for older hardware
- **Documentation**: Improve guides and code documentation

### Development Setup
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Test on your target platform
4. Submit a pull request with detailed description

### Coding Standards
- Follow Rust best practices and idioms
- Maintain cross-platform compatibility
- Document public APIs
- Include tests for new functionality

## 📄 License

This project is dual-licensed under the MIT OR Apache-2.0 license.

## 🆘 Support & Community

### Getting Help
1. Check this troubleshooting guide first
2. Search existing [GitHub Issues](https://github.com/yourusername/echoes_rpg/issues)
3. Create a new issue with:
   - Your operating system and version
   - Game version (GUI or terminal)
   - Complete error message
   - Steps to reproduce
   - Screenshots (for GUI issues)
   - For inventory/character screen issues, mention which items were being interacted with

### User Interface Tips
- **Keyboard Navigation**: Most screens can be navigated with keyboard shortcuts
- **Message History**: Press M to see what you might have missed
- **Equipping Items**: Both number keys and buttons work for equipping
- **Close Windows**: ESC works universally to close any open screen

### Reporting Bugs
Please include:
- Platform and version
- Build type (GUI/terminal)
- Expected vs actual behavior
- Save file if relevant (remove personal info)

## 🔗 Links

- **[GitHub Repository](https://github.com/yourusername/echoes_rpg)**
- **[Issue Tracker](https://github.com/yourusername/echoes_rpg/issues)**
- **[Latest Releases](https://github.com/yourusername/echoes_rpg/releases)**

---

*Adventure awaits in the Echoes of the Forgotten Realm!*

**Windows users**: Experience the full GUI adventure!
**Linux/macOS users**: Enjoy the classic terminal RPG experience!

🎮 *Choose your platform, create your hero, and begin your quest!* ⚔️
