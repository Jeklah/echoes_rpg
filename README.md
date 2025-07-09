# Echoes of the Forgotten Realm

A cross-platform text-based RPG adventure game built with Rust, featuring turn-based combat, character progression, and procedurally generated dungeons.

## 🎮 Quick Start

### Windows (Recommended: GUI)
1. Download `echoes_rpg-windows-gui.exe` from releases
2. Double-click to run - no installation required

### Linux/macOS (Terminal)
1. Download the appropriate binary
2. Make executable: `chmod +x echoes_rpg-linux`
3. Run: `./echoes_rpg-linux`

## 🎯 Game Features

- **Turn-based Combat** with strategic abilities and items
- **Character Progression** across 4 unique classes (Warrior, Mage, Ranger, Cleric)
- **Equipment System** with weapons, armor, and accessories
- **Procedural Dungeons** with increasing difficulty
- **Fog of War** exploration system
- **Cross-Platform** with optimized interfaces for each platform

## 🖥️ Platform Support

| Platform | Interface | Status |
|----------|-----------|--------|
| **Windows 10+** | 🎮 GUI | ✅ Fully Supported |
| **Linux** | 🖥️ Terminal | ✅ Fully Supported |
| **macOS** | 🖥️ Terminal | ✅ Fully Supported |

## 🎮 Controls

### Exploration
- **Arrow Keys** - Move character
- **G** - Get items/loot chests
- **I** - Open inventory
- **C** - View character stats
- **Q** - Quit game

### Combat
- **1** - Attack
- **2** - Use ability
- **3** - Use item
- **4** - Flee

### Inventory (GUI)
- **1-9** - Quick equip items
- **Equip/Use Buttons** - Interact with items
- **M** - Toggle message log
- **ESC** - Close screens

## 🎨 Game Symbols

| Symbol | Meaning | Symbol | Meaning |
|--------|---------|--------|---------|
| `@` | Player | `E` | Enemy |
| `C` | Chest | `!` | Item |
| `#` | Wall | `.` | Floor |
| `>` | Stairs | `+` | Door |
| `<` | Stairs up | `E` | Exit (green) |

## 🆕 Recent Updates

### v0.4.0 - Core System Refactoring
- **40-60% faster performance** with stats system overhaul
- **Type-safe architecture** with compile-time guarantees
- **Unified inventory system** for better maintainability
- **Memory optimization** with direct field access
- **Backward compatible** with existing save files

### v0.3.0 - Enhanced GUI Interface
- **Interactive inventory management** with buttons and shortcuts
- **New character screen** showing detailed stats and equipment
- **Message log system** with persistent game event history
- **Visual feedback** with equipped item highlighting
- **Cross-platform bug fixes** for item generation

### Key Improvements
- **Input System**: Fixed character creation and eliminated duplicate key presses
- **Fog of War**: Modular architecture with consistent rendering
- **Platform Optimization**: Windows frame limiting, Linux/macOS terminal optimization

## 🔧 Troubleshooting

### Common Issues
- **Terminal colors**: Set `export TERM=xterm-256color`
- **Character creation**: Press **Enter** to confirm name
- **Input lag**: Fixed in latest version - update if experiencing issues

### Save Files
- **Windows**: `%APPDATA%\EchoesRPG\`
- **macOS**: `~/Library/Application Support/EchoesRPG/`
- **Linux**: `~/.local/share/echoes_rpg/`

## 🧩 Technical Stack

### Core
- **Rust** - Performance and safety
- **Serde** - Save/load system
- **Crossterm** - Terminal interface

### GUI (Windows)
- **eframe/egui** - Native GUI framework
- **Immediate mode** - Responsive interface

### Architecture
- **Modular design** with separated concerns
- **Cross-platform compatibility** without code duplication
- **Performance optimized** for smooth gameplay
- **Type-safe** with compile-time guarantees

## 🎯 Strategy Tips

- **Equipment matters** - Always check chests and upgrade gear
- **Resource management** - Save healing items for tough fights
- **Explore thoroughly** - Don't rush to the next level
- **Use abilities strategically** - They're limited per dungeon
- **GUI shortcuts** - Use number keys (1-9) for quick item access

## 🤝 Contributing

Contributions welcome! Focus areas:
- Platform testing and compatibility
- Feature development (classes, items, dungeons)
- Performance optimization
- Documentation improvements

## 📄 License

MIT OR Apache-2.0

## 🔗 Links

- **[GitHub Repository](https://github.com/yourusername/echoes_rpg)**
- **[Latest Releases](https://github.com/yourusername/echoes_rpg/releases)**
- **[Issue Tracker](https://github.com/yourusername/echoes_rpg/issues)**

---

**System Requirements**: Windows 7+, macOS 10.12+, Linux (modern distribution) • 4MB RAM • 50MB storage